use rodio::{OutputStreamHandle, Sink};
use sea_orm::DatabaseConnection;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct AppMetadata {
    pub currently_playing_file_path: Option<String>,
    pub currently_playing_duration: Option<std::time::Duration>,
}

pub struct AppData {
    pub metadata: Arc<Mutex<AppMetadata>>,
    pub stream_handle: Arc<OutputStreamHandle>,
    pub sink: Arc<Mutex<Sink>>,
    pub db: Arc<DatabaseConnection>,
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
            db: Arc::new(db),
            metadata: Arc::new(Mutex::new(AppMetadata {
                currently_playing_duration: None,
                currently_playing_file_path: None,
            })),
            sink: Arc::new(Mutex::new(sink)),
            stream_handle: Arc::new(stream_handle),
        }
    }
}

impl Clone for AppData {
    fn clone(&self) -> Self {
        Self {
            metadata: self.metadata.clone(),
            stream_handle: self.stream_handle.clone(),
            sink: self.sink.clone(),
            db: self.db.clone(),
        }
    }
}
