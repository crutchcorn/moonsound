// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod macos_interop;
mod music;
mod state;
mod windows;

use migration::{Migrator, MigratorTrait};
use rodio::OutputStreamBuilder;
use sea_orm::Database;
use state::AppDataNew;
use tauri::{Manager};

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

    // Create the output stream and leak it to keep it alive for the program lifetime
    // This is necessary because OutputStream on macOS is not Send
    let stream = OutputStreamBuilder::open_default_stream().unwrap();
    let mixer: &'static _ = Box::leak(Box::new(stream.mixer().clone()));
    // Leak the stream to keep it alive - it will be cleaned up on program exit
    Box::leak(Box::new(stream));

    tauri::Builder::default()
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .plugin(tauri_plugin_decorum::init())
        .setup(move |app| {
            let state = state::AppData::new(AppDataNew { db, mixer });
            app.manage(state.clone());

            if cfg!(target_os = "macos") {
                #[cfg(target_os = "macos")]
                macos_interop::now_playing::setup_handlers(Box::leak(Box::new(
                    app.app_handle().clone(),
                )));
            }

            let main_window = app.get_webview_window("main").unwrap();
            windows::make_window_effects(main_window);

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
            music::add_folder,
            windows::make_window_effect
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
