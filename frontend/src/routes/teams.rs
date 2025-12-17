use axum::{
    extract::{Query, State},
    response::{Html, IntoResponse},
    Extension,
};
use serde::Deserialize;

use crate::app_state::AppState;
use crate::auth::Session;
use crate::i18n::Locale;
use crate::service::teams::{self, TeamFilters};
use crate::views::{
    layout::admin_layout,
    pages::teams::{team_list_content, teams_page},
};

#[derive(Debug, Deserialize)]
pub struct TeamsQuery {
    #[serde(default = "default_page")]
    page: usize,
    #[serde(default = "default_page_size")]
    page_size: usize,
    name: Option<String>,
    country_id: Option<i64>,
    #[serde(default)]
    sort: String,
    #[serde(default = "default_sort_order")]
    order: String,
}

fn default_page() -> usize {
    1
}

fn default_page_size() -> usize {
    20
}

fn default_sort_order() -> String {
    "asc".to_string()
}

/// GET /teams - Teams list page
pub async fn teams_list_get(
    Extension(session): Extension<Session>,
    State(state): State<AppState>,
    Query(query): Query<TeamsQuery>,
) -> impl IntoResponse {
    let locale = Locale::English;

    // Build filters
    let filters = TeamFilters {
        name: query.name.clone(),
        country_id: query.country_id,
    };

    // Get teams
    let result = match teams::get_teams(&state.db, &filters, query.page, query.page_size).await {
        Ok(result) => result,
        Err(e) => {
            tracing::error!("Failed to fetch teams: {}", e);
            return Html(
                admin_layout(
                    "Teams",
                    &session,
                    "/teams",
                    &state.i18n,
                    locale,
                    crate::views::components::error::error_message("Failed to load teams"),
                )
                .into_string(),
            );
        }
    };

    // Get countries for filter
    let countries = teams::get_countries(&state.db).await.unwrap_or_default();

    let content = teams_page(&result, &filters, &countries);
    Html(admin_layout("Teams", &session, "/teams", &state.i18n, locale, content).into_string())
}

/// GET /teams/list - HTMX endpoint for table updates
pub async fn teams_list_htmx(
    State(state): State<AppState>,
    Query(query): Query<TeamsQuery>,
) -> impl IntoResponse {
    let filters = TeamFilters {
        name: query.name.clone(),
        country_id: query.country_id,
    };

    let result = match teams::get_teams(&state.db, &filters, query.page, query.page_size).await {
        Ok(result) => result,
        Err(e) => {
            tracing::error!("Failed to fetch teams: {}", e);
            return Html(
                crate::views::components::error::error_message("Failed to load teams")
                    .into_string(),
            );
        }
    };

    Html(team_list_content(&result, &filters).into_string())
}
