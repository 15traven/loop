use mslnk::ShellLink;

pub fn autorun() {
    let exe_path = std::env::current_exe()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap();

    let username = whoami::username();
    let lnk = format!(
        r"C:\Users\{}\AppData\Roaming\Microsoft\Windows\Start Menu\Programs\Startup\loop.lnk",
        username
    );
    let sl = ShellLink::new(exe_path).unwrap();
    sl.create_lnk(lnk).unwrap()
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