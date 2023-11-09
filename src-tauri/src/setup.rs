use std::error::Error;

use tauri::App;

use crate::window;

pub fn handler(app: &mut App) -> Result<(), Box<dyn Error>> {
    if cfg!(any(target_os = "macos", target_os = "windows")) {
        window::decor(app, "main");
        window::decor(app, "tray");
        window::decor(app, "panel");
    };
    Ok(())
}
