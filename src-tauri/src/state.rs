use rodio::{OutputStream, OutputStreamHandle, Sink};
use sea_orm::DatabaseConnection;

pub struct AppData {
    pub currently_playing_file_path: Option<String>,
    pub stream_handle: OutputStreamHandle,
    pub sink: Sink,
    pub currently_playing_duration: Option<std::time::Duration>,
    pub conn: DatabaseConnection,
}

impl AppData {
    pub fn new(conn: DatabaseConnection, stream_handle: OutputStreamHandle) -> Self {
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
