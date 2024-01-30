use std::{
    error::Error,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
    time::SystemTime,
};

use crossbeam_channel::bounded;
use rdev::{
    listen, Button,
    EventType::{ButtonPress, ButtonRelease, KeyRelease},
    Key::CapsLock,
};
use tauri::{App, Manager};

use crate::{common, shortcut, tray, window};

pub fn handler(app: &mut App) -> Result<(), Box<dyn Error>> {
    app.on_tray_icon_event(tray::handler);

    let handle = app.handle();

    window::panel(handle);

    let key_panel = app.get_window("panel").expect("Failed to get panel window");
    let mouse_panel = key_panel.clone();

    let key_cap = Arc::new(AtomicU64::new(0));
    let mouse_cap = Arc::new(AtomicU64::new(0));

    let (key_s, key_r) = bounded(1);
    let (mouse_s, mouse_r) = bounded(1);

    // 监听快捷键
    tokio::spawn(async move {
        while let Ok(()) = key_r.recv() {
            shortcut::show(&key_panel).expect("Shortcut key call failed")
        }
    });

    // 监听划词
    tokio::spawn(async move {
        while let Ok(()) = mouse_r.recv() {
            shortcut::show(&mouse_panel).expect("Shortcut key call failed")
        }
    });

    tokio::spawn(async {
        listen(move |event| match event.event_type {
            KeyRelease(CapsLock) => {
                if !common::PIN.load(Ordering::SeqCst) {
                    let old = key_cap.load(std::sync::atomic::Ordering::SeqCst);

                    let now = SystemTime::now();
                    let timestamp = now
                        .duration_since(SystemTime::UNIX_EPOCH)
                        .expect("Time went backwards");
                    let now = timestamp.as_millis() as u64;

                    if now < old + 1000 {
                        key_s.send(()).expect("Channel send failed");
                        key_cap.store(0, std::sync::atomic::Ordering::SeqCst);
                    } else {
                        key_cap.store(now, std::sync::atomic::Ordering::SeqCst);
                    }
                }
            }
            ButtonPress(Button::Left) => {
                if common::PIN.load(Ordering::SeqCst) {
                    let now = SystemTime::now();
                    let timestamp = now
                        .duration_since(SystemTime::UNIX_EPOCH)
                        .expect("Time went backwards");
                    let now = timestamp.as_millis() as u64;
                    mouse_cap.store(now, Ordering::SeqCst);
                }
            }
            ButtonRelease(Button::Left) => {
                if common::PIN.load(Ordering::SeqCst) {
                    let now = SystemTime::now();
                    let timestamp = now
                        .duration_since(SystemTime::UNIX_EPOCH)
                        .expect("Time went backwards");
                    let now = timestamp.as_millis() as u64;

                    let old = mouse_cap.load(Ordering::SeqCst);
                    // TODO This time may need to be adjusted
                    if now > old + 500 {
                        mouse_s.send(()).expect("Channel send failed");
                    }
                }
            }
            _ => (),
        })
    });

    Ok(())
}
