#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use config::{Config, CONFIG};
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
///
/// Translate text
#[tauri::command]
async fn translate(context: String) -> Resp<TransVO> {
    api::translate(&context).await.into()
}

/// 写入剪贴板
///
/// Write to clipboard
#[tauri::command]
fn copy(context: String) -> Resp<()> {
    clip::set(context).into()
}

/// 获取配置
///
/// Get config
#[tauri::command]
fn get_config() -> Resp<Config> {
    Ok(CONFIG.lock().clone()).into()
}

/// 开启代理
///
/// Enable proxy
#[tauri::command]
fn enable_proxy() {
    config::enable_proxy();
}

/// 关闭代理
///
/// Disable proxy
#[tauri::command]
fn disable_proxy() {
    config::disable_proxy();
}

/// 设置代理地址
///
/// Set proxy url
#[tauri::command]
fn set_proxy_url(url: String) {
    config::set_proxy_url(url);
}

#[tokio::main]
async fn main() {
    // 全局初始化
    // Global initialization
    common::init().await;

    tauri::Builder::default()
        .system_tray(tray::new())
        .on_system_tray_event(tray::handler)
        .setup(setup::handler)
        .invoke_handler(tauri::generate_handler![
            copy,
            translate,
            get_config,
            enable_proxy,
            disable_proxy,
            set_proxy_url
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
