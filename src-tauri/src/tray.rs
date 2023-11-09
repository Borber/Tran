use mouse_position::mouse_position::Mouse;
use tauri::{AppHandle, Manager, PhysicalPosition, SystemTray, SystemTrayEvent, SystemTrayMenu};

pub fn new() -> SystemTray {
    let tray_menu = SystemTrayMenu::new();
    SystemTray::new().with_menu(tray_menu)
}

pub fn handler(app: &AppHandle, event: SystemTrayEvent) {
    // 右键点击时, 显示托盘菜单
    //
    // Right-click to show the tray menu
    if let SystemTrayEvent::RightClick { .. } = event {
        let position = Mouse::get_mouse_position();
        match position {
            Mouse::Position { x, y } => {
                let tray = app.get_window("tray").unwrap();
                let size = tray.outer_size().unwrap();
                let pos = PhysicalPosition {
                    x: x - 10,
                    y: y - size.height as i32 + 10,
                };
                tray.emit("tray", pos).unwrap();
            }
            Mouse::Error => println!("Error getting mouse position"),
        }
    }
}
