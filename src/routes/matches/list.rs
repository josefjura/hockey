use axum::{
    extract::{Query, State},
    response::{Html, IntoResponse},
    Extension,
};
use maud::html;
use serde::Deserialize;

use crate::app_state::AppState;
use crate::auth::Session;
use crate::common::pagination::SortOrder;
use crate::i18n::TranslationContext;
use crate::service::matches::{self, MatchFilters, SortField};
use crate::views::{
    layout::admin_layout,
    pages::matches::{match_list_content, matches_page},
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

#[derive(Debug, Deserialize)]
pub struct TeamsForSeasonQuery {
    season_id: i64,
}

/// GET /matches - Matches list page
pub async fn matches_get(
    Extension(session): Extension<Session>,
    Extension(t): Extension<TranslationContext>,
    State(state): State<AppState>,
    Query(query): Query<MatchesQuery>,
) -> impl IntoResponse {
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
                    &t,
                    crate::views::components::error::error_message(
                        &t,
                        t.messages.error_failed_to_load_matches(),
                    ),
                )
                .into_string(),
            );
        }
    };

    // Get filter data
    let seasons = matches::get_seasons(&state.db).await.unwrap_or_default();
    let teams = matches::get_teams(&state.db).await.unwrap_or_default();

    let content = matches_page(
        &t,
        &result,
        &filters,
        &sort_field,
        &sort_order,
        &seasons,
        &teams,
    );
    Html(admin_layout("Matches", &session, "/matches", &t, content).into_string())
}

/// GET /matches/list - HTMX endpoint for table updates
pub async fn matches_list_partial(
    Extension(t): Extension<TranslationContext>,
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
                crate::views::components::error::error_message(
                    &t,
                    t.messages.error_failed_to_load_matches(),
                )
                .into_string(),
            );
        }
    };

    Html(match_list_content(&t, &result, &filters, &sort_field, &sort_order).into_string())
}

/// GET /matches/teams-for-season - HTMX endpoint to get teams for a selected season
pub async fn teams_for_season(
    Extension(t): Extension<TranslationContext>,
    State(state): State<AppState>,
    Query(query): Query<TeamsForSeasonQuery>,
) -> impl IntoResponse {
    let teams = matches::get_teams_for_season(&state.db, query.season_id)
        .await
        .unwrap_or_default();

    // Return HTML with out-of-band swaps to update both home and away team dropdowns
    // This ensures both dropdowns get updated independently
    Html(
        html! {
            // Primary response (will be ignored since we're using oob swaps)
            div {}
            // Out-of-band swap for home team dropdown (create modal)
            select id="home_team_id" hx-swap-oob="true" name="home_team_id" class="team-select" required
                style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;" {
                option value="" { (t.messages.matches_select_team()) }
                @for (id, name) in &teams {
                    option value=(id) { (name) }
                }
            }
            // Out-of-band swap for away team dropdown (create modal)
            select id="away_team_id" hx-swap-oob="true" name="away_team_id" class="team-select" required
                style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;" {
                option value="" { (t.messages.matches_select_team()) }
                @for (id, name) in &teams {
                    option value=(id) { (name) }
                }
            }
            // Out-of-band swap for home team dropdown (edit modal)
            select id="edit_home_team_id" hx-swap-oob="true" name="home_team_id" class="edit-team-select" required
                style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;" {
                option value="" { (t.messages.matches_select_team()) }
                @for (id, name) in &teams {
                    option value=(id) { (name) }
                }
            }
            // Out-of-band swap for away team dropdown (edit modal)
            select id="edit_away_team_id" hx-swap-oob="true" name="away_team_id" class="edit-team-select" required
                style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;" {
                option value="" { (t.messages.matches_select_team()) }
                @for (id, name) in &teams {
                    option value=(id) { (name) }
                }
            }
        }
        .into_string(),
    )
}
