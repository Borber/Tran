use anyhow::Result;
use mouse_position::mouse_position::Mouse;
use tauri::Manager;
use tauri::WebviewWindow;

use selection::get_text;

/// 鼠标坐标与选中内容
///
/// Mouse coordinates and selected content
#[derive(Debug, Clone, serde::Serialize)]
pub struct ShowVO {
    pub x: i32,
    pub y: i32,
    pub content: String,
    pub pin: bool,
}

pub fn show(panel: &WebviewWindow, pin: bool) -> Result<()> {
    let content = get_text();
    if content.is_empty() {
        return Ok(());
    }

    if pin {
        panel
            .emit(
                "show",
                ShowVO {
                    x: 0,
                    y: 0,
                    content,
                    pin,
                },
            )
            .expect("Failed to emit show event");
    } else {
        let position = Mouse::get_mouse_position();
        match position {
            Mouse::Position { mut x, mut y } => {
                #[cfg(target_os = "macos")]
                {
                    let monitor = panel
                        .current_monitor()
                        .expect("Failed to get panel current monitor")
                        .expect("Panel is none");
                    let scale_factor = monitor.scale_factor();
                    x = (x as f64 * scale_factor) as i32;
                    y = (y as f64 * scale_factor) as i32;
                }

                // 计算偏移量
                // Calculate the offset
                x -= 60;
                y += 20;

                panel
                    .emit("show", ShowVO { x, y, content, pin })
                    .expect("Failed to emit show event");
            }
            Mouse::Error => println!("Error getting mouse position"),
        };
    }

    Ok(())
}
