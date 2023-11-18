use std::path::PathBuf;

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
