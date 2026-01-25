use tauri::{AppHandle, Manager};
use crate::windows;

#[tauri::command]
pub fn make_window_effect(
    app: AppHandle,
    name: &str
) -> Result<(), String> {
    let main_window = app.get_webview_window(name).unwrap();
    windows::make_window_effects(main_window);
    Ok(())
}
