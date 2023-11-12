use enigo::{Enigo, KeyboardControllable};
use mouse_position::mouse_position::Mouse;
use tauri::{PhysicalPosition, Window};

use crate::clip;

#[derive(Debug, Clone, serde::Serialize)]
pub struct ShowVO {
    pub x: i32,
    pub y: i32,
    pub context: String,
}

pub fn show(panel: &Window) {
    let mut enigo = Enigo::new();
    enigo.key_sequence_parse("{ALT}{+CTRL}c{-CTRL}{ALT}");
    let context = clip::get();
    println!("context: {}", context);
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
