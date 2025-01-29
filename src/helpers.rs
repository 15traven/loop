use std::path::Path;
use winreg::{
    enums::{
        HKEY_CURRENT_USER, 
        KEY_WRITE
    }, RegKey
};

pub fn autorun() {
    let exe_path = std::env::current_exe()
    .unwrap()
    .into_os_string()
    .into_string()
    .unwrap();

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let path = Path::new("Software")
        .join("Microsoft")
        .join("Windows")
        .join("CurrentVersion")
        .join("Run");

    let key = hkcu.open_subkey_with_flags(path, KEY_WRITE).unwrap();
    let _ = key.set_value("tpass", &exe_path);
}

pub fn load_icon(path: &std::path::Path) -> tray_icon::Icon {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open(path)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    tray_icon::Icon::from_rgba(
        icon_rgba, 
        icon_width, 
        icon_height
    ).expect("Failed to open icon")
}