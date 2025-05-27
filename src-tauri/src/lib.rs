use enigo::{Enigo, Keyboard};
use std::str::FromStr;
use std::sync::Mutex;
use std::{error::Error, fs};
use tauri::AppHandle;
use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
    ActivationPolicy, Manager,
};
use tauri_plugin_autostart::{MacosLauncher, ManagerExt};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};

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
fn unregister_target_shortcut(app: AppHandle) {
    app.global_shortcut()
        .unregister(*app.state::<AppState>().target_shortcut.lock().unwrap())
        .unwrap();
}

#[tauri::command]
fn update_target_shortcut(app: AppHandle, shortcut_str: String) -> Result<(), String> {
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

#[tauri::command]
fn quit_app(app: AppHandle) {
    store_persistent_state(&app);
    app.exit(0);
}

pub fn run() {
    let meeting_url = generate_meeting_url();

    tauri::Builder::default()
        .plugin(tauri_plugin_window_state::Builder::new().build())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            None,
        ))
        .plugin(tauri_plugin_process::init())
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            update_target_shortcut,
            unregister_target_shortcut,
            quit_app,
        ])
        .setup(move |app| {
            if cfg!(debug_assertions) {
                enable_debug(app).unwrap();
            }

            load_persistent_state(app.handle());

            // hide the dock icon for macos
            #[cfg(target_os = "macos")]
            app.set_activation_policy(ActivationPolicy::Accessory);

            disable_exit_on_close(app);

            app.autolaunch().enable().unwrap();

            // TODO: remove `meeting_url` and add it to `AppState`
            register_shortcut_listener(app, meeting_url).unwrap();

            build_tray(app);

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn store_persistent_state(app: &AppHandle) {
    let persistent_store_file = app.path().app_data_dir().unwrap().join(".target_shortcut");
    fs::create_dir_all(persistent_store_file.parent().unwrap()).unwrap();

    fs::write(
        &persistent_store_file,
        app.state::<AppState>()
            .target_shortcut
            .lock()
            .unwrap()
            .into_string(),
    )
    .unwrap();
}

fn load_persistent_state(app: &AppHandle) {
    let persistent_store_file = app.path().app_data_dir().unwrap().join(".target_shortcut");
    let target_shortcut = &app.state::<AppState>().target_shortcut;

    if persistent_store_file.exists() {
        let stored_target_shortcut = fs::read_to_string(persistent_store_file).unwrap();
        *target_shortcut.lock().unwrap() = Shortcut::from_str(&stored_target_shortcut).unwrap();
    } else {
        fs::write(
            &persistent_store_file,
            target_shortcut.lock().unwrap().into_string(),
        )
        .unwrap();
    }
}

fn disable_exit_on_close(app: &mut tauri::App) {
    let window = app.get_webview_window("main").unwrap();
    window.clone().on_window_event(move |event| {
        if let tauri::WindowEvent::CloseRequested { api, .. } = event {
            api.prevent_close();
            window.hide().unwrap();
        }
    });
}

fn enable_debug(app: &mut tauri::App) -> Result<(), Box<dyn Error>> {
    app.handle().plugin(
        tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
    )?;
    Ok(())
}

fn build_tray(app: &mut tauri::App) {
    let settings = MenuItem::with_id(app, "settings", "Settings", true, None::<&str>).unwrap();
    let quit = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>).unwrap();
    let menu = Menu::with_items(app, &[&settings, &quit]).unwrap();

    let icon = app.default_window_icon().unwrap().clone();

    TrayIconBuilder::new()
        .icon(icon)
        .menu(&menu)
        .show_menu_on_left_click(true)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "quit" => quit_app(app.clone()),
            "settings" => {
                let window = app.get_webview_window("main").unwrap();
                window.show().unwrap();
                window.set_focus().unwrap();
            }
            _ => unreachable!(),
        })
        .build(app)
        .unwrap();
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
