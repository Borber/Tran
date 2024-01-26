use std::{
    error::Error,
    sync::{
        atomic::{AtomicU32, AtomicU64},
        Arc,
    },
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

    let sec = Arc::new(AtomicU64::new(0));
    let milli = Arc::new(AtomicU32::new(0));

    std::thread::spawn(move || {
        listen(move |event| {
            if let KeyRelease(CapsLock) = event.event_type {
                let old_sec = sec.load(std::sync::atomic::Ordering::SeqCst);
                let old_milli = milli.load(std::sync::atomic::Ordering::SeqCst);

                let now = SystemTime::now();
                let timestamp = now
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .expect("Time went backwards");
                let now_sec = timestamp.as_secs();
                let now_milli = timestamp.subsec_millis();

                if now_sec - old_sec == 0 || now_sec - old_sec == 1 && now_milli < old_milli {
                    shortcut::show(&panel).expect("Shortcut key call failed")
                }

                sec.store(now_sec, std::sync::atomic::Ordering::SeqCst);
                milli.store(now_milli, std::sync::atomic::Ordering::SeqCst);
            }
        })
    });
    Ok(())
}
