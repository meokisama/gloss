use enigo::{Direction, Key, Keyboard};
use tauri::App;

/// Unassigned virtual key. Tapping it before releasing Alt or Win stops the target app
/// from opening its menu bar or the Start menu on the lone modifier release.
const VK_MASK: u32 = 0xE8;

pub fn init(_app: &mut App) {}

pub fn send_copy() -> Result<(), String> {
    let mut e = super::enigo()?;
    let mut run = || -> enigo::InputResult<()> {
        e.key(Key::Other(VK_MASK), Direction::Click)?;
        // The hotkey's modifiers are usually still held; Ctrl+Alt+C is not a copy.
        for modifier in [Key::Alt, Key::Shift, Key::Meta, Key::Control] {
            e.key(modifier, Direction::Release)?;
        }
        e.key(Key::Control, Direction::Press)?;
        e.key(Key::C, Direction::Click)?;
        e.key(Key::Control, Direction::Release)
    };
    run().map_err(|err| err.to_string())
}
