#![allow(unused_parens)]
#![allow(clippy::needless_return)]

mod constants;
mod util;

use std::collections::HashMap;
use std::ffi::OsStr;
use std::sync::{Arc, RwLock, RwLockReadGuard};
use std::time::Duration;

use itertools::Itertools;
use sysinfo::{Pid, ProcessRefreshKind, ProcessesToUpdate, System};
use tokio::time::Instant;

use constants::{DISCORD_PROCESS_NAMES, PROCESS_CPU_USAGE_THRESHHOLD};

struct AppState {
    system: Arc<RwLock<System>>,
    cache: Arc<RwLock<Vec<String>>>,
    currently_leaking: HashMap<Pid, Instant>,
}

impl AppState {
    fn new(system: System) -> Self {
        let cache = Arc::new(RwLock::new(Vec::new()));
        let system = Arc::new(RwLock::new(system));
        return Self {
            system,
            cache,
            currently_leaking: HashMap::new(),
        };
    }

    /// iterating over the entire valid process names array is useless since in most cases, only one
    /// instance of discord is running at a time, so the active process(es) are cached instead, ensuring
    /// that only they are checked
    pub fn get_active_discord_processes(sys: RwLockReadGuard<System>) -> Vec<String> {
        let mut active_processes: Vec<String> = Vec::new();

        DISCORD_PROCESS_NAMES.iter().for_each(|name| {
            for x in sys.processes_by_exact_name(OsStr::new(name)) {
                if !x.name().is_empty() {
                    active_processes
                        .push((*name).parse().expect("Expected to parse application name"));
                }
            }
        });

        return active_processes;
    }

    pub fn spawn_cache_updater(&self) {
        let cache = Arc::clone(&self.cache);
        let system = Arc::clone(&self.system);

        tokio::spawn(async move {
            loop {
                tokio::time::sleep(Duration::from_secs(180)).await;
                let sys = system.read().expect("System lock poisoned");
                let mut apps = AppState::get_active_discord_processes(sys);
                let mut cache = cache.write().expect("System lock poisoned");
                cache.clear();
                cache.append(&mut apps);
            }
        });
    }
}

async fn run(sys: System) -> Result<(), Box<dyn std::error::Error>> {
    let mut state = AppState::new(sys);

    // initial cache population so the program doesn't have to wait for the updater
    {
        let sys = state.system.read().expect("System lock poisoned");
        let mut cache = state.cache.write().expect("System lock poisoned");
        cache.append(&mut AppState::get_active_discord_processes(sys));
    }

    state.spawn_cache_updater();

    // sysinfo needs an initial point of comparison, otherwise the first loop call returns nothing
    // which just wastes a cycle
    {
        let mut sys = state.system.write().expect("System lock poisoned");
        sys.refresh_processes_specifics(
            ProcessesToUpdate::All,
            true,
            ProcessRefreshKind::nothing().with_cpu(),
        );
    }

    tokio::time::sleep(Duration::from_millis(500)).await;

    loop {
        {
            let mut sys = state.system.write().expect("System lock poisoned");
            sys.refresh_processes_specifics(
                ProcessesToUpdate::All,
                true,
                ProcessRefreshKind::nothing().with_cpu(),
            );

            let cache = state.cache.read().expect("System lock poisoned");
            cache.iter().unique().for_each(|name| {
                if let Some(proc) = sys
                    .processes_by_name(OsStr::new(name))
                    .find(|x| x.cpu_usage() > PROCESS_CPU_USAGE_THRESHHOLD)
                {
                    if !state.currently_leaking.contains_key(&proc.pid()) {
                        util::send_system_notif();
                    }
                    state.currently_leaking.insert(proc.pid(), Instant::now());
                }
            });
        }

        // this might not be the most elegant solution, but it'll do the trick for now :p
        state.currently_leaking.retain(|_pid, instant| {
            instant.elapsed() <= Duration::from_mins(10)
        });

        tokio::time::sleep(Duration::from_secs(20)).await;
    }
}

#[tokio::main]
async fn main() {
    if !matches!(std::env::consts::OS, "linux" | "macos") {
        eprintln!("Your current OS is not yet supported.");
        std::process::exit(1);
    }

    let sys = System::new_all();
    tokio::time::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL).await;

    run(sys).await.unwrap();
}
