use std::thread::sleep;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use tauri::AppHandle;
use tauri_plugin_clipboard_manager::ClipboardExt;

use crate::platform;

const COPY_TIMEOUT: Duration = Duration::from_millis(600);
const POLL_INTERVAL: Duration = Duration::from_millis(20);
const PASTE_SETTLE: Duration = Duration::from_millis(500);

/// Reads the foreground app's selection by making it copy, then puts the old clipboard text back.
/// A marker is written first so an empty selection (clipboard unchanged) is told apart from a copy.
/// Blocking: call it off the main thread.
pub fn capture(app: &AppHandle) -> Result<String, String> {
    let clipboard = app.clipboard();
    let previous = clipboard.read_text().ok();
    let nanos = SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_nanos()).unwrap_or_default();
    let marker = format!("gloss-marker-{nanos}");
    clipboard.write_text(marker.clone()).map_err(|e| e.to_string())?;

    let copied = platform::send_copy().map(|_| {
        let deadline = Instant::now() + COPY_TIMEOUT;
        while Instant::now() < deadline {
            sleep(POLL_INTERVAL);
            match clipboard.read_text() {
                Ok(text) if text != marker => return Some(text),
                _ => {}
            }
        }
        None
    });

    let _ = match previous {
        Some(text) => clipboard.write_text(text),
        None => clipboard.clear(),
    };
    Ok(copied?.unwrap_or_default().trim().to_string())
}

/// Pastes `text` over the foreground app's selection, then puts the old clipboard text back.
/// Blocking: call it off the main thread.
pub fn paste(app: &AppHandle, text: &str) -> Result<(), String> {
    let clipboard = app.clipboard();
    let previous = clipboard.read_text().ok();
    clipboard.write_text(text).map_err(|e| e.to_string())?;
    let pasted = platform::send_paste();
    // Apps read the clipboard after handling the key event; restoring at once would paste the old text.
    sleep(PASTE_SETTLE);
    let _ = match previous {
        Some(text) => clipboard.write_text(text),
        None => clipboard.clear(),
    };
    pasted
}
