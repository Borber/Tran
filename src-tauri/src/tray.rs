use tauri::{AppHandle, Manager, SystemTray, SystemTrayEvent};

pub fn new() -> SystemTray {
    SystemTray::new()
}

pub fn handler(app: &AppHandle, _: SystemTrayEvent) {
    let window = app.get_window("main").expect("Failed to get main window");

    if window.is_visible().unwrap_or_default() {
        let _ = window.hide();
    } else {
        let _ = window.show();
        let _ = window.set_focus();
    }
}
