use anyhow::Context;
use clap::Parser;
use dotenvy::dotenv;
use sqlx::sqlite::SqlitePoolOptions;

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

    let db = SqlitePoolOptions::new()
        .max_connections(50)
        .connect(&config.database_url)
        .await
        .context("could not connect to database_url")?;

    http::serve(config, db).await;

    Ok(())
}
