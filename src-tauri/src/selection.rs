use std::thread::sleep;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use tauri::AppHandle;
use tauri_plugin_clipboard_manager::ClipboardExt;

use crate::platform;

const COPY_TIMEOUT: Duration = Duration::from_millis(600);
const POLL_INTERVAL: Duration = Duration::from_millis(20);

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
