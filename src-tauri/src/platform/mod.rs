//! Everything that differs per OS. Each backend exposes the same items: `init(app)` once at startup,
//! `send_copy()` and `send_paste()` for the foreground app, and `foreground()` returning a `Focus`
//! that `restore_focus(app, &focus)` switches back to.

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub use windows::*;

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
pub use macos::*;

#[cfg(not(any(target_os = "windows", target_os = "macos")))]
mod other;
#[cfg(not(any(target_os = "windows", target_os = "macos")))]
pub use other::*;

fn enigo() -> Result<enigo::Enigo, String> {
    enigo::Enigo::new(&enigo::Settings::default()).map_err(|e| format!("Cannot simulate input: {e}"))
}
