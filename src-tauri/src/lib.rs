mod commands;
mod hotkey;
mod languages;
mod platform;
mod popup;
mod selection;
mod settings;
mod translate;
mod tray;
mod write;

use std::sync::Mutex;

use tauri::{Manager, WindowEvent};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _, _| tray::show_settings(app)))
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_autostart::Builder::new().build())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .manage(popup::Anchor::default())
        .manage(write::WriteFocus::default())
        .invoke_handler(tauri::generate_handler![
            commands::get_settings,
            commands::default_settings,
            commands::style_presets,
            commands::languages,
            commands::save_settings,
            commands::translate,
            commands::list_models,
            commands::replace_selection,
            commands::resize_popup,
            commands::hide_popup,
        ])
        .setup(|app| {
            platform::init(app);
            let loaded = settings::load(app.handle());
            if let Err(e) = hotkey::register(app.handle(), &loaded) {
                eprintln!("{e}");
            }
            let first_run = loaded.api_key().is_empty();
            app.manage(settings::SettingsState(Mutex::new(loaded)));
            tray::create(app)?;
            if first_run {
                tray::show_settings(app.handle());
            }
            Ok(())
        })
        .on_window_event(|window, event| {
            // Closing a window keeps the app alive in the tray.
            if let WindowEvent::CloseRequested { api, .. } = event {
                api.prevent_close();
                let _ = window.hide();
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
