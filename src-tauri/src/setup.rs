use std::error::Error;
use std::sync::atomic::Ordering;
use std::time::SystemTime;

use rdev::{
    Button::Left,
    Event,
    EventType::{ButtonPress, ButtonRelease},
};
use tauri::{App, Manager};

use crate::{common::TIME, shortcut, util};

pub fn handler(app: &mut App) -> Result<(), Box<dyn Error>> {
    let panel = app.get_window("panel").expect("Failed to get panel window");

    // 监听鼠标左键释放
    // Listen mouse left button release
    tokio::spawn(async {
        rdev::listen(move |event: Event| match event.event_type {
            ButtonPress(Left) => {
                // 储存 按下时间戳
                // Store press timestamp
                TIME.store(
                    event
                        .time
                        .duration_since(SystemTime::UNIX_EPOCH)
                        .unwrap()
                        .as_millis() as usize,
                    Ordering::Relaxed,
                )
            }
            ButtonRelease(Left) => {
                // 识别选择状态
                // Recognize selection status
                let now = event
                    .time
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_millis() as usize;
                let time = TIME.load(Ordering::Relaxed);
                if now - time < 300 {
                    return;
                }
                shortcut::show(&panel).expect("Shortcut key call failed");
            }
            _ => {}
        })
    });

    Ok(())
}
