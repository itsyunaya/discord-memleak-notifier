use std::ops::Div;
use sysinfo::Process;

use crate::constants::{CPU_CORES_LOGICAL, NOTIF_BODY, NOTIF_TITLE};

pub fn get_total_cpu_usage(proc: &Process) -> f32 {
    let per_core_usage = proc.cpu_usage();

    return per_core_usage.div(CPU_CORES_LOGICAL.get().unwrap())
}

#[cfg(target_os = "linux")]
pub fn notify_linux() {
    use notify_rust::Notification;
    if let Err(e) = Notification::new()
        .summary(NOTIF_TITLE)
        .body(NOTIF_BODY)
        .show() {
        eprintln!("Failed to show notification: {}", e);
    }
}

#[cfg(target_os = "macos")]
fn notify_macos() {
    use mac_notification_sys::send_notification;
    if let Err(e) = send_notification(
        NOTIF_TITLE,
        None,
        NOTIF_BODY,
        None
    ) {
        eprintln!("Failed to send notification: {}", e);
    }
}

pub fn send_system_notif() {
    #[cfg(target_os = "linux")]
    notify_linux();

    #[cfg(target_os = "macos")]
    notify_macos();
}