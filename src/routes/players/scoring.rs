use axum::{
    extract::{Path, Query, State},
    response::{Html, IntoResponse},
    Extension,
};
use serde::Deserialize;

use crate::app_state::AppState;
use crate::auth::Session;
use crate::common::pagination::SortOrder;
use crate::i18n::TranslationContext;
use crate::service::players::{self, PlayerScoringFilters, ScoringEventSortField};
use crate::views::{
    components::error::error_message,
    layout::admin_layout,
    pages::player_scoring::{player_scoring_list_content, player_scoring_page},
};

#[derive(Debug, Deserialize)]
pub struct PlayerScoringQuery {
    #[serde(default = "default_page")]
    page: usize,
    #[serde(default = "default_page_size")]
    page_size: usize,
    #[serde(default, deserialize_with = "crate::utils::empty_string_as_none")]
    event_type: Option<String>,
    #[serde(default, deserialize_with = "crate::utils::empty_string_as_none_i64")]
    season_id: Option<i64>,
    #[serde(default, deserialize_with = "crate::utils::empty_string_as_none_i64")]
    team_id: Option<i64>,
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

/// GET /players/{id}/scoring - Player scoring events page
pub async fn player_scoring_get(
    Extension(session): Extension<Session>,
    Extension(t): Extension<TranslationContext>,
    State(state): State<AppState>,
    Path(player_id): Path<i64>,
    Query(query): Query<PlayerScoringQuery>,
) -> impl IntoResponse {
    // Build filters
    let filters = PlayerScoringFilters {
        event_type: query.event_type.clone(),
        season_id: query.season_id,
        team_id: query.team_id,
        date_from: query.date_from.clone(),
        date_to: query.date_to.clone(),
    };

    // Parse sort parameters
    let sort_field = ScoringEventSortField::from_str(&query.sort);
    let sort_order = SortOrder::from_str(&query.order);

    // Fetch all player scoring page data from business layer
    let page_data = match crate::business::players::get_player_scoring_page_data(
        &state.db,
        player_id,
        &filters,
        &sort_field,
        &sort_order,
        query.page,
        query.page_size,
    )
    .await
    {
        Ok(Some(data)) => data,
        Ok(None) => {
            return Html(
                admin_layout(
                    "Player Not Found",
                    &session,
                    "/players",
                    &t,
                    error_message("Player not found"),
                )
                .into_string(),
            );
        }
        Err(e) => {
            tracing::error!("Failed to fetch player scoring page data: {}", e);
            return Html(
                admin_layout(
                    "Error",
                    &session,
                    "/players",
                    &t,
                    error_message("Failed to load player scoring data"),
                )
                .into_string(),
            );
        }
    };

    let content = player_scoring_page(
        &t,
        &page_data.player,
        &page_data.season_stats,
        &page_data.scoring_events,
        &filters,
        &sort_field,
        &sort_order,
        &page_data.seasons,
        &page_data.teams,
    );

    Html(
        admin_layout(
            &format!("{} - Scoring", page_data.player.name),
            &session,
            "/players",
            &t,
            content,
        )
        .into_string(),
    )
}

/// GET /players/{id}/scoring/list - HTMX endpoint for table updates
pub async fn player_scoring_list_partial(
    Extension(t): Extension<TranslationContext>,
    State(state): State<AppState>,
    Path(player_id): Path<i64>,
    Query(query): Query<PlayerScoringQuery>,
) -> impl IntoResponse {
    let filters = PlayerScoringFilters {
        event_type: query.event_type.clone(),
        season_id: query.season_id,
        team_id: query.team_id,
        date_from: query.date_from.clone(),
        date_to: query.date_to.clone(),
    };

    let sort_field = ScoringEventSortField::from_str(&query.sort);
    let sort_order = SortOrder::from_str(&query.order);

    let result = match players::get_player_scoring_events(
        &state.db,
        player_id,
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
            tracing::error!("Failed to fetch player scoring events: {}", e);
            return Html(error_message("Failed to load scoring events").into_string());
        }
    };

    Html(
        player_scoring_list_content(&t, player_id, &result, &filters, &sort_field, &sort_order)
            .into_string(),
    )
}
