use serde::Serialize;

#[derive(Default, Serialize)]
pub struct AppData {
    pub currently_playing_file_path: Option<String>,
}
