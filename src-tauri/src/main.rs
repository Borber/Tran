#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod setup;
mod shortcut;
mod tray;
mod window;

#[tauri::command]
fn translate() -> String {
    "Hello, world!".to_string()
}

fn main() {
    tauri::Builder::default()
        .system_tray(tray::new())
        .on_system_tray_event(tray::handler)
        .setup(setup::handler)
        .invoke_handler(tauri::generate_handler![translate])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
