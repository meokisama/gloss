use enigo::{Direction, Key, Keyboard};
use tauri::{App, AppHandle};

/// Nothing to keep: the window manager refocuses the previous window when the popup hides.
pub struct Focus;

pub fn init(_app: &mut App) {}

pub fn send_copy() -> Result<(), String> {
    send_ctrl('c')
}

pub fn send_paste() -> Result<(), String> {
    send_ctrl('v')
}

pub fn foreground() -> Focus {
    Focus
}

pub fn restore_focus(_app: &AppHandle, _focus: &Focus) -> Result<(), String> {
    Ok(())
}

fn send_ctrl(key: char) -> Result<(), String> {
    let mut e = super::enigo()?;
    let mut run = || -> enigo::InputResult<()> {
        e.key(Key::Control, Direction::Press)?;
        e.key(Key::Unicode(key), Direction::Click)?;
        e.key(Key::Control, Direction::Release)
    };
    run().map_err(|err| err.to_string())
}
