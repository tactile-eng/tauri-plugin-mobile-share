use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

use crate::models::*;

pub fn init<R: Runtime, C: DeserializeOwned>(
    app: &AppHandle<R>,
    _api: PluginApi<R, C>,
) -> crate::Result<Test<R>> {
    Ok(Test(app.clone()))
}

/// Access to the test APIs.
pub struct Test<R: Runtime>(AppHandle<R>);

impl<R: Runtime> Test<R> {
    pub fn share_text(&self, _payload: ShareText) -> crate::Result<bool> {
        Ok(false)
    }

    pub fn share_binary(&self, _payload: ShareBinary) -> crate::Result<bool> {
        Ok(false)
    }
}
