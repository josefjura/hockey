use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub mod business;
pub mod routes;
pub mod service;

/// A hockey team representing a country.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Team {
    pub id: i64,
    /// The name of the team.
    pub name: Option<String>,
    /// The country this team represents.
    pub country_id: i64,
    /// The name of the country.
    pub country_name: String,
    /// The ISO2 code of the country.
    pub country_iso2_code: String,
    /// URL path to the team logo image.
    pub logo_path: Option<String>,
    /// When the team was created.
    pub created_at: String,
    /// When the team was last updated.
    pub updated_at: String,
}

/// Lightweight team representation for dropdowns.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TeamList {
    pub id: i64,
    /// The name of the team.
    pub name: Option<String>,
}

/// Player information in a team roster.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TeamRosterPlayer {
    pub player_id: i64,
    pub player_name: String,
    pub country_name: String,
    pub contract_id: i64,
}

/// Team participation in a season with roster.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TeamSeasonParticipation {
    pub season_id: i64,
    pub season_name: String,
    pub participation_id: i64,
    pub roster: Vec<TeamRosterPlayer>,
}

/// Comprehensive team detail response.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TeamDetail {
    pub team: Team,
    pub participations: Vec<TeamSeasonParticipation>,
}
