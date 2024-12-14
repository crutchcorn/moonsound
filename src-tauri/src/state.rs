use rodio::{OutputStream, OutputStreamHandle, Sink};
use sea_orm::DatabaseConnection;

pub struct AppData {
    pub currently_playing_file_path: Option<String>,
    pub sink: Sink,
    pub stream_handle: OutputStreamHandle,
    pub conn: DatabaseConnection,
}

impl AppData {
    pub fn new(conn: DatabaseConnection) -> Self {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink: Sink = Sink::try_new(&stream_handle).unwrap();

        Self {
            conn,
            currently_playing_file_path: None,
            sink,
            stream_handle,
        }
    }
}
