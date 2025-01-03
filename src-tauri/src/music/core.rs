use crate::music::tags::get_keys_for_standard_tag;
use crate::music::types::PeriodicCallback;
use crate::state::AppData;
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use entity::song;
use rodio::{Decoder, Source};
use sea_orm::{ActiveModelTrait, Set};
use serde::Serialize;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::time::Duration;
use symphonia::core::formats::probe::Hint;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::{MetadataOptions, StandardTag};

#[derive(Serialize)]
pub struct PlayerState {
    pub volume: f32,
    pub speed: f32,
    pub paused: bool,
    pub currently_playing_file_path: Option<String>,
    pub currently_playing_duration: Option<std::time::Duration>,
}

#[derive(Serialize, Clone)]
pub struct MetadataResult {
    pub tags: serde_json::Map<String, serde_json::Value>,
    pub visuals: serde_json::Map<String, serde_json::Value>,
}

pub async fn import_file(app_data: &AppData, path: &str) -> Result<(), String> {
    song::ActiveModel {
        path: Set(path.parse().unwrap()),
        ..Default::default()
    }
    .save(&app_data.db)
    .await
    .map(|_| ())
    .map_err(|e| e.to_string())
}

pub fn read_metadata(path: &str) -> Result<MetadataResult, String> {
    let path = Path::new(path);

    let src = File::open(path).map_err(|e| e.to_string())?;
    let mss = MediaSourceStream::new(Box::new(src), Default::default());

    let mut hint = Hint::new();
    hint.with_extension(
        path.extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or_default(),
    );

    let meta_opts: MetadataOptions = Default::default();
    let fmt_opts: FormatOptions = Default::default();

    let mut probed = symphonia::default::get_probe()
        .probe(&hint, mss, fmt_opts, meta_opts)
        .expect("unsupported format");

    let mut metadata = probed.metadata();
    let revision = metadata.skip_to_latest();

    let tags = revision.as_ref().map(|revision| revision.tags());

    let visuals = revision.as_ref().map(|revision| revision.visuals());

    let mut tag_map = serde_json::Map::new();
    let mut visual_map = serde_json::Map::new();

    match (tags, visuals) {
        (Some(tags), Some(visuals)) => {
            // Process tags
            for tag in tags {
                let key = tag
                    .clone()
                    .std
                    .map(|std_tag| get_keys_for_standard_tag(std_tag))
                    .unwrap_or_else(|| tag.clone().raw.key.to_string());
                let value = tag.clone().raw.value.to_string();

                tag_map.insert(key, serde_json::Value::String(value));
            }

            // Process visuals
            for (index, visual) in visuals.iter().enumerate() {
                let base64_data = BASE64.encode(&visual.data);
                let media_type = visual.media_type.clone().unwrap_or("image/png".to_string());
                let data_url = format!("data:{};base64,{}", media_type, base64_data);

                visual_map.insert(
                    format!("visual_{}", index),
                    serde_json::json!({
                        "media_type": media_type,
                        "dimensions": format!("{}x{}", visual.dimensions.map_or(0, |d| d.width), visual.dimensions.map_or(0, |d| d.height)),
                        "data": data_url,
                        "tags": visual.tags.iter().map(|tag| {
                            let key =
                                tag.clone().std.map(|std_tag| get_keys_for_standard_tag(std_tag)).unwrap_or_else(|| tag.clone().raw.key.to_string());
                            let value = tag.clone().raw.value.to_string();

                            (key, serde_json::Value::String(value))
                        }).collect::<serde_json::Map<String, serde_json::Value>>()
                    }),
                );
            }

            Ok(MetadataResult {
                tags: tag_map,
                visuals: visual_map,
            })
        }
        _ => Ok(MetadataResult {
            tags: tag_map,
            visuals: visual_map,
        }),
    }
}

pub fn play_audio(
    app_data: &AppData,
    path: &str,
    on_periodic: PeriodicCallback,
) -> Result<Duration, String> {
    let path = Path::new(path);
    let file = BufReader::new(File::open(path).map_err(|e| e.to_string())?);
    let source = Decoder::new(file).map_err(|e| e.to_string())?;
    let duration = source.total_duration().unwrap_or_default();

    // Periodic access is closed when the sink is stopped (Is this correct?)
    let periodic_access = source.periodic_access(Duration::from_secs(1), on_periodic);

    app_data.sink.clear();
    app_data.sink.append(periodic_access);

    Ok(duration)
}

pub fn stop(app_data: &AppData) {
    app_data.sink.stop();
}

pub fn set_volume(app_data: &AppData, volume: f32) {
    app_data.sink.set_volume(volume);
}

pub fn set_speed(app_data: &AppData, speed: f32) {
    app_data.sink.set_speed(speed);
}

pub fn seek_to(app_data: &AppData, position: std::time::Duration) -> Result<(), String> {
    app_data.sink.try_seek(position).map_err(|e| e.to_string())
}

pub fn pause(app_data: &AppData) {
    app_data.sink.pause();
}

pub fn is_playing(app_data: &AppData) -> bool {
    !app_data.sink.is_paused()
}

pub fn resume(app_data: &AppData) {
    app_data.sink.play();
}

pub fn get_player_state(app_data: &AppData) -> PlayerState {
    let metadata = app_data.metadata.lock().unwrap();
    PlayerState {
        volume: app_data.sink.volume(),
        speed: app_data.sink.speed(),
        paused: app_data.sink.is_paused(),
        currently_playing_file_path: metadata.currently_playing_file_path.clone(),
        currently_playing_duration: metadata.currently_playing_duration.clone(),
    }
}

pub fn get_position(app_data: &AppData) -> std::time::Duration {
    app_data.sink.get_pos()
}
