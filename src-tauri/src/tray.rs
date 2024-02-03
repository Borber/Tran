use anyhow::Result;
use tauri::{
    menu::{Menu, MenuEvent, MenuItem},
    tray::TrayIconBuilder,
    AppHandle, Manager, Wry,
};

use crate::config;

/// 切换设置窗口显示状态
///
/// Switch setting window display state
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

fn menu(handle: &AppHandle) -> Result<Menu<Wry>, tauri::Error> {
    let mirror = MenuItem::with_id(handle, "mirror", "Mirror", true, None::<&str>);
    let google = MenuItem::with_id(handle, "google", "Google", true, None::<&str>);
    let exit = MenuItem::with_id(handle, "exit", "Exit", true, None::<&str>);
    Menu::with_items(handle, &[&mirror, &google, &exit])
}

fn handler(app: &AppHandle, event: MenuEvent) {
    match event.id.as_ref() {
        "mirror" => config::mode(0),
        "google" => config::mode(1),
        "exit" => {
            let panel = app.get_window("panel").unwrap();
            let _ = panel.hide();
            app.exit(0)
        }
        _ => {}
    }
}
