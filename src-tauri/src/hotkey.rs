use tauri::AppHandle;
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

use crate::popup::{self, Session};
use crate::settings::Settings;
use crate::{selection, write};

pub fn register(app: &AppHandle, s: &Settings) -> Result<(), String> {
    app.global_shortcut().unregister_all().map_err(|e| e.to_string())?;
    on(app, &s.hotkey, translate)?;
    on(app, &s.write_hotkey, write::start)
}

fn on(app: &AppHandle, hotkey: &str, action: fn(&AppHandle)) -> Result<(), String> {
    app.global_shortcut()
        .on_shortcut(hotkey, move |app, _, event| {
            if event.state != ShortcutState::Pressed {
                return;
            }
            let app = app.clone();
            // Capture sleeps while the target app copies; keep it off the event loop.
            std::thread::spawn(move || action(&app));
        })
        .map_err(|e| format!("Cannot register hotkey \"{hotkey}\": {e}"))
}

fn translate(app: &AppHandle) {
    let text = selection::capture(app).unwrap_or_default();
    popup::show(app, Session { text, write_lang: None });
}
