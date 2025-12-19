use axum::{
    extract::{Path, Query, State},
    response::{Html, IntoResponse},
    Extension,
};
use serde::Deserialize;

use crate::app_state::AppState;
use crate::auth::Session;
use crate::common::pagination::SortOrder;
use crate::i18n::Locale;
use crate::service::matches::{self, MatchFilters, SortField};
use crate::views::{
    layout::admin_layout,
    pages::matches::{match_detail_page, match_list_content, matches_page},
};

#[derive(Debug, Deserialize)]
pub struct MatchesQuery {
    #[serde(default = "default_page")]
    page: usize,
    #[serde(default = "default_page_size")]
    page_size: usize,
    #[serde(default, deserialize_with = "crate::utils::empty_string_as_none_i64")]
    season_id: Option<i64>,
    #[serde(default, deserialize_with = "crate::utils::empty_string_as_none_i64")]
    team_id: Option<i64>,
    #[serde(default, deserialize_with = "crate::utils::empty_string_as_none")]
    status: Option<String>,
    #[serde(default, deserialize_with = "crate::utils::empty_string_as_none")]
    date_from: Option<String>,
    #[serde(default, deserialize_with = "crate::utils::empty_string_as_none")]
    date_to: Option<String>,
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
    "date".to_string()
}

fn default_order() -> String {
    "desc".to_string()
}

/// GET /matches - Matches list page
pub async fn matches_get(
    Extension(session): Extension<Session>,
    State(state): State<AppState>,
    Query(query): Query<MatchesQuery>,
) -> impl IntoResponse {
    let locale = Locale::English;

    // Build filters
    let filters = MatchFilters {
        season_id: query.season_id,
        team_id: query.team_id,
        status: query.status.clone(),
        date_from: query.date_from.clone(),
        date_to: query.date_to.clone(),
    };

    // Parse sort parameters
    let sort_field = SortField::from_str(&query.sort);
    let sort_order = SortOrder::from_str(&query.order);

    // Get matches
    let result = match matches::get_matches(
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
            tracing::error!("Failed to fetch matches: {}", e);
            return Html(
                admin_layout(
                    "Matches",
                    &session,
                    "/matches",
                    &state.i18n,
                    locale,
                    crate::views::components::error::error_message("Failed to load matches"),
                )
                .into_string(),
            );
        }
    };

    // Get filter data
    let seasons = matches::get_seasons(&state.db).await.unwrap_or_default();
    let teams = matches::get_teams(&state.db).await.unwrap_or_default();

    let content = matches_page(&result, &filters, &sort_field, &sort_order, &seasons, &teams);
    Html(admin_layout("Matches", &session, "/matches", &state.i18n, locale, content).into_string())
}

/// GET /matches/list - HTMX endpoint for table updates
pub async fn matches_list_partial(
    State(state): State<AppState>,
    Query(query): Query<MatchesQuery>,
) -> impl IntoResponse {
    let filters = MatchFilters {
        season_id: query.season_id,
        team_id: query.team_id,
        status: query.status.clone(),
        date_from: query.date_from.clone(),
        date_to: query.date_to.clone(),
    };

    // Parse sort parameters
    let sort_field = SortField::from_str(&query.sort);
    let sort_order = SortOrder::from_str(&query.order);

    let result = match matches::get_matches(
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
            tracing::error!("Failed to fetch matches: {}", e);
            return Html(
                crate::views::components::error::error_message("Failed to load matches")
                    .into_string(),
            );
        }
    };

    Html(match_list_content(&result, &filters, &sort_field, &sort_order).into_string())
}

/// GET /matches/{id} - Match detail page
pub async fn match_detail(
    Extension(session): Extension<Session>,
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    let locale = Locale::English;

    // Get match detail
    let match_detail = match matches::get_match_detail(&state.db, id).await {
        Ok(Some(detail)) => detail,
        Ok(None) => {
            return Html(
                admin_layout(
                    "Match Not Found",
                    &session,
                    "/matches",
                    &state.i18n,
                    locale,
                    crate::views::components::error::error_message("Match not found"),
                )
                .into_string(),
            );
        }
        Err(e) => {
            tracing::error!("Failed to fetch match detail: {}", e);
            return Html(
                admin_layout(
                    "Error",
                    &session,
                    "/matches",
                    &state.i18n,
                    locale,
                    crate::views::components::error::error_message("Failed to load match detail"),
                )
                .into_string(),
            );
        }
    };

    let content = match_detail_page(&match_detail);
    Html(
        admin_layout(
            "Match Detail",
            &session,
            "/matches",
            &state.i18n,
            locale,
            content,
        )
        .into_string(),
    )
}

/// POST /matches/{id}/delete - Delete match
pub async fn match_delete(State(state): State<AppState>, Path(id): Path<i64>) -> impl IntoResponse {
    match matches::delete_match(&state.db, id).await {
        Ok(true) => {
            // Redirect to matches list using HTMX redirect header
            Html(r#"<div hx-redirect="/matches"></div>"#.to_string())
        }
        Ok(false) => {
            Html(crate::views::components::error::error_message("Match not found").into_string())
        }
        Err(e) => {
            tracing::error!("Failed to delete match: {}", e);
            Html(
                crate::views::components::error::error_message("Failed to delete match")
                    .into_string(),
            )
        }
    }
}
