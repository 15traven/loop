pub const APP_NAME: &str = "loop";

pub const ADMIN_AL_REGKEY: &str = "SOFTWARE\\WOW6432Node\\Microsoft\\Windows\\CurrentVersion\\Run";
pub const ADMIN_TASK_MANAGER_OVERRIDE_REGKEY: &str =
    r"SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\StartupApproved\Run32";
pub const TASK_MANAGER_OVERRIDE_ENABLED_VALUE: [u8; 12] = [
    0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];
pub const TASK_MANAGER_OVERRIDE_DISABLED_VALUE: [u8; 12] = [
    0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];