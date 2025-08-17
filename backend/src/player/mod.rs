use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub mod business;
pub mod routes;
pub mod service;

/// A hockey player representing a country.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Player {
    pub id: i64,
    /// The name of the player.
    pub name: String,
    /// The country this player represents.
    pub country_id: i64,
    /// The name of the country.
    pub country_name: String,
    /// The ISO2 code of the country.
    pub country_iso2_code: String,
    /// URL path to the player photo image.
    pub photo_path: Option<String>,
    /// When the player was created.
    pub created_at: String,
    /// When the player was last updated.
    pub updated_at: String,
}
