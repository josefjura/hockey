use sqlx::SqlitePool;

use crate::auth::SessionStore;

#[derive(Clone)]
pub struct AppState {
    pub db: SqlitePool,
    pub sessions: SessionStore,
    pub session_secret: String,
}

impl AppState {
    pub fn new(db: SqlitePool, sessions: SessionStore, session_secret: String) -> Self {
        Self {
            db,
            sessions,
            session_secret,
        }
    }
}
