use sqlx::SqlitePool;
use std::sync::Arc;

use crate::auth::SessionStore;
use crate::i18n::I18n;

#[derive(Clone)]
pub struct AppState {
    pub db: SqlitePool,
    pub sessions: SessionStore,
    pub i18n: Arc<I18n>,
}

impl AppState {
    pub fn new(db: SqlitePool, sessions: SessionStore) -> Self {
        Self {
            db,
            sessions,
            i18n: Arc::new(I18n::new()),
        }
    }
}
