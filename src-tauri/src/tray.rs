use serde_json::json;
use tauri::menu::{CheckMenuItem, IsMenuItem, Menu, MenuItem, PredefinedMenuItem, Submenu};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::{App, AppHandle, Emitter, Manager};

use crate::languages::LANGUAGES;
use crate::settings::{self, Settings};

const TRAY_ID: &str = "main";
const SETTINGS_LABEL: &str = "main";
const TARGET_PREFIX: &str = "target:";
const WRITE_PREFIX: &str = "write:";

pub fn create(app: &App) -> tauri::Result<()> {
    let mut tray = TrayIconBuilder::with_id(TRAY_ID)
        .tooltip("Gloss")
        .menu(&menu(app.handle(), &settings::current(app.handle()))?)
        .show_menu_on_left_click(false)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "settings" => show_settings(app),
            "quit" => app.exit(0),
            id => {
                if let Some(lang) = id.strip_prefix(TARGET_PREFIX) {
                    set_language(app, |s| s.target_lang = lang.into(), json!({ "targetLang": lang }));
                } else if let Some(lang) = id.strip_prefix(WRITE_PREFIX) {
                    set_language(app, |s| s.write_lang = lang.into(), json!({ "writeLang": lang }));
                }
            }
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click { button: MouseButton::Left, button_state: MouseButtonState::Up, .. } = event {
                show_settings(tray.app_handle());
            }
        });
    if let Some(icon) = app.default_window_icon() {
        tray = tray.icon(icon.clone());
    }
    tray.build(app)?;
    Ok(())
}

/// Rebuilds the menu so its check marks follow settings saved elsewhere.
pub fn refresh(app: &AppHandle, s: &Settings) {
    if let (Some(tray), Ok(menu)) = (app.tray_by_id(TRAY_ID), menu(app, s)) {
        let _ = tray.set_menu(Some(menu));
    }
}

pub fn show_settings(app: &AppHandle) {
    if let Some(win) = app.get_webview_window(SETTINGS_LABEL) {
        let _ = win.show();
        let _ = win.unminimize();
        let _ = win.set_focus();
    }
}

fn menu(app: &AppHandle, s: &Settings) -> tauri::Result<Menu<tauri::Wry>> {
    Menu::with_items(
        app,
        &[
            &language_menu(app, "Translate into", TARGET_PREFIX, &s.target_lang)?,
            &language_menu(app, "Translate my writing into", WRITE_PREFIX, &s.write_lang)?,
            &PredefinedMenuItem::separator(app)?,
            &MenuItem::with_id(app, "settings", "Settings", true, None::<&str>)?,
            &MenuItem::with_id(app, "quit", "Quit Gloss", true, None::<&str>)?,
        ],
    )
}

/// A language typed in Settings that is not in the list still shows, checked, at the top.
fn language_menu(app: &AppHandle, title: &str, prefix: &str, current: &str) -> tauri::Result<Submenu<tauri::Wry>> {
    let custom = (!LANGUAGES.contains(&current)).then_some(current);
    let items = custom
        .into_iter()
        .chain(LANGUAGES.iter().copied())
        .map(|lang| CheckMenuItem::with_id(app, format!("{prefix}{lang}"), lang, true, lang == current, None::<&str>))
        .collect::<tauri::Result<Vec<_>>>()?;
    let refs: Vec<&dyn IsMenuItem<tauri::Wry>> = items.iter().map(|item| item as &dyn IsMenuItem<tauri::Wry>).collect();
    Submenu::with_items(app, title, true, &refs)
}

/// `patch` goes to the Settings window, which merges it so unsaved edits there are kept.
fn set_language(app: &AppHandle, apply: impl FnOnce(&mut Settings), patch: serde_json::Value) {
    let mut s = settings::current(app);
    apply(&mut s);
    if settings::persist(app, &s).is_ok() {
        refresh(app, &s);
        let _ = app.emit_to(SETTINGS_LABEL, "settings:patch", patch);
    }
}
