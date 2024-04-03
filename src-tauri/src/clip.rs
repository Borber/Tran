use anyhow::Result;
use arboard::Clipboard;

/// 写入剪贴板
///
/// Write to clipboard
pub fn set(content: String) -> Result<()> {
    let mut clipboard = Clipboard::new()?;
    clipboard.set_text(content)?;
    Ok(())
}

/// 读取剪贴板文本
///
/// Read clipboard text
pub fn get() -> Result<String> {
    let mut clipboard = Clipboard::new()?;

    // 因为有时因为电脑的电源策略, 你可能会需要长时间的等待劫持复制的程序写入剪贴板
    // Because of the power policy of the computer, you may need to wait for long periods of time to write to the clipboard
    for _ in 0..100 {
        if let Ok(text) = clipboard.get_text() {
            return Ok(text);
        } else {
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
    }
    Err(anyhow::anyhow!("Failed to get clipboard text"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clip() {
        let content = "test".to_string();
        assert!(set(content.clone()).is_ok());
        assert_eq!(get().unwrap(), content);
    }
}
