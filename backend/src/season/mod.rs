use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub mod business;
pub mod routes;
pub mod service;

/// A hockey season of an event.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Season {
    pub id: i64,
    /// The year of the season.
    pub year: i64,
    /// The display name of the season.
    pub display_name: Option<String>,
    /// The event this season belongs to.
    pub event_id: i64,
    /// The name of the event.
    pub event_name: String,
    /// When the season was created.
    pub created_at: String,
    /// When the season was last updated.
    pub updated_at: String,
}

/// Lightweight season representation for dropdowns.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SeasonList {
    pub id: i64,
    /// The display name of the season.
    pub name: Option<String>,
    /// The year of the season.
    pub year: i64,
    /// The name of the event this season belongs to.
    pub event_name: String,
}

/// Player representation for dropdowns.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PlayerDropdown {
    pub id: i64,
    /// The name of the player.
    pub name: String,
    /// The nationality of the player.
    pub nationality: String,
}
