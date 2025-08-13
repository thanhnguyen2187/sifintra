use diesel::SqliteConnection;

pub struct AppState {
    pub conn: SqliteConnection,
}

