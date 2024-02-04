use anyhow::Result;
use tauri::{
    menu::{Menu, MenuEvent, MenuItem, Submenu},
    tray::TrayIconBuilder,
    AppHandle, Manager, Wry,
};

use crate::config;

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
    let github = MenuItem::with_id(handle, "github", "GitHub", true, None::<&str>)
        .expect("Failed to create menu item github");
    let mirror = MenuItem::with_id(handle, "mirror", "Mirror", true, None::<&str>)
        .expect("Failed to create menu item mirror");
    let google = MenuItem::with_id(handle, "google", "Google", true, None::<&str>)
        .expect("Failed to create menu item google");
    let mode = Submenu::with_items(handle, "Mode", true, &[&mirror, &google])
        .expect("Failed to create submenu item mod.");
    let exit = MenuItem::with_id(handle, "exit", "Exit", true, None::<&str>)
        .expect("Failed to create menu item exit");
    Menu::with_items(handle, &[&github, &mode, &exit])
        .map_err(|_| anyhow::anyhow!("Failed to create menu"))
}

fn handler(app: &AppHandle, event: MenuEvent) {
    match event.id.as_ref() {
        "github" => {
            let _ = open::that("https://github.com/Borber/Tran");
        }
        "mirror" => config::mode(0),
        "google" => config::mode(1),
        "exit" => {
            let panel = app.get_webview_window("panel").unwrap();
            let _ = panel.hide();
            app.exit(0)
        }
        _ => {}
    }
}
