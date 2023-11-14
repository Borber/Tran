#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use api::TransVO;

mod api;
mod common;
mod lang;
mod setup;
mod shortcut;
mod tray;
mod window;

#[tauri::command]
async fn translate(context: String) -> TransVO {
    match api::translate(&context).await {
        Ok(res) => res,
        Err(_) => TransVO {
            word: false,
            trans: Some("翻译失败".to_string()),
            dicts: None,
        },
    }
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
