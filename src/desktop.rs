use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

use crate::models::*;

pub fn init<R: Runtime, C: DeserializeOwned>(
    app: &AppHandle<R>,
    _api: PluginApi<R, C>,
) -> crate::Result<MobileShare<R>> {
    Ok(MobileShare(app.clone()))
}

/// Access to the test APIs.
pub struct MobileShare<R: Runtime>(AppHandle<R>);

impl<R: Runtime> MobileShare<R> {
    pub fn share_text(&self, _payload: ShareText) -> crate::Result<bool> {
        Ok(false)
    }

    pub fn share_binary(&self, _payload: ShareBinary) -> crate::Result<bool> {
        Ok(false)
    }
}
