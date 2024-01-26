use std::{
    error::Error,
    sync::{atomic::AtomicUsize, Arc},
    time::SystemTime,
};

use rdev::{listen, EventType::KeyRelease, Key::CapsLock};
use tauri::{App, Manager};

use crate::{shortcut, util};

pub fn handler(app: &mut App) -> Result<(), Box<dyn Error>> {
    if cfg!(any(target_os = "windows", target_os = "macos")) {
        util::decor(app, "panel");
    };
    let panel = app.get_window("panel").expect("Failed to get panel window");

    let cap = Arc::new(AtomicUsize::new(0));

    std::thread::spawn(move || {
        listen(move |event| {
            if let KeyRelease(CapsLock) = event.event_type {
                let now = SystemTime::now();
                let timestamp = now
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .expect("Time went backwards");
                let now = timestamp.as_millis() as usize;
                let old = cap.load(std::sync::atomic::Ordering::Relaxed);
                if now - old < 1000 {
                    shortcut::show(&panel).expect("Shortcut key call failed")
                }
                cap.store(now, std::sync::atomic::Ordering::Relaxed)
            }
        })
    });

    // 全局快捷键
    // Global shortc    ut
    // app.global_shortcut_manager()
    //     .register("Alt + X", move || {
    //         shortcut::show(&panel).expect("Shortcut key call failed")
    //     })
    //     .expect("Failed to register global shortcut");
    Ok(())
}
