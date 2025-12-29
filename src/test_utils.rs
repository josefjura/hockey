//! Test utilities for integration and unit tests
//!
//! This module provides shared test helpers including:
//! - Test database setup
//! - Seed data functions
//! - Test session management
//! - Test app creation for route tests

use crate::app_state::AppState;
use crate::auth::{Session, SessionStore};
use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use axum_extra::extract::cookie::Cookie;
use sqlx::SqlitePool;

/// Create a test application with routes for integration testing
///
/// This creates the full application router with all routes and middleware,
/// using the provided database pool. Use with axum-test for route testing.
pub fn create_test_app(pool: SqlitePool) -> Router {
    let session_store = SessionStore::new(pool.clone());
    let state = AppState::new(pool, session_store);

    // Public routes
    let public_routes = Router::new()
        .route("/auth/login", get(crate::routes::auth::login_get))
        .route("/auth/login", post(crate::routes::auth::login_post))
        .route("/auth/logout", post(crate::routes::auth::logout_post));

    // Protected routes
    let protected_routes = Router::new()
        .route("/teams", get(crate::routes::teams::teams_get))
        .route("/teams/list", get(crate::routes::teams::teams_list_partial))
        .route("/teams/new", get(crate::routes::teams::team_create_form))
        .route("/teams", post(crate::routes::teams::team_create))
        .route("/teams/:id", get(crate::routes::teams::team_detail))
        .route("/teams/:id/edit", get(crate::routes::teams::team_edit_form))
        .route("/teams/:id", post(crate::routes::teams::team_update))
        .route("/teams/:id/delete", post(crate::routes::teams::team_delete))
        .route("/players", get(crate::routes::players::players_get))
        .route(
            "/players/list",
            get(crate::routes::players::players_list_partial),
        )
        .route("/events", get(crate::routes::events::events_get))
        .route(
            "/events/list",
            get(crate::routes::events::events_list_partial),
        )
        .route("/seasons", get(crate::routes::seasons::seasons_get))
        .route(
            "/seasons/list",
            get(crate::routes::seasons::seasons_list_partial),
        )
        .route("/matches", get(crate::routes::matches::matches_get))
        .route(
            "/matches/list",
            get(crate::routes::matches::matches_list_partial),
        )
        .route(
            "/management",
            get(crate::routes::management::management_get),
        )
        .layer(middleware::from_fn_with_state(
            state.clone(),
            crate::auth::require_auth,
        ));

    // Build the complete application
    Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .with_state(state)
        .layer(middleware::from_fn(
            crate::i18n::middleware::translation_context_middleware,
        ))
}

/// Create a test session for authenticated route testing
///
/// Creates a session for a test user and stores it in the database.
/// Returns the Session object which can be used with `session_cookie()`.
pub async fn create_test_session(pool: &SqlitePool) -> Session {
    let store = SessionStore::new(pool.clone());

    // Create or get test user
    let user = sqlx::query!(
        r#"
        INSERT INTO users (email, name, password_hash)
        VALUES ('test@example.com', 'Test User', 'hash')
        ON CONFLICT(email) DO UPDATE SET email = email
        RETURNING id, email, name
        "#
    )
    .fetch_one(pool)
    .await
    .expect("Failed to create test user");

    store
        .create_session(
            user.id,
            user.email,
            user.name.unwrap_or_else(|| "Test User".to_string()),
        )
        .await
}

/// Generate a session cookie from a Session
///
/// Creates a Cookie that can be used with axum-test's `add_cookie()` method
/// to authenticate requests in route tests.
pub fn session_cookie(session: &Session) -> Cookie<'static> {
    Cookie::build(("hockey_session", session.id.clone()))
        .http_only(true)
        .secure(false) // Not secure in tests
        .same_site(axum_extra::extract::cookie::SameSite::Lax)
        .path("/")
        .into()
}

/// Seed test countries into the database
///
/// Inserts a standard set of countries for testing, including:
/// - IIHF members and non-members
/// - Historical countries (Soviet Union, East Germany)
/// - Countries with various attribute combinations
pub async fn seed_test_countries(pool: &SqlitePool) {
    sqlx::query!(
        r#"
        INSERT OR IGNORE INTO country (id, name, iihf, iocCode, iso2Code, isHistorical, years, enabled)
        VALUES
            (1, 'Canada', 1, 'CAN', 'ca', 0, NULL, 1),
            (2, 'United States', 1, 'USA', 'us', 0, NULL, 1),
            (3, 'Russia', 1, 'RUS', 'ru', 0, NULL, 1),
            (4, 'Finland', 1, 'FIN', 'fi', 0, NULL, 1),
            (5, 'Sweden', 1, 'SWE', 'se', 0, NULL, 1),
            (6, 'Czech Republic', 1, 'CZE', 'cz', 0, NULL, 1),
            (7, 'Slovakia', 1, 'SVK', 'sk', 0, NULL, 1),
            (8, 'Switzerland', 1, 'SUI', 'ch', 0, NULL, 1),
            (9, 'Germany', 1, 'GER', 'de', 0, NULL, 1),
            (10, 'Austria', 1, 'AUT', 'at', 0, NULL, 1),
            (11, 'Latvia', 1, 'LAT', 'lv', 0, NULL, 1),
            (12, 'Norway', 1, 'NOR', 'no', 0, NULL, 1),
            (13, 'Denmark', 1, 'DEN', 'dk', 0, NULL, 1),
            (14, 'France', 1, 'FRA', 'fr', 0, NULL, 1),
            (15, 'Belarus', 1, 'BLR', 'by', 0, NULL, 1),
            (16, 'Soviet Union', 1, 'URS', 'su', 1, '1922-1991', 0),
            (17, 'East Germany', 1, 'GDR', NULL, 1, '1949-1990', 0),
            (18, 'Czechoslovakia', 1, 'TCH', NULL, 1, '1920-1992', 0),
            (19, 'Japan', 1, 'JPN', 'jp', 0, NULL, 1),
            (20, 'South Korea', 1, 'KOR', 'kr', 0, NULL, 1)
        "#
    )
    .execute(pool)
    .await
    .expect("Failed to seed countries");
}

/// Seed test teams into the database
///
/// Inserts a set of hockey teams for testing. Requires countries to be seeded first.
pub async fn seed_test_teams(pool: &SqlitePool) {
    // Ensure countries exist first
    seed_test_countries(pool).await;

    sqlx::query!(
        r#"
        INSERT OR IGNORE INTO team (id, name, country_id)
        VALUES
            (1, 'Team Canada', 1),
            (2, 'Team USA', 2),
            (3, 'Team Russia', 3),
            (4, 'Team Finland', 4),
            (5, 'Team Sweden', 5)
        "#
    )
    .execute(pool)
    .await
    .expect("Failed to seed teams");
}

/// Seed test players into the database
///
/// Inserts a set of players for testing. Requires countries to be seeded first.
pub async fn seed_test_players(pool: &SqlitePool) {
    // Ensure countries exist first
    seed_test_countries(pool).await;

    sqlx::query!(
        r#"
        INSERT OR IGNORE INTO player (id, name, country_id, birth_date, position)
        VALUES
            (1, 'Wayne Gretzky', 1, '1961-01-26', 'Forward'),
            (2, 'Mario Lemieux', 1, '1965-10-05', 'Forward'),
            (3, 'Bobby Orr', 1, '1948-03-20', 'Defense'),
            (4, 'Gordie Howe', 1, '1928-03-31', 'Forward'),
            (5, 'Pavel Datsyuk', 3, '1978-07-20', 'Forward')
        "#
    )
    .execute(pool)
    .await
    .expect("Failed to seed players");
}

/// Seed test events into the database
///
/// Inserts hockey events (tournaments) for testing. Requires countries to be seeded first.
pub async fn seed_test_events(pool: &SqlitePool) {
    // Ensure countries exist first
    seed_test_countries(pool).await;

    sqlx::query!(
        r#"
        INSERT OR IGNORE INTO event (id, name, country_id)
        VALUES
            (1, 'Winter Olympics', 1),
            (2, 'World Championship', 4),
            (3, 'World Cup', 2)
        "#
    )
    .execute(pool)
    .await
    .expect("Failed to seed events");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[sqlx::test(migrations = "./migrations")]
    async fn test_seed_countries(pool: SqlitePool) {
        seed_test_countries(&pool).await;

        let count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM country")
            .fetch_one(&pool)
            .await
            .unwrap();

        assert!(count >= 20, "Should have at least 20 countries seeded");
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn test_seed_teams(pool: SqlitePool) {
        seed_test_teams(&pool).await;

        let count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM team")
            .fetch_one(&pool)
            .await
            .unwrap();

        assert_eq!(count, 5, "Should have 5 teams seeded");
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn test_create_session(pool: SqlitePool) {
        let session = create_test_session(&pool).await;

        assert_eq!(session.user_email, "test@example.com");
        assert_eq!(session.user_name, "Test User");
        assert!(!session.is_expired());
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn test_session_cookie(pool: SqlitePool) {
        let session = create_test_session(&pool).await;
        let cookie = session_cookie(&session);

        assert_eq!(cookie.name(), "hockey_session");
        assert_eq!(cookie.value(), session.id);
        assert!(cookie.http_only().unwrap());
        assert_eq!(cookie.path(), Some("/"));
    }
}
