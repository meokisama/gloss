use std::sync::Mutex;

use tauri::{AppHandle, Emitter, Manager, PhysicalPosition, PhysicalSize, WebviewWindow};

use crate::settings::{self, PopupPosition, Settings};

pub const LABEL: &str = "popup";
/// Logical height used until the page reports its real content height.
const INITIAL_HEIGHT: f64 = 120.0;
const EDGE_MARGIN: f64 = 16.0;

/// Cursor position at the moment the hotkey fired; resizes re-place the popup around it.
#[derive(Default)]
pub struct Anchor(pub Mutex<PhysicalPosition<f64>>);

pub fn show(app: &AppHandle, text: String) {
    let Some(win) = app.get_webview_window(LABEL) else { return };
    let cursor = app.cursor_position().unwrap_or_default();
    *app.state::<Anchor>().0.lock().unwrap() = cursor;
    place(app, &win, &settings::current(app), INITIAL_HEIGHT, false);
    let _ = win.show();
    let _ = win.set_focus();
    let _ = app.emit_to(LABEL, "popup:source", text);
}

pub fn resize(app: &AppHandle, height: f64, keep_position: bool) {
    if let Some(win) = app.get_webview_window(LABEL) {
        place(app, &win, &settings::current(app), height, keep_position);
    }
}

pub fn hide(app: &AppHandle) {
    let Some(win) = app.get_webview_window(LABEL) else { return };
    let mut s = settings::current(app);
    if s.popup_position == PopupPosition::Remember
        && let Ok(pos) = win.outer_position()
    {
        s.last_popup_pos = Some((pos.x, pos.y));
        let _ = settings::persist(app, &s);
    }
    let _ = win.hide();
}

fn place(app: &AppHandle, win: &WebviewWindow, s: &Settings, logical_height: f64, keep_position: bool) {
    let cursor = *app.state::<Anchor>().0.lock().unwrap();
    let monitor = app.monitor_from_point(cursor.x, cursor.y).ok().flatten().or_else(|| app.primary_monitor().ok().flatten());
    let Some(monitor) = monitor else { return };
    let scale = monitor.scale_factor();
    let area = monitor.work_area();
    let (left, top) = (area.position.x as f64, area.position.y as f64);
    let (right, bottom) = (left + area.size.width as f64, top + area.size.height as f64);

    let w = (s.popup_width as f64 * scale).min(right - left);
    let h = (logical_height.min(s.popup_max_height as f64) * scale).min(bottom - top);
    let gap = s.popup_offset as f64 * scale;
    let margin = EDGE_MARGIN * scale;
    let below = || if cursor.y + gap + h > bottom { cursor.y - gap - h } else { cursor.y + gap };
    let above = || if cursor.y - gap - h < top { cursor.y + gap } else { cursor.y - gap - h };

    let (x, y) = if keep_position {
        win.outer_position().map(|p| (p.x as f64, p.y as f64)).unwrap_or((cursor.x, below()))
    } else {
        match s.popup_position {
            PopupPosition::Cursor => (cursor.x - w / 2.0, below()),
            PopupPosition::AboveCursor => (cursor.x - w / 2.0, above()),
            PopupPosition::Center => (left + (right - left - w) / 2.0, top + (bottom - top - h) / 3.0),
            PopupPosition::TopRight => (right - w - margin, top + margin),
            PopupPosition::Remember => s.last_popup_pos.map(|(x, y)| (x as f64, y as f64)).unwrap_or((cursor.x - w / 2.0, below())),
        }
    };
    let x = x.clamp(left, right - w);
    let y = y.clamp(top, bottom - h);

    let _ = win.set_size(PhysicalSize::new(w.round() as u32, h.round() as u32));
    let _ = win.set_position(PhysicalPosition::new(x.round() as i32, y.round() as i32));
}
