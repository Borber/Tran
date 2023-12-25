use tauri::{AppHandle, Manager, SystemTray, SystemTrayEvent};

use crate::window;

pub fn new() -> SystemTray {
    SystemTray::new()
}

/// 切换 main窗口显示状态
///
/// Switch main window display state
pub fn handler(app: &AppHandle, _: SystemTrayEvent) {
    match app.get_window("main") {
        Some(window) => {
            window.close().expect("Failed to close main window");
        }
        None => window::main(app),
    }
}
