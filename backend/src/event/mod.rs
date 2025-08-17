use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub mod business;
pub mod routes;
pub mod service;

/// A single Event item.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Event {
    pub id: i64,
    /// The name of the event.
    pub name: String,
    /// The country hosting the event (optional).
    pub country_id: Option<i64>,
    /// The name of the country (optional).
    pub country_name: Option<String>,
    /// The ISO2 code of the country (optional).
    pub country_iso2_code: Option<String>,
    /// When the event was created.
    pub created_at: String,
    /// When the event was last updated.
    pub updated_at: String,
}
