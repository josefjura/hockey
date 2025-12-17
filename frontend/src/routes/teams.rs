use axum::{
    extract::{Query, State},
    response::{Html, IntoResponse},
    Extension, Form,
};
use serde::Deserialize;

use crate::app_state::AppState;
use crate::auth::Session;
use crate::i18n::Locale;
use crate::service::teams::{self, CreateTeamEntity, TeamFilters};
use crate::views::{
    layout::admin_layout,
    pages::teams::{team_create_modal, team_list_content, teams_page},
};

#[derive(Debug, Deserialize)]
pub struct TeamsQuery {
    #[serde(default = "default_page")]
    page: usize,
    #[serde(default = "default_page_size")]
    page_size: usize,
    #[serde(default, deserialize_with = "crate::utils::empty_string_as_none")]
    name: Option<String>,
    #[serde(default, deserialize_with = "crate::utils::empty_string_as_none_i64")]
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

#[derive(Debug, Deserialize)]
pub struct CreateTeamForm {
    name: String,
    country_id: Option<i64>,
}

/// GET /teams - Teams list page
pub async fn teams_get(
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
pub async fn teams_list_partial(
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

/// GET /teams/new - Show create modal
pub async fn team_create_form(State(state): State<AppState>) -> impl IntoResponse {
    let countries = teams::get_countries(&state.db).await.unwrap_or_default();
    Html(team_create_modal(&countries, None).into_string())
}

/// POST /teams - Create new team
pub async fn team_create(
    State(state): State<AppState>,
    Form(form): Form<CreateTeamForm>,
) -> impl IntoResponse {
    // Validation
    let name = form.name.trim();
    if name.is_empty() {
        let countries = teams::get_countries(&state.db).await.unwrap_or_default();
        return Html(
            team_create_modal(&countries, Some("Team name cannot be empty")).into_string(),
        );
    }

    if name.len() > 255 {
        let countries = teams::get_countries(&state.db).await.unwrap_or_default();
        return Html(
            team_create_modal(&countries, Some("Team name cannot exceed 255 characters"))
                .into_string(),
        );
    }

    // Create team
    match teams::create_team(
        &state.db,
        CreateTeamEntity {
            name: name.to_string(),
            country_id: form.country_id,
        },
    )
    .await
    {
        Ok(_) => {
            // Return HTMX response to close modal and reload table
            Html("<div hx-get=\"/teams/list\" hx-target=\"#teams-table\" hx-trigger=\"load\" hx-swap=\"outerHTML\"></div>".to_string())
        }
        Err(e) => {
            tracing::error!("Failed to create team: {}", e);
            let countries = teams::get_countries(&state.db).await.unwrap_or_default();
            Html(team_create_modal(&countries, Some("Failed to create team")).into_string())
        }
    }
}
