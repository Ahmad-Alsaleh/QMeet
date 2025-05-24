use enigo::{Enigo, Keyboard as _};
use std::error::Error;
use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
    // ActivationPolicy,
};
use tauri_plugin_autostart::{MacosLauncher, ManagerExt};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};

pub fn run() {
    let target_shortcut = Shortcut::new(Some(Modifiers::CONTROL | Modifiers::ALT), Code::KeyP);
    let meeting_url = generate_meeting_url();

    tauri::Builder::default()
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            None,
        ))
        .plugin(tauri_plugin_process::init())
        .setup(move |app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            // FIXME: since this is used now, ig no need for the html :)
            // or you know what, maybe find a better tool to create
            // trays/system-trays using rust.
            // #[cfg(target_os = "macos")]
            // app.set_activation_policy(ActivationPolicy::Accessory);

            app.autolaunch().enable().unwrap();

            register_shortcut_listener(app, target_shortcut, meeting_url)?;

            let quit_menu_item = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&quit_menu_item])?;
            let icon = app.default_window_icon().unwrap().clone();
            TrayIconBuilder::new()
                .icon(icon)
                .menu(&menu)
                .show_menu_on_left_click(true)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "quit" => app.exit(0),
                    _ => unreachable!(),
                })
                .build(app)?;

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![my_custom_command])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn register_shortcut_listener(
    app: &mut tauri::App,
    target_shortcut: Shortcut,
    meeting_url: String,
) -> Result<(), Box<dyn Error>> {
    app.handle().plugin(
        tauri_plugin_global_shortcut::Builder::new()
            .with_handler(move |_app, shortcut, event| {
                if shortcut == &target_shortcut && matches!(event.state(), ShortcutState::Pressed) {
                    println!("Shortcut Pressed");
                    Enigo::new(&enigo::Settings::default())
                        .unwrap()
                        .text(&meeting_url)
                        .expect("Couldn't paste meeting URL");
                }
            })
            .build(),
    )?;
    app.global_shortcut().register(target_shortcut)?;
    Ok(())
}

pub fn generate_meeting_url() -> String {
    // TODO: this should use Google Meet API
    String::from("meet.google.com/xio-xfkn-wsm")
}

#[tauri::command]
fn my_custom_command() {
    println!("my custom command");
}
