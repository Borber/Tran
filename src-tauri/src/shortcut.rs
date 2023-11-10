use mouse_position::mouse_position::Mouse;
use tauri::{PhysicalPosition, Window};

pub fn show(panel: &Window) {
    let position = Mouse::get_mouse_position();
    match position {
        Mouse::Position { x, y } => {
            let pos = PhysicalPosition { x, y };
            panel.emit("show", pos).unwrap();
        }
        Mouse::Error => println!("Error getting mouse position"),
    }
}
