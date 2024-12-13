// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod music;
mod state;

use std::sync::Mutex;
use tauri::Manager;
use tauri_plugin_decorum::WebviewWindowExt;
use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .plugin(tauri_plugin_decorum::init())
        .setup(|app| {
            app.manage(Mutex::new(state::AppData::default()));

            let main_window = app.get_webview_window("main").unwrap();
            main_window.create_overlay_titlebar().unwrap();
            if cfg!(target_os = "macos") {
                main_window.make_transparent().unwrap();

                #[cfg(target_os = "macos")]
                apply_vibrancy(&main_window, NSVisualEffectMaterial::HudWindow, None, None)
                    .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");
            } else if cfg!(target_os = "windows") {
                #[cfg(target_os = "windows")]
                apply_blur(&main_window, Some((18, 18, 18, 125)))
                    .expect("Unsupported platform! 'apply_blur' is only supported on Windows");
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            music::read_mp3_metadata,
            music::play,
            music::stop,
            music::get_redux_store_state,
            music::set_volume,
            music::set_speed,
            music::seek_to,
            music::pause,
            music::resume,
            music::get_position,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
