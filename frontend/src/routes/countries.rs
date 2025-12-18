use axum::{
    extract::{Path, Query, State},
    response::{Html, IntoResponse, Json},
    Extension,
};
use serde::Deserialize;

use crate::app_state::AppState;
use crate::auth::session::Session;
use crate::i18n::Locale;
use crate::service::countries::{self, CountryFilters};
use crate::views::{layout::admin_layout, pages::countries::countries_page};

#[derive(Debug, Deserialize)]
pub struct CountriesQuery {
    #[serde(default, deserialize_with = "crate::utils::empty_string_as_none")]
    search: Option<String>,
    #[serde(default)]
    iihf_only: bool,
    #[serde(default)]
    enabled_only: bool,
}

/// GET /countries - Countries management page
pub async fn countries_get(
    Extension(session): Extension<Session>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let locale = Locale::English;
    let content = countries_page();
    Html(admin_layout(
        "Countries",
        &session,
        "/countries",
        &state.i18n,
        locale,
        content,
    )
    .into_string())
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

/// POST /api/countries/:id/toggle - Toggle country enabled status
pub async fn country_toggle_enabled(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    match countries::toggle_country_enabled(&state.db, id).await {
        Ok(Some(new_status)) => Json(serde_json::json!({
            "enabled": new_status
        }))
        .into_response(),
        Ok(None) => (
            axum::http::StatusCode::NOT_FOUND,
            "Country not found",
        )
            .into_response(),
        Err(e) => {
            tracing::error!("Failed to toggle country enabled status: {}", e);
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to update country",
            )
                .into_response()
        }
    }
}
