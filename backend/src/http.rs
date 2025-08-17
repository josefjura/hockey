use std::sync::Arc;

use aide::{axum::ApiRouter, openapi::OpenApi};
use axum::Extension;
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
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .layer(Extension(ApiContext {
            config: Arc::new(config),
            db,
        }));

    let listener = TcpListener::bind(&bind_addr).await.unwrap();

    info!("Server running at http://{}", bind_addr);
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
