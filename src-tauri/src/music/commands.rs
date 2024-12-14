use crate::state::AppData;
use std::sync::Mutex;
use tauri::{AppHandle, Emitter, State};

use super::core;

#[tauri::command]
pub fn read_mp3_metadata(path: &str) -> Result<core::MetadataResult, String> {
    core::read_mp3_metadata(path)
}

#[tauri::command]
pub fn play(app: AppHandle, path: &str, state: State<'_, Mutex<AppData>>) -> Result<(), String> {
    let mut state = state.lock().unwrap();
    state.currently_playing_file_path = Some(path.to_string());

    core::play_audio(path)?;
    app.emit("SERVER_SYNC_EVENT", "").unwrap();
    Ok(())
}

#[tauri::command]
pub fn stop(app: AppHandle, state: State<'_, Mutex<AppData>>) {
    let mut state = state.lock().unwrap();
    state.currently_playing_file_path = None;

    core::stop();
    app.emit("SERVER_SYNC_EVENT", "").unwrap();
}

#[tauri::command]
pub fn set_volume(app: AppHandle, volume: f32) {
    core::set_volume(volume);
    app.emit("SERVER_SYNC_EVENT", "").unwrap();
}

#[tauri::command]
pub fn set_speed(app: AppHandle, speed: f32) {
    core::set_speed(speed);
    app.emit("SERVER_SYNC_EVENT", "").unwrap();
}

#[tauri::command]
pub fn seek_to(app: AppHandle, position: std::time::Duration) -> Result<(), String> {
    core::seek_to(position)?;
    app.emit("SERVER_SYNC_EVENT", "").unwrap();
    Ok(())
}

#[tauri::command]
pub fn pause(app: AppHandle) {
    core::pause();
    app.emit("SERVER_SYNC_EVENT", "").unwrap();
}

#[tauri::command]
pub fn resume(app: AppHandle) {
    core::resume();
    app.emit("SERVER_SYNC_EVENT", "").unwrap();
}

#[tauri::command]
pub fn get_redux_store_state(state: State<'_, Mutex<AppData>>) -> core::PlayerState {
    let app_data = state.lock().unwrap();
    core::get_player_state(&app_data)
}

#[tauri::command]
pub fn get_position() -> std::time::Duration {
    core::get_position()
}
