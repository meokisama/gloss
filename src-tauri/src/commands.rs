use tauri::AppHandle;
use tauri_plugin_autostart::ManagerExt;

use crate::languages::LANGUAGES;
use crate::settings::{self, Settings};
use crate::{hotkey, popup, translate, tray, write};

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
pub fn languages() -> &'static [&'static str] {
    LANGUAGES
}

#[tauri::command]
pub fn save_settings(app: AppHandle, mut new: Settings) -> Result<(), String> {
    let old = settings::current(&app);
    new.last_popup_pos = old.last_popup_pos;
    if let Err(e) = hotkey::register(&app, &new) {
        let _ = hotkey::register(&app, &old);
        return Err(e);
    }
    let autolaunch = app.autolaunch();
    let _ = if new.launch_at_login { autolaunch.enable() } else { autolaunch.disable() };
    settings::persist(&app, &new)?;
    tray::refresh(&app, &new);
    Ok(())
}

/// `settings` lets the Settings window test values that are not saved yet. `write_lang` translates the user's own text.
#[tauri::command]
pub async fn translate(
    app: AppHandle,
    text: String,
    style: String,
    settings: Option<Settings>,
    write_lang: Option<String>,
) -> Result<String, String> {
    let mut s = settings.unwrap_or_else(|| settings::current(&app));
    // Writing goes one way: text already in that language stays in it rather than switching to the fallback.
    if let Some(lang) = write_lang {
        s.fallback_lang = lang.clone();
        s.target_lang = lang;
    }
    translate::translate(&s, &text, &style).await
}

#[tauri::command]
pub async fn replace_selection(app: AppHandle, text: String) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || write::replace(&app, &text)).await.map_err(|e| e.to_string())?
}

/// Takes the unsaved settings so the list matches the key and base URL being edited.
#[tauri::command]
pub async fn list_models(settings: Settings) -> Result<Vec<String>, String> {
    translate::list_models(&settings).await
}

#[tauri::command]
pub fn resize_popup(app: AppHandle, height: f64, keep_position: bool) {
    popup::resize(&app, height, keep_position);
}

#[tauri::command]
pub fn hide_popup(app: AppHandle) {
    popup::hide(&app);
}
