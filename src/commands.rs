use tauri::{command, AppHandle, Runtime};

use crate::models::*;
use crate::Result;
use crate::TestExt;

#[command]
pub(crate) async fn share_text<R: Runtime>(app: AppHandle<R>, payload: ShareText) -> Result<bool> {
    app.test().share_text(payload)
}

#[command]
pub(crate) async fn share_binary<R: Runtime>(app: AppHandle<R>, payload: ShareBinary) -> Result<bool> {
    app.test().share_binary(payload)
}
