use axum::{
    extract::{Path, Query, State},
    http::{HeaderMap, HeaderName},
    response::{Html, IntoResponse},
    Extension, Form,
};
use axum_extra::extract::CookieJar;
use serde::Deserialize;

use crate::app_state::AppState;
use crate::auth::Session;
use crate::common::pagination::SortOrder;
use crate::routes::locale::get_locale_from_cookies;
use crate::service::matches::{
    self, CreateMatchEntity, CreateScoreEventEntity, MatchFilters, SortField, UpdateMatchEntity,
    UpdateScoreEventEntity,
};
use crate::views::{
    layout::admin_layout,
    pages::matches::{
        match_create_modal, match_detail_page, match_edit_modal, match_list_content, matches_page,
        score_event_create_modal, score_event_edit_modal,
    },
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
pub struct CreateMatchForm {
    season_id: i64,
    home_team_id: i64,
    away_team_id: i64,
    #[serde(default)]
    home_score_unidentified: i32,
    #[serde(default)]
    away_score_unidentified: i32,
    #[serde(default, deserialize_with = "crate::utils::empty_string_as_none")]
    match_date: Option<String>,
    status: String,
    #[serde(default, deserialize_with = "crate::utils::empty_string_as_none")]
    venue: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateMatchForm {
    season_id: i64,
    home_team_id: i64,
    away_team_id: i64,
    #[serde(default)]
    home_score_unidentified: i32,
    #[serde(default)]
    away_score_unidentified: i32,
    #[serde(default, deserialize_with = "crate::utils::empty_string_as_none")]
    match_date: Option<String>,
    status: String,
    #[serde(default, deserialize_with = "crate::utils::empty_string_as_none")]
    venue: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateScoreEventForm {
    team_id: i64,
    #[serde(default, deserialize_with = "crate::utils::empty_string_as_none_i64")]
    scorer_id: Option<i64>,
    #[serde(default, deserialize_with = "crate::utils::empty_string_as_none_i64")]
    assist1_id: Option<i64>,
    #[serde(default, deserialize_with = "crate::utils::empty_string_as_none_i64")]
    assist2_id: Option<i64>,
    period: i32,
    #[serde(default, deserialize_with = "crate::utils::empty_string_as_none_i32")]
    time_minutes: Option<i32>,
    #[serde(default, deserialize_with = "crate::utils::empty_string_as_none_i32")]
    time_seconds: Option<i32>,
    #[serde(default, deserialize_with = "crate::utils::empty_string_as_none")]
    goal_type: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateScoreEventForm {
    team_id: i64,
    #[serde(default, deserialize_with = "crate::utils::empty_string_as_none_i64")]
    scorer_id: Option<i64>,
    #[serde(default, deserialize_with = "crate::utils::empty_string_as_none_i64")]
    assist1_id: Option<i64>,
    #[serde(default, deserialize_with = "crate::utils::empty_string_as_none_i64")]
    assist2_id: Option<i64>,
    period: i32,
    #[serde(default, deserialize_with = "crate::utils::empty_string_as_none_i32")]
    time_minutes: Option<i32>,
    #[serde(default, deserialize_with = "crate::utils::empty_string_as_none_i32")]
    time_seconds: Option<i32>,
    #[serde(default, deserialize_with = "crate::utils::empty_string_as_none")]
    goal_type: Option<String>,
}

/// GET /matches - Matches list page
pub async fn matches_get(
    Extension(session): Extension<Session>,
    State(state): State<AppState>,
    jar: CookieJar,
    Query(query): Query<MatchesQuery>,
) -> impl IntoResponse {
    let locale = get_locale_from_cookies(&jar);

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

    let content = matches_page(
        &state.i18n,
        locale,
        &result,
        &filters,
        &sort_field,
        &sort_order,
        &seasons,
        &teams,
    );
    Html(
        admin_layout(
            "Matches",
            &session,
            "/matches",
            &state.i18n,
            locale,
            content,
        )
        .into_string(),
    )
}

/// GET /matches/list - HTMX endpoint for table updates
pub async fn matches_list_partial(
    State(state): State<AppState>,
    jar: CookieJar,
    Query(query): Query<MatchesQuery>,
) -> impl IntoResponse {
    let locale = get_locale_from_cookies(&jar);

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

    Html(
        match_list_content(
            &state.i18n,
            locale,
            &result,
            &filters,
            &sort_field,
            &sort_order,
        )
        .into_string(),
    )
}

/// GET /matches/new - Show create modal
pub async fn match_create_form(State(state): State<AppState>, jar: CookieJar) -> impl IntoResponse {
    let locale = get_locale_from_cookies(&jar);

    // Get seasons and teams for dropdowns
    let seasons = matches::get_seasons(&state.db).await.unwrap_or_default();
    let teams = matches::get_teams(&state.db).await.unwrap_or_default();

    Html(match_create_modal(&state.i18n, locale, None, &seasons, &teams).into_string())
}

/// POST /matches - Create new match
pub async fn match_create(
    State(state): State<AppState>,
    jar: CookieJar,
    Form(form): Form<CreateMatchForm>,
) -> impl IntoResponse {
    let locale = get_locale_from_cookies(&jar);

    // Get seasons and teams for dropdowns (in case we need to show the form again with error)
    let seasons = matches::get_seasons(&state.db).await.unwrap_or_default();
    let teams = matches::get_teams(&state.db).await.unwrap_or_default();

    // Validation
    if form.home_team_id == form.away_team_id {
        return Html(
            match_create_modal(
                &state.i18n,
                locale,
                Some("Home and away teams must be different"),
                &seasons,
                &teams,
            )
            .into_string(),
        );
    }

    if form.home_score_unidentified < 0 || form.away_score_unidentified < 0 {
        return Html(
            match_create_modal(
                &state.i18n,
                locale,
                Some("Scores cannot be negative"),
                &seasons,
                &teams,
            )
            .into_string(),
        );
    }

    // Create match
    match matches::create_match(
        &state.db,
        CreateMatchEntity {
            season_id: form.season_id,
            home_team_id: form.home_team_id,
            away_team_id: form.away_team_id,
            home_score_unidentified: form.home_score_unidentified,
            away_score_unidentified: form.away_score_unidentified,
            match_date: form.match_date,
            status: form.status,
            venue: form.venue,
        },
    )
    .await
    {
        Ok(_) => {
            // Return HTMX response to close modal and reload table
            Html("<div hx-get=\"/matches/list\" hx-target=\"#matches-table\" hx-trigger=\"load\" hx-swap=\"outerHTML\"></div>".to_string())
        }
        Err(e) => {
            tracing::error!("Failed to create match: {}", e);
            Html(
                match_create_modal(
                    &state.i18n,
                    locale,
                    Some("Failed to create match"),
                    &seasons,
                    &teams,
                )
                .into_string(),
            )
        }
    }
}

/// GET /matches/{id}/edit - Show edit modal
pub async fn match_edit_form(
    State(state): State<AppState>,
    jar: CookieJar,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    let locale = get_locale_from_cookies(&jar);

    let match_entity = match matches::get_match_by_id(&state.db, id).await {
        Ok(Some(m)) => m,
        Ok(None) => {
            return Html(
                crate::views::components::error::error_message("Match not found").into_string(),
            );
        }
        Err(e) => {
            tracing::error!("Failed to fetch match: {}", e);
            return Html(
                crate::views::components::error::error_message("Failed to load match")
                    .into_string(),
            );
        }
    };

    // Get seasons and teams for dropdowns
    let seasons = matches::get_seasons(&state.db).await.unwrap_or_default();
    let teams = matches::get_teams(&state.db).await.unwrap_or_default();

    Html(match_edit_modal(&state.i18n, locale, &match_entity, None, &seasons, &teams).into_string())
}

/// POST /matches/{id} - Update match
pub async fn match_update(
    State(state): State<AppState>,
    jar: CookieJar,
    Path(id): Path<i64>,
    Form(form): Form<UpdateMatchForm>,
) -> impl IntoResponse {
    // Get match for re-showing form on error
    let match_entity = matches::get_match_by_id(&state.db, id).await.ok().flatten();
    let seasons = matches::get_seasons(&state.db).await.unwrap_or_default();
    let teams = matches::get_teams(&state.db).await.unwrap_or_default();

    let locale = get_locale_from_cookies(&jar);

    // Validation
    if form.home_team_id == form.away_team_id {
        return Html(
            match_edit_modal(
                &state.i18n,
                locale,
                &match_entity.unwrap(),
                Some("Home and away teams must be different"),
                &seasons,
                &teams,
            )
            .into_string(),
        )
        .into_response();
    }

    if form.home_score_unidentified < 0 || form.away_score_unidentified < 0 {
        return Html(
            match_edit_modal(
                &state.i18n,
                locale,
                &match_entity.unwrap(),
                Some("Scores cannot be negative"),
                &seasons,
                &teams,
            )
            .into_string(),
        )
        .into_response();
    }

    // Update match
    match matches::update_match(
        &state.db,
        id,
        UpdateMatchEntity {
            season_id: form.season_id,
            home_team_id: form.home_team_id,
            away_team_id: form.away_team_id,
            home_score_unidentified: form.home_score_unidentified,
            away_score_unidentified: form.away_score_unidentified,
            match_date: form.match_date,
            status: form.status,
            venue: form.venue,
        },
    )
    .await
    {
        Ok(true) => {
            // Redirect back to match detail page using HX-Redirect header
            let mut headers = HeaderMap::new();
            headers.insert(
                HeaderName::from_static("hx-redirect"),
                format!("/matches/{}", id).parse().unwrap(),
            );
            (headers, Html("".to_string())).into_response()
        }
        Ok(false) => Html(
            match_edit_modal(
                &state.i18n,
                locale,
                &match_entity.unwrap(),
                Some("Match not found"),
                &seasons,
                &teams,
            )
            .into_string(),
        )
        .into_response(),
        Err(e) => {
            tracing::error!("Failed to update match: {}", e);
            Html(
                match_edit_modal(
                    &state.i18n,
                    locale,
                    &match_entity.unwrap(),
                    Some("Failed to update match"),
                    &seasons,
                    &teams,
                )
                .into_string(),
            )
            .into_response()
        }
    }
}

/// GET /matches/{id} - Match detail page
pub async fn match_detail(
    Extension(session): Extension<Session>,
    State(state): State<AppState>,
    jar: CookieJar,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    let locale = get_locale_from_cookies(&jar);

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

    let content = match_detail_page(&state.i18n, locale, &match_detail);
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

// Score Event Handlers

/// GET /matches/{match_id}/score-events/new - Show create score event modal
pub async fn score_event_create_form(
    State(state): State<AppState>,
    jar: CookieJar,
    Path(match_id): Path<i64>,
) -> impl IntoResponse {
    let locale = get_locale_from_cookies(&jar);

    // Get match to get home and away teams
    let match_info = match matches::get_match_by_id(&state.db, match_id).await {
        Ok(Some(m)) => m,
        Ok(None) => {
            return Html(
                crate::views::components::error::error_message("Match not found").into_string(),
            );
        }
        Err(e) => {
            tracing::error!("Failed to fetch match: {}", e);
            return Html(
                crate::views::components::error::error_message("Failed to load match")
                    .into_string(),
            );
        }
    };

    // Get players for both teams
    let home_players = matches::get_players_for_team(&state.db, match_info.home_team_id)
        .await
        .unwrap_or_default();
    let away_players = matches::get_players_for_team(&state.db, match_info.away_team_id)
        .await
        .unwrap_or_default();

    Html(
        score_event_create_modal(
            &state.i18n,
            locale,
            None,
            &match_info,
            &home_players,
            &away_players,
        )
        .into_string(),
    )
}

/// POST /matches/{match_id}/score-events - Create new score event
pub async fn score_event_create(
    State(state): State<AppState>,
    jar: CookieJar,
    Path(match_id): Path<i64>,
    Form(form): Form<CreateScoreEventForm>,
) -> impl IntoResponse {
    let locale = get_locale_from_cookies(&jar);

    // Get match info for re-showing form on error
    let match_info = matches::get_match_by_id(&state.db, match_id)
        .await
        .ok()
        .flatten();
    let home_players = matches::get_players_for_team(
        &state.db,
        match_info.as_ref().map(|m| m.home_team_id).unwrap_or(0),
    )
    .await
    .unwrap_or_default();
    let away_players = matches::get_players_for_team(
        &state.db,
        match_info.as_ref().map(|m| m.away_team_id).unwrap_or(0),
    )
    .await
    .unwrap_or_default();

    // Validation
    if form.period < 1 || form.period > 5 {
        return Html(
            score_event_create_modal(
                &state.i18n,
                locale,
                Some("Period must be between 1 and 5"),
                &match_info.unwrap(),
                &home_players,
                &away_players,
            )
            .into_string(),
        )
        .into_response();
    }

    if let Some(minutes) = form.time_minutes {
        if minutes < 0 || minutes > 60 {
            return Html(
                score_event_create_modal(
                    &state.i18n,
                    locale,
                    Some("Minutes must be between 0 and 60"),
                    &match_info.unwrap(),
                    &home_players,
                    &away_players,
                )
                .into_string(),
            )
            .into_response();
        }
    }

    if let Some(seconds) = form.time_seconds {
        if seconds < 0 || seconds > 59 {
            return Html(
                score_event_create_modal(
                    &state.i18n,
                    locale,
                    Some("Seconds must be between 0 and 59"),
                    &match_info.unwrap(),
                    &home_players,
                    &away_players,
                )
                .into_string(),
            )
            .into_response();
        }
    }

    // Create score event
    match matches::create_score_event(
        &state.db,
        CreateScoreEventEntity {
            match_id,
            team_id: form.team_id,
            scorer_id: form.scorer_id,
            assist1_id: form.assist1_id,
            assist2_id: form.assist2_id,
            period: form.period,
            time_minutes: form.time_minutes,
            time_seconds: form.time_seconds,
            goal_type: form.goal_type,
        },
    )
    .await
    {
        Ok(_) => {
            // Redirect back to match detail page using HX-Redirect header
            let mut headers = HeaderMap::new();
            headers.insert(
                HeaderName::from_static("hx-redirect"),
                format!("/matches/{}", match_id).parse().unwrap(),
            );
            (headers, Html("".to_string())).into_response()
        }
        Err(e) => {
            tracing::error!("Failed to create score event: {}", e);
            Html(
                score_event_create_modal(
                    &state.i18n,
                    locale,
                    Some("Failed to create goal"),
                    &match_info.unwrap(),
                    &home_players,
                    &away_players,
                )
                .into_string(),
            )
            .into_response()
        }
    }
}

/// GET /matches/score-events/{id}/edit - Show edit score event modal
pub async fn score_event_edit_form(
    State(state): State<AppState>,
    jar: CookieJar,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    let locale = get_locale_from_cookies(&jar);

    let score_event = match matches::get_score_event_by_id(&state.db, id).await {
        Ok(Some(se)) => se,
        Ok(None) => {
            return Html(
                crate::views::components::error::error_message("Score event not found")
                    .into_string(),
            );
        }
        Err(e) => {
            tracing::error!("Failed to fetch score event: {}", e);
            return Html(
                crate::views::components::error::error_message("Failed to load score event")
                    .into_string(),
            );
        }
    };

    // Get match to get home and away teams
    let match_info = match matches::get_match_by_id(&state.db, score_event.match_id).await {
        Ok(Some(m)) => m,
        Ok(None) => {
            return Html(
                crate::views::components::error::error_message("Match not found").into_string(),
            );
        }
        Err(e) => {
            tracing::error!("Failed to fetch match: {}", e);
            return Html(
                crate::views::components::error::error_message("Failed to load match")
                    .into_string(),
            );
        }
    };

    // Get players for both teams
    let home_players = matches::get_players_for_team(&state.db, match_info.home_team_id)
        .await
        .unwrap_or_default();
    let away_players = matches::get_players_for_team(&state.db, match_info.away_team_id)
        .await
        .unwrap_or_default();

    Html(
        score_event_edit_modal(
            &state.i18n,
            locale,
            None,
            &score_event,
            &match_info,
            &home_players,
            &away_players,
        )
        .into_string(),
    )
}

/// POST /matches/score-events/{id} - Update score event
pub async fn score_event_update(
    State(state): State<AppState>,
    jar: CookieJar,
    Path(id): Path<i64>,
    Form(form): Form<UpdateScoreEventForm>,
) -> impl IntoResponse {
    let locale = get_locale_from_cookies(&jar);

    // Get score event and match for re-showing form on error
    let score_event = matches::get_score_event_by_id(&state.db, id)
        .await
        .ok()
        .flatten();
    let match_id = score_event.as_ref().map(|se| se.match_id).unwrap_or(0);
    let match_info = matches::get_match_by_id(&state.db, match_id)
        .await
        .ok()
        .flatten();
    let home_players = matches::get_players_for_team(
        &state.db,
        match_info.as_ref().map(|m| m.home_team_id).unwrap_or(0),
    )
    .await
    .unwrap_or_default();
    let away_players = matches::get_players_for_team(
        &state.db,
        match_info.as_ref().map(|m| m.away_team_id).unwrap_or(0),
    )
    .await
    .unwrap_or_default();

    // Validation
    if form.period < 1 || form.period > 5 {
        return Html(
            score_event_edit_modal(
                &state.i18n,
                locale,
                Some("Period must be between 1 and 5"),
                &score_event.unwrap(),
                &match_info.unwrap(),
                &home_players,
                &away_players,
            )
            .into_string(),
        )
        .into_response();
    }

    if let Some(minutes) = form.time_minutes {
        if minutes < 0 || minutes > 60 {
            return Html(
                score_event_edit_modal(
                    &state.i18n,
                    locale,
                    Some("Minutes must be between 0 and 60"),
                    &score_event.unwrap(),
                    &match_info.unwrap(),
                    &home_players,
                    &away_players,
                )
                .into_string(),
            )
            .into_response();
        }
    }

    if let Some(seconds) = form.time_seconds {
        if seconds < 0 || seconds > 59 {
            return Html(
                score_event_edit_modal(
                    &state.i18n,
                    locale,
                    Some("Seconds must be between 0 and 59"),
                    &score_event.unwrap(),
                    &match_info.unwrap(),
                    &home_players,
                    &away_players,
                )
                .into_string(),
            )
            .into_response();
        }
    }

    // Update score event
    match matches::update_score_event(
        &state.db,
        id,
        UpdateScoreEventEntity {
            team_id: form.team_id,
            scorer_id: form.scorer_id,
            assist1_id: form.assist1_id,
            assist2_id: form.assist2_id,
            period: form.period,
            time_minutes: form.time_minutes,
            time_seconds: form.time_seconds,
            goal_type: form.goal_type,
        },
    )
    .await
    {
        Ok(true) => {
            // Redirect back to match detail page using HX-Redirect header
            let mut headers = HeaderMap::new();
            headers.insert(
                HeaderName::from_static("hx-redirect"),
                format!("/matches/{}", match_id).parse().unwrap(),
            );
            (headers, Html("".to_string())).into_response()
        }
        Ok(false) => Html(
            score_event_edit_modal(
                &state.i18n,
                locale,
                Some("Score event not found"),
                &score_event.unwrap(),
                &match_info.unwrap(),
                &home_players,
                &away_players,
            )
            .into_string(),
        )
        .into_response(),
        Err(e) => {
            tracing::error!("Failed to update score event: {}", e);
            Html(
                score_event_edit_modal(
                    &state.i18n,
                    locale,
                    Some("Failed to update goal"),
                    &score_event.unwrap(),
                    &match_info.unwrap(),
                    &home_players,
                    &away_players,
                )
                .into_string(),
            )
            .into_response()
        }
    }
}

/// POST /matches/score-events/{id}/delete - Delete score event
pub async fn score_event_delete(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    // Get match_id before deleting
    let match_id = match matches::get_score_event_by_id(&state.db, id).await {
        Ok(Some(se)) => se.match_id,
        Ok(None) => {
            return Html(
                crate::views::components::error::error_message("Score event not found")
                    .into_string(),
            )
            .into_response();
        }
        Err(e) => {
            tracing::error!("Failed to fetch score event: {}", e);
            return Html(
                crate::views::components::error::error_message("Failed to load score event")
                    .into_string(),
            )
            .into_response();
        }
    };

    match matches::delete_score_event(&state.db, id).await {
        Ok(true) => {
            // Redirect back to match detail page using HX-Redirect header
            let mut headers = HeaderMap::new();
            headers.insert(
                HeaderName::from_static("hx-redirect"),
                format!("/matches/{}", match_id).parse().unwrap(),
            );
            (headers, Html("".to_string())).into_response()
        }
        Ok(false) => Html(
            crate::views::components::error::error_message("Score event not found").into_string(),
        )
        .into_response(),
        Err(e) => {
            tracing::error!("Failed to delete score event: {}", e);
            Html(
                crate::views::components::error::error_message("Failed to delete score event")
                    .into_string(),
            )
            .into_response()
        }
    }
}
