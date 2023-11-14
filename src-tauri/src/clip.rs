use arboard::Clipboard;

// 获取剪贴板内容
pub fn get() -> String {
    let mut clipboard = Clipboard::new().unwrap();
    match clipboard.get_text() {
        Ok(text) => text,
        Err(_) => "获取剪贴板内容失败".to_string(),
    }
}

// TODO 写入剪贴板
pub fn set() {}
