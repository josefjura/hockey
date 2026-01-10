mod app_state;
mod assets;
mod auth;
mod business;
mod common;
mod config;
mod i18n;
mod routes;
mod service;
mod utils;
mod validation;
mod views;

#[cfg(test)]
pub mod test_utils;

use app_state::AppState;
use axum::{
    extract::{Path, Request, State},
    middleware,
    response::Html,
    routing::{get, post},
    Extension, Router,
};
use i18n::TranslationContext;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use std::net::SocketAddr;
use tower_http::{compression::CompressionLayer, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use auth::{rate_limit_login, require_auth, LoginRateLimiter, SessionStore};
use views::{layout::admin_layout, pages::dashboard::dashboard_page};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "hockey=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let config = config::Config::from_env()?;
    tracing::info!("Starting hockey management application");
    tracing::info!("Environment: {:?}", config.environment);
    tracing::info!("Database: {}", config.database_url);

    // Set up database connection pool with foreign keys enabled
    let connection_options = config
        .database_url
        .parse::<SqliteConnectOptions>()?
        .foreign_keys(true);

    let db_pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(connection_options)
        .await?;

    // Run migrations
    tracing::info!("Running database migrations...");
    sqlx::migrate!("./migrations").run(&db_pool).await?;
    tracing::info!("Migrations completed successfully");

    // Create session store
    let session_store = SessionStore::new(db_pool.clone());

    // Create app state
    let state = AppState::new(
        db_pool,
        session_store.clone(),
        config.session_secret.clone(),
        config.is_production(),
    );

    // Start background task to cleanup expired sessions
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(3600)); // Every hour
        loop {
            interval.tick().await;
            match session_store.cleanup_expired().await {
                Ok(_) => tracing::debug!("Cleaned up expired sessions"),
                Err(e) => tracing::error!("Failed to cleanup expired sessions: {}", e),
            }
        }
    });

    // Create rate limiter for login endpoint
    let login_rate_limiter = LoginRateLimiter::new();

    // Public routes (no authentication required)
    let public_routes = Router::new()
        .route("/auth/login", get(routes::auth::login_get))
        .route(
            "/auth/login",
            post(routes::auth::login_post).layer(middleware::from_fn_with_state(
                login_rate_limiter,
                rate_limit_login,
            )),
        )
        .route("/auth/logout", post(routes::auth::logout_post))
        .route("/locale/:code", get(routes::locale::set_locale));

    // Protected routes (authentication required)
    let protected_routes = Router::new()
        .route("/", get(root_handler))
        .route("/dashboard/stats", get(dashboard_stats_get))
        .route("/management", get(routes::management::management_get))
        .route("/countries", get(routes::countries::countries_get))
        .route("/api/countries", get(routes::countries::countries_list_api))
        .route(
            "/api/countries/:id/toggle",
            post(routes::countries::country_toggle_enabled),
        )
        .route("/events", get(routes::events::events_get))
        .route("/events/list", get(routes::events::events_list_partial))
        .route("/events/:id", get(routes::events::event_detail))
        .route("/events/new", get(routes::events::event_create_form))
        .route("/events", post(routes::events::event_create))
        .route("/events/:id/edit", get(routes::events::event_edit_form))
        .route("/events/:id", post(routes::events::event_update))
        .route("/events/:id/delete", post(routes::events::event_delete))
        .route(
            "/events/:event_id/seasons/new",
            get(routes::seasons::event_season_create_form),
        )
        .route("/teams", get(routes::teams::teams_get))
        .route("/teams/list", get(routes::teams::teams_list_partial))
        .route("/teams/new", get(routes::teams::team_create_form))
        .route("/teams", post(routes::teams::team_create))
        .route("/teams/:id", get(routes::teams::team_detail))
        .route("/teams/:id/edit", get(routes::teams::team_edit_form))
        .route("/teams/:id", post(routes::teams::team_update))
        .route("/teams/:id/delete", post(routes::teams::team_delete))
        .route(
            "/team-participations/new",
            get(routes::team_participations::team_participation_create_form),
        )
        .route(
            "/team-participations",
            post(routes::team_participations::team_participation_create),
        )
        .route("/players", get(routes::players::players_get))
        .route("/players/list", get(routes::players::players_list_partial))
        .route("/players/new", get(routes::players::player_create_form))
        .route("/players", post(routes::players::player_create))
        .route("/players/:id", get(routes::players::player_detail))
        .route("/players/:id/edit", get(routes::players::player_edit_form))
        .route("/players/:id", post(routes::players::player_update))
        .route("/players/:id/delete", post(routes::players::player_delete))
        .route(
            "/players/:id/scoring",
            get(routes::players::player_scoring_get),
        )
        .route(
            "/players/:id/scoring/list",
            get(routes::players::player_scoring_list_partial),
        )
        .route(
            "/players/:id/event-stats/new",
            get(routes::players::event_stats_create_form),
        )
        .route(
            "/players/:id/event-stats",
            post(routes::players::event_stats_create),
        )
        .route(
            "/players/:player_id/event-stats/:id/edit",
            get(routes::players::event_stats_edit_form),
        )
        .route(
            "/players/:player_id/event-stats/:id",
            post(routes::players::event_stats_update),
        )
        .route(
            "/players/:player_id/event-stats/:id/delete",
            post(routes::players::event_stats_delete),
        )
        // Property change routes
        .route(
            "/players/:id/property-changes/new",
            get(routes::players::property_change_create_form),
        )
        .route(
            "/players/:id/property-changes",
            post(routes::players::property_change_create),
        )
        .route(
            "/players/:player_id/property-changes/:id/edit",
            get(routes::players::property_change_edit_form),
        )
        .route(
            "/players/:player_id/property-changes/:id",
            post(routes::players::property_change_update),
        )
        .route(
            "/players/:player_id/property-changes/:id/delete",
            post(routes::players::property_change_delete),
        )
        .route("/seasons", get(routes::seasons::seasons_get))
        .route("/seasons/list", get(routes::seasons::seasons_list_partial))
        .route("/seasons/new", get(routes::seasons::season_create_form))
        .route("/seasons", post(routes::seasons::season_create))
        .route("/seasons/:id", get(routes::seasons::season_detail))
        .route("/seasons/:id/edit", get(routes::seasons::season_edit_form))
        .route("/seasons/:id", post(routes::seasons::season_update))
        .route("/seasons/:id/delete", post(routes::seasons::season_delete))
        .route(
            "/seasons/:season_id/teams/add",
            get(routes::seasons::season_add_team_form),
        )
        .route(
            "/seasons/:season_id/teams",
            post(routes::seasons::season_add_team),
        )
        .route(
            "/team-participations/:id/delete",
            post(routes::seasons::team_participation_delete),
        )
        .route(
            "/team-participations/:id/roster",
            get(routes::player_contracts::roster_get),
        )
        .route(
            "/team-participations/:id/roster/add-player",
            get(routes::player_contracts::roster_add_player_form),
        )
        .route(
            "/team-participations/:id/roster",
            post(routes::player_contracts::roster_add_player),
        )
        .route(
            "/player-contracts/:id/delete",
            post(routes::player_contracts::player_contract_delete),
        )
        .route("/matches", get(routes::matches::matches_get))
        .route("/matches/list", get(routes::matches::matches_list_partial))
        .route(
            "/matches/teams-for-season",
            get(routes::matches::teams_for_season),
        )
        .route("/matches/new", get(routes::matches::match_create_form))
        .route("/matches", post(routes::matches::match_create))
        .route("/matches/:id", get(routes::matches::match_detail))
        .route("/matches/:id/edit", get(routes::matches::match_edit_form))
        .route("/matches/:id", post(routes::matches::match_update))
        .route("/matches/:id/delete", post(routes::matches::match_delete))
        .route(
            "/matches/:match_id/score-events/new",
            get(routes::matches::score_event_create_form),
        )
        .route(
            "/matches/:match_id/score-events",
            post(routes::matches::score_event_create),
        )
        .route(
            "/matches/score-events/:id/edit",
            get(routes::matches::score_event_edit_form),
        )
        .route(
            "/matches/score-events/:id",
            post(routes::matches::score_event_update),
        )
        .route(
            "/matches/score-events/:id/delete",
            post(routes::matches::score_event_delete),
        )
        .layer(middleware::from_fn_with_state(state.clone(), require_auth));

    // Health check (no auth)
    let health_routes = Router::new()
        .route("/health", get(health_handler))
        .route("/liveness", get(health_handler))
        .route("/readiness", get(readiness_handler));

    // Static assets routes (embedded or from filesystem in dev mode)
    let static_routes = Router::new().route("/static/*path", get(static_asset_handler));

    // Build the complete application
    let app = Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .merge(health_routes)
        .merge(static_routes)
        .with_state(state)
        .layer(middleware::from_fn(
            i18n::middleware::translation_context_middleware,
        ))
        .layer(CompressionLayer::new().gzip(true))
        .layer(TraceLayer::new_for_http());

    // Start the server
    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    tracing::info!("Server listening on http://{}", addr);
    tracing::info!("Login at http://{}:{}/auth/login", addr.ip(), config.port);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn root_handler(
    Extension(t): Extension<TranslationContext>,
    State(state): State<AppState>,
    request: Request,
) -> Html<String> {
    // Extract session from request extensions (added by require_auth middleware)
    let session = request
        .extensions()
        .get::<auth::Session>()
        .expect("Session should be available in protected route")
        .clone();

    // Fetch dashboard stats
    let stats = service::dashboard::get_dashboard_stats(&state.db)
        .await
        .unwrap_or_default();

    let recent_activity = service::dashboard::get_recent_activity(&state.db)
        .await
        .unwrap_or_default();

    let content = dashboard_page(&t, &stats, &recent_activity);
    let html = admin_layout("Dashboard", &session, "/", &t, content);

    Html(html.into_string())
}

/// GET /dashboard/stats - Returns dashboard stats partial for HTMX updates
async fn dashboard_stats_get(
    Extension(t): Extension<TranslationContext>,
    State(state): State<AppState>,
) -> Html<String> {
    let stats = service::dashboard::get_dashboard_stats(&state.db)
        .await
        .unwrap_or_default();

    Html(views::pages::dashboard::dashboard_stats_partial(&t, &stats).into_string())
}

async fn health_handler() -> &'static str {
    "OK"
}

async fn readiness_handler(State(state): State<AppState>) -> impl axum::response::IntoResponse {
    match sqlx::query("SELECT 1").execute(&state.db).await {
        Ok(_) => (axum::http::StatusCode::OK, "OK"),
        Err(e) => {
            tracing::error!("Readiness check failed: {}", e);
            (axum::http::StatusCode::SERVICE_UNAVAILABLE, "Unavailable")
        }
    }
}

async fn static_asset_handler(Path(path): Path<String>) -> impl axum::response::IntoResponse {
    assets::serve_static_asset(&path).await
}
