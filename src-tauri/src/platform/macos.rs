use std::thread::sleep;
use std::time::Duration;

use enigo::{Direction, Key, Keyboard};
use tauri::{App, AppHandle};

/// kVK_ANSI_C and kVK_ANSI_V. `Key::Unicode` would look up the keyboard layout, which macOS only allows on the main thread.
const KEYCODE_C: u32 = 0x08;
const KEYCODE_V: u32 = 0x09;

/// Nothing to keep: hiding the app hands focus back to the app that had it.
pub struct Focus;

pub fn init(app: &mut App) {
    app.set_activation_policy(tauri::ActivationPolicy::Accessory);
}

pub fn send_copy() -> Result<(), String> {
    send_cmd(KEYCODE_C)
}

pub fn send_paste() -> Result<(), String> {
    send_cmd(KEYCODE_V)
}

pub fn foreground() -> Focus {
    Focus
}

/// Also hides the Settings window if it is open. The delay lets the activation land before the paste.
pub fn restore_focus(app: &AppHandle, _focus: &Focus) -> Result<(), String> {
    app.hide().map_err(|e| e.to_string())?;
    sleep(Duration::from_millis(150));
    Ok(())
}

/// Needs the Accessibility permission; enigo opens the system prompt the first time.
/// Held modifiers do not leak in: enigo posts events independent of the physical keyboard state.
fn send_cmd(keycode: u32) -> Result<(), String> {
    let mut e = super::enigo()?;
    let mut run = || -> enigo::InputResult<()> {
        e.key(Key::Meta, Direction::Press)?;
        e.key(Key::Other(keycode), Direction::Click)?;
        e.key(Key::Meta, Direction::Release)
    };
    run().map_err(|err| err.to_string())
}
