
// change from 30 to actual memleak value later
pub const PROCESS_CPU_USAGE_THRESHHOLD: f32 = 30f32;

// in order: normal process, arch binary, canary client, vesktop (vencord)
pub const DISCORD_PROCESS_NAMES: [&str; 4] = ["discord", "discord-bin", "discordcanary", "vesktop"];

pub const NOTIF_TITLE: &str = "Discord Memory Leak";
pub const NOTIF_BODY: &str = "Your Discord app is experiencing a memory leak. If you are currently streaming, try restarting it";