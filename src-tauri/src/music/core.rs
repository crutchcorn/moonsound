use crate::state::AppData;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
use serde::Serialize;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;

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

#[derive(Serialize)]
pub struct MetadataResult {
    tags: serde_json::Map<String, serde_json::Value>,
    visuals: serde_json::Map<String, serde_json::Value>,
}

pub fn read_mp3_metadata(path: &str) -> Result<MetadataResult, String> {
    let path = Path::new(path);

    let src = File::open(path).map_err(|e| e.to_string())?;
    let mss = MediaSourceStream::new(Box::new(src), Default::default());

    let mut hint = Hint::new();
    hint.with_extension("mp3");

    let meta_opts: MetadataOptions = Default::default();
    let fmt_opts: FormatOptions = Default::default();

    let mut probed = symphonia::default::get_probe()
        .format(&hint, mss, &fmt_opts, &meta_opts)
        .expect("unsupported format");

    let revision = probed
        .metadata
        .get()
        .and_then(|mut metadata| metadata.skip_to_latest().cloned())
        .map(|meta_revision| meta_revision.clone())
        .unwrap();

    let tags = revision.tags();
    let visuals = revision.visuals();

    let mut tag_map = serde_json::Map::new();
    let mut visual_map = serde_json::Map::new();

    // Process tags
    for tag in tags {
        let key = tag
            .std_key
            .map(|k| format!("{:?}", k))
            .unwrap_or_else(|| tag.key.to_string());
        tag_map.insert(key, serde_json::Value::String(tag.value.to_string()));
    }

    // Process visuals
    for (index, visual) in visuals.iter().enumerate() {
        visual_map.insert(
            format!("visual_{}", index),
            serde_json::json!({
                "media_type": visual.media_type.to_string(),
                "dimensions": format!("{}x{}", visual.dimensions.map_or(0, |d| d.width), visual.dimensions.map_or(0, |d| d.height)),
                "data": visual.data,
                "tags": visual.tags.iter().map(|tag| {
                    let key = tag
                        .std_key
                        .map(|k| format!("{:?}", k))
                        .unwrap_or_else(|| tag.key.to_string());
                    (key, serde_json::Value::String(tag.value.to_string()))
                }).collect::<serde_json::Map<String, serde_json::Value>>()
            })
        );
    }

    Ok(MetadataResult {
        tags: tag_map,
        visuals: visual_map,
    })
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
