use std::thread::sleep;
use std::time::{Duration, Instant};

use enigo::{Direction, Key, Keyboard};
use tauri::{App, AppHandle};
use windows_sys::Win32::UI::WindowsAndMessaging::{GetForegroundWindow, SetForegroundWindow};

/// Unassigned virtual key. Tapping it before releasing Alt or Win stops the target app
/// from opening its menu bar or the Start menu on the lone modifier release.
const VK_MASK: u32 = 0xE8;
const FOCUS_TIMEOUT: Duration = Duration::from_millis(500);

/// The window that had focus, as an integer since `HWND` is a raw pointer and not `Send`.
pub struct Focus(isize);

pub fn init(_app: &mut App) {}

pub fn send_copy() -> Result<(), String> {
    send_ctrl(Key::C)
}

pub fn send_paste() -> Result<(), String> {
    send_ctrl(Key::V)
}

pub fn foreground() -> Focus {
    Focus(unsafe { GetForegroundWindow() } as isize)
}

/// Allowed because Gloss owns the foreground (the popup has focus) when this runs.
pub fn restore_focus(_app: &AppHandle, focus: &Focus) -> Result<(), String> {
    let hwnd = focus.0 as _;
    if unsafe { SetForegroundWindow(hwnd) } == 0 {
        return Err("Could not switch back to the window you were typing in.".into());
    }
    let deadline = Instant::now() + FOCUS_TIMEOUT;
    while unsafe { GetForegroundWindow() } != hwnd {
        if Instant::now() > deadline {
            return Err("The window you were typing in did not take focus back.".into());
        }
        sleep(Duration::from_millis(20));
    }
    Ok(())
}

fn send_ctrl(key: Key) -> Result<(), String> {
    let mut e = super::enigo()?;
    let mut run = || -> enigo::InputResult<()> {
        e.key(Key::Other(VK_MASK), Direction::Click)?;
        // The hotkey's modifiers are usually still held; Ctrl+Alt+C is not a copy.
        for modifier in [Key::Alt, Key::Shift, Key::Meta, Key::Control] {
            e.key(modifier, Direction::Release)?;
        }
        e.key(Key::Control, Direction::Press)?;
        e.key(key, Direction::Click)?;
        e.key(Key::Control, Direction::Release)
    };
    run().map_err(|err| err.to_string())
}
