#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use manager::api;
use manager::api::TransVO;
use resp::Resp;
use tauri::{
    utils::acl::{
        resolved::{CommandKey, ResolvedCommand},
        ExecutionContext,
    },
    AppHandle,
};

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
async fn translate(content: String) -> Resp<TransVO> {
    api::translate(&content).await.into()
}

/// 写入剪贴板
///
/// Write to clipboard
#[tauri::command]
fn copy(content: String) -> Resp<()> {
    clip::set(content).into()
}

/// 切换模式
///
/// Switch mode
#[tauri::command]
fn switch_mode(mode: usize) {
    config::mode(mode);
}

/// 打开指定链接
///
/// Open the specified link
#[tauri::command]
async fn open(url: String) -> Resp<()> {
    open::that(url).map_err(anyhow::Error::msg).into()
}

/// 固定窗口标识
#[tauri::command]
async fn pin(state: bool) {
    common::PIN.store(state, std::sync::atomic::Ordering::SeqCst);
}

/// 检测更新
#[tauri::command]
async fn update(app: AppHandle) -> Resp<bool> {
    let old = app.package_info().version.to_string();
    manager::check::update(&old).await.into()
}

#[tokio::main]
async fn main() {
    // 全局初始化
    // Global initialization
    common::init().await;

    let mut context = tauri::generate_context!("tauri.conf.json");

    for cmd in [
        "plugin:app|version",
        "plugin:event|listen",
        "plugin:window|show",
        "plugin:window|hide",
        "plugin:window|set_size",
        "plugin:window|set_focus",
        "plugin:window|set_position",
        "plugin:window|internal_on_mousemove",
        "plugin:window|internal_on_mousedown",
        "plugin:window|start_dragging",
    ] {
        context.resolved_acl().allowed_commands.insert(
            CommandKey {
                name: cmd.into(),
                context: ExecutionContext::Local,
            },
            ResolvedCommand {
                windows: vec!["*".parse().unwrap()],
                ..Default::default()
            },
        );
    }

    tauri::Builder::default()
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_single_instance::init(|_, _, _| {}))
        .setup(setup::handler)
        .invoke_handler(tauri::generate_handler![
            copy,
            open,
            translate,
            switch_mode,
            pin,
            update,
        ])
        .run(context)
        .expect("error while running tauri application");
}
