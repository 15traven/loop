use std::time::Duration;
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
    let connected_icon_path = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/connected_icon.png");
    let connected_icon = load_icon(std::path::Path::new(connected_icon_path));

    let disconnected_icon_path = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/disconnected_icon.png");
    let disconnected_icon = load_icon(std::path::Path::new(disconnected_icon_path));

    let tray_icon = TrayIconBuilder::new()
        .with_icon(connected_icon.clone())
        .build()
        .unwrap();

   loop {
        match check_connection().await {
            Ok(_) => {
                let _ = tray_icon.set_icon(Some(connected_icon.clone()));
            }
            Err(_err) => {
                let _ = tray_icon.set_icon(Some(disconnected_icon.clone()));
            }
        }
        sleep(Duration::from_secs(20)).await;
   }
}