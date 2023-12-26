use tauri::AppHandle;

pub fn main(app: &AppHandle) {
    tauri::WindowBuilder::new(app, "main", tauri::WindowUrl::App("/".into()))
        .title("Tran")
        .inner_size(320.0, 320.0)
        .fullscreen(false)
        .resizable(false)
        .minimizable(false)
        .maximizable(false)
        .decorations(false)
        .always_on_top(true)
        .visible(false)
        .focused(true)
        .center()
        .build()
        .expect("Failed to build main window");
}
