use std::sync::atomic::Ordering;

use anyhow::Result;
use tauri::{
    menu::{CheckMenuItem, Menu, MenuEvent, MenuItem, Submenu},
    tray::TrayIconBuilder,
    AppHandle, Manager, Wry,
};

use crate::config::{self, MODE};

/// 初始化托盘菜单
///
/// Initialize tray menu
pub fn init(app: &AppHandle) -> Result<()> {
    let menu = menu(app)?;
    let _ = TrayIconBuilder::with_id("menu")
        .tooltip("Tran")
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu)
        .on_menu_event(handler)
        .build(app);
    Ok(())
}

fn menu(handle: &AppHandle) -> Result<Menu<Wry>> {
    let flag = MODE.load(Ordering::SeqCst);
    let mirror = CheckMenuItem::with_id(handle, "mirror", "Mirror", true, flag, None::<&str>)
        .expect("Failed to create menu item mirror");
    let google = CheckMenuItem::with_id(handle, "google", "Google", true, !flag, None::<&str>)
        .expect("Failed to create menu item google");
    let mode = Submenu::with_items(handle, "Mode", true, &[&mirror, &google])
        .expect("Failed to create submenu item mod.");
    let github = MenuItem::with_id(handle, "github", "GitHub", true, None::<&str>)
        .expect("Failed to create menu item github");
    let telegram = MenuItem::with_id(handle, "telegram", "Telegram", true, None::<&str>)
        .expect("Failed to create menu item telegram");
    let version = MenuItem::with_id(handle, "version", "v.0.2.8", false, None::<&str>)
        .expect("Failed to create menu item version");
    let about = Submenu::with_items(handle, "About", true, &[&github, &telegram, &version])
        .expect("Failed to create submenu item mod.");
    let exit = MenuItem::with_id(handle, "exit", "Exit", true, None::<&str>)
        .expect("Failed to create menu item exit");
    Menu::with_items(handle, &[&mode, &about, &exit])
        .map_err(|_| anyhow::anyhow!("Failed to create menu"))
}

fn fresh(app: &AppHandle) {
    let _ = app.tray().unwrap().set_menu(Some(menu(app).unwrap()));
}

fn handler(app: &AppHandle, event: MenuEvent) {
    match event.id.as_ref() {
        "mirror" => {
            config::mode(true);
        }
        "google" => {
            config::mode(false);
        }
        "github" => {
            let _ = open::that("https://github.com/Borber/Tran");
        }
        "telegram" => {
            let _ = open::that("https://t.me/tran_rust");
        }
        "exit" => {
            let panel = app.get_webview_window("panel").unwrap();
            let _ = panel.hide();
            app.exit(0)
        }
        _ => {}
    }
    fresh(app);
}
