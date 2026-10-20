//! Auto update from GitHub Releases: download in the background, then offer a restart from the tray.

use std::sync::Mutex;
use std::time::Duration;

use tauri::{AppHandle, Manager};
use tauri_plugin_updater::{Update, UpdaterExt};

use crate::{settings, tray};

/// A tray app can run for weeks, so checking only at launch is not enough.
const CHECK_EVERY: Duration = Duration::from_secs(6 * 60 * 60);

/// An update downloaded and waiting for the user to restart.
#[derive(Default)]
pub struct Pending(Mutex<Option<(Update, Vec<u8>)>>);

pub fn start(app: &AppHandle) {
    let app = app.clone();
    std::thread::spawn(move || {
        while pending_version(&app).is_none() {
            if let Err(e) = tauri::async_runtime::block_on(download(&app)) {
                eprintln!("Update check failed: {e}");
            }
            std::thread::sleep(CHECK_EVERY);
        }
    });
}

pub fn pending_version(app: &AppHandle) -> Option<String> {
    app.state::<Pending>().0.lock().unwrap().as_ref().map(|(update, _)| update.version.clone())
}

pub fn install(app: &AppHandle) {
    let Some((update, bytes)) = app.state::<Pending>().0.lock().unwrap().take() else { return };
    match update.install(bytes) {
        Ok(()) => app.restart(),
        Err(e) => eprintln!("Update install failed: {e}"),
    }
}

async fn download(app: &AppHandle) -> tauri_plugin_updater::Result<()> {
    let Some(update) = app.updater()?.check().await? else { return Ok(()) };
    let bytes = update.download(|_, _| {}, || {}).await?;
    *app.state::<Pending>().0.lock().unwrap() = Some((update, bytes));
    tray::refresh(app, &settings::current(app));
    Ok(())
}
