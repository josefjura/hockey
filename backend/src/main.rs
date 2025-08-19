use anyhow::Context;
use clap::Parser;
use dotenvy::dotenv;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::ConnectOptions;
use std::str::FromStr;

use crate::config::Config;

mod auth;
mod common;
mod config;
mod country;
mod docs;
mod errors;
mod event;
mod http;
mod r#match;
mod player;
mod player_contract;
mod season;
mod team;
mod team_participation;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenv().ok();

    let config = Config::parse();

    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_thread_names(true)
        .with_ansi(true)
        .init();

    aide::generate::on_error(|error| {
        println!("{error}");
    });

    aide::generate::extract_schemas(true);

    // Set up SQLite connection options with auto-creation
    let connection_options = SqliteConnectOptions::from_str(&config.database_url)?
        .create_if_missing(true)
        .log_statements(tracing::log::LevelFilter::Debug);

    let db = SqlitePoolOptions::new()
        .max_connections(50)
        .connect_with(connection_options)
        .await
        .context("could not connect to database_url")?;

    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&db)
        .await
        .context("could not run database migrations")?;

    http::serve(config, db).await;

    Ok(())
}
