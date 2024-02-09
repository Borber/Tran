use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use anyhow::Result;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use crate::util;

/// 配置
///
/// Config
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// 模式, true: 镜像模式, false: 直连模式
    ///
    /// Mode, true: mirror mode, false: direct mode
    pub mode: bool,
}

/// 全局配置
///
/// Global config
pub static MODE: Lazy<Arc<AtomicBool>> = Lazy::new(|| Arc::new(AtomicBool::new(true)));

/// 读取配置
///
/// read config
pub fn load() {
    let exe_dir = util::get_exe_dir();
    let path = exe_dir.join("config.json");
    // 检测文件是否存在
    // Check if file exists
    let exists = path.try_exists();
    if exists.is_ok() && exists.expect("Failed to check if file exists") {
        let config = std::fs::read_to_string(path).expect("Failed to read config");
        let config = serde_json::from_str::<Config>(&config).expect("Failed to parse config");
        MODE.store(config.mode, Ordering::SeqCst);
    } else {
        MODE.store(true, Ordering::SeqCst);
    }
}

/// 保存配置
///
/// save config
fn save() -> Result<()> {
    let config = Config {
        mode: MODE.load(Ordering::SeqCst),
    };
    let exe_dir = util::get_exe_dir();
    let path = exe_dir.join("config.json");
    let config = serde_json::to_string_pretty(&config).expect("Failed to serialize config");
    std::fs::write(path, config).expect("Failed to write config");
    Ok(())
}

/// 切换模式
///
/// switch mode
pub fn mode(mode: bool) {
    MODE.store(mode, Ordering::SeqCst);
    save().expect("Failed to save config after switch mode");
}
