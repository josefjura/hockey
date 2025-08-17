use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub mod business;
pub mod routes;
pub mod service;

pub use business::TeamParticipationBusinessLogic;

/// A single TeamParticipation item.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TeamParticipation {
    pub id: i64,
    pub team_id: i64,
    pub season_id: i64,
}
