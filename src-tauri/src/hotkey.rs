use tauri::AppHandle;
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::{popup, selection};

pub fn register(app: &AppHandle, hotkey: &str) -> Result<(), String> {
    let shortcuts = app.global_shortcut();
    shortcuts.unregister_all().map_err(|e| e.to_string())?;
    shortcuts
        .on_shortcut(hotkey, |app, _, event| {
            if event.state != ShortcutState::Pressed {
                return;
            }
            let app = app.clone();
            // Capture sleeps while the target app copies; keep it off the event loop.
            std::thread::spawn(move || {
                let text = selection::capture(&app).unwrap_or_default();
                popup::show(&app, text);
            });
        })
        .map_err(|e| format!("Cannot register hotkey \"{hotkey}\": {e}"))
}
