use std::path::PathBuf;
use tauri::{App, Manager};

/// 获取可执行文件位置
///
/// Get executable file path
pub fn get_exe_dir() -> PathBuf {
    std::env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf()
}

/// 增加窗口装饰 依赖于 `window_shadows`
///
/// Adds window decorations, depends on `window_shadows`
pub fn decor_by_window(window: &tauri::Window) {
    if cfg!(any(target_os = "macos", target_os = "windows")) {
        match window_shadows::set_shadow(window, true) {
            Ok(_) => {}
            Err(e) => {
                println!("Error setting shadow: {}", e);
            }
        }
    };
}

/// 通过窗口名称增加窗口装饰 依赖于 `window_shadows`
///
/// Adds window decorations by window name, depends on `window_shadows`
pub fn decor(app: &mut App, name: &str) {
    match app.get_window(name) {
        Some(window) => decor_by_window(&window),
        None => println!("Decor: window `{}` not found", name),
    }
}
