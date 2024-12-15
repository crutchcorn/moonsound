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

/**
 * This code may seem bad. It is.
 *
 * But also, I have no idea how to dynamically generate the keys for the tags in a safe way,
 * as I don't currently believe Rust supports this. If it does 🤷??
 */
fn get_keys_for_standard_tag(tag: StandardTag) -> String {
    match tag {
        StandardTag::AccurateRipCount(_) => "AccurateRipCount",
        StandardTag::AccurateRipCountAllOffsets(_) => "AccurateRipCountAllOffsets",
        StandardTag::AccurateRipCountWithOffset(_) => "AccurateRipCountWithOffset",
        StandardTag::AccurateRipCrc(_) => "AccurateRipCrc",
        StandardTag::AccurateRipDiscId(_) => "AccurateRipDiscId",
        StandardTag::AccurateRipId(_) => "AccurateRipId",
        StandardTag::AccurateRipOffset(_) => "AccurateRipOffset",
        StandardTag::AccurateRipResult(_) => "AccurateRipResult",
        StandardTag::AccurateRipTotal(_) => "AccurateRipTotal",
        StandardTag::AcoustIdFingerprint(_) => "AcoustIdFingerprint",
        StandardTag::AcoustIdId(_) => "AcoustIdId",
        StandardTag::Album(_) => "Album",
        StandardTag::AlbumArtist(_) => "AlbumArtist",
        StandardTag::Arranger(_) => "Arranger",
        StandardTag::Artist(_) => "Artist",
        StandardTag::Author(_) => "Author",
        StandardTag::Bpm(_) => "Bpm",
        StandardTag::CdToc(_) => "CdToc",
        StandardTag::Comment(_) => "Comment",
        StandardTag::CompilationFlag(_) => "CompilationFlag",
        StandardTag::Composer(_) => "Composer",
        StandardTag::Conductor(_) => "Conductor",
        StandardTag::ContentAdvisory(_) => "ContentAdvisory",
        StandardTag::Copyright(_) => "Copyright",
        StandardTag::CueToolsDbDiscConfidence(_) => "CueToolsDbDiscConfidence",
        StandardTag::CueToolsDbTrackConfidence(_) => "CueToolsDbTrackConfidence",
        StandardTag::Date(_) => "Date",
        StandardTag::Description(_) => "Description",
        StandardTag::DiscNumber(_) => "DiscNumber",
        StandardTag::DiscSubtitle(_) => "DiscSubtitle",
        StandardTag::DiscTotal(_) => "DiscTotal",
        StandardTag::EncodedBy(_) => "EncodedBy",
        StandardTag::Encoder(_) => "Encoder",
        StandardTag::EncoderSettings(_) => "EncoderSettings",
        StandardTag::EncodingDate(_) => "EncodingDate",
        StandardTag::Engineer(_) => "Engineer",
        StandardTag::Ensemble(_) => "Ensemble",
        StandardTag::Genre(_) => "Genre",
        StandardTag::Grouping(_) => "Grouping",
        StandardTag::IdentAsin(_) => "IdentAsin",
        StandardTag::IdentBarcode(_) => "IdentBarcode",
        StandardTag::IdentCatalogNumber(_) => "IdentCatalogNumber",
        StandardTag::IdentEanUpn(_) => "IdentEanUpn",
        StandardTag::IdentIsbn(_) => "IdentIsbn",
        StandardTag::IdentIsrc(_) => "IdentIsrc",
        StandardTag::IdentPn(_) => "IdentPn",
        StandardTag::IdentPodcast(_) => "IdentPodcast",
        StandardTag::IdentUpc(_) => "IdentUpc",
        StandardTag::IndexNumber(_) => "IndexNumber",
        StandardTag::InitialKey(_) => "InitialKey",
        StandardTag::InternetRadioName(_) => "InternetRadioName",
        StandardTag::InternetRadioOwner(_) => "InternetRadioOwner",
        StandardTag::Label(_) => "Label",
        StandardTag::LabelCode(_) => "LabelCode",
        StandardTag::Language(_) => "Language",
        StandardTag::License(_) => "License",
        StandardTag::Lyricist(_) => "Lyricist",
        StandardTag::Lyrics(_) => "Lyrics",
        StandardTag::MediaFormat(_) => "MediaFormat",
        StandardTag::MixDj(_) => "MixDj",
        StandardTag::MixEngineer(_) => "MixEngineer",
        StandardTag::Mood(_) => "Mood",
        StandardTag::MovementName(_) => "MovementName",
        StandardTag::MovementNumber(_) => "MovementNumber",
        StandardTag::MovementTotal(_) => "MovementTotal",
        StandardTag::Mp3GainAlbumMinMax(_) => "Mp3GainAlbumMinMax",
        StandardTag::Mp3GainMinMax(_) => "Mp3GainMinMax",
        StandardTag::Mp3GainUndo(_) => "Mp3GainUndo",
        StandardTag::MusicBrainzAlbumArtistId(_) => "MusicBrainzAlbumArtistId",
        StandardTag::MusicBrainzAlbumId(_) => "MusicBrainzAlbumId",
        StandardTag::MusicBrainzArtistId(_) => "MusicBrainzArtistId",
        StandardTag::MusicBrainzDiscId(_) => "MusicBrainzDiscId",
        StandardTag::MusicBrainzGenreId(_) => "MusicBrainzGenreId",
        StandardTag::MusicBrainzLabelId(_) => "MusicBrainzLabelId",
        StandardTag::MusicBrainzOriginalAlbumId(_) => "MusicBrainzOriginalAlbumId",
        StandardTag::MusicBrainzOriginalArtistId(_) => "MusicBrainzOriginalArtistId",
        StandardTag::MusicBrainzRecordingId(_) => "MusicBrainzRecordingId",
        StandardTag::MusicBrainzReleaseGroupId(_) => "MusicBrainzReleaseGroupId",
        StandardTag::MusicBrainzReleaseStatus(_) => "MusicBrainzReleaseStatus",
        StandardTag::MusicBrainzReleaseTrackId(_) => "MusicBrainzReleaseTrackId",
        StandardTag::MusicBrainzReleaseType(_) => "MusicBrainzReleaseType",
        StandardTag::MusicBrainzTrackId(_) => "MusicBrainzTrackId",
        StandardTag::MusicBrainzTrmId(_) => "MusicBrainzTrmId",
        StandardTag::MusicBrainzWorkId(_) => "MusicBrainzWorkId",
        StandardTag::Narrator(_) => "Narrator",
        StandardTag::Opus(_) => "Opus",
        StandardTag::OriginalAlbum(_) => "OriginalAlbum",
        StandardTag::OriginalArtist(_) => "OriginalArtist",
        StandardTag::OriginalDate(_) => "OriginalDate",
        StandardTag::OriginalFile(_) => "OriginalFile",
        StandardTag::OriginalLyricist(_) => "OriginalLyricist",
        StandardTag::OriginalWriter(_) => "OriginalWriter",
        StandardTag::OriginalYear(_) => "OriginalYear",
        StandardTag::Owner(_) => "Owner",
        StandardTag::Part(_) => "Part",
        StandardTag::PartNumber(_) => "PartNumber",
        StandardTag::PartTotal(_) => "PartTotal",
        StandardTag::Performer(_) => "Performer",
        StandardTag::PlayCounter(_) => "PlayCounter",
        StandardTag::PodcastCategory(_) => "PodcastCategory",
        StandardTag::PodcastDescription(_) => "PodcastDescription",
        StandardTag::PodcastFlag(_) => "PodcastFlag",
        StandardTag::PodcastKeywords(_) => "PodcastKeywords",
        StandardTag::Producer(_) => "Producer",
        StandardTag::ProductionCopyright(_) => "ProductionCopyright",
        StandardTag::PurchaseDate(_) => "PurchaseDate",
        StandardTag::Rating(_) => "Rating",
        StandardTag::RecordingDate(_) => "RecordingDate",
        StandardTag::RecordingLocation(_) => "RecordingLocation",
        StandardTag::RecordingTime(_) => "RecordingTime",
        StandardTag::ReleaseCountry(_) => "ReleaseCountry",
        StandardTag::ReleaseDate(_) => "ReleaseDate",
        StandardTag::Remixer(_) => "Remixer",
        StandardTag::ReplayGainAlbumGain(_) => "ReplayGainAlbumGain",
        StandardTag::ReplayGainAlbumPeak(_) => "ReplayGainAlbumPeak",
        StandardTag::ReplayGainAlbumRange(_) => "ReplayGainAlbumRange",
        StandardTag::ReplayGainReferenceLoudness(_) => "ReplayGainReferenceLoudness",
        StandardTag::ReplayGainTrackGain(_) => "ReplayGainTrackGain",
        StandardTag::ReplayGainTrackPeak(_) => "ReplayGainTrackPeak",
        StandardTag::ReplayGainTrackRange(_) => "ReplayGainTrackRange",
        StandardTag::Script(_) => "Script",
        StandardTag::Soloist(_) => "Soloist",
        StandardTag::SortAlbum(_) => "SortAlbum",
        StandardTag::SortAlbumArtist(_) => "SortAlbumArtist",
        StandardTag::SortArtist(_) => "SortArtist",
        StandardTag::SortComposer(_) => "SortComposer",
        StandardTag::SortTrackTitle(_) => "SortTrackTitle",
        StandardTag::SortTvShowTitle(_) => "SortTvShowTitle",
        StandardTag::TaggingDate(_) => "TaggingDate",
        StandardTag::TermsOfUse(_) => "TermsOfUse",
        StandardTag::TrackNumber(_) => "TrackNumber",
        StandardTag::TrackSubtitle(_) => "TrackSubtitle",
        StandardTag::TrackTitle(_) => "TrackTitle",
        StandardTag::TrackTotal(_) => "TrackTotal",
        StandardTag::TvEpisode(_) => "TvEpisode",
        StandardTag::TvEpisodeTitle(_) => "TvEpisodeTitle",
        StandardTag::TvNetwork(_) => "TvNetwork",
        StandardTag::TvSeason(_) => "TvSeason",
        StandardTag::TvShowTitle(_) => "TvShowTitle",
        StandardTag::Url(_) => "Url",
        StandardTag::UrlArtist(_) => "UrlArtist",
        StandardTag::UrlCopyright(_) => "UrlCopyright",
        StandardTag::UrlInternetRadio(_) => "UrlInternetRadio",
        StandardTag::UrlLabel(_) => "UrlLabel",
        StandardTag::UrlOfficial(_) => "UrlOfficial",
        StandardTag::UrlPayment(_) => "UrlPayment",
        StandardTag::UrlPodcast(_) => "UrlPodcast",
        StandardTag::UrlPurchase(_) => "UrlPurchase",
        StandardTag::UrlSource(_) => "UrlSource",
        StandardTag::Version(_) => "Version",
        StandardTag::Work(_) => "Work",
        StandardTag::Writer(_) => "Writer",
        _ => {
            // Twitch said that the previous placeholder would get me in trouble for royalties, this should totally solve that :)
            panic!("AT THE DISCOGRAPHY OF MY FAVORITE ARTIST");
        }
    }
        .to_string()
}

#[derive(Serialize)]
pub struct PlayerState {
    pub volume: f32,
    pub speed: f32,
    pub paused: bool,
    pub currently_playing_file_path: Option<String>,
    pub currently_playing_duration: Option<std::time::Duration>,
}

#[derive(Serialize)]
pub struct MetadataResult {
    tags: serde_json::Map<String, serde_json::Value>,
    visuals: serde_json::Map<String, serde_json::Value>,
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
                let data_url = format!("url(data:{};base64,{})", media_type, base64_data);

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
