#![windows_subsystem = "windows"]

use std::{thread::{self, sleep}, time::Duration};
use tao::{
    event::Event,
    event_loop::{ControlFlow, EventLoopBuilder},
};
use tray_icon::{
    menu::{
        AboutMetadata, Menu, MenuEvent, 
        MenuItem, PredefinedMenuItem
    }, 
    TrayIcon, TrayIconBuilder, Icon
};

enum UserEvent {
    MenuEvent(tray_icon::menu::MenuEvent),
}

fn check_connection(
    mut tray_icon: TrayIcon,
    connected_icon: Icon,
    disconnected_icon: Icon
) {    
    thread::spawn(move || {
        loop {
            let res = reqwest::blocking::get("https://google.com");
            match res {
                Ok(_)=> {
                    tray_icon.set_icon(Some(connected_icon.clone())).unwrap();
                }
                Err(_err) => {
                    tray_icon.set_icon(Some(disconnected_icon.clone())).unwrap();
                }
            }
            sleep(Duration::from_secs(20));
        }
    });
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
    let connected_icon_path = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/connected_icon.png");
    let connected_icon = load_icon(std::path::Path::new(connected_icon_path));

    let disconnected_icon_path = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/disconnected_icon.png");
    let disconnected_icon = load_icon(std::path::Path::new(disconnected_icon_path));

    let event_loop = EventLoopBuilder::<UserEvent>::with_user_event().build();

    let proxy = event_loop.create_proxy();
    MenuEvent::set_event_handler(Some(move |evnet| {
        let _ = proxy.send_event(UserEvent::MenuEvent(evnet));
    }));

    let tray_menu = Menu::new();
    let exit_item = MenuItem::new("Exit", true, None);
    let _ = tray_menu.append_items(&[
        &PredefinedMenuItem::about(
            None, 
            Some(AboutMetadata {
                name: Some("Loop".to_string()),
                ..Default::default()
            })
        ),
        &PredefinedMenuItem::separator(),
        &exit_item
    ]);

    let mut tray_icon: Option<TrayIcon> = None;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::NewEvents(tao::event::StartCause::Init) => {
                tray_icon = Some(
                    TrayIconBuilder::new()
                        .with_menu(Box::new(tray_menu.clone()))
                        .with_icon(connected_icon.clone())
                        .build()
                        .unwrap()
                );

                check_connection(
                    tray_icon.as_ref().unwrap().clone(), 
                    connected_icon.clone(), 
                    disconnected_icon.clone()
                );
            }
            Event::UserEvent(UserEvent::MenuEvent(event)) => {
                if event.id == exit_item.id() {
                    tray_icon.take();
                    *control_flow = ControlFlow::Exit;
                }
            }
            _ => {}
        }
    })
}