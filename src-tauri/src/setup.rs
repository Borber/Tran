use std::{
    error::Error,
    sync::{atomic::AtomicU64, Arc},
    time::SystemTime,
};

use crossbeam_channel::bounded;
use rdev::{listen, EventType::KeyRelease, Key::CapsLock};
use tauri::{App, Manager};

use crate::{shortcut, util};

pub fn handler(app: &mut App) -> Result<(), Box<dyn Error>> {
    if cfg!(any(target_os = "windows", target_os = "macos")) {
        util::decor(app, "panel");
    };
    let panel = app.get_window("panel").expect("Failed to get panel window");

    let cap = Arc::new(AtomicU64::new(0));
    let (s, r) = bounded(1);

    std::thread::spawn(move || {
        while let Ok(()) = r.recv() {
            shortcut::show(&panel).expect("Shortcut key call failed")
        }
    });

    std::thread::spawn(|| {
        listen(move |event| {
            if let KeyRelease(CapsLock) = event.event_type {
                let old = cap.load(std::sync::atomic::Ordering::SeqCst);

                let now = SystemTime::now();
                let timestamp = now
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .expect("Time went backwards");
                let now = timestamp.as_millis() as u64;

                if now < old + 1000 {
                    s.send(()).expect("Channel send failed");
                    cap.store(0, std::sync::atomic::Ordering::SeqCst);
                } else {
                    cap.store(now, std::sync::atomic::Ordering::SeqCst);
                }
            }
        })
    });

    Ok(())
}
