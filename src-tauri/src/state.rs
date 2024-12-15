use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStreamHandle, Sink};
use sea_orm::DatabaseConnection;
use std::sync::Mutex;
use rodio::source::PeriodicAccess;
use crate::music::types::PeriodicCallback;

pub struct AppMetadata {
    pub currently_playing_file_path: Option<String>,
    pub currently_playing_duration: Option<std::time::Duration>,
}

pub struct AppData {
    pub metadata: Mutex<AppMetadata>,
    pub stream_handle: OutputStreamHandle,
    pub sink: Sink,
    pub db: DatabaseConnection,
}

pub struct AppDataNew {
    pub db: DatabaseConnection,
    pub stream_handle: OutputStreamHandle,
}

impl AppData {
    pub fn new(args: AppDataNew) -> Self {
        let AppDataNew { db, stream_handle } = args;
        let sink: Sink = Sink::try_new(&stream_handle).unwrap();

        Self {
            db,
            metadata: Mutex::new(AppMetadata {
                currently_playing_duration: None,
                currently_playing_file_path: None,
            }),
            sink,
            stream_handle
        }
    }
}
