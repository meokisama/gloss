//! The writing hotkey: translate what the user typed into `write_lang` and paste it back over the selection.

use std::sync::Mutex;

use tauri::{AppHandle, Manager};

use crate::platform::{self, Focus};
use crate::popup::{self, Session};
use crate::{selection, settings};

/// Window the writing hotkey was pressed in, where the translation is pasted.
#[derive(Default)]
pub struct WriteFocus(Mutex<Option<Focus>>);

/// Blocking: call it off the main thread.
pub fn start(app: &AppHandle) {
    *app.state::<WriteFocus>().0.lock().unwrap() = Some(platform::foreground());
    let text = selection::capture(app).unwrap_or_default();
    popup::show(app, Session { text, write_lang: Some(settings::current(app).write_lang) });
}

/// Blocking: call it off the main thread.
pub fn replace(app: &AppHandle, text: &str) -> Result<(), String> {
    if !popup::is_focused(app) {
        return Err("Gloss lost focus, so nothing was pasted. Copy the translation instead.".into());
    }
    popup::hide(app);
    let focus = app.state::<WriteFocus>().0.lock().unwrap().take();
    let focus = focus.ok_or("Press the writing hotkey in the window you are typing in.")?;
    platform::restore_focus(app, &focus)?;
    selection::paste(app, text)
}
