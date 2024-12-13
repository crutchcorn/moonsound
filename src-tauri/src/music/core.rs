use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
use serde::Serialize;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;
use crate::state::AppData;

thread_local! {
    static AUDIO: (OutputStream, OutputStreamHandle) = OutputStream::try_default().unwrap();
    pub static STREAM_HANDLE : OutputStreamHandle = AUDIO.with(|(_, h)| h.clone());
    pub static SINK: Sink = STREAM_HANDLE.with(|handle| Sink::try_new(handle).unwrap());
}

#[derive(Serialize)]
pub struct PlayerState {
    pub volume: f32,
    pub speed: f32,
    pub paused: bool,
    pub currently_playing_file_path: Option<String>,
}

pub fn read_mp3_metadata(path: &str) -> Result<serde_json::Value, String> {
    let path = Path::new(path);

    let src = std::fs::File::open(path).expect("failed to open media");
    let mss = MediaSourceStream::new(Box::new(src), Default::default());

    let mut hint = Hint::new();
    hint.with_extension("mp3");

    let meta_opts: MetadataOptions = Default::default();
    let fmt_opts: FormatOptions = Default::default();

    let mut probed = symphonia::default::get_probe()
        .format(&hint, mss, &fmt_opts, &meta_opts)
        .expect("unsupported format");

    let tags = probed
        .metadata
        .get()
        .and_then(|mut metadata| metadata.skip_to_latest().cloned())
        .map(|meta_revisions| meta_revisions.tags().to_vec());

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

pub fn play_audio(path: &str) -> Result<(), String> {
    let path = Path::new(path);
    let file = BufReader::new(File::open(path).map_err(|e| e.to_string())?);
    let source = Decoder::new(file).map_err(|e| e.to_string())?;

    SINK.with(|sink| sink.append(source));
    Ok(())
}

pub fn stop() {
    SINK.with(|sink| sink.stop());
}

pub fn set_volume(volume: f32) {
    SINK.with(|sink| sink.set_volume(volume));
}

pub fn set_speed(speed: f32) {
    SINK.with(|sink| sink.set_speed(speed));
}

pub fn seek_to(position: std::time::Duration) -> Result<(), String> {
    SINK.with(|sink| sink.try_seek(position))
        .map_err(|e| e.to_string())
}

pub fn pause() {
    SINK.with(|sink| sink.pause());
}

pub fn resume() {
    SINK.with(|sink| sink.play());
}

pub fn get_player_state(app_data: &AppData) -> PlayerState {
    SINK.with(|sink| PlayerState {
        volume: sink.volume(),
        speed: sink.speed(),
        paused: sink.is_paused(),
        currently_playing_file_path: app_data.currently_playing_file_path.clone(),
    })
}

pub fn get_position() -> std::time::Duration {
    SINK.with(|sink| sink.get_pos())
} 