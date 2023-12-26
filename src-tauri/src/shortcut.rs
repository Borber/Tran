use anyhow::Result;
use mouse_position::mouse_position::Mouse;
use tauri::Window;

use selection::get_text;

/// 鼠标坐标与选中内容
///
/// Mouse coordinates and selected content
#[derive(Debug, Clone, serde::Serialize)]
pub struct ShowVO {
    pub x: i32,
    pub y: i32,
    pub context: String,
}

pub fn show(panel: &Window) -> Result<()> {
    let context = get_text();

    if context.is_empty() {
        return Ok(());
    }

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
            x -= 40;
            y += 20;

            // 边界检查
            // Boundary check
            if x < 0 {
                x = 0;
            }
            panel.emit("show", ShowVO { x, y, context }).unwrap();
        }
        Mouse::Error => println!("Error getting mouse position"),
    };
    Ok(())
}
