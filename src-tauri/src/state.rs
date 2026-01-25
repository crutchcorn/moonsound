use rodio::Sink;
use rodio::mixer::Mixer;
use sea_orm::DatabaseConnection;
use std::sync::{Arc, Mutex};

pub struct AppMetadata {
    pub currently_playing_file_path: Option<String>,
    pub currently_playing_duration: Option<std::time::Duration>,
}

#[derive(Clone)]
pub struct AppData {
    pub metadata: Arc<Mutex<AppMetadata>>,
    pub sink: Arc<Sink>,
    pub db: DatabaseConnection,
}

pub struct AppDataNew {
    pub db: DatabaseConnection,
    pub mixer: &'static Mixer,
}

impl AppData {
    pub fn new(args: AppDataNew) -> Self {
        let AppDataNew { db, mixer } = args;
        let sink: Sink = Sink::connect_new(mixer);

        Self {
            db,
            metadata: Arc::new(Mutex::new(AppMetadata {
                currently_playing_duration: None,
                currently_playing_file_path: None,
            })),
            sink: Arc::new(sink),
        }
    }
}
