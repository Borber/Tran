use std::sync::Arc;

use anyhow::Result;
use once_cell::sync::Lazy;
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};

use crate::util;

/// 全局配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// 是否启用代理, 若不启用, 默认使用镜像
    pub proxy: bool,
    /// 代理地址, 为空则直连
    pub url: String,
}

/// 全局配置
pub static CONFIG: Lazy<Arc<Mutex<Config>>> = Lazy::new(|| Arc::new(Mutex::new(load())));

// 读取配置
pub fn load() -> Config {
    let exe_dir = util::get_exe_dir();
    let path = exe_dir.join("config.json");
    // 检测文件是否存在
    let exists = path.try_exists();
    if exists.is_ok() && exists.unwrap() {
        let config = std::fs::read_to_string(path).unwrap();
        serde_json::from_str(&config).unwrap()
    } else {
        Config {
            proxy: false,
            url: "".to_string(),
        }
    }
}

/// 保存配置
fn save(config: &Config) -> Result<()> {
    // 获取可执行文件位置
    let exe_dir = util::get_exe_dir();
    let path = exe_dir.join("config.json");
    let config = serde_json::to_string_pretty(config).unwrap();
    std::fs::write(path, config).unwrap();
    Ok(())
}

// 开启代理
pub fn enable_proxy() {
    CONFIG.lock().proxy = true;

    // 保存配置
    save(&CONFIG.lock()).unwrap();
}

// 关闭代理
pub fn disable_proxy() {
    CONFIG.lock().proxy = false;
    // 保存配置
    save(&CONFIG.lock()).unwrap();
}

// 设置代理地址
pub fn set_proxy_url(url: String) {
    CONFIG.lock().url = url;
    // 保存配置
    save(&CONFIG.lock()).unwrap();
}
