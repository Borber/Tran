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
    let f = MODE.load(Ordering::SeqCst);
    let k = config::KEY.load(Ordering::SeqCst);
    let mirror = CheckMenuItem::with_id(handle, "mirror", "Mirror", true, f, None::<&str>)?;
    let google = CheckMenuItem::with_id(handle, "google", "Google", true, !f, None::<&str>)?;
    let mode = Submenu::with_items(handle, "Mode", true, &[&mirror, &google])?;
    let shift = CheckMenuItem::with_id(handle, "shift", "Shift", true, k == 0, None::<&str>)?;
    let ctrl = CheckMenuItem::with_id(handle, "ctrl", "Ctrl", true, k == 1, None::<&str>)?;
    let caps = CheckMenuItem::with_id(handle, "caps", "Caps", true, k == 2, None::<&str>)?;
    let key = Submenu::with_items(handle, "Key", true, &[&shift, &ctrl, &caps])?;
    let github = MenuItem::with_id(handle, "github", "GitHub", true, None::<&str>)?;
    let telegram = MenuItem::with_id(handle, "telegram", "Telegram", true, None::<&str>)?;
    let version = MenuItem::with_id(handle, "version", "v0.2.14", false, None::<&str>)?;
    let about = Submenu::with_items(handle, "About", true, &[&github, &telegram, &version])?;
    let exit = MenuItem::with_id(handle, "exit", "Exit", true, None::<&str>)?;
    Menu::with_items(handle, &[&mode, &key, &about, &exit])
        .map_err(|_| anyhow::anyhow!("Failed to create menu"))
}

fn fresh(app: &AppHandle) {
    let _ = app
        .tray_by_id("menu")
        .unwrap()
        .set_menu(Some(menu(app).unwrap()));
}

fn handler(app: &AppHandle, event: MenuEvent) {
    match event.id.as_ref() {
        "mirror" => {
            config::mode(true);
        }
        "google" => {
            config::mode(false);
        }
        "shift" => {
            config::key(0);
        }
        "ctrl" => {
            config::key(1);
        }
        "caps" => {
            config::key(2);
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
