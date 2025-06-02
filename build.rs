const COMMANDS: &[&str] = &["share", "share_text", "share_binary"];

fn main() {
    tauri_plugin::Builder::new(COMMANDS)
        .android_path("android")
        .ios_path("ios")
        .build();
}
