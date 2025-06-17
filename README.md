# Tauri Plugin Mobile Share

| Platform | Supported |
| -------- | --------- |
| iOS      | ✓         |
| Android  | ?         |
| Linux    | x         |
| Windows  | x         |
| macOS    | x         |

## Purpose

This project exists to allow for a simple and convenient way to leverage the "share" functionality of mobile platforms, which in some cases (_*cough*_ iOS _*cough*_) is the only meaningful way to save data to a user's device. Of course, there is the option of simply using the Web Share API standard, which will let you interact with said functionality with zero imports and should work in almost every circumstance. Should you find yourself in need of something else, hopefully this project will be of use.

## Install

Here's how you can install this project in your app:

1. Add this line to your project's `cargo.toml`: `tauri-plugin-mobile-share = { git = "https://github.com/tactile-eng/tauri-plugin-mobile-share", branch = "release" }`
2. Add this line to your project's `package.json` dependency section: `"plugin-mobile-share": "tactile-eng/tauri-plugin-mobile-share#release"`
3. Use `npm` or your preferred alternative to install stuff: `npm install`

## Usage

First you need to register the core plugin with Tauri:

`src-tauri/src/lib.rs`

```rust
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_mobile_share::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

Afterwards all the plugin's APIs are available through the JavaScript guest bindings:

```javascript
import { shareText, shareBinary } from "plugin-mobile-share";

/** Text Document data is just a string, so...that's pretty simple */
shareText(textData, { name: "My File", ext: "txt" });

/**
 * binary data must be encoded as a base64 string, so assuming you have some `file` of type `Blob`:
 * `const base64String = Buffer.from(await file.arrayBuffer()).toString("base64")`
 */
shareBinary(binaryData, { name: "My File", ext: "png" });
```

## On Android (`?`) Support

Obviously, we should support Android, but at the momemnt, only iOS is supported. To the best of our knowledge, there shouldn't be any techncial issue with adding Android support, and encourage anyone to do so!

## Acknowledgements

I would like to thank the contributors of the [Tauri Barcode Scanner Plugin](https://github.com/tauri-apps/plugins-workspace/tree/v2/plugins/barcode-scanner), without whose examples--both literal and figurative--this project would have taken significantly more time and effort to complete.
