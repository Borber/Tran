#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use enigo::{Enigo, KeyboardControllable};

mod clip;
mod setup;
mod shortcut;
mod tray;
mod window;

#[tauri::command]
async fn translate() -> String {
    // TODO 发送复制按键
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    let mut enigo = Enigo::new();
    enigo.key_sequence_parse("{+CTRL}c{-CTRL}");
    let context = clip::get();
    println!("context: {:?}", context);
    context
}
//
fn main() {
    tauri::Builder::default()
        .system_tray(tray::new())
        .on_system_tray_event(tray::handler)
        .setup(setup::handler)
        .invoke_handler(tauri::generate_handler![translate])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
