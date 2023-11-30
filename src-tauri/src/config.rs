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
    /// 模式, 0: 镜像模式, 1: 直连模式, 2: 代理模式
    ///
    /// Mode, 0: mirror mode, 1: direct mode, 2: proxy mode
    pub mode: usize,
    /// 代理地址, 为空则直连
    ///
    /// Proxy address, empty means direct
    pub url: String,
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
    if exists.is_ok() && exists.unwrap() {
        let config = std::fs::read_to_string(path).unwrap();
        serde_json::from_str(&config).unwrap()
    } else {
        Config {
            mode: 0,
            url: "".to_string(),
        }
    }
}

/// 保存配置
///
/// save config
fn save(config: &Config) -> Result<()> {
    let exe_dir = util::get_exe_dir();
    let path = exe_dir.join("config.json");
    let config = serde_json::to_string_pretty(config).unwrap();
    std::fs::write(path, config).unwrap();
    Ok(())
}

/// 切换模式
///
/// switch mode
pub fn mode(mode: usize) {
    CONFIG.lock().mode = mode;
    save(&CONFIG.lock()).unwrap();
}

/// 设置代理地址
///
/// set proxy url
pub fn set_proxy_url(url: String) {
    CONFIG.lock().url = url;
    save(&CONFIG.lock()).unwrap();
}
