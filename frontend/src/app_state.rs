use sqlx::SqlitePool;

use crate::auth::SessionStore;

#[derive(Clone)]
pub struct AppState {
    pub db: SqlitePool,
    pub sessions: SessionStore,
}

impl AppState {
    pub fn new(db: SqlitePool, sessions: SessionStore) -> Self {
        Self { db, sessions }
    }
}
