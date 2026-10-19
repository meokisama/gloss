use enigo::{Direction, Key, Keyboard};
use tauri::App;

pub fn init(_app: &mut App) {}

pub fn send_copy() -> Result<(), String> {
    let mut e = super::enigo()?;
    let mut run = || -> enigo::InputResult<()> {
        e.key(Key::Control, Direction::Press)?;
        e.key(Key::Unicode('c'), Direction::Click)?;
        e.key(Key::Control, Direction::Release)
    };
    run().map_err(|err| err.to_string())
}
