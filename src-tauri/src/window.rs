use anyhow::Result;
use tauri::AppHandle;

pub fn panel(app: &AppHandle) -> Result<()> {
    tauri::WindowBuilder::new(app, "panel", tauri::WindowUrl::App("/panel".into()))
        .title("Tran")
        .inner_size(256.0, 100.0)
        .fullscreen(false)
        .resizable(false)
        .minimizable(false)
        .maximizable(false)
        .decorations(false)
        .always_on_top(true)
        .skip_taskbar(true)
        .visible(false)
        .shadow(true)
        .center()
        .build()?;
    Ok(())
}
