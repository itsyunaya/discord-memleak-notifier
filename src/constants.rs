use std::sync::OnceLock;

pub const PROCESS_CPU_USAGE_THRESHHOLD: f32 = 10f32;
pub static CPU_CORES_LOGICAL: OnceLock<f32> = OnceLock::new();

// in order: normal process, arch binary, canary client, vesktop (vencord)
pub const DISCORD_PROCESS_NAMES: [&str; 4] = ["discord", "discord-bin", "discordcanary", "vesktop"];

pub const NOTIF_TITLE: &str = "dmn";
pub const NOTIF_BODY: &str = "Your Discord app is experiencing a resource leak. If you are currently streaming, try restarting it";
