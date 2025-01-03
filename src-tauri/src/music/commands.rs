use crate::state::AppData;
use rodio::Decoder;
use tauri::{AppHandle, Emitter, Manager, State};

use std::fs::File;
use std::io::BufReader;

use super::core;

#[tauri::command]
pub fn read_metadata(path: &str) -> Result<core::MetadataResult, String> {
    core::read_metadata(path)
}

#[tauri::command]
pub async fn import_file(path: &str, state: State<'_, AppData>) -> Result<(), String> {
    core::import_file(&state, path).await
}

#[tauri::command]
pub fn play(app: AppHandle, path: &str, state: State<'_, AppData>) -> Result<(), String> {
    let mut metadata = state.metadata.lock().unwrap();
    metadata.currently_playing_file_path = Some(path.to_string());

    let app_handle = app.clone();
    let callback = Box::new(move |_source: &mut Decoder<BufReader<File>>| {
        app_handle
            .emit(
                "PLAYBACK_POSITION_UPDATE",
                app_handle.state::<AppData>().sink.lock().unwrap().get_pos(),
            )
            .unwrap();
    });

    let duration = core::play_audio(&state, path, callback)?;
    metadata.currently_playing_duration = Some(duration);
    crate::macos_interop::now_playing::set_now_playing();
    app.emit("SERVER_SYNC_EVENT", "").unwrap();
    Ok(())
}

#[tauri::command]
pub fn stop(app: AppHandle, state: State<'_, AppData>) {
    let mut metadata = state.metadata.lock().unwrap();
    metadata.currently_playing_file_path = None;

    core::stop(&state);
    app.emit("SERVER_SYNC_EVENT", "").unwrap();
}

#[tauri::command]
pub fn set_volume(app: AppHandle, volume: f32, state: State<'_, AppData>) {
    core::set_volume(&state, volume);
    app.emit("SERVER_SYNC_EVENT", "").unwrap();
}

#[tauri::command]
pub fn set_speed(app: AppHandle, speed: f32, state: State<'_, AppData>) {
    core::set_speed(&state, speed);
    app.emit("SERVER_SYNC_EVENT", "").unwrap();
}

#[tauri::command]
pub fn seek_to(
    app: AppHandle,
    position: std::time::Duration,
    state: State<'_, AppData>,
) -> Result<(), String> {
    core::seek_to(&state, position)?;
    app.emit("SERVER_SYNC_EVENT", "").unwrap();
    Ok(())
}

#[tauri::command]
pub fn pause(app: AppHandle, state: State<'_, AppData>) {
    core::pause(&state);
    app.emit("SERVER_SYNC_EVENT", "").unwrap();
}

#[tauri::command]
pub fn resume(app: AppHandle, state: State<'_, AppData>) {
    core::resume(&state);
    app.emit("SERVER_SYNC_EVENT", "").unwrap();
}

#[tauri::command]
pub fn get_redux_store_state(state: State<'_, AppData>) -> core::PlayerState {
    core::get_player_state(&state)
}

#[tauri::command]
pub fn get_position(state: State<'_, AppData>) -> std::time::Duration {
    core::get_position(&state)
}
