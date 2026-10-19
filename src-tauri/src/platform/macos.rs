use enigo::{Direction, Key, Keyboard};
use tauri::App;

/// kVK_ANSI_C. `Key::Unicode` would look up the keyboard layout, which macOS only allows on the main thread.
const KEYCODE_C: u32 = 0x08;

pub fn init(app: &mut App) {
    app.set_activation_policy(tauri::ActivationPolicy::Accessory);
}

/// Needs the Accessibility permission; enigo opens the system prompt the first time.
/// Held modifiers do not leak in: enigo posts events independent of the physical keyboard state.
pub fn send_copy() -> Result<(), String> {
    let mut e = super::enigo()?;
    let mut run = || -> enigo::InputResult<()> {
        e.key(Key::Meta, Direction::Press)?;
        e.key(Key::Other(KEYCODE_C), Direction::Click)?;
        e.key(Key::Meta, Direction::Release)
    };
    run().map_err(|err| err.to_string())
}
