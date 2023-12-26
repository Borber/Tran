use std::error::Error;

use tauri::{App, GlobalShortcutManager, Manager};

use crate::{shortcut, util};

pub fn handler(app: &mut App) -> Result<(), Box<dyn Error>> {
    if cfg!(any(target_os = "windows", target_os = "macos")) {
        util::decor(app, "panel");
    };
    let panel = app.get_window("panel").expect("Failed to get panel window");

    // 全局快捷键
    // Global shortcut
    app.global_shortcut_manager()
        .register("Alt + X", move || {
            shortcut::show(&panel).expect("Shortcut key call failed")
        })
        .expect("Failed to register global shortcut");
    Ok(())
}
