use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};

thread_local! {
    static AUDIO: (OutputStream, OutputStreamHandle) = OutputStream::try_default().unwrap();
    pub static STREAM_HANDLE : OutputStreamHandle = AUDIO.with(|(_, h)| h.clone());
    pub static SINK: Sink = STREAM_HANDLE.with(|handle| Sink::try_new(handle).unwrap());
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
fn play_sound(path: &str) {
    let path = Path::new(path);
    let file = BufReader::new(File::open(path).unwrap());
    let source = Decoder::new(file).unwrap();
    SINK.with(|sink| sink.append(source));
}

#[tauri::command]
fn stop_sound() {
    SINK.with(|sink| sink.stop());
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![read_mp3_metadata, play_sound, stop_sound])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
