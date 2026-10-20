use serde_json::{Value, json};
use tauri::menu::{CheckMenuItem, IsMenuItem, Menu, MenuItem, PredefinedMenuItem, Submenu};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::{App, AppHandle, Emitter, Manager};

use crate::languages::LANGUAGES;
use crate::settings::{self, Settings};
use crate::translate::STYLES;
use crate::update;

const TRAY_ID: &str = "main";
const SETTINGS_LABEL: &str = "main";
/// Settings fields the menu sets. Item ids are `<field>:<value>`, with the field named as in the camelCase JSON.
const FIELDS: [&str; 4] = ["targetLang", "style", "writeLang", "writeStyle"];

pub fn create(app: &App) -> tauri::Result<()> {
    let mut tray = TrayIconBuilder::with_id(TRAY_ID)
        .tooltip("Gloss")
        .menu(&menu(app.handle(), &settings::current(app.handle()))?)
        .show_menu_on_left_click(false)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "settings" => show_settings(app),
            "update" => update::install(app),
            "quit" => app.exit(0),
            id => {
                if let Some((field, value)) = id.split_once(':').filter(|(field, _)| FIELDS.contains(field)) {
                    set_field(app, field, value);
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
    let styles: Vec<(&str, &str)> = STYLES.iter().map(|p| (p.id, p.label)).collect();
    let menu = Menu::with_items(
        app,
        &[
            &choice_menu(app, "Translate into", "targetLang", &s.target_lang, &languages(&s.target_lang))?,
            &choice_menu(app, "Translate style", "style", &s.style, &styles)?,
            &PredefinedMenuItem::separator(app)?,
            &choice_menu(app, "My writing into", "writeLang", &s.write_lang, &languages(&s.write_lang))?,
            &choice_menu(app, "My writing style", "writeStyle", &s.write_style, &styles)?,
            &PredefinedMenuItem::separator(app)?,
            &MenuItem::with_id(app, "settings", "Settings", true, None::<&str>)?,
            &MenuItem::with_id(app, "quit", "Quit Gloss", true, None::<&str>)?,
        ],
    )?;
    if let Some(version) = update::pending_version(app) {
        menu.prepend_items(&[
            &MenuItem::with_id(app, "update", format!("Restart to update to {version}"), true, None::<&str>)?,
            &PredefinedMenuItem::separator(app)?,
        ])?;
    }
    Ok(menu)
}

/// A language typed in Settings that is not in the list still shows, checked, at the top.
fn languages(current: &str) -> Vec<(&str, &str)> {
    let custom = (!LANGUAGES.contains(&current)).then_some(current);
    custom.into_iter().chain(LANGUAGES.iter().copied()).map(|lang| (lang, lang)).collect()
}

fn choice_menu(app: &AppHandle, title: &str, field: &str, current: &str, options: &[(&str, &str)]) -> tauri::Result<Submenu<tauri::Wry>> {
    let items = options
        .iter()
        .map(|&(value, label)| CheckMenuItem::with_id(app, format!("{field}:{value}"), label, true, value == current, None::<&str>))
        .collect::<tauri::Result<Vec<_>>>()?;
    let refs: Vec<&dyn IsMenuItem<tauri::Wry>> = items.iter().map(|item| item as &dyn IsMenuItem<tauri::Wry>).collect();
    Submenu::with_items(app, title, true, &refs)
}

/// Goes through JSON so one function covers every field in `FIELDS`. The patch goes to the Settings window,
/// which merges it so unsaved edits there are kept.
fn set_field(app: &AppHandle, field: &str, value: &str) {
    let Ok(mut json) = serde_json::to_value(settings::current(app)) else { return };
    json[field] = Value::from(value);
    let Ok(s) = serde_json::from_value::<Settings>(json) else { return };
    if settings::persist(app, &s).is_ok() {
        refresh(app, &s);
        let _ = app.emit_to(SETTINGS_LABEL, "settings:patch", json!({ field: value }));
    }
}
