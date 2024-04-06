use std::path::PathBuf;

use selection::get_text;

use crate::clip;

/// 模拟获取复制文本
///
/// Simulate getting copy text
pub fn content(fallback: bool) -> String {
    // 模拟复制获取文本
    // Simulate copy and get text
    let content = get_text();

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
