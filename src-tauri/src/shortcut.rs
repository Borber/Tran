use std::sync::atomic::Ordering;

use active_win_pos_rs::get_active_window;
use anyhow::Result;
use mouse_position::mouse_position::Mouse;
use tauri::Manager;
use tauri::WebviewWindow;

use selection::get_text;

use crate::clip;
use crate::common::{OLD, PIN};

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
    // 避免拖动翻译窗口导致触发翻译
    match get_active_window() {
        Ok(active_window) => {
            if active_window.title == "Tran" {
                return Ok(());
            }
        }
        Err(_) => {
            println!("error occurred while getting the active window");
        }
    }

    let s_copy = get_text();

    let content = if s_copy.is_empty() {
        // 获取系统剪贴板内容
        let copy = clip::get()?;

        if copy.is_empty() {
            return Ok(());
        } else {
            copy
        }
    } else {
        s_copy
    };

    if PIN.load(Ordering::SeqCst) && content == OLD.read().as_str() {
        // 避免重复翻译
        return Ok(());
    } else {
        let mut old = OLD.write();
        *old = content.clone();
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

                // 应该暂时保证窗口不被关闭
                // pin when shortcut
                PIN.store(true, Ordering::SeqCst);

                panel
                    .emit("show", ShowVO { x, y, content, pin })
                    .expect("Failed to emit show event");
            }
            Mouse::Error => println!("Error getting mouse position"),
        };
    }

    Ok(())
}
