use std::error::Error;

use tauri::{App, GlobalShortcutManager, Manager};

use crate::{shortcut, window};

pub fn handler(app: &mut App) -> Result<(), Box<dyn Error>> {
    if cfg!(any(target_os = "macos", target_os = "windows")) {
        window::decor(app, "main");
        window::decor(app, "panel");
    };
    let panel = app.get_window("panel").unwrap();

    // 全局快捷键
    // Global shortcut
    app.global_shortcut_manager()
        .register("Alt + X", move || shortcut::show(&panel).unwrap())
        .expect("Failed to register global shortcut");
    Ok(())
}
