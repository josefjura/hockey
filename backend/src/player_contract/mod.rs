use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub mod business;
pub mod routes;
pub mod service;

pub use business::PlayerContractBusinessLogic;

/// A single PlayerContract item.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PlayerContract {
    pub id: i64,
    pub team_participation_id: i64,
    pub player_id: i64,
}
