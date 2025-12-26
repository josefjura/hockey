use axum::{
    extract::{Path, Query, State},
    http::{HeaderMap, HeaderName},
    response::{Html, IntoResponse},
    Extension, Form,
};
use serde::Deserialize;

use crate::app_state::AppState;
use crate::auth::Session;
use crate::i18n::TranslationContext;
use crate::service::seasons::{
    self, CreateSeasonEntity, SeasonFilters, SortField, SortOrder, UpdateSeasonEntity,
};
use crate::service::team_participations::{self, CreateTeamParticipationEntity};
use crate::views::{
    components::{error::error_message, htmx::htmx_reload_table},
    layout::admin_layout,
    pages::season_detail::{add_team_modal, season_detail_page},
    pages::seasons::{season_create_modal, season_edit_modal, season_list_content, seasons_page},
};

#[derive(Debug, Deserialize)]
pub struct SeasonsQuery {
    #[serde(default = "default_page")]
    page: usize,
    #[serde(default = "default_page_size")]
    page_size: usize,
    #[serde(default, deserialize_with = "crate::utils::empty_string_as_none_i64")]
    event_id: Option<i64>,
    #[serde(default, deserialize_with = "crate::utils::empty_string_as_none_i64")]
    year: Option<i64>,
    #[serde(default = "default_sort")]
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

fn default_sort() -> String {
    "year".to_string()
}

fn default_sort_order() -> String {
    "desc".to_string()
}

#[derive(Debug, Deserialize)]
pub struct CreateSeasonForm {
    year: i64,
    display_name: Option<String>,
    event_id: i64,
    #[serde(default, deserialize_with = "crate::utils::empty_string_as_none")]
    return_url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateSeasonForm {
    year: i64,
    display_name: Option<String>,
    event_id: i64,
}

#[derive(Debug, Deserialize)]
pub struct AddTeamForm {
    team_id: i64,
}

/// GET /seasons - Seasons list page
pub async fn seasons_get(
    Extension(session): Extension<Session>,
    Extension(t): Extension<TranslationContext>,
    State(state): State<AppState>,
    Query(query): Query<SeasonsQuery>,
) -> impl IntoResponse {
    // Build filters
    let filters = SeasonFilters {
        event_id: query.event_id,
        year: query.year,
    };

    // Parse sorting
    let sort_field = SortField::from_str(&query.sort);
    let sort_order = SortOrder::from_str(&query.order);

    // Get seasons
    let result = match seasons::get_seasons(
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
            tracing::error!("Failed to fetch seasons: {}", e);
            return Html(
                admin_layout(
                    "Seasons",
                    &session,
                    "/seasons",
                    &t,
                    crate::views::components::error::error_message("Failed to load seasons"),
                )
                .into_string(),
            );
        }
    };

    // Get events for filter
    let events = seasons::get_events(&state.db).await.unwrap_or_default();

    let content = seasons_page(&t, &result, &filters, &sort_field, &sort_order, &events);
    Html(admin_layout("Seasons", &session, "/seasons", &t, content).into_string())
}

/// GET /seasons/list - HTMX endpoint for table updates
pub async fn seasons_list_partial(
    Extension(t): Extension<TranslationContext>,
    State(state): State<AppState>,
    Query(query): Query<SeasonsQuery>,
) -> impl IntoResponse {
    let filters = SeasonFilters {
        event_id: query.event_id,
        year: query.year,
    };

    // Parse sorting
    let sort_field = SortField::from_str(&query.sort);
    let sort_order = SortOrder::from_str(&query.order);

    let result = match seasons::get_seasons(
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
            tracing::error!("Failed to fetch seasons: {}", e);
            return Html(
                crate::views::components::error::error_message("Failed to load seasons")
                    .into_string(),
            );
        }
    };

    Html(season_list_content(&t, &result, &filters, &sort_field, &sort_order).into_string())
}

/// GET /seasons/new - Show create modal
pub async fn season_create_form(
    Extension(t): Extension<TranslationContext>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let events = seasons::get_events(&state.db).await.unwrap_or_default();
    Html(season_create_modal(&t, None, &events, None).into_string())
}

/// GET /events/{event_id}/seasons/new - Show create modal for specific event
pub async fn event_season_create_form(
    Extension(t): Extension<TranslationContext>,
    State(state): State<AppState>,
    Path(event_id): Path<i64>,
) -> impl IntoResponse {
    // Get event info
    let event = match crate::service::events::get_event_by_id(&state.db, event_id).await {
        Ok(Some(event)) => event,
        Ok(None) => {
            return Html(
                crate::views::components::error::error_message("Event not found").into_string(),
            );
        }
        Err(e) => {
            tracing::error!("Failed to fetch event: {}", e);
            return Html(
                crate::views::components::error::error_message("Failed to load event")
                    .into_string(),
            );
        }
    };

    let events = vec![(event.id, event.name)];
    let return_url = format!("/events/{}", event_id);
    Html(
        crate::views::pages::seasons::season_create_modal_with_return(
            &t,
            None,
            &events,
            Some(event_id),
            Some(&return_url),
        )
        .into_string(),
    )
}

/// POST /seasons - Create new season
pub async fn season_create(
    Extension(t): Extension<TranslationContext>,
    State(state): State<AppState>,
    Form(form): Form<CreateSeasonForm>,
) -> impl IntoResponse {
    // Get events for form re-render on error
    let events = seasons::get_events(&state.db).await.unwrap_or_default();

    // Validation
    if form.year < 1900 || form.year > 2100 {
        return Html(
            season_create_modal(
                &t,
                Some("Year must be between 1900 and 2100"),
                &events,
                None,
            )
            .into_string(),
        );
    }

    if let Some(display_name) = &form.display_name {
        let trimmed = display_name.trim();
        if !trimmed.is_empty() && trimmed.len() > 255 {
            return Html(
                season_create_modal(
                    &t,
                    Some("Display name cannot exceed 255 characters"),
                    &events,
                    None,
                )
                .into_string(),
            );
        }
    }

    // Create season
    match seasons::create_season(
        &state.db,
        CreateSeasonEntity {
            year: form.year,
            display_name: form.display_name.and_then(|s| {
                let trimmed = s.trim();
                if trimmed.is_empty() {
                    None
                } else {
                    Some(trimmed.to_string())
                }
            }),
            event_id: form.event_id,
        },
    )
    .await
    {
        Ok(_) => {
            // Return HTMX response based on context
            if let Some(return_url) = &form.return_url {
                // Redirect to the return URL (e.g., event detail page)
                Html(format!(
                    "<div hx-get=\"{}\" hx-target=\"body\" hx-push-url=\"true\" hx-trigger=\"load\" hx-swap=\"innerHTML\"></div>",
                    return_url
                ))
            } else {
                // Reload the seasons table (default behavior)
                htmx_reload_table("/seasons/list", "seasons-table")
            }
        }
        Err(e) => {
            tracing::error!("Failed to create season: {}", e);
            Html(
                season_create_modal(&t, Some("Failed to create season"), &events, None)
                    .into_string(),
            )
        }
    }
}

/// GET /seasons/{id}/edit - Show edit modal
pub async fn season_edit_form(
    Extension(t): Extension<TranslationContext>,
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    let events = seasons::get_events(&state.db).await.unwrap_or_default();

    let season = match seasons::get_season_by_id(&state.db, id).await {
        Ok(Some(season)) => season,
        Ok(None) => {
            return Html(
                crate::views::components::error::error_message("Season not found").into_string(),
            );
        }
        Err(e) => {
            tracing::error!("Failed to fetch season: {}", e);
            return Html(
                crate::views::components::error::error_message("Failed to load season")
                    .into_string(),
            );
        }
    };

    Html(season_edit_modal(&t, &season, None, &events).into_string())
}

/// POST /seasons/{id} - Update season
pub async fn season_update(
    Extension(t): Extension<TranslationContext>,
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Form(form): Form<UpdateSeasonForm>,
) -> impl IntoResponse {
    let events = seasons::get_events(&state.db).await.unwrap_or_default();

    // Validation
    if form.year < 1900 || form.year > 2100 {
        let season = seasons::get_season_by_id(&state.db, id)
            .await
            .ok()
            .flatten();
        let Some(season) = season else {
            return Html(error_message("Season not found").into_string());
        };
        return Html(
            season_edit_modal(
                &t,
                &season,
                Some("Year must be between 1900 and 2100"),
                &events,
            )
            .into_string(),
        );
    }

    if let Some(display_name) = &form.display_name {
        let trimmed = display_name.trim();
        if !trimmed.is_empty() && trimmed.len() > 255 {
            let season = seasons::get_season_by_id(&state.db, id)
                .await
                .ok()
                .flatten();
            let Some(season) = season else {
                return Html(error_message("Season not found").into_string());
            };
            return Html(
                season_edit_modal(
                    &t,
                    &season,
                    Some("Display name cannot exceed 255 characters"),
                    &events,
                )
                .into_string(),
            );
        }
    }

    // Update season
    match seasons::update_season(
        &state.db,
        id,
        UpdateSeasonEntity {
            year: form.year,
            display_name: form.display_name.and_then(|s| {
                let trimmed = s.trim();
                if trimmed.is_empty() {
                    None
                } else {
                    Some(trimmed.to_string())
                }
            }),
            event_id: form.event_id,
        },
    )
    .await
    {
        Ok(true) => {
            // Return HTMX response to close modal and reload table
            Html("<div hx-get=\"/seasons/list\" hx-target=\"#seasons-table\" hx-trigger=\"load\" hx-swap=\"outerHTML\"></div>".to_string())
        }
        Ok(false) => {
            Html(error_message("Season not found").into_string())
        }
        Err(e) => {
            tracing::error!("Failed to update season: {}", e);
            let season = seasons::get_season_by_id(&state.db, id)
                .await
                .ok()
                .flatten();
            let Some(season) = season else {
                return Html(error_message("Season not found").into_string());
            };
            Html(
                season_edit_modal(
                    &t,
                    &season,
                    Some("Failed to update season"),
                    &events,
                )
                .into_string(),
            )
        }
    }
}

/// POST /seasons/{id}/delete - Delete season
pub async fn season_delete(
    Extension(t): Extension<TranslationContext>,
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Query(query): Query<SeasonsQuery>,
) -> impl IntoResponse {
    match seasons::delete_season(&state.db, id).await {
        Ok(true) => {
            // Reload the table content after successful delete
            let filters = SeasonFilters {
                event_id: query.event_id,
                year: query.year,
            };

            let sort_field = SortField::from_str(&query.sort);
            let sort_order = SortOrder::from_str(&query.order);

            let result = match seasons::get_seasons(
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
                    tracing::error!("Failed to fetch seasons after delete: {}", e);
                    return Html(
                        crate::views::components::error::error_message("Failed to reload seasons")
                            .into_string(),
                    );
                }
            };

            Html(season_list_content(&t, &result, &filters, &sort_field, &sort_order).into_string())
        }
        Ok(false) => {
            Html(crate::views::components::error::error_message("Season not found").into_string())
        }
        Err(e) => {
            tracing::error!("Failed to delete season: {}", e);
            Html(
                crate::views::components::error::error_message("Failed to delete season")
                    .into_string(),
            )
        }
    }
}

/// GET /seasons/{id} - Season detail page
pub async fn season_detail(
    Extension(session): Extension<Session>,
    Extension(t): Extension<TranslationContext>,
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    let detail = match seasons::get_season_detail(&state.db, id).await {
        Ok(Some(detail)) => detail,
        Ok(None) => {
            return Html(
                admin_layout(
                    "Season Not Found",
                    &session,
                    "/seasons",
                    &t,
                    crate::views::components::error::error_message("Season not found"),
                )
                .into_string(),
            );
        }
        Err(e) => {
            tracing::error!("Failed to fetch season detail: {}", e);
            return Html(
                admin_layout(
                    "Error",
                    &session,
                    "/seasons",
                    &t,
                    crate::views::components::error::error_message("Failed to load season"),
                )
                .into_string(),
            );
        }
    };

    let content = season_detail_page(&t, &detail);
    Html(admin_layout("Season Detail", &session, "/seasons", &t, content).into_string())
}

/// GET /seasons/{season_id}/teams/add - Show add team modal
pub async fn season_add_team_form(
    Extension(t): Extension<TranslationContext>,
    State(state): State<AppState>,
    Path(season_id): Path<i64>,
) -> impl IntoResponse {
    let available_teams = team_participations::get_available_teams_for_season(&state.db, season_id)
        .await
        .unwrap_or_default();

    Html(add_team_modal(&t, season_id, None, &available_teams).into_string())
}

/// POST /seasons/{season_id}/teams - Add team to season
pub async fn season_add_team(
    Extension(t): Extension<TranslationContext>,
    State(state): State<AppState>,
    Path(season_id): Path<i64>,
    Form(form): Form<AddTeamForm>,
) -> axum::response::Response {
    // Get available teams for form re-render on error
    let available_teams = team_participations::get_available_teams_for_season(&state.db, season_id)
        .await
        .unwrap_or_default();

    // Check if team is already in season
    match team_participations::is_team_in_season(&state.db, season_id, form.team_id).await {
        Ok(true) => {
            return Html(
                add_team_modal(
                    &t,
                    season_id,
                    Some("This team is already participating in this season"),
                    &available_teams,
                )
                .into_string(),
            )
            .into_response();
        }
        Err(e) => {
            tracing::error!("Failed to check team participation: {}", e);
            return Html(
                add_team_modal(
                    &t,
                    season_id,
                    Some("Failed to check team participation"),
                    &available_teams,
                )
                .into_string(),
            )
            .into_response();
        }
        _ => {}
    }

    // Create team participation
    match team_participations::add_team_to_season(
        &state.db,
        CreateTeamParticipationEntity {
            team_id: form.team_id,
            season_id,
        },
    )
    .await
    {
        Ok(_) => {
            // Return HTMX redirect to season detail page
            let mut headers = HeaderMap::new();
            headers.insert(
                HeaderName::from_static("hx-redirect"),
                format!("/seasons/{}", season_id)
                    .parse()
                    .expect("Valid redirect URL should parse"),
            );
            (headers, Html("".to_string())).into_response()
        }
        Err(e) => {
            tracing::error!("Failed to add team to season: {}", e);
            Html(
                add_team_modal(
                    &t,
                    season_id,
                    Some("Failed to add team. Please try again."),
                    &available_teams,
                )
                .into_string(),
            )
            .into_response()
        }
    }
}

/// POST /team-participations/{id}/delete - Remove team from season
pub async fn team_participation_delete(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> axum::response::Response {
    // Get season_id before deleting for redirect
    let season_id = match team_participations::get_season_id_for_participation(&state.db, id).await
    {
        Ok(Some(season_id)) => season_id,
        Ok(None) => {
            return Html(
                crate::views::components::error::error_message("Team participation not found")
                    .into_string(),
            )
            .into_response();
        }
        Err(e) => {
            tracing::error!("Failed to fetch team participation: {}", e);
            return Html(
                crate::views::components::error::error_message("Failed to remove team from season")
                    .into_string(),
            )
            .into_response();
        }
    };

    match team_participations::remove_team_from_season(&state.db, id).await {
        Ok(true) => {
            // Return HTMX redirect to season detail page
            let mut headers = HeaderMap::new();
            headers.insert(
                HeaderName::from_static("hx-redirect"),
                format!("/seasons/{}", season_id)
                    .parse()
                    .expect("Valid redirect URL should parse"),
            );
            (headers, Html("".to_string())).into_response()
        }
        Ok(false) => Html(
            crate::views::components::error::error_message("Team participation not found")
                .into_string(),
        )
        .into_response(),
        Err(e) => {
            tracing::error!("Failed to remove team from season: {}", e);
            Html(
                crate::views::components::error::error_message("Failed to remove team from season")
                    .into_string(),
            )
            .into_response()
        }
    }
}
