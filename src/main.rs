#![windows_subsystem = "windows"]

use tao::{
    event::Event,
    event_loop::{ControlFlow, EventLoopBuilder},
};
use tray_icon::{
    menu::{
        AboutMetadata, CheckMenuItem, Menu, MenuEvent, MenuItem, PredefinedMenuItem, Submenu
    }, 
    TrayIcon, TrayIconBuilder
};

mod helpers;
mod types;
mod history;
mod gui;
mod autorun;
mod constants;

use helpers::load_icon;

enum UserEvent {
    MenuEvent(tray_icon::menu::MenuEvent),
}

fn main() {
    let connected_icon_path = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/connected_icon.png");
    let connected_icon = load_icon(std::path::Path::new(connected_icon_path));

    let disconnected_icon_path = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/disconnected_icon.png");
    let disconnected_icon = load_icon(std::path::Path::new(disconnected_icon_path));

    let event_loop = EventLoopBuilder::<UserEvent>::with_user_event().build();

    let proxy = event_loop.create_proxy();
    MenuEvent::set_event_handler(Some(move |event| {
        let _ = proxy.send_event(UserEvent::MenuEvent(event));
    }));

    let preferences_submenu = Submenu::new("Preferences", true);
    let autorun_item = CheckMenuItem::new("Run at startup", true, true, None);
    let _ = preferences_submenu.append_items(&[
        &autorun_item
    ]);

    let more_submenu = Submenu::new("More", true);
    let history_item = MenuItem::new("History", true, None);
    let _ = more_submenu.append_items(&[
        &history_item,
        &PredefinedMenuItem::about(
            None, 
            Some(AboutMetadata {
                name: Some("Loop".to_string()),
                ..Default::default()
            })
        )
    ]);

    let tray_menu = Menu::new();
    let quit_item = MenuItem::new("Quit", true, None);
    let _ = tray_menu.append_items(&[
        &preferences_submenu,
        &more_submenu,
        &PredefinedMenuItem::separator(),
        &quit_item
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

                if autorun::register().is_ok() {
                    let is_enabled = autorun::is_enabled();
                    if is_enabled.is_err() {
                        let _ = autorun::enable();
                        autorun_item.set_checked(true);
                    } else {
                        autorun_item.set_checked(is_enabled.unwrap());
                    }
                }

                helpers::check_connection(
                    tray_icon.as_ref().unwrap().clone(), 
                    connected_icon.clone(), 
                    disconnected_icon.clone()
                );
            }
            Event::UserEvent(UserEvent::MenuEvent(event)) => {
                if event.id == history_item.id() {
                    let _ = gui::history_window::show();
                }

                if event.id == autorun_item.id() {
                    let _ = match autorun::is_enabled().unwrap() {
                        true => autorun::disable(),
                        false => autorun::enable()
                    };
                }

                if event.id == quit_item.id() {
                    tray_icon.take();
                    *control_flow = ControlFlow::Exit;
                }
            }
            _ => {}
        }
    })
}