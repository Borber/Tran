#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use api::TransVO;
use config::save;
use resp::Resp;

mod api;
mod clip;
mod common;
mod config;
mod lang;
mod manager;
mod resp;
mod setup;
mod shortcut;
mod tray;
mod util;
mod window;

/// 翻译文本
#[tauri::command]
async fn translate(context: String) -> Resp<TransVO> {
    save().unwrap();
    api::translate(&context).await.into()
}

/// 写入剪贴板
#[tauri::command]
fn copy(context: String) -> Result<(), String> {
    match clip::set(context) {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}

#[tokio::main]
async fn main() {
    // 全局初始化
    common::init().await;

    tauri::Builder::default()
        .system_tray(tray::new())
        .on_system_tray_event(tray::handler)
        .setup(setup::handler)
        .invoke_handler(tauri::generate_handler![copy, translate])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
