use std::{path::PathBuf, sync::atomic::Ordering, time::SystemTime};

use selection::get_text;

use crate::{clip, common::SIMULATION};

/// 模拟获取复制文本
///
/// Simulate getting copy text
pub fn content(fallback: bool) -> String {
    // 更改模拟标识, 按键监听
    // Change the simulation flag, trigger the key
    SIMULATION.store(true, Ordering::SeqCst);
    // 模拟复制获取文本
    // Simulate copy and get text
    let content = get_text();
    // 重置模拟标识
    // Reset the simulation flag
    SIMULATION.store(false, Ordering::SeqCst);

    if content.is_empty() && fallback {
        // 获取系统剪贴板内容
        match clip::get() {
            Ok(copy) => copy,
            Err(e) => {
                println!("error occurred while getting clipboard content: {:?}", e);
                String::default()
            }
        }
    } else {
        content
    }
}

/// 获取当前时间
///
/// Get current time
pub fn now() -> u64 {
    let now = SystemTime::now();
    let timestamp = now
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("Time went backwards");
    timestamp.as_millis() as u64
}

/// 获取可执行文件位置
///
/// Get executable file path
pub fn get_exe_dir() -> PathBuf {
    std::env::current_exe()
        .expect("Failed to get current executable path")
        .parent()
        .expect("Failed to get current executable parent directory")
        .to_path_buf()
}
