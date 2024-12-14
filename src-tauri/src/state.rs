use rodio::{OutputStream, OutputStreamHandle, Sink};

pub struct AppData {
    pub currently_playing_file_path: Option<String>,
    pub sink: Sink,
    pub stream_handle: OutputStreamHandle,
}

impl Default for AppData {
    fn default() -> Self {
        let (_stream, stream_handle )= OutputStream::try_default().unwrap();
        let sink: Sink = Sink::try_new(&stream_handle).unwrap();

        Self {
            currently_playing_file_path: None,
            sink,
            stream_handle,
        }
    }
}
