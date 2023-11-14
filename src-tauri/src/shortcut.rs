use enigo::{Enigo, KeyboardControllable};
use mouse_position::mouse_position::Mouse;
use tauri::{PhysicalPosition, Window};

use crate::clip;

/// 鼠标坐标与选中内容
#[derive(Debug, Clone, serde::Serialize)]
pub struct ShowVO {
    pub x: i32,
    pub y: i32,
    pub context: String,
}

pub fn show(panel: &Window) {
    // TODO 复制之前保存剪贴板内容
    let mut enigo = Enigo::new();
    enigo.key_sequence_parse("{ALT}{+CTRL}c{-CTRL}{ALT}");
    let context = clip::get();
    // TODO 获取之后恢复剪贴板内容
    let position = Mouse::get_mouse_position();
    match position {
        Mouse::Position { x, y } => {
            let pos = PhysicalPosition { x, y };
            panel
                .emit(
                    "show",
                    ShowVO {
                        x: pos.x,
                        y: pos.y,
                        context,
                    },
                )
                .unwrap();
        }
        Mouse::Error => println!("Error getting mouse position"),
    }
}
