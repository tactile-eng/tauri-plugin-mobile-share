# Tauri Plugin Mobile Share

| Platform | Supported |
| -------- | --------- |
| iOS      | ✓         |
| Android  | x         |
| Linux    | x         |
| Windows  | x         |
| macOS    | x         |

## Purpose

This project exists to allow for a simple and convenient way to leverage the "share" functionality of mobile platforms, which in some cases (_*cough*_ iOS _*cough*_) is the only meaningful way to save data to a user's device. Of course, there is the option of simply using the [Web Share API](https://developer.mozilla.org/en-US/docs/Web/API/Web_Share_API) standard, which will let you interact with said functionality with zero imports and should work in almost every circumstance. Should you find yourself in need of something else, hopefully this project will be of use.

## Install

Here's how you can install this project in your app:

1. Use `cargo` to install in your tauri `src-tauri` directory: `cargo add tauri-plugin-mobile-share`
2. Use `npm` (or your preferred alternative) to install the guest bindings in your `package.json`'s directory: `npm install tauri-plugin-mobile-share`

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

```ts
import { shareBinary, shareText } from "tauri-plugin-mobile-share";
import type { ShareMeta } from "tauri-plugin-mobile-share";

const textData = "lorem ipsum dolor sit amet...";
const textMetadata: ShareMeta = { name: "My File", ext: "txt" };

shareText(textData, textMetadata);

const file: Blob; // not included: your Blob file...
const binaryData = Buffer.from(await file.readBinary()).toString("base64");
const binaryMetadata: ShareMeta = { name: "My File", ext: "png" };

shareBinary(binaryData, binaryMetadata);
```

## TODO

- [ ] Add Android Support
- [ ] Simplify JS guest bindings

## Acknowledgements

I would like to thank the contributors of the [Tauri Barcode Scanner Plugin](https://github.com/tauri-apps/plugins-workspace/tree/v2/plugins/barcode-scanner), without whose examples--both literal and figurative--this project would have taken significantly more time and effort to complete.
