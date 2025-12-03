use std::sync::Arc;
use tokio::signal;

use aide::{axum::ApiRouter, openapi::OpenApi};
use axum::extract::State;
use axum::routing::get;
use axum::{http::Method, response::Json, Extension};
use serde_json::{json, Value};
use sqlx::SqlitePool;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};
use tracing::{info, warn};

use crate::{
    auth::{auth_routes, JwtManager},
    config::Config,
    country::routes::country_routes,
    docs::{api_docs, docs_routes},
    event::routes::event_routes,
    player::routes::player_routes,
    player_contract::routes::player_contract_routes,
    r#match::routes::match_routes,
    season::routes::season_routes,
    team::routes::team_routes,
    team_participation::routes::team_participation_routes,
};

#[derive(Clone)]
pub struct ApiContext {
    #[allow(dead_code)]
    pub config: Arc<Config>,
    pub db: SqlitePool,
    pub jwt_manager: Arc<JwtManager>,
}

impl ApiContext {
    pub fn new(db: SqlitePool, config: Config) -> Result<Self, Box<dyn std::error::Error>> {
        let jwt_manager = JwtManager::new(&config.jwt_private_key_path, &config.jwt_public_key_path)?;

        Ok(Self {
            config: Arc::new(config),
            db,
            jwt_manager: Arc::new(jwt_manager),
        })
    }
}

fn create_cors_layer(config: &Config) -> CorsLayer {
    let mut cors = CorsLayer::new();

    // Configure origins
    if config.cors_origins == "*" {
        cors = cors.allow_origin(Any);
    } else {
        let origins: Result<Vec<_>, _> = config
            .cors_origins
            .split(',')
            .map(|s| s.trim().parse::<axum::http::HeaderValue>())
            .collect();
        if let Ok(origins) = origins {
            cors = cors.allow_origin(origins);
        } else {
            tracing::warn!(
                "Invalid CORS origins configuration, falling back to allowing any origin"
            );
            cors = cors.allow_origin(Any);
        }
    }

    // Configure methods
    if config.cors_methods == "*" {
        cors = cors.allow_methods(Any);
    } else {
        let methods: Result<Vec<_>, _> = config
            .cors_methods
            .split(',')
            .map(|s| s.trim().parse::<Method>())
            .collect();
        if let Ok(methods) = methods {
            cors = cors.allow_methods(methods);
        } else {
            tracing::warn!(
                "Invalid CORS methods configuration, falling back to allowing any method"
            );
            cors = cors.allow_methods(Any);
        }
    }

    // Configure headers
    if config.cors_headers == "*" {
        cors = cors.allow_headers(Any);
    } else {
        let headers: Result<Vec<_>, _> = config
            .cors_headers
            .split(',')
            .map(|s| s.trim().parse::<axum::http::HeaderName>())
            .collect();
        if let Ok(headers) = headers {
            cors = cors.allow_headers(headers);
        } else {
            tracing::warn!(
                "Invalid CORS headers configuration, falling back to allowing any header"
            );
            cors = cors.allow_headers(Any);
        }
    }

    cors
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            info!("Received Ctrl+C, shutting down gracefully...");
        },
        _ = terminate => {
            info!("Received SIGTERM, shutting down gracefully...");
        },
    }
}

async fn health_check(State(ctx): State<ApiContext>) -> Json<Value> {
    // Simple health check - verify database connection
    let db_status = match sqlx::query("SELECT 1").execute(&ctx.db).await {
        Ok(_) => "healthy",
        Err(_) => "unhealthy",
    };

    Json(json!({
        "status": if db_status == "healthy" { "ok" } else { "error" },
        "database": db_status,
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

async fn readiness_check(State(ctx): State<ApiContext>) -> Json<Value> {
    // More comprehensive readiness check
    let db_ready = match sqlx::query("SELECT COUNT(*) FROM sqlite_master WHERE type='table'")
        .fetch_one(&ctx.db)
        .await
    {
        Ok(_) => true,
        Err(_) => false,
    };

    Json(json!({
        "ready": db_ready,
        "database": if db_ready { "ready" } else { "not_ready" },
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

pub async fn serve(config: Config, db: SqlitePool) {
    let bind_addr = format!("{}:{}", config.host, config.port);
    let mut api = OpenApi::default();
    // Bootstrapping an API is both more intuitive with Axum than Actix-web but also
    // a bit more confusing at the same time.
    //
    // Coming from Actix-web, I would expect to pass the router into `ServiceBuilder` and not
    // the other way around.
    //
    // It does look nicer than the mess of `move || {}` closures you have to do with Actix-web,
    // which, I suspect, largely has to do with how it manages its own worker threads instead of
    // letting Tokio do it.
    let jwt_manager = JwtManager::new(&config.jwt_private_key_path, &config.jwt_public_key_path)
        .expect("Failed to initialize JWT manager");

    let api_context = ApiContext {
        config: Arc::new(config.clone()),
        db,
        jwt_manager: Arc::new(jwt_manager),
    };

    let app = ApiRouter::new()
        //.nest_api_service("/todo", todo_routes(state.clone()))
        .nest_api_service("/auth", auth_routes())
        .nest_api_service("/event", event_routes())
        .nest_api_service("/country", country_routes())
        .nest_api_service("/team", team_routes())
        .nest_api_service("/team-participation", team_participation_routes())
        .nest_api_service("/match", match_routes())
        .nest_api_service("/player", player_routes())
        .nest_api_service("/player-contract", player_contract_routes())
        .nest_api_service("/season", season_routes())
        .nest_api_service("/docs", docs_routes())
        .finish_api_with(&mut api, api_docs)
        .route("/health", get(health_check))
        .route("/ready", get(readiness_check))
        .layer(Extension(Arc::new(api)))
        .layer(create_cors_layer(&config))
        .layer(Extension(api_context.jwt_manager.clone()))
        .layer(Extension(api_context.clone()))
        .with_state(api_context);

    let listener = TcpListener::bind(&bind_addr).await.unwrap();

    info!("Server running at http://{}", bind_addr);

    // Create the server with graceful shutdown
    let server =
        axum::serve(listener, app.into_make_service()).with_graceful_shutdown(shutdown_signal());

    // Run the server
    if let Err(e) = server.await {
        warn!("Server error: {}", e);
    } else {
        info!("Server shutdown gracefully");
    }
}
