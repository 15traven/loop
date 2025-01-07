use tray_icon::TrayIconBuilder;

fn check_connection() -> Result<(), reqwest::Error>{
    let res = reqwest::blocking::get("https://google.com");
    match res {
        Ok(_) => Ok(()),
        Err(err) => Err(err)
    }
}

fn load_icon(path: &std::path::Path) -> tray_icon::Icon {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open(path)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    tray_icon::Icon::from_rgba(icon_rgba, icon_width, icon_height).expect("Failed to open icon")
}

fn main() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/icon.png");
    let icon = load_icon(std::path::Path::new(path));

    let tray_icon = TrayIconBuilder::new()
        .with_tooltip("tray")
        .with_icon(icon)
        .build()
        .unwrap();

    loop {}
}