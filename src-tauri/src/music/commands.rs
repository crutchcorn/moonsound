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

    let duration = core::play_audio(&state, path)?;
    state.currently_playing_duration = Some(duration);
    app.emit("SERVER_SYNC_EVENT", "").unwrap();
    Ok(())
}

#[tauri::command]
pub fn stop(app: AppHandle, state: State<'_, Mutex<AppData>>) {
    let mut state = state.lock().unwrap();
    state.currently_playing_file_path = None;

    core::stop(&state);
    app.emit("SERVER_SYNC_EVENT", "").unwrap();
}

#[tauri::command]
pub fn set_volume(app: AppHandle, volume: f32, state: State<'_, Mutex<AppData>>) {
    let state = state.lock().unwrap();
    core::set_volume(&state, volume);
    app.emit("SERVER_SYNC_EVENT", "").unwrap();
}

#[tauri::command]
pub fn set_speed(app: AppHandle, speed: f32, state: State<'_, Mutex<AppData>>) {
    let state = state.lock().unwrap();
    core::set_speed(&state, speed);
    app.emit("SERVER_SYNC_EVENT", "").unwrap();
}

#[tauri::command]
pub fn seek_to(
    app: AppHandle,
    position: std::time::Duration,
    state: State<'_, Mutex<AppData>>,
) -> Result<(), String> {
    let state = state.lock().unwrap();
    core::seek_to(&state, position)?;
    app.emit("SERVER_SYNC_EVENT", "").unwrap();
    Ok(())
}

#[tauri::command]
pub fn pause(app: AppHandle, state: State<'_, Mutex<AppData>>) {
    let state = state.lock().unwrap();
    core::pause(&state);
    app.emit("SERVER_SYNC_EVENT", "").unwrap();
}

#[tauri::command]
pub fn resume(app: AppHandle, state: State<'_, Mutex<AppData>>) {
    let state = state.lock().unwrap();
    core::resume(&state);
    app.emit("SERVER_SYNC_EVENT", "").unwrap();
}

#[tauri::command]
pub fn get_redux_store_state(state: State<'_, Mutex<AppData>>) -> core::PlayerState {
    let app_data = state.lock().unwrap();
    core::get_player_state(&app_data)
}

#[tauri::command]
pub fn get_position(state: State<'_, Mutex<AppData>>) -> std::time::Duration {
    let state = state.lock().unwrap();
    core::get_position(&state)
}
