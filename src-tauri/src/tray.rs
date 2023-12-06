use mouse_position::mouse_position::Mouse;
use tauri::{AppHandle, Manager, PhysicalPosition, SystemTray, SystemTrayEvent, SystemTrayMenu};

pub fn new() -> SystemTray {
    let tray_menu = SystemTrayMenu::new();
    SystemTray::new().with_menu(tray_menu)
}

pub fn handler(app: &AppHandle, event: SystemTrayEvent) {
    // 右键点击时, 显示托盘菜单
    // Right-click to show the tray menu
    if let SystemTrayEvent::RightClick { .. } = event {
        let position = Mouse::get_mouse_position();
        match position {
            Mouse::Position { mut x, mut y } => {
                let tray = app.get_window("tray").unwrap();

                // 获取窗口大小
                // Get the window size
                let (w_width, w_height) = match tray.outer_size() {
                    Ok(size) => (size.width as i32, size.height as i32),
                    Err(_) => (140, 60),
                };

                // 获取窗口大小
                let (width, height) = match tray.current_monitor() {
                    Ok(Some(monitor)) => {
                        (monitor.size().width as i32, monitor.size().height as i32)
                    }
                    _ => (0, 0),
                };

                // 通过鼠标位置判断托盘区位置
                // Determine the position of the tray based on the mouse position
                // 右下角
                // Right-bottom
                if x >= width / 2 && y >= height / 2 {
                    // 窗口上移, 左移
                    // Move the window up and left
                    x -= 10;
                    y = y - w_height + 10;
                } else if x >= width / 2 && y < height / 2 {
                    x -= 10;
                    y += 10;
                } else if y >= height / 2 {
                    x += 10;
                    y = y - w_height + 10;
                } else {
                    x += 10;
                    y += 10;
                }

                // 边界检查
                // Boundary check
                if x < 0 {
                    x = 0;
                }
                if x > width - w_width {
                    x = width - w_width;
                }
                if y < 0 {
                    y = 0;
                }
                if y > height - w_height {
                    y = height - w_height;
                }

                let pos = PhysicalPosition { x, y };
                tray.emit("tray", pos).unwrap();
            }
            Mouse::Error => println!("Error getting mouse position"),
        }
    }
}
