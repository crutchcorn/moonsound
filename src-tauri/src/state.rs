use sea_orm::DatabaseConnection;

pub struct AppData {
    pub currently_playing_file_path: Option<String>,
    pub currently_playing_duration: Option<std::time::Duration>,
    pub conn: DatabaseConnection,
}

impl AppData {
    pub fn new(conn: DatabaseConnection) -> Self {
        Self {
            conn,
            currently_playing_duration: None,
            currently_playing_file_path: None,
        }
    }
}
