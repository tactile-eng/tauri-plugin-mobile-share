// use serde::de::DeserializeOwned;
// use tauri::{
//     plugin::{PluginApi, PluginHandle},
//     AppHandle, Runtime,
// };

// use crate::models::*;

// #[cfg(target_os = "ios")]
// tauri::ios_plugin_binding!(init_plugin_test);

// // initializes the Kotlin or Swift plugin classes
// pub fn init<R: Runtime, C: DeserializeOwned>(
//     _app: &AppHandle<R>,
//     api: PluginApi<R, C>,
// ) -> crate::Result<Test<R>> {
//     #[cfg(target_os = "android")]
//     let handle = api.register_android_plugin("", "ExamplePlugin")?;
//     #[cfg(target_os = "ios")]
//     let handle = api.register_ios_plugin(init_plugin_test)?;
//     Ok(Test(handle))
// }

// /// Access to the test APIs.
// pub struct Test<R: Runtime>(PluginHandle<R>);

// impl<R: Runtime> Test<R> {
//     pub fn share_text(&self, payload: ShareText) -> crate::Result<bool> {
//         self.0
//             .run_mobile_plugin("shareText", payload)
//             .map_err(Into::into)
//     }

//     pub fn share_binary(&self, payload: ShareBinary) -> crate::Result<bool> {
//         self.0
//             .run_mobile_plugin("shareBinary", payload)
//             .map_err(Into::into)
//     }
// }
