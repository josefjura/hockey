use std::sync::Arc;

use aide::{axum::ApiRouter, openapi::OpenApi};
use axum::{Extension, http::Method};
use sqlx::SqlitePool;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};
use tracing::info;

use crate::{
    auth::auth_routes,
    config::Config,
    country::routes::country_routes,
    docs::{api_docs, docs_routes},
    event::routes::event_routes,
    r#match::routes::match_routes,
    player::routes::player_routes,
    player_contract::routes::player_contract_routes,
    season::routes::season_routes,
    team::routes::team_routes,
    team_participation::routes::team_participation_routes,
};

#[derive(Clone)]
pub struct ApiContext {
    #[allow(dead_code)]
    pub config: Arc<Config>,
    pub db: SqlitePool,
}

impl ApiContext {
    pub fn new(db: SqlitePool, config: Config) -> Self {
        Self {
            config: Arc::new(config),
            db,
        }
    }
}

fn create_cors_layer(config: &Config) -> CorsLayer {
    let mut cors = CorsLayer::new();
    
    // Configure origins
    if config.cors_origins == "*" {
        cors = cors.allow_origin(Any);
    } else {
        let origins: Result<Vec<_>, _> = config.cors_origins
            .split(',')
            .map(|s| s.trim().parse::<axum::http::HeaderValue>())
            .collect();
        if let Ok(origins) = origins {
            cors = cors.allow_origin(origins);
        } else {
            tracing::warn!("Invalid CORS origins configuration, falling back to allowing any origin");
            cors = cors.allow_origin(Any);
        }
    }
    
    // Configure methods
    if config.cors_methods == "*" {
        cors = cors.allow_methods(Any);
    } else {
        let methods: Result<Vec<_>, _> = config.cors_methods
            .split(',')
            .map(|s| s.trim().parse::<Method>())
            .collect();
        if let Ok(methods) = methods {
            cors = cors.allow_methods(methods);
        } else {
            tracing::warn!("Invalid CORS methods configuration, falling back to allowing any method");
            cors = cors.allow_methods(Any);
        }
    }
    
    // Configure headers
    if config.cors_headers == "*" {
        cors = cors.allow_headers(Any);
    } else {
        let headers: Result<Vec<_>, _> = config.cors_headers
            .split(',')
            .map(|s| s.trim().parse::<axum::http::HeaderName>())
            .collect();
        if let Ok(headers) = headers {
            cors = cors.allow_headers(headers);
        } else {
            tracing::warn!("Invalid CORS headers configuration, falling back to allowing any header");
            cors = cors.allow_headers(Any);
        }
    }
    
    cors
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
        .layer(Extension(Arc::new(api)))
        .layer(create_cors_layer(&config))
        .layer(Extension(ApiContext {
            config: Arc::new(config.clone()),
            db,
        }));

    let listener = TcpListener::bind(&bind_addr).await.unwrap();

    info!("Server running at http://{}", bind_addr);
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
