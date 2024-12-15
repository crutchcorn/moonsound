use rodio::{OutputStream, OutputStreamHandle, Sink};
use sea_orm::DatabaseConnection;

pub struct AppData {
    pub currently_playing_file_path: Option<String>,
    pub stream_handle: OutputStreamHandle,
    pub sink: Sink,
    pub currently_playing_duration: Option<std::time::Duration>,
    pub conn: DatabaseConnection,
}

pub struct AppDataNew {
    pub conn: DatabaseConnection,
    pub stream_handle: OutputStreamHandle,
}

impl AppData {
    pub fn new(args: AppDataNew) -> Self {
        let AppDataNew { conn, stream_handle } = args;
        let sink: Sink = Sink::try_new(&stream_handle).unwrap();

        Self {
            conn,
            currently_playing_duration: None,
            currently_playing_file_path: None,
            sink,
            stream_handle,
        }
    }
}
