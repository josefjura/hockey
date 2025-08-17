use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub mod routes;
pub mod service;

/// A single Todo item.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Country {
    pub id: i64,
    pub name: String,
    pub enabled: bool,
    pub iihf: bool,
    pub is_historical: bool,
    pub iso2_code: String,
    pub ioc_code: String,
}
