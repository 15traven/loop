use std::{thread, time::Duration};
use tokio::time::sleep;
use tray_icon::TrayIconBuilder;

async fn check_connection() -> Result<(), reqwest::Error>{
    let res = reqwest::get("https://google.com").await;
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

#[tokio::main]
async fn main() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/icon.png");
    let icon = load_icon(std::path::Path::new(path));

    let tray_icon = TrayIconBuilder::new()
        .with_tooltip("tray")
        .with_icon(icon)
        .build()
        .unwrap();

   loop {
        match check_connection().await {
            Ok(_) => println!("Work"),
            Err(err) => println!("Error")
        }
        sleep(Duration::from_secs(20)).await;
   }
}