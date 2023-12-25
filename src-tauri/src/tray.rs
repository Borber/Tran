use anyhow::Result;
use mouse_position::mouse_position::Mouse;
use tauri::{AppHandle, LogicalPosition, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu};

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
                // 定义Tray大小
                // Define the window size
                let width = 140;
                let height = 60;

                // 通过主窗口来获取显示器尺寸
                // Get the monitor size from the main window
                let main = app.get_window("main").unwrap();
                let (scale_factor, m_width, m_height) = match main.current_monitor() {
                    Ok(Some(monitor)) => {
                        let scale_factor = monitor.scale_factor();
                        let size = monitor.size();
                        (
                            scale_factor,
                            (size.width as f64 / scale_factor) as i32,
                            (size.height as f64 / scale_factor) as i32,
                        )
                    }
                    _ => (1.0, 0, 0),
                };

                #[cfg(target_os = "windows")]
                {
                    x = (x as f64 / scale_factor) as i32;
                    y = (y as f64 / scale_factor) as i32;
                }

                // 通过鼠标位置判断托盘区位置
                // Determine the position of the tray based on the mouse position
                // 右下角
                // Right-bottom
                if x >= m_width / 2 && y >= m_height / 2 {
                    // 窗口上移, 左移
                    // Move the window up and left
                    x -= 10;
                    y = y - height + 10;
                } else if x >= m_width / 2 && y < m_height / 2 {
                    x -= 10;
                    y += 10;
                } else if y >= m_height / 2 {
                    x += 10;
                    y = y - height + 10;
                } else {
                    x += 10;
                    y += 10;
                }

                // 边界检查
                // Boundary check
                if x < 0 {
                    x = 0;
                }
                if x > m_width - width {
                    x = m_width - width;
                }
                if y < 0 {
                    y = 0;
                }
                if y > m_height - height {
                    y = m_height - height;
                }

                // 判断 tray 是否存在
                // Determine if the tray exists
                match app.get_window("tray") {
                    Some(tray) => {
                        tray.set_position(LogicalPosition::new(x, y))
                            .expect("Failed to set tray position");
                    }
                    None => {
                        tray(app, width, height, x, y).expect("Failed to create tray");
                    }
                }
            }
            Mouse::Error => println!("Error getting mouse position"),
        }
    }
}

/// 创建Tray窗口
///
/// Create Tray window
pub fn tray(app: &AppHandle, width: i32, height: i32, x: i32, y: i32) -> Result<()> {
    tauri::WindowBuilder::new(app, "tray", tauri::WindowUrl::App("/tray".into()))
        .inner_size(width as f64, height as f64)
        .fullscreen(false)
        .resizable(false)
        .decorations(false)
        .transparent(true)
        .visible(true)
        .always_on_top(true)
        .skip_taskbar(true)
        .position(x as f64, y as f64)
        .build()?;
    // TODO 可能需要将计算位置调整到这里, 以解决不同显示器之间的问题
    Ok(())
}
