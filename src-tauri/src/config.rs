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
    /// 模式, true: 镜像模式, false: 直连模式
    ///
    /// Mode, true: mirror mode, false: direct mode
    #[serde(default = "mode_default")]
    pub mode: bool,

    /// 快捷键
    ///
    /// Key, 0: shift, 1: ctrl, 2: caps
    #[serde(default = "key_default")]
    pub key: u8,

    /// 主题
    ///
    /// Theme
    #[serde(default = "theme_default")]
    pub theme: String,
}

fn mode_default() -> bool {
    true
}

fn key_default() -> u8 {
    0
}

fn theme_default() -> String {
    "dark".to_string()
}

pub static CONFIG: Lazy<Mutex<Config>> = Lazy::new(|| Mutex::new(load()));

/// 读取配置
///
/// read config
pub fn load() -> Config {
    let exe_dir = util::get_exe_dir();
    let path = exe_dir.join("config.json");
    // 检测文件是否存在
    // Check if file exists
    let exists = path.try_exists();
    let json = if exists.is_ok() && exists.expect("Failed to check if file exists") {
        std::fs::read_to_string(path).expect("Failed to read config")
    } else {
        "{}".to_string()
    };

    serde_json::from_str::<Config>(&json).expect("Failed to parse config")
}

/// 保存配置
///
/// save config
fn save() -> Result<()> {
    let exe_dir = util::get_exe_dir();
    let path = exe_dir.join("config.json");
    let config =
        serde_json::to_string_pretty(&CONFIG.lock().clone()).expect("Failed to serialize config");
    std::fs::write(path, config).expect("Failed to write config");
    Ok(())
}

/// 获取全局配置
///
/// get global config
pub fn config() -> Config {
    CONFIG.lock().clone()
}

/// 获取模式
///
/// get mode
pub fn mode() -> bool {
    CONFIG.lock().mode
}

/// 获取快捷键
///
/// get key
pub fn key() -> u8 {
    CONFIG.lock().key
}

/// 获取主题
///
/// get theme
pub fn theme() -> String {
    CONFIG.lock().theme.clone()
}

/// 切换模式
///
/// switch mode
pub fn set_mode(mode: bool) {
    CONFIG.lock().mode = mode;
    save().expect("Failed to save config after switch mode");
}

/// 切换快捷键
///
/// switch key
pub fn set_key(key: u8) {
    CONFIG.lock().key = key;
    save().expect("Failed to save config after switch key");
}

/// 切换主题
///
/// switch theme
pub fn set_theme(theme: &str) {
    CONFIG.lock().theme = theme.to_string();
    save().expect("Failed to save config after switch theme");
}
