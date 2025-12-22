use axum::{
    extract::{Path, Query, State},
    response::{Html, IntoResponse},
    Extension, Form,
};
use serde::Deserialize;

use crate::app_state::AppState;
use crate::auth::Session;
use crate::i18n::TranslationContext;
use crate::service::teams::{
    self, CreateTeamEntity, SortField, SortOrder, TeamFilters, UpdateTeamEntity,
};
use crate::views::{
    layout::admin_layout,
    pages::team_detail::team_detail_page,
    pages::teams::{team_create_modal, team_edit_modal, team_list_content, teams_page},
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
    #[serde(default = "default_sort")]
    sort: String,
    #[serde(default = "default_order")]
    order: String,
}

fn default_page() -> usize {
    1
}

fn default_page_size() -> usize {
    20
}

fn default_sort() -> String {
    "name".to_string()
}

fn default_order() -> String {
    "asc".to_string()
}

#[derive(Debug, Deserialize)]
pub struct CreateTeamForm {
    name: String,
    country_id: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTeamForm {
    name: String,
    country_id: Option<i64>,
}

/// GET /teams - Teams list page
pub async fn teams_get(
    Extension(session): Extension<Session>,
    Extension(t): Extension<TranslationContext>,
    State(state): State<AppState>,
    Query(query): Query<TeamsQuery>,
) -> impl IntoResponse {
    // Build filters
    let filters = TeamFilters {
        name: query.name.clone(),
        country_id: query.country_id,
    };

    // Parse sort parameters
    let sort_field = SortField::from_str(&query.sort);
    let sort_order = SortOrder::from_str(&query.order);

    // Get teams
    let result = match teams::get_teams(
        &state.db,
        &filters,
        &sort_field,
        &sort_order,
        query.page,
        query.page_size,
    )
    .await
    {
        Ok(result) => result,
        Err(e) => {
            tracing::error!("Failed to fetch teams: {}", e);
            return Html(
                admin_layout(
                    "Teams",
                    &session,
                    "/teams",
                    &t,
                    crate::views::components::error::error_message("Failed to load teams"),
                )
                .into_string(),
            );
        }
    };

    // Get countries for filter
    let countries = teams::get_countries(&state.db).await.unwrap_or_default();

    let content = teams_page(&t, &result, &filters, &sort_field, &sort_order, &countries);
    Html(admin_layout("Teams", &session, "/teams", &t, content).into_string())
}

/// GET /teams/list - HTMX endpoint for table updates
pub async fn teams_list_partial(
    Extension(t): Extension<TranslationContext>,
    State(state): State<AppState>,
    Query(query): Query<TeamsQuery>,
) -> impl IntoResponse {
    let filters = TeamFilters {
        name: query.name.clone(),
        country_id: query.country_id,
    };

    // Parse sort parameters
    let sort_field = SortField::from_str(&query.sort);
    let sort_order = SortOrder::from_str(&query.order);

    let result = match teams::get_teams(
        &state.db,
        &filters,
        &sort_field,
        &sort_order,
        query.page,
        query.page_size,
    )
    .await
    {
        Ok(result) => result,
        Err(e) => {
            tracing::error!("Failed to fetch teams: {}", e);
            return Html(
                crate::views::components::error::error_message("Failed to load teams")
                    .into_string(),
            );
        }
    };

    Html(team_list_content(&t, &result, &filters, &sort_field, &sort_order).into_string())
}

/// GET /teams/new - Show create modal
pub async fn team_create_form(Extension(t): Extension<TranslationContext>) -> impl IntoResponse {
    Html(team_create_modal(&t, None).into_string())
}

/// POST /teams - Create new team
pub async fn team_create(
    Extension(t): Extension<TranslationContext>,
    State(state): State<AppState>,
    Form(form): Form<CreateTeamForm>,
) -> impl IntoResponse {
    // Validation
    let name = form.name.trim();
    if name.is_empty() {
        return Html(team_create_modal(&t, Some("Team name cannot be empty")).into_string());
    }

    if name.len() > 255 {
        return Html(
            team_create_modal(&t, Some("Team name cannot exceed 255 characters")).into_string(),
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
            Html(team_create_modal(&t, Some("Failed to create team")).into_string())
        }
    }
}

/// GET /teams/{id}/edit - Show edit modal
pub async fn team_edit_form(
    Extension(t): Extension<TranslationContext>,
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    let team = match teams::get_team_by_id(&state.db, id).await {
        Ok(Some(team)) => team,
        Ok(None) => {
            return Html(
                crate::views::components::error::error_message("Team not found").into_string(),
            );
        }
        Err(e) => {
            tracing::error!("Failed to fetch team: {}", e);
            return Html(
                crate::views::components::error::error_message("Failed to load team").into_string(),
            );
        }
    };

    Html(team_edit_modal(&t, &team, None).into_string())
}

/// POST /teams/{id} - Update team
pub async fn team_update(
    Extension(t): Extension<TranslationContext>,
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Form(form): Form<UpdateTeamForm>,
) -> impl IntoResponse {
    // Validation
    let name = form.name.trim();
    if name.is_empty() {
        let team = teams::get_team_by_id(&state.db, id).await.ok().flatten();
        return Html(
            team_edit_modal(&t, &team.unwrap(), Some("Team name cannot be empty")).into_string(),
        );
    }

    if name.len() > 255 {
        let team = teams::get_team_by_id(&state.db, id).await.ok().flatten();
        return Html(
            team_edit_modal(
                &t,
                &team.unwrap(),
                Some("Team name cannot exceed 255 characters"),
            )
            .into_string(),
        );
    }

    // Update team
    match teams::update_team(
        &state.db,
        id,
        UpdateTeamEntity {
            name: name.to_string(),
            country_id: form.country_id,
        },
    )
    .await
    {
        Ok(true) => {
            // Return HTMX response to close modal and reload table
            Html("<div hx-get=\"/teams/list\" hx-target=\"#teams-table\" hx-trigger=\"load\" hx-swap=\"outerHTML\"></div>".to_string())
        }
        Ok(false) => {
            let team = teams::get_team_by_id(&state.db, id).await.ok().flatten();
            Html(team_edit_modal(&t, &team.unwrap(), Some("Team not found")).into_string())
        }
        Err(e) => {
            tracing::error!("Failed to update team: {}", e);
            let team = teams::get_team_by_id(&state.db, id).await.ok().flatten();
            Html(team_edit_modal(&t, &team.unwrap(), Some("Failed to update team")).into_string())
        }
    }
}

/// POST /teams/{id}/delete - Delete team
pub async fn team_delete(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Query(query): Query<TeamsQuery>,
) -> impl IntoResponse {
    match teams::delete_team(&state.db, id).await {
        Ok(true) => {
            // Build URL to reload table with current filters and sorting
            let mut reload_url = format!(
                "/teams/list?page={}&page_size={}&sort={}&order={}",
                query.page, query.page_size, query.sort, query.order
            );

            if let Some(name) = &query.name {
                reload_url.push_str(&format!("&name={}", urlencoding::encode(name)));
            }

            if let Some(country_id) = query.country_id {
                reload_url.push_str(&format!("&country_id={}", country_id));
            }

            // Return HTMX response to reload table with filters
            Html(format!(
                "<div hx-get=\"{}\" hx-target=\"#teams-table\" hx-trigger=\"load\" hx-swap=\"outerHTML\"></div>",
                reload_url
            ))
        }
        Ok(false) => {
            Html(crate::views::components::error::error_message("Team not found").into_string())
        }
        Err(e) => {
            tracing::error!("Failed to delete team: {}", e);
            Html(
                crate::views::components::error::error_message("Failed to delete team")
                    .into_string(),
            )
        }
    }
}

/// GET /teams/{id} - Team detail page
pub async fn team_detail(
    Extension(session): Extension<Session>,
    Extension(t): Extension<TranslationContext>,
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    let detail = match teams::get_team_detail(&state.db, id).await {
        Ok(Some(detail)) => detail,
        Ok(None) => {
            return Html(
                admin_layout(
                    "Team Not Found",
                    &session,
                    "/teams",
                    &t,
                    crate::views::components::error::error_message("Team not found"),
                )
                .into_string(),
            );
        }
        Err(e) => {
            tracing::error!("Failed to fetch team detail: {}", e);
            return Html(
                admin_layout(
                    "Error",
                    &session,
                    "/teams",
                    &t,
                    crate::views::components::error::error_message("Failed to load team"),
                )
                .into_string(),
            );
        }
    };

    let content = team_detail_page(&t, &detail);
    Html(admin_layout("Team Detail", &session, "/teams", &t, content).into_string())
}
