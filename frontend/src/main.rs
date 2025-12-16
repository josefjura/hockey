mod app_state;
mod auth;
mod config;
mod i18n;
mod routes;
mod service;
mod views;

use app_state::AppState;
use axum::{
    extract::Request,
    middleware,
    response::Html,
    routing::{get, post},
    Router,
};
use sqlx::sqlite::SqlitePoolOptions;
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use auth::{require_auth, SessionStore};
use i18n::Locale;
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

    // Set up database connection pool
    let db_pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await?;

    // Run migrations
    tracing::info!("Running database migrations...");
    sqlx::migrate!("./migrations").run(&db_pool).await?;
    tracing::info!("Migrations completed successfully");

    // Create session store
    let session_store = SessionStore::new();

    // Create app state
    let state = AppState::new(db_pool, session_store.clone());

    // Start background task to cleanup expired sessions
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(3600)); // Every hour
        loop {
            interval.tick().await;
            session_store.cleanup_expired().await;
            tracing::debug!("Cleaned up expired sessions");
        }
    });

    // Public routes (no authentication required)
    let public_routes = Router::new()
        .route("/auth/login", get(routes::auth::login_get))
        .route("/auth/login", post(routes::auth::login_post))
        .route("/auth/logout", post(routes::auth::logout_post));

    // Protected routes (authentication required)
    let protected_routes = Router::new()
        .route("/", get(root_handler))
        .route("/events", get(routes::events::events_list_get))
        .route("/events/list", get(routes::events::events_list_htmx))
        .route("/events/new", get(routes::events::event_create_get))
        .route("/events", post(routes::events::event_create_post))
        .route("/events/:id/edit", get(routes::events::event_edit_get))
        .route("/events/:id", post(routes::events::event_update_post))
        .route(
            "/events/:id/delete",
            post(routes::events::event_delete_post),
        )
        .route("/matches/:id", get(routes::matches::match_detail_get))
        .route(
            "/matches/:id/delete",
            post(routes::matches::match_delete_post),
        )
        .layer(middleware::from_fn_with_state(state.clone(), require_auth));

    // Health check (no auth)
    let health_routes = Router::new().route("/health", get(health_handler));

    // Build the complete application
    let app = Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .merge(health_routes)
        .with_state(state)
        .layer(TraceLayer::new_for_http());

    // Start the server
    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    tracing::info!("Server listening on http://{}", addr);
    tracing::info!("Login at http://{}:{}/auth/login", addr.ip(), config.port);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn root_handler(request: Request) -> Html<String> {
    // Extract session from request extensions (added by require_auth middleware)
    let session = request
        .extensions()
        .get::<auth::Session>()
        .expect("Session should be available in protected route")
        .clone();

    // For now, default to English locale
    // TODO: Get locale from user preferences or cookie
    let locale = Locale::English;

    let content = dashboard_page();
    let html = admin_layout("Dashboard", &session, "/", locale, content);

    Html(html.into_string())
}

async fn health_handler() -> &'static str {
    "OK"
}
