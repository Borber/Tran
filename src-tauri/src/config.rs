use std::sync::Arc;

use anyhow::Result;
use once_cell::sync::Lazy;
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};

use crate::util;

/// 配置
///
/// Config
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// 模式, 0: 镜像模式, 1: 直连模式
    ///
    /// Mode, 0: mirror mode, 1: direct mode
    pub mode: usize,
}

/// 全局配置
///
/// Global config
pub static CONFIG: Lazy<Arc<Mutex<Config>>> = Lazy::new(|| Arc::new(Mutex::new(load())));

/// 读取配置
///
/// read config
pub fn load() -> Config {
    let exe_dir = util::get_exe_dir();
    let path = exe_dir.join("config.json");
    // 检测文件是否存在
    // Check if file exists
    let exists = path.try_exists();
    if exists.is_ok() && exists.expect("Failed to check if file exists") {
        let config = std::fs::read_to_string(path).expect("Failed to read config");
        serde_json::from_str(&config).expect("Failed to parse config")
    } else {
        Config { mode: 0 }
    }
}

/// 保存配置
///
/// save config
fn save(config: &Config) -> Result<()> {
    let exe_dir = util::get_exe_dir();
    let path = exe_dir.join("config.json");
    let config = serde_json::to_string_pretty(config).expect("Failed to serialize config");
    std::fs::write(path, config).expect("Failed to write config");
    Ok(())
}

/// 切换模式
///
/// switch mode
pub fn mode(mode: usize) {
    CONFIG.lock().mode = mode;
    save(&CONFIG.lock()).expect("Failed to save config after switch mode");
}
