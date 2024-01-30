use std::path::PathBuf;

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
