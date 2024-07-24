use std::sync::atomic::Ordering;

use anyhow::Result;
use mouse_position::mouse_position::Mouse;
use tauri::{Emitter, LogicalSize, PhysicalPosition, WebviewWindow};

use crate::{
    common::PIN,
    manager::api::{translate, TransVO},
    resp::R,
};

/// 鼠标坐标与选中内容
///
/// Mouse coordinates and selected content
#[derive(Debug, Clone, serde::Serialize)]
pub struct ShowVO {
    pub content: String,
}

pub fn show(panel: &WebviewWindow, content: String) -> Result<()> {
    // 如果内容为空则直接返回
    // If the content is empty, return directly
    if content.is_empty() {
        return Ok(());
    }

    // 将 content 翻译, 翻译结束发送到前端显示事件
    // Translate the content and send it to the front end display event
    let sander = panel.clone();
    tauri::async_runtime::spawn(async move {
        let result = translate(&content).await;
        sander
            .emit::<R<TransVO>>("show", result.into())
            .expect("Failed to emit show event");
    });

    // 如果固定则完成
    // If pined, complete
    if !PIN.load(Ordering::SeqCst) {
        // 未固定则发送事件
        // If not pined, send event
        let position = Mouse::get_mouse_position();
        let (x, y) = match position {
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
                (x, y)
            }
            Mouse::Error => {
                println!("Error getting mouse position");
                (0, 0)
            }
        };

        // 设置窗口位置
        // Set the window position
        panel.set_position(PhysicalPosition { x, y })?;

        // 设置窗口大小
        // Set the window size
        panel.set_size(LogicalSize {
            width: 256,
            height: 100,
        })?;

        // 设置窗口可见
        // Set the window visible
        panel.show()?;

        // 窗口获得焦点
        // Get the window focus
        panel.set_focus()?;
    }

    Ok(())
}
