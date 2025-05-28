use enigo::{Enigo, Keyboard};
use std::fs;
use std::str::FromStr;
use std::sync::Mutex;
use tauri::AppHandle;
use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
    ActivationPolicy, Manager,
};
use tauri_plugin_autostart::{MacosLauncher, ManagerExt};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};

const PERSISTENT_STATE_FILE: &str = ".qmeet_state";

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
fn unregister_target_shortcut(app: AppHandle) -> Result<(), String> {
    let state = app.state::<AppState>();
    let shortcut = *state
        .target_shortcut
        .lock()
        .map_err(|e| format!("Failed to acquire shortcut lock: {e}"))?;

    app.global_shortcut()
        .unregister(shortcut)
        .map_err(|e| format!("Failed to unregister shortcut: {e}"))?;

    Ok(())
}

#[tauri::command]
fn update_target_shortcut(app: AppHandle, new_shortcut_str: String) -> Result<(), String> {
    let new_shortcut = Shortcut::from_str(&new_shortcut_str)
        .map_err(|e| format!("Failed to parse shortcut '{new_shortcut_str}': {e}"))?;

    let state = app.state::<AppState>();

    // unregister old shortcut and register new one
    let mut old_shortcut = state
        .target_shortcut
        .lock()
        .map_err(|e| format!("Failed to acquire shortcut lock: {e}"))?;
    app.global_shortcut()
        .unregister(*old_shortcut)
        .map_err(|e| format!("Failed to unregister old shortcut: {e}"))?;

    *old_shortcut = new_shortcut;

    drop(old_shortcut);

    app.global_shortcut()
        .register(new_shortcut)
        .map_err(|e| format!("Failed to register new shortcut: {e}"))?;

    println!("Updated target shortcut to: {new_shortcut_str}");

    Ok(())
}

#[tauri::command]
fn quit_app(app: AppHandle) {
    if let Err(e) = store_persistent_state(&app) {
        eprintln!("Warning: Failed to store state before quitting: {e}");
    }
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
            // TODO: handle the error of these functions
            load_persistent_state(app.handle())
                .map_err(|e| format!("Failed to load persistent state: {e}"))?;

            // hide the dock icon for macos
            #[cfg(target_os = "macos")]
            app.set_activation_policy(ActivationPolicy::Accessory);

            disable_exit_on_close(app);

            app.autolaunch()
                .enable()
                .map_err(|e| format!("Failed to enable app auto launch: {e}"))?;

            // TODO: remove `meeting_url` and add it to `AppState`
            register_shortcut_listener(app, meeting_url)
                .map_err(|e| format!("Failed registering shortcut listener: {e}"))?;

            build_tray(app).map_err(|e| format!("Failed building app tray: {e}"))?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn store_persistent_state(app: &AppHandle) -> Result<(), String> {
    let app_data_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data directory: {e}"))?;

    fs::create_dir_all(&app_data_dir)
        .map_err(|e| format!("Failed to create app data directory: {e}"))?;

    let persistent_store_file = app_data_dir.join(PERSISTENT_STATE_FILE);

    let state = app.state::<AppState>();
    let shortcut = state
        .target_shortcut
        .lock()
        .map_err(|e| format!("Failed to acquire shortcut lock: {e}"))?;
    fs::write(persistent_store_file, shortcut.into_string())
        .map_err(|e| format!("Failed to write target shortcut: {e}"))?;
    drop(shortcut);

    Ok(())
}

fn load_persistent_state(app: &AppHandle) -> Result<(), String> {
    let persistent_store_file = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data directory: {e}"))?
        .join(PERSISTENT_STATE_FILE);

    if persistent_store_file.exists() {
        let stored_target_shortcut = fs::read_to_string(persistent_store_file)
            .map_err(|e| format!("Failed to read persistent store file: {e}"))?;

        let state = app.state::<AppState>();
        let mut target_shortcut = state
            .target_shortcut
            .lock()
            .map_err(|e| format!("Failed to acquire shortcut lock: {e}"))?;

        *target_shortcut = Shortcut::from_str(&stored_target_shortcut)
            .map_err(|e| format!("Failed to parse shortcut '{stored_target_shortcut}': {e}"))?;
    } else {
        store_persistent_state(app)
            .map_err(|e| format!("Failed to create persistent store: {e}"))?;
    }

    Ok(())
}

fn disable_exit_on_close(app: &mut tauri::App) {
    let window = match app.get_webview_window("main") {
        Some(window) => window,
        None => {
            eprintln!("Failed to disable exit on close, main window does not exist");
            return;
        }
    };

    window.clone().on_window_event(move |event| {
        if let tauri::WindowEvent::CloseRequested { api, .. } = event {
            api.prevent_close();
            if let Err(e) = window.hide() {
                eprintln!("Failed to hide window: {e}");
            }
        }
    });
}

fn build_tray(app: &mut tauri::App) -> Result<(), String> {
    let settings = MenuItem::with_id(app, "settings", "Settings", true, None::<&str>)
        .map_err(|e| format!("Failed to create menu item: {e}"))?;
    let quit = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)
        .map_err(|e| format!("Failed to create menu item: {e}"))?;
    let menu = Menu::with_items(app, &[&settings, &quit])
        .map_err(|e| format!("Failed to create menu: {e}"))?;

    let icon = app
        .default_window_icon()
        .ok_or("No default window icon found")?
        .clone();

    TrayIconBuilder::new()
        .icon(icon)
        .menu(&menu)
        .show_menu_on_left_click(true)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "quit" => quit_app(app.clone()),
            "settings" => {
                if let Some(window) = app.get_webview_window("main") {
                    if let Err(e) = window.show() {
                        eprintln!("Failed to show window: {}", e);
                    }
                    if let Err(e) = window.set_focus() {
                        eprintln!("Failed to focus window: {}", e);
                    }
                }
            }
            _ => unreachable!(),
        })
        .build(app)
        .map_err(|e| format!("Failed to create tray: {e}"))?;

    Ok(())
}

fn register_shortcut_listener(app: &mut tauri::App, meeting_url: String) -> Result<(), String> {
    app.handle()
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(move |app, shortcut, event| {
                    if event.state() != ShortcutState::Pressed {
                        return;
                    }

                    let state = app.state::<AppState>();
                    let target_shortcut = match state.target_shortcut.lock() {
                        Ok(target_shortcut) => target_shortcut,
                        Err(e) => {
                            eprintln!("Failed to acquire shortcut lock: {e}");
                            return;
                        }
                    };

                    if *shortcut == *target_shortcut {
                        if let Err(e) = paste_meeting_url(&meeting_url) {
                            eprintln!("Failed pasting meeting URL: {e}");
                        }
                    }
                })
                .build(),
        )
        .map_err(|e| format!("Failed to create global shortcut: {e}"))?;

    let state = app.state::<AppState>();
    let target_shortcut = state
        .target_shortcut
        .lock()
        .map_err(|e| format!("Failed to acquire shortcut lock: {e}"))?;

    app.global_shortcut()
        .register(*target_shortcut)
        .map_err(|e| format!("Failed to register shortcut: {e}"))?;

    Ok(())
}

fn paste_meeting_url(url: &str) -> Result<(), String> {
    let settings = enigo::Settings::default();
    let mut typer = Enigo::new(&settings)
        .map_err(|e| format!("Failed to create `Enigo` event emitter: {e}"))?;
    typer
        .text(url)
        .map_err(|e| format!("Failed pasting meeting URL: {e}"))?;

    Ok(())
}

pub fn generate_meeting_url() -> String {
    // TODO: this should use Google Meet API
    String::from("meet.google.com/xio-xfkn-wsm")
}
