use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
use serde::Serialize;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::sync::Mutex;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;
use tauri::{Manager, State};

thread_local! {
    static AUDIO: (OutputStream, OutputStreamHandle) = OutputStream::try_default().unwrap();
    pub static STREAM_HANDLE : OutputStreamHandle = AUDIO.with(|(_, h)| h.clone());
    pub static SINK: Sink = STREAM_HANDLE.with(|handle| Sink::try_new(handle).unwrap());
}

#[derive(Default, Serialize)]
struct AppData {
    currently_playing_file_path: Option<String>,
}

#[tauri::command]
fn read_mp3_metadata(path: &str) -> Result<serde_json::Value, String> {
    let path = Path::new(path);

    // Open the media source.
    let src = std::fs::File::open(path).expect("failed to open media");

    // Create the media source stream.
    let mss = MediaSourceStream::new(Box::new(src), Default::default());

    // Create a probe hint using the file's extension. [Optional]
    let mut hint = Hint::new();
    hint.with_extension("mp3");

    // Use the default options for metadata and format readers.
    let meta_opts: MetadataOptions = Default::default();
    let fmt_opts: FormatOptions = Default::default();

    // Probe the media source.
    let mut probed = symphonia::default::get_probe()
        .format(&hint, mss, &fmt_opts, &meta_opts)
        .expect("unsupported format");

    let tags = probed
        .metadata
        .get()
        .and_then(|mut metadata| metadata.skip_to_latest().cloned())
        .map(|meta_revisions| meta_revisions.tags().to_vec());

    println!("{:?}", tags);

    // Convert the metadata to a JSON object.
    let mut map = serde_json::Map::new();
    if let Some(tags) = tags {
        for tag in tags {
            let key = tag
                .std_key
                .map(|k| format!("{:?}", k))
                .unwrap_or_else(|| tag.key.to_string());
            map.insert(key, serde_json::Value::String(tag.value.to_string()));
        }
    }
    Ok(serde_json::Value::Object(map))
}

#[tauri::command]
fn play_sound(path: &str, state: State<'_, Mutex<AppData>>) -> Result<(), String> {
    let path = Path::new(path);
    let file = BufReader::new(File::open(path).map_err(|e| e.to_string())?);
    let source = Decoder::new(file).map_err(|e| e.to_string())?;

    // Update the state with new path
    let mut state = state.lock().unwrap();
    state.currently_playing_file_path = Some(path.to_string_lossy().into_owned());

    SINK.with(|sink| sink.append(source));
    Ok(())
}

#[tauri::command]
fn stop_sound(state: State<'_, Mutex<AppData>>) {
    // Clear the currently playing file path when stopping
    let mut state = state.lock().unwrap();
    state.currently_playing_file_path = None;

    SINK.with(|sink| sink.stop());
}

#[tauri::command]
fn set_volume(volume: f32) {
    SINK.with(|sink| sink.set_volume(volume));
}

#[tauri::command]
fn set_speed(speed: f32) {
    SINK.with(|sink| sink.set_speed(speed));
}

#[tauri::command]
fn seek(position: std::time::Duration) -> Result<(), String> {
    SINK.with(|sink| sink.try_seek(position))
        .map_err(|e| e.to_string())
}

#[derive(Serialize)]
struct PlayerState {
    volume: f32,
    speed: f32,
    paused: bool,
    position: std::time::Duration,
    currently_playing_file_path: Option<String>,
}

#[tauri::command]
fn get_redux_store_state(state: State<'_, Mutex<AppData>>) -> PlayerState {
    let app_data = state.lock().unwrap();

    SINK.with(|sink| PlayerState {
        volume: sink.volume(),
        speed: sink.speed(),
        paused: sink.is_paused(),
        position: sink.get_pos(),
        currently_playing_file_path: app_data.currently_playing_file_path.clone(),
    })
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            app.manage(Mutex::new(AppData::default()));
            Ok(())
        })
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            read_mp3_metadata,
            play_sound,
            stop_sound,
            get_redux_store_state,
            set_volume,
            set_speed,
            seek
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
