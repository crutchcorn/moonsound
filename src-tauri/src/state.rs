use rodio::{OutputStreamHandle, Sink};
use sea_orm::DatabaseConnection;
use std::sync::{Arc, Mutex};

pub struct AppMetadata {
    pub currently_playing_file_path: Option<String>,
    pub currently_playing_duration: Option<std::time::Duration>,
}

#[derive(Clone)]
pub struct AppData {
    pub metadata: Arc<Mutex<AppMetadata>>,
    pub stream_handle: OutputStreamHandle,
    pub sink: Arc<Sink>,
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
            metadata: Arc::new(Mutex::new(AppMetadata {
                currently_playing_duration: None,
                currently_playing_file_path: None,
            })),
            sink: Arc::new(sink),
            stream_handle,
        }
    }
}
