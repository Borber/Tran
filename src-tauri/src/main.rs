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

/// 切换模式
///
/// Switch mode
#[tauri::command]
fn switch_mode(mode: usize) {
    config::mode(mode);
}

/// 设置代理地址
///
/// Set proxy url
#[tauri::command]
fn set_proxy_url(url: String) {
    config::set_proxy_url(url);
}

/// 打开Github
///
/// Open Github
#[tauri::command]
async fn open_github() -> Resp<()> {
    open::that("https://github.com/Borber/tran")
        .map_err(anyhow::Error::msg)
        .into()
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
            open_github,
            translate,
            get_config,
            switch_mode,
            set_proxy_url
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
