use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

pub use models::*;

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::MobileShare;
#[cfg(mobile)]
use mobile::MobileShare;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the test APIs.
pub trait TestExt<R: Runtime> {
    fn test(&self) -> &MobileShare<R>;
}

impl<R: Runtime, T: Manager<R>> crate::TestExt<R> for T {
    fn test(&self) -> &MobileShare<R> {
        self.state::<MobileShare<R>>().inner()
    }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("mobile_share")
        .invoke_handler(tauri::generate_handler![commands::share_text, commands::share_binary])
        .setup(|app, api| {
            #[cfg(mobile)]
            let mobile_share = mobile::init(app, api)?;
            #[cfg(desktop)]
            let mobile_share = desktop::init(app, api)?;
            app.manage(mobile_share);
            Ok(())
        })
        .build()
}
