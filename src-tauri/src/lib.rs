use enigo::{Enigo, Keyboard as _};
use std::error::Error;
use std::str::FromStr;
use std::sync::Mutex;
use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
    Manager,
};
use tauri_plugin_autostart::{MacosLauncher, ManagerExt};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};

// a global state to store the current shortcut
pub struct AppState {
    pub target_shortcut: Mutex<Shortcut>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            target_shortcut: Mutex::new(Shortcut::new(
                Some(Modifiers::CONTROL | Modifiers::ALT),
                Code::KeyP,
            )),
        }
    }
}

#[tauri::command]
fn update_target_shortcut(app: tauri::AppHandle, shortcut_str: String) -> Result<(), String> {
    let shortcut = Shortcut::from_str(&shortcut_str)
        .map_err(|e| format!("Failed to parse shortcut: {}", e))?;

    let target_shortcut = &app.state::<AppState>().target_shortcut;

    app.global_shortcut()
        .unregister(*target_shortcut.lock().unwrap())
        .unwrap();
    *target_shortcut.lock().unwrap() = shortcut;
    app.global_shortcut().register(shortcut).unwrap();

    println!("Updated target shortcut to: {}", shortcut_str);

    Ok(())
}

pub fn run() {
    let meeting_url = generate_meeting_url();

    tauri::Builder::default()
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            None,
        ))
        .plugin(tauri_plugin_process::init())
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![update_target_shortcut])
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

            // TODO: remove `meeting_url` and add it to `AppState`
            register_shortcut_listener(app, meeting_url)?;

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
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn register_shortcut_listener(
    app: &mut tauri::App,
    meeting_url: String,
) -> Result<(), Box<dyn Error>> {
    app.handle().plugin(
        tauri_plugin_global_shortcut::Builder::new()
            .with_handler(move |app, shortcut, event| {
                if *shortcut == *app.state::<AppState>().target_shortcut.lock().unwrap()
                    && event.state() == ShortcutState::Pressed
                {
                    println!("Shortcut Pressed");
                    Enigo::new(&enigo::Settings::default())
                        .unwrap()
                        .text(&meeting_url)
                        .expect("Couldn't paste meeting URL");
                }
            })
            .build(),
    )?;
    app.global_shortcut()
        .register(*app.state::<AppState>().target_shortcut.lock().unwrap())?;
    Ok(())
}

pub fn generate_meeting_url() -> String {
    // TODO: this should use Google Meet API
    String::from("meet.google.com/xio-xfkn-wsm")
}
