#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use config::Config;
use manager::api;
use manager::api::TransVO;
use resp::Resp;

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

/// 更新配置
#[tauri::command]
fn update(config: Config) -> Resp<()> {
    config::update(config).into()
}

#[tokio::main]
async fn main() {
    // 全局初始化
    common::init().await;

    tauri::Builder::default()
        .system_tray(tray::new())
        .on_system_tray_event(tray::handler)
        .setup(setup::handler)
        .invoke_handler(tauri::generate_handler![copy, translate, update])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
