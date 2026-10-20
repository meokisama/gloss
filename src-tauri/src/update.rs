//! Auto update from GitHub Releases: install quietly at launch, or download in the background and install on quit.

use std::sync::Mutex;
use std::time::Duration;

use tauri::{AppHandle, Manager};
use tauri_plugin_updater::{Update, UpdaterExt};

use crate::{settings, tray};

/// A tray app can run for weeks, so checking only at launch is not enough.
const CHECK_EVERY: Duration = Duration::from_secs(6 * 60 * 60);

/// An update downloaded and waiting for the user to restart or quit.
#[derive(Default)]
pub struct Pending(Mutex<Option<(Update, Vec<u8>)>>);

pub fn start(app: &AppHandle) {
    let app = app.clone();
    std::thread::spawn(move || {
        check(&app);
        // Nothing is in progress right after launch, so an update found then installs without asking.
        install(&app, true);
        while pending_version(&app).is_none() {
            std::thread::sleep(CHECK_EVERY);
            check(&app);
        }
    });
}

pub fn pending_version(app: &AppHandle) -> Option<String> {
    app.state::<Pending>().0.lock().unwrap().as_ref().map(|(update, _)| update.version.clone())
}

/// Installs the downloaded update, if any. On Windows this exits the app to run the installer.
pub fn install(app: &AppHandle, restart: bool) {
    let Some((update, bytes)) = app.state::<Pending>().0.lock().unwrap().take() else { return };
    match update.restart_after_install(restart).install(bytes) {
        Ok(()) if restart => app.restart(),
        Ok(()) => {}
        Err(e) => eprintln!("Update install failed: {e}"),
    }
}

fn check(app: &AppHandle) {
    if let Err(e) = tauri::async_runtime::block_on(download(app)) {
        eprintln!("Update check failed: {e}");
    }
}

async fn download(app: &AppHandle) -> tauri_plugin_updater::Result<()> {
    let Some(update) = app.updater()?.check().await? else { return Ok(()) };
    let bytes = update.download(|_, _| {}, || {}).await?;
    *app.state::<Pending>().0.lock().unwrap() = Some((update, bytes));
    tray::refresh(app, &settings::current(app));
    Ok(())
}
