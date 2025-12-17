use axum::{
    extract::{Query, State},
    response::{IntoResponse, Json},
};
use serde::Deserialize;

use crate::app_state::AppState;
use crate::service::countries::{self, CountryFilters};

#[derive(Debug, Deserialize)]
pub struct CountriesQuery {
    #[serde(default, deserialize_with = "crate::utils::empty_string_as_none")]
    search: Option<String>,
    #[serde(default)]
    iihf_only: bool,
    #[serde(default)]
    enabled_only: bool,
}

/// GET /api/countries - JSON API endpoint for country selector
pub async fn countries_list_api(
    State(state): State<AppState>,
    Query(query): Query<CountriesQuery>,
) -> impl IntoResponse {
    let filters = CountryFilters {
        search: query.search,
        iihf_only: query.iihf_only,
        enabled_only: query.enabled_only,
    };

    match countries::get_countries(&state.db, &filters).await {
        Ok(countries) => Json(countries).into_response(),
        Err(e) => {
            tracing::error!("Failed to fetch countries: {}", e);
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to fetch countries",
            )
                .into_response()
        }
    }
}
