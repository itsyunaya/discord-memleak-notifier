use crate::constants::{NOTIF_BODY, NOTIF_TITLE};

#[cfg(target_os = "linux")]
pub fn notify_linux() {
    use notify_rust::Notification;
    use crate::{};
    Notification::new()
        .summary(NOTIF_TITLE)
        .body(NOTIF_BODY)
        .show().expect("Expected to be able to show notification");
}

#[cfg(target_os = "macos")]
fn notify_macos() {
    use mac_notification_sys::send_notification;
    use crate::{};
    send_notification(
        NOTIF_TITLE,
        None,
        NOTIF_BODY,
        None
    ).expect("Expected to be able to send notification");
}

pub fn send_system_notif() {
    #[cfg(target_os = "linux")]
    notify_linux();

    #[cfg(target_os = "macos")]
    notify_macos();
}