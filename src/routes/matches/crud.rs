use axum::{
    extract::{Path, State},
    http::{HeaderMap, HeaderName},
    response::{Html, IntoResponse},
    Extension, Form,
};
use serde::Deserialize;

use crate::app_state::AppState;
use crate::i18n::TranslationContext;
use crate::service::matches::{self, CreateMatchEntity, UpdateMatchEntity};
use crate::views::{
    components::htmx::htmx_reload_table,
    pages::matches::{match_create_modal, match_edit_modal},
};

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

/// GET /matches/new - Show create modal
pub async fn match_create_form(
    Extension(t): Extension<TranslationContext>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    // Get seasons for dropdown (teams will be loaded dynamically based on season)
    let seasons = matches::get_seasons(&state.db).await.unwrap_or_default();

    Html(match_create_modal(&t, None, &seasons, &[]).into_string())
}

/// POST /matches - Create new match
pub async fn match_create(
    Extension(t): Extension<TranslationContext>,
    State(state): State<AppState>,
    Form(form): Form<CreateMatchForm>,
) -> impl IntoResponse {
    // Get seasons for dropdown (in case we need to show the form again with error)
    let seasons = matches::get_seasons(&state.db).await.unwrap_or_default();
    let teams = matches::get_teams_for_season(&state.db, form.season_id)
        .await
        .unwrap_or_default();

    // Validation
    if form.home_team_id == form.away_team_id {
        return Html(
            match_create_modal(
                &t,
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
            match_create_modal(&t, Some("Scores cannot be negative"), &seasons, &teams)
                .into_string(),
        )
        .into_response();
    }

    // Validate that both teams participate in the selected season
    match matches::validate_teams_in_season(
        &state.db,
        form.season_id,
        form.home_team_id,
        form.away_team_id,
    )
    .await
    {
        Ok(true) => {
            // Both teams participate, proceed
        }
        Ok(false) => {
            return Html(
                match_create_modal(
                    &t,
                    Some("Both teams must participate in the selected season"),
                    &seasons,
                    &teams,
                )
                .into_string(),
            )
            .into_response();
        }
        Err(e) => {
            tracing::error!("Failed to validate teams in season: {}", e);
            return Html(
                match_create_modal(
                    &t,
                    Some("Failed to validate team participation"),
                    &seasons,
                    &teams,
                )
                .into_string(),
            )
            .into_response();
        }
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
            use axum::http::header::{HeaderMap, HeaderName};

            // Return HTMX response to close modal and reload table
            // Trigger entity-created event for dashboard stats update
            let mut headers = HeaderMap::new();
            headers.insert(
                HeaderName::from_static("hx-trigger"),
                "entity-created".parse().unwrap(),
            );
            (headers, htmx_reload_table("/matches/list", "matches-table")).into_response()
        }
        Err(e) => {
            tracing::error!("Failed to create match: {}", e);
            Html(
                match_create_modal(&t, Some("Failed to create match"), &seasons, &teams)
                    .into_string(),
            )
            .into_response()
        }
    }
}

/// GET /matches/{id}/edit - Show edit modal
pub async fn match_edit_form(
    Extension(t): Extension<TranslationContext>,
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
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

    // Get seasons for dropdown and teams for the current season
    let seasons = matches::get_seasons(&state.db).await.unwrap_or_default();
    let teams = matches::get_teams_for_season(&state.db, match_entity.season_id)
        .await
        .unwrap_or_default();

    Html(match_edit_modal(&t, &match_entity, None, &seasons, &teams).into_string())
}

/// POST /matches/{id} - Update match
pub async fn match_update(
    Extension(t): Extension<TranslationContext>,
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Form(form): Form<UpdateMatchForm>,
) -> impl IntoResponse {
    // Get match for re-showing form on error
    let match_entity = matches::get_match_by_id(&state.db, id).await.ok().flatten();

    let Some(match_entity) = match_entity else {
        return Html(
            crate::views::components::error::error_message("Match not found").into_string(),
        )
        .into_response();
    };

    let seasons = matches::get_seasons(&state.db).await.unwrap_or_default();
    let teams = matches::get_teams_for_season(&state.db, form.season_id)
        .await
        .unwrap_or_default();

    // Validation
    if form.home_team_id == form.away_team_id {
        return Html(
            match_edit_modal(
                &t,
                &match_entity,
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
                &t,
                &match_entity,
                Some("Scores cannot be negative"),
                &seasons,
                &teams,
            )
            .into_string(),
        )
        .into_response();
    }

    // Validate that both teams participate in the selected season
    match matches::validate_teams_in_season(
        &state.db,
        form.season_id,
        form.home_team_id,
        form.away_team_id,
    )
    .await
    {
        Ok(true) => {
            // Both teams participate, proceed
        }
        Ok(false) => {
            return Html(
                match_edit_modal(
                    &t,
                    &match_entity,
                    Some("Both teams must participate in the selected season"),
                    &seasons,
                    &teams,
                )
                .into_string(),
            )
            .into_response();
        }
        Err(e) => {
            tracing::error!("Failed to validate teams in season: {}", e);
            return Html(
                match_edit_modal(
                    &t,
                    &match_entity,
                    Some("Failed to validate team participation"),
                    &seasons,
                    &teams,
                )
                .into_string(),
            )
            .into_response();
        }
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
                format!("/matches/{}", id)
                    .parse()
                    .expect("Valid redirect URL should parse"),
            );
            (headers, Html("".to_string())).into_response()
        }
        Ok(false) => Html(
            match_edit_modal(&t, &match_entity, Some("Match not found"), &seasons, &teams)
                .into_string(),
        )
        .into_response(),
        Err(e) => {
            tracing::error!("Failed to update match: {}", e);
            Html(
                match_edit_modal(
                    &t,
                    &match_entity,
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
