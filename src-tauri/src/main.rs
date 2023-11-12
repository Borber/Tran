#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod api;
mod clip;
mod setup;
mod shortcut;
mod tray;
mod window;

#[tauri::command]
async fn translate(context: String) -> String {
    let result = api::translate(&context).await.unwrap();
    println!("result: {}", result);
    result
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
