use std::{
    sync::atomic::Ordering,
    thread::{sleep, spawn},
    time::SystemTime,
};

use crossbeam_channel::bounded;
use mouse_position::mouse_position::Mouse;
use rdev::{
    Button,
    EventType::{ButtonPress, ButtonRelease, KeyRelease},
    Key,
};
use tauri::{App, Manager};

use crate::{
    common::{self, PIN},
    shortcut, tray, window,
};

pub fn handler(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    let handle = app.handle();

    // 初始化窗口
    // Initialize the window
    window::panel(handle);

    // 初始化托盘
    // Initialize the tray
    tray::init(handle)?;

    let paneld = app
        .get_webview_window("panel")
        .expect("Failed to get panel window");

    let key_panel = paneld.clone();
    let mouse_panel = paneld.clone();

    let (key_s, key_r) = bounded(1);
    let (mouse_s, mouse_r) = bounded(1);

    // 监听快捷键
    // Listen for shortcut keys
    spawn(move || {
        while let Ok(()) = key_r.recv() {
            if !PIN.load(Ordering::SeqCst) {
                shortcut::show(&key_panel, false).expect("Shortcut key call failed")
            }
        }
    });

    // 监听划词
    // Listen for word selection
    spawn(move || {
        while let Ok(()) = mouse_r.recv() {
            if PIN.load(Ordering::SeqCst) {
                shortcut::show(&mouse_panel, true).expect("Selection call failed")
            }
        }
    });

    // 监听快捷键 和 鼠标操作
    spawn(move || {
        // 双击
        let mut double_cap = 0;
        // 划词翻译
        let mut selected_cap = 0;
        // 双击鼠标左键
        let mut double_click_cap = 0;
        let mut double_click_x = 0;
        let mut double_click_y = 0;

        rdev::listen(move |event| match event.event_type {
            KeyRelease(Key::ShiftLeft) => {
                let old = double_cap;

                let now = SystemTime::now();
                let timestamp = now
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .expect("Time went backwards");
                let now = timestamp.as_millis() as u64;

                if now < old + 1000 {
                    key_s.send(()).expect("Channel send failed");
                    double_cap = 0;
                } else {
                    double_cap = now;
                }
            }
            ButtonPress(Button::Left) => {
                if common::PIN.load(Ordering::SeqCst) {
                    let now = SystemTime::now();
                    let timestamp = now
                        .duration_since(SystemTime::UNIX_EPOCH)
                        .expect("Time went backwards");
                    let now = timestamp.as_millis() as u64;
                    selected_cap = now;
                }
            }
            ButtonRelease(Button::Left) => {
                if common::PIN.load(Ordering::SeqCst) {
                    let now = SystemTime::now();
                    let timestamp = now
                        .duration_since(SystemTime::UNIX_EPOCH)
                        .expect("Time went backwards");
                    let now = timestamp.as_millis() as u64;

                    let old = selected_cap;
                    if now >= old + 500 {
                        mouse_s.send(()).expect("Channel send failed");
                        return;
                    }

                    // 检测双击
                    let old = double_click_cap;
                    let x = double_click_x;
                    let y = double_click_y;

                    let position = Mouse::get_mouse_position();
                    match position {
                        Mouse::Position { x: x1, y: y1 } => {
                            // 判断双击时间间隔
                            // 判断双击是否在同一位置
                            if now < old + 500 && x == x1 && y == y1 {
                                mouse_s.send(()).expect("Channel send failed");
                            } else {
                                double_click_cap = now;
                                double_click_x = x1;
                                double_click_y = y1;
                            }
                        }
                        Mouse::Error => println!("Error getting mouse position"),
                    };
                }
            }
            KeyRelease(_) => {
                // 仅处理连续双击按键的情况, 时间满足但中间若有其他按键按下则忽略
                // Only handle continuous double clicks
                double_cap = 0;
            }

            _ => (),
        })
    });

    let panel = paneld.clone();

    let sender = paneld.clone();

    // 监听panel移动
    spawn(move || {
        panel.listen("tauri://move", move |_| {
            if !PIN.load(Ordering::SeqCst) {
                PIN.store(true, Ordering::SeqCst);

                // 发送通知, 避免拖动翻译窗口后直接双击关闭固定窗口
                // Send notification to avoid dragging the translation window to close the pinned window
                sender.emit("pin", ()).expect("Failed to emit pin event");
            }
        })
    });

    let panel = paneld.clone();

    // 监听panel焦点
    // Listen for panel focus
    spawn(move || {
        loop {
            if !PIN.load(Ordering::SeqCst) && !panel.is_focused().unwrap_or(false) {
                let _ = panel.hide();
                // 窗口隐藏后, 清空翻译结果
                // Clear the translation result after the window is hidden
                let _ = panel.emit("clean", ());
                PIN.store(false, Ordering::SeqCst)
            }
            sleep(std::time::Duration::from_millis(100));
        }
    });

    Ok(())
}
