use tauri::AppHandle;
use tauri_plugin_autostart::ManagerExt;

use crate::settings::{self, Settings};
use crate::{hotkey, popup, translate};

#[tauri::command]
pub fn get_settings(app: AppHandle) -> Settings {
    settings::current(&app)
}

#[tauri::command]
pub fn default_settings() -> Settings {
    Settings::default()
}

#[tauri::command]
pub fn style_presets() -> &'static [translate::StylePreset] {
    translate::STYLES
}

#[tauri::command]
pub fn save_settings(app: AppHandle, mut new: Settings) -> Result<(), String> {
    let old = settings::current(&app);
    new.last_popup_pos = old.last_popup_pos;
    if let Err(e) = hotkey::register(&app, &new.hotkey) {
        let _ = hotkey::register(&app, &old.hotkey);
        return Err(e);
    }
    let autolaunch = app.autolaunch();
    let _ = if new.launch_at_login { autolaunch.enable() } else { autolaunch.disable() };
    settings::persist(&app, &new)
}

/// `settings` lets the Settings window test values that are not saved yet.
#[tauri::command]
pub async fn translate(app: AppHandle, text: String, style: String, settings: Option<Settings>) -> Result<String, String> {
    let s = settings.unwrap_or_else(|| settings::current(&app));
    translate::translate(&s, &text, &style).await
}

#[tauri::command]
pub fn resize_popup(app: AppHandle, height: f64, keep_position: bool) {
    popup::resize(&app, height, keep_position);
}

#[tauri::command]
pub fn hide_popup(app: AppHandle) {
    popup::hide(&app);
}
