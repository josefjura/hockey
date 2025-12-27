use axum::{
    extract::{Path, State},
    http::{HeaderMap, HeaderName},
    response::{Html, IntoResponse},
    Extension, Form,
};
use serde::Deserialize;

use crate::app_state::AppState;
use crate::i18n::TranslationContext;
use crate::service::matches::{self, CreateScoreEventEntity, UpdateScoreEventEntity};
use crate::views::pages::matches::{score_event_create_modal, score_event_edit_modal};

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

/// GET /matches/{match_id}/score-events/new - Show create score event modal
pub async fn score_event_create_form(
    Extension(t): Extension<TranslationContext>,
    State(state): State<AppState>,
    Path(match_id): Path<i64>,
) -> impl IntoResponse {
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
        score_event_create_modal(&t, None, &match_info, &home_players, &away_players).into_string(),
    )
}

/// POST /matches/{match_id}/score-events - Create new score event
pub async fn score_event_create(
    Extension(t): Extension<TranslationContext>,
    State(state): State<AppState>,
    Path(match_id): Path<i64>,
    Form(form): Form<CreateScoreEventForm>,
) -> impl IntoResponse {
    // Get match info for re-showing form on error
    let match_info = matches::get_match_by_id(&state.db, match_id)
        .await
        .ok()
        .flatten();

    let Some(match_info) = match_info else {
        return Html(
            crate::views::components::error::error_message("Match not found").into_string(),
        )
        .into_response();
    };

    let home_players = matches::get_players_for_team(&state.db, match_info.home_team_id)
        .await
        .unwrap_or_default();
    let away_players = matches::get_players_for_team(&state.db, match_info.away_team_id)
        .await
        .unwrap_or_default();

    // Validation
    if form.period < 1 || form.period > 5 {
        return Html(
            score_event_create_modal(
                &t,
                Some("Period must be between 1 and 5"),
                &match_info,
                &home_players,
                &away_players,
            )
            .into_string(),
        )
        .into_response();
    }

    if let Some(minutes) = form.time_minutes {
        if !(0..=60).contains(&minutes) {
            return Html(
                score_event_create_modal(
                    &t,
                    Some("Minutes must be between 0 and 60"),
                    &match_info,
                    &home_players,
                    &away_players,
                )
                .into_string(),
            )
            .into_response();
        }
    }

    if let Some(seconds) = form.time_seconds {
        if !(0..=59).contains(&seconds) {
            return Html(
                score_event_create_modal(
                    &t,
                    Some("Seconds must be between 0 and 59"),
                    &match_info,
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
                format!("/matches/{}", match_id)
                    .parse()
                    .expect("Valid redirect URL should parse"),
            );
            (headers, Html("".to_string())).into_response()
        }
        Err(e) => {
            tracing::error!("Failed to create score event: {}", e);
            Html(
                score_event_create_modal(
                    &t,
                    Some("Failed to create goal"),
                    &match_info,
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
    Extension(t): Extension<TranslationContext>,
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
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
            &t,
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
    Extension(t): Extension<TranslationContext>,
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Form(form): Form<UpdateScoreEventForm>,
) -> impl IntoResponse {
    // Get score event and match for re-showing form on error
    let score_event = matches::get_score_event_by_id(&state.db, id)
        .await
        .ok()
        .flatten();

    let Some(score_event) = score_event else {
        return Html(
            crate::views::components::error::error_message("Score event not found").into_string(),
        )
        .into_response();
    };

    let match_id = score_event.match_id;
    let match_info = matches::get_match_by_id(&state.db, match_id)
        .await
        .ok()
        .flatten();

    let Some(match_info) = match_info else {
        return Html(
            crate::views::components::error::error_message("Match not found").into_string(),
        )
        .into_response();
    };

    let home_players = matches::get_players_for_team(&state.db, match_info.home_team_id)
        .await
        .unwrap_or_default();
    let away_players = matches::get_players_for_team(&state.db, match_info.away_team_id)
        .await
        .unwrap_or_default();

    // Validation
    if form.period < 1 || form.period > 5 {
        return Html(
            score_event_edit_modal(
                &t,
                Some("Period must be between 1 and 5"),
                &score_event,
                &match_info,
                &home_players,
                &away_players,
            )
            .into_string(),
        )
        .into_response();
    }

    if let Some(minutes) = form.time_minutes {
        if !(0..=60).contains(&minutes) {
            return Html(
                score_event_edit_modal(
                    &t,
                    Some("Minutes must be between 0 and 60"),
                    &score_event,
                    &match_info,
                    &home_players,
                    &away_players,
                )
                .into_string(),
            )
            .into_response();
        }
    }

    if let Some(seconds) = form.time_seconds {
        if !(0..=59).contains(&seconds) {
            return Html(
                score_event_edit_modal(
                    &t,
                    Some("Seconds must be between 0 and 59"),
                    &score_event,
                    &match_info,
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
                format!("/matches/{}", match_id)
                    .parse()
                    .expect("Valid redirect URL should parse"),
            );
            (headers, Html("".to_string())).into_response()
        }
        Ok(false) => Html(
            score_event_edit_modal(
                &t,
                Some("Score event not found"),
                &score_event,
                &match_info,
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
                    &t,
                    Some("Failed to update goal"),
                    &score_event,
                    &match_info,
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
                format!("/matches/{}", match_id)
                    .parse()
                    .expect("Valid redirect URL should parse"),
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
