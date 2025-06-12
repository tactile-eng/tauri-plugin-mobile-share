#![cfg(mobile)]

use tauri::{
    plugin::{Builder, PluginHandle, TauriPlugin},
    Manager, Runtime,
};

// pub use models::*;

#[cfg(mobile)]
mod mobile;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_test);

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the test APIs.
pub trait TestExt<R: Runtime> {
    fn test(&self) -> &Test<R>;
}

impl<R: Runtime, T: Manager<R>> crate::TestExt<R> for T {
    fn test(&self) -> &Test<R> {
        self.state::<Test<R>>().inner()
    }
}

pub struct Test<R: Runtime>(PluginHandle<R>);

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("test")
        .setup(|app, api| {
            #[cfg(target_os = "android")]
            let handle = api.register_android_plugin("", "ExamplePlugin")?;
            #[cfg(target_os = "ios")]
            let handle = api.register_ios_plugin(init_plugin_test)?;

            app.manage(Test(handle));
            Ok(())
        })
        .build()
}
