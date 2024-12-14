use sea_orm::DatabaseConnection;

pub struct AppData {
    pub currently_playing_file_path: Option<String>,
    pub conn: DatabaseConnection,
}

impl AppData {
    pub fn new(conn: DatabaseConnection) -> Self {
        Self {
            conn,
            currently_playing_file_path: None,
        }
    }
}
