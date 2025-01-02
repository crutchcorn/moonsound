// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod music;
mod state;
mod macos_interop;

use migration::{Migrator, MigratorTrait};
use rodio::OutputStream;
use sea_orm::Database;
use state::AppDataNew;
use tauri::{Manager, Theme};
use tauri_plugin_decorum::WebviewWindowExt;
use window_vibrancy::{apply_mica, apply_vibrancy, NSVisualEffectMaterial};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() {
    let app_data_dir = dirs::data_dir().map(|dir| dir.join("moonsound")).unwrap();

    // If app_data_dir does not exist, create it
    if !app_data_dir.exists() {
        std::fs::create_dir_all(&app_data_dir).unwrap();
    }

    let db_url = format!(
        "sqlite://{}?mode=rwc",
        app_data_dir.join("music.db").to_string_lossy()
    );

    let db = Database::connect(db_url)
        .await
        .expect("Database connection failed");
    Migrator::up(&db, None).await.unwrap();

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .plugin(tauri_plugin_decorum::init())
        .setup(|app| {
            let state = state::AppData::new(AppDataNew { db, stream_handle });
            app.manage(state);

            let main_window = app.get_webview_window("main").unwrap();
            main_window.create_overlay_titlebar().unwrap();
            if cfg!(target_os = "macos") {
                #[cfg(target_os = "macos")]
                main_window.make_transparent().unwrap();

                #[cfg(target_os = "macos")]
                apply_vibrancy(&main_window, NSVisualEffectMaterial::HudWindow, None, None)
                    .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");
            } else if cfg!(target_os = "windows") {
                #[cfg(target_os = "windows")]
                apply_mica(
                    &main_window,
                    if main_window.theme().unwrap() == Theme::Dark {
                        Some(true)
                    } else {
                        None
                    },
                )
                .expect("Unsupported platform! 'apply_mica' is only supported on Windows");
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            music::read_metadata,
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
