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
    for _ in 0..100 {
        if let Ok(text) = clipboard.get_text() {
            println!("get clipboard text: {}", text);
            return Ok(text);
        } else {
            println!("Failed to get clipboard");
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
