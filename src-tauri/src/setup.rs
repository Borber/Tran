use std::{
    sync::{
        atomic::{AtomicI32, AtomicU64, Ordering},
        Arc,
    },
    time::SystemTime,
};

use crossbeam_channel::bounded;
use mouse_position::mouse_position::Mouse;
use rdev::{
    Button,
    EventType::{ButtonPress, ButtonRelease, KeyRelease},
    Key::CapsLock,
};
use tauri::{App, Manager};
use tokio::time::sleep;

use crate::{
    common::{self, PIN},
    shortcut, tray, window,
};

pub fn handler(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    let handle = app.handle();

    window::panel(handle);

    tray::init(handle)?;

    let key_panel = app
        .get_webview_window("panel")
        .expect("Failed to get panel window");
    let mouse_panel = key_panel.clone();

    let (key_s, key_r) = bounded(1);
    let (mouse_s, mouse_r) = bounded(1);

    // 监听快捷键
    tokio::spawn(async move {
        while let Ok(()) = key_r.recv() {
            // pin when shortcut
            // 在快捷键调用时, 应该暂时保证窗口不被关闭
            common::PIN.store(true, Ordering::SeqCst);
            shortcut::show(&key_panel, false).expect("Shortcut key call failed")
        }
    });

    // 监听划词
    tokio::spawn(async move {
        while let Ok(()) = mouse_r.recv() {
            shortcut::show(&mouse_panel, true).expect("Shortcut key call failed")
        }
    });

    // 监听快捷键 和 鼠标操作
    tokio::spawn(async {
        // 双击capslock
        let double_capslock_cap = Arc::new(AtomicU64::new(0));
        // 划词翻译
        let selected_cap = Arc::new(AtomicU64::new(0));
        // 双击鼠标左键
        let double_click_cap = Arc::new(AtomicU64::new(0));
        let double_click_x = Arc::new(AtomicI32::new(0));
        let double_click_y = Arc::new(AtomicI32::new(0));

        rdev::listen(move |event| match event.event_type {
            KeyRelease(CapsLock) => {
                let old = double_capslock_cap.load(std::sync::atomic::Ordering::SeqCst);

                let now = SystemTime::now();
                let timestamp = now
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .expect("Time went backwards");
                let now = timestamp.as_millis() as u64;

                if now < old + 1000 {
                    key_s.send(()).expect("Channel send failed");
                    double_capslock_cap.store(0, std::sync::atomic::Ordering::SeqCst);
                } else {
                    double_capslock_cap.store(now, std::sync::atomic::Ordering::SeqCst);
                }
            }
            ButtonPress(Button::Left) => {
                if common::PIN.load(Ordering::SeqCst) {
                    let now = SystemTime::now();
                    let timestamp = now
                        .duration_since(SystemTime::UNIX_EPOCH)
                        .expect("Time went backwards");
                    let now = timestamp.as_millis() as u64;
                    selected_cap.store(now, Ordering::SeqCst);
                }
            }
            ButtonRelease(Button::Left) => {
                if common::PIN.load(Ordering::SeqCst) {
                    let now = SystemTime::now();
                    let timestamp = now
                        .duration_since(SystemTime::UNIX_EPOCH)
                        .expect("Time went backwards");
                    let now = timestamp.as_millis() as u64;

                    let old = selected_cap.load(Ordering::SeqCst);
                    if now >= old + 500 {
                        mouse_s.send(()).expect("Channel send failed");
                        return;
                    }

                    // 检测双击
                    let old = double_click_cap.load(Ordering::SeqCst);
                    let x = double_click_x.load(Ordering::SeqCst);
                    let y = double_click_y.load(Ordering::SeqCst);

                    let position = Mouse::get_mouse_position();
                    match position {
                        Mouse::Position { x: x1, y: y1 } => {
                            // 判断双击时间间隔
                            // 判断双击是否在同一位置
                            if now < old + 500 && x == x1 && y == y1 {
                                mouse_s.send(()).expect("Channel send failed");
                            } else {
                                double_click_cap.store(now, Ordering::SeqCst);
                                double_click_x.store(x1, Ordering::SeqCst);
                                double_click_y.store(y1, Ordering::SeqCst);
                            }
                        }
                        Mouse::Error => println!("Error getting mouse position"),
                    };
                }
            }
            _ => (),
        })
    });

    let panel = app
        .get_webview_window("panel")
        .expect("Failed to get panel window");

    // 监听panel移动
    tokio::spawn(async move {
        panel.listen("tauri://move", move |_| {
            if !PIN.load(Ordering::SeqCst) {
                PIN.store(true, Ordering::SeqCst)
            }
        })
    });

    let panel = app
        .get_webview_window("panel")
        .expect("Failed to get panel window");
    // 监听panel焦点
    tokio::spawn(async move {
        loop {
            if !PIN.load(Ordering::SeqCst) && !panel.is_focused().unwrap_or(false) {
                let _ = panel.hide();
                PIN.store(false, Ordering::SeqCst)
            }
            sleep(std::time::Duration::from_millis(100)).await;
        }
    });

    Ok(())
}
