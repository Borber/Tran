use tauri::{tray::TrayIconEvent, AppHandle, Manager};

use crate::window;

/// 切换设置窗口显示状态
///
/// Switch setting window display state
pub fn handler(app: &AppHandle, _: TrayIconEvent) {
    match app.get_window("setting") {
        Some(window) => {
            window.hide().expect("Failed to hide main window");
            window.close().expect("Failed to close main window");
        }
        None => window::setting(app),
    }
}
