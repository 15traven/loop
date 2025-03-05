use std::{
    thread::{self, sleep}, 
    time::Duration
};
use mslnk::ShellLink;
use tray_icon::{TrayIcon, Icon};
use history::types::{HistoryRecord, ConnectionStatus};

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

pub fn check_connection(
    mut tray_icon: TrayIcon,
    connected_icon: Icon,
    disconnected_icon: Icon
) {    
    thread::spawn(move || {
        let mut prev_status: ConnectionStatus = ConnectionStatus::Neutral;

        loop {
            let status: bool = match reqwest::blocking::get("https://google.com") {
                Ok(_) => true,
                Err(_err) => {
                    match reqwest::blocking::get("https://github.com") {
                        Ok(_) => true,
                        Err(_err) => false
                    }
                }
            };

            match status {
                true => {
                    tray_icon.set_icon(Some(connected_icon.clone())).unwrap();

                    if prev_status != ConnectionStatus::Online {
                        let _ = history::save(HistoryRecord::online());
                    }

                    prev_status = ConnectionStatus::Online;
                }
                false => {
                    tray_icon.set_icon(Some(disconnected_icon.clone())).unwrap();

                    if prev_status != ConnectionStatus::Offline {
                        let _ = history::save(HistoryRecord::offline());
                    }

                    prev_status = ConnectionStatus::Offline;
                }
            }
            sleep(Duration::from_secs(20));
        }
    });
}