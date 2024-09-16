use std::sync::{atomic::Ordering, Arc};

use mouse_position::mouse_position::Mouse;
use rdev::{
    Button,
    EventType::{ButtonPress, ButtonRelease, KeyPress, KeyRelease},
};
use tauri::{App, Emitter, Listener, Manager};
use tokio::sync::Notify;

use crate::{
    common::{self, OLD, PIN, SIMULATION, TMP_PIN},
    shortcut, tray, util, window,
};

pub fn handler(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    let handle = app.handle();

    // 初始化窗口
    // Initialize the window
    window::panel(handle);

    // 初始化托盘
    // Initialize the tray
    tray::init(handle)?;

    let panel = app
        .get_webview_window("panel")
        .expect("Failed to get panel window");

    let key_notifier = Arc::new(Notify::new());
    let mouse_notifier = Arc::new(Notify::new());

    // 监听快捷键
    // Listen for shortcut keys
    tokio::spawn({
        let panel = panel.clone();
        let key_notifier = key_notifier.clone();
        async move {
            loop {
                key_notifier.notified().await;
                // 防止在 panel 中再次翻译
                // Prevent the translation from being repeated in the panel
                if panel.is_focused().unwrap_or(false) {
                    continue;
                }
                if !PIN.load(Ordering::SeqCst) {
                    // 模拟复制获取文本, fallback 到系统剪贴板
                    // Simulate copy and get text, fallback to system clipboard
                    let content = util::content(true);
                    // 临时固定
                    // Temporary pined
                    TMP_PIN.store(true, Ordering::SeqCst);
                    shortcut::show(&panel, content).expect("Shortcut key call failed")
                }
            }
        }
    });

    // 监听划词
    // Listen for word selection
    tokio::spawn({
        let panel = panel.clone();
        let mouse_notifier = mouse_notifier.clone();
        async move {
            loop {
                mouse_notifier.notified().await;
                // 防止在 panel 中再次翻译
                // Prevent the translation from being repeated in the panel
                if panel.is_focused().unwrap_or(false) {
                    continue;
                }
                if PIN.load(Ordering::SeqCst) {
                    // 模拟复制获取文本, 不 fallback 到系统剪贴板
                    // Simulate copy and get text, do not fallback to system clipboard
                    let content = util::content(false);

                    // 避免重复翻译
                    // Avoid repeated translation
                    if content != OLD.read().as_str() {
                        let mut old = OLD.write();
                        old.clone_from(&content);
                        shortcut::show(&panel, content).expect("Selection call failed")
                    }
                }
            }
        }
    });

    // 监听快捷键 和 鼠标操作
    std::thread::spawn({
        let key_notifier = key_notifier.clone();
        let mouse_position = mouse_notifier.clone();
        move || {
            // 检测是否快速按下并抬起按键
            // Check if the key is quickly pressed and released
            let mut fast = 0;
            // 双击
            // Double click
            let mut double = 0;
            // 划词翻译
            // Selection translation
            let mut selected = 0;
            // 双击鼠标左键
            // Double click mouse left
            let mut double_click = 0;
            let mut double_click_x = 0;
            let mut double_click_y = 0;

            // 确定按键
            // Confirm the key
            // let k =
            // let key = Key::ShiftLeft;

            rdev::listen(move |event| match event.event_type {
                KeyPress(k) => {
                    // 如果按键不是设置的按键则忽略
                    // If the key is not the setting key, ignore
                    if k != util::key() {
                        return;
                    }
                    // 如果在模拟中则忽略
                    // If in simulation, ignore
                    if SIMULATION.load(Ordering::SeqCst) {
                        return;
                    }

                    if fast == 0 {
                        let now = util::now();
                        fast = now;
                    }
                }
                KeyRelease(k) => {
                    // 如果按键不是设置的按键则忽略
                    // If the key is not the setting key, ignore
                    if k != util::key() {
                        // 仅处理连续双击按键的情况, 时间满足但中间若有其他按键按下则忽略
                        // Only handle continuous double clicks
                        double = 0;
                        return;
                    }
                    // 如果在模拟中则忽略
                    // If in simulation, ignore
                    if SIMULATION.load(Ordering::SeqCst) {
                        return;
                    }

                    let now = util::now();

                    if now > fast + 500 {
                        fast = 0;
                        return;
                    }
                    fast = 0;

                    let old = double;
                    if now < old + 1000 {
                        key_notifier.notify_waiters();
                        double = 0;
                    } else {
                        double = now;
                    }
                }
                ButtonPress(Button::Left) => {
                    if common::PIN.load(Ordering::SeqCst) {
                        let now = util::now();
                        selected = now;
                    }
                }
                ButtonRelease(Button::Left) => {
                    if common::PIN.load(Ordering::SeqCst) {
                        let now = util::now();

                        let old = selected;
                        if now >= old + 500 {
                            mouse_position.notify_waiters();
                            return;
                        }

                        // 检测双击
                        let old = double_click;
                        let x = double_click_x;
                        let y = double_click_y;

                        let position = Mouse::get_mouse_position();
                        match position {
                            Mouse::Position { x: x1, y: y1 } => {
                                // 判断双击时间间隔
                                // 判断双击是否在同一位置
                                if now < old + 500 && x == x1 && y == y1 {
                                    mouse_notifier.notify_waiters();
                                } else {
                                    double_click = now;
                                    double_click_x = x1;
                                    double_click_y = y1;
                                }
                            }
                            Mouse::Error => println!("Error getting mouse position"),
                        };
                    }
                }
                _ => (),
            })
        }
    });

    // 当panel获取焦点,并移动时, 固定窗口
    // Pin the window when the panel gets focus and moves
    tokio::spawn({
        let panel = panel.clone();
        let check = panel.clone();
        async move {
            panel.listen("tauri://move", move |_| {
                if check.is_focused().unwrap_or(false) {
                    PIN.store(true, Ordering::SeqCst);
                }
            })
        }
    });

    // 检测是否应该隐藏窗口
    // Check if the window should be hidden
    tokio::spawn({
        let panel = panel.clone();
        async move {
            loop {
                if !TMP_PIN.load(Ordering::SeqCst)
                    && !PIN.load(Ordering::SeqCst)
                    && !panel.is_focused().unwrap_or(false)
                {
                    let _ = panel.hide();
                    // 窗口隐藏后, 清空翻译结果
                    // Clear the translation result after the window is hidden
                    let _ = panel.emit("reset", ());
                    PIN.store(false, Ordering::SeqCst)
                }
                tokio::time::sleep(std::time::Duration::from_millis(100)).await;
            }
        }
    });

    Ok(())
}
