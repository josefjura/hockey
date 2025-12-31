use axum::{
    extract::{Path, State},
    response::{Html, IntoResponse},
    Extension, Form,
};
use serde::Deserialize;
use sqlx::SqlitePool;

use crate::app_state::AppState;
use crate::i18n::TranslationContext;
use crate::service::players::{self, PlayerEntity};
use crate::views::{
    components::{error::error_message, htmx::htmx_reload_page},
    pages::player_event_stats::{event_stats_create_modal, event_stats_edit_modal},
};

#[derive(Debug, Deserialize)]
pub struct EventStatsForm {
    event_id: i64,
    goals_total: i32,
    assists_total: i32,
}

#[derive(Debug, Deserialize)]
pub struct EventStatsUpdateForm {
    goals_total: i32,
    assists_total: i32,
}

/// Helper function to fetch player by ID with consistent error handling
async fn get_player_or_error(
    db: &SqlitePool,
    player_id: i64,
) -> Result<PlayerEntity, Html<String>> {
    match players::get_player_by_id(db, player_id).await {
        Ok(Some(player)) => Ok(player),
        Ok(None) => Err(Html(error_message("Player not found").into_string())),
        Err(e) => {
            tracing::error!("Failed to fetch player: {}", e);
            Err(Html(error_message("Failed to load player").into_string()))
        }
    }
}

/// GET /players/{id}/event-stats/new - Show create modal
pub async fn event_stats_create_form(
    Extension(t): Extension<TranslationContext>,
    State(state): State<AppState>,
    Path(player_id): Path<i64>,
) -> impl IntoResponse {
    let player = match get_player_or_error(&state.db, player_id).await {
        Ok(p) => p,
        Err(err) => return err,
    };

    // Get all events for dropdown
    let events = players::get_all_events(&state.db).await.unwrap_or_default();

    Html(event_stats_create_modal(&t, &player, &events, None).into_string())
}

/// POST /players/{id}/event-stats - Create new event stats
pub async fn event_stats_create(
    Extension(t): Extension<TranslationContext>,
    State(state): State<AppState>,
    Path(player_id): Path<i64>,
    Form(form): Form<EventStatsForm>,
) -> impl IntoResponse {
    let player = match get_player_or_error(&state.db, player_id).await {
        Ok(p) => p,
        Err(err) => return err,
    };

    // Validation
    if form.goals_total < 0 || form.assists_total < 0 {
        let events = players::get_all_events(&state.db).await.unwrap_or_default();
        return Html(
            event_stats_create_modal(
                &t,
                &player,
                &events,
                Some("Goals and assists cannot be negative"),
            )
            .into_string(),
        );
    }

    // Create or update event stats atomically (no orphaned zero-value records on failure)
    match players::create_or_update_player_event_stats(
        &state.db,
        player_id,
        form.event_id,
        form.goals_total,
        form.assists_total,
    )
    .await
    {
        Ok(_) => htmx_reload_page(),
        Err(e) => {
            tracing::error!("Failed to save event stats: {}", e);
            let events = players::get_all_events(&state.db).await.unwrap_or_default();
            Html(
                event_stats_create_modal(&t, &player, &events, Some("Failed to save statistics"))
                    .into_string(),
            )
        }
    }
}

/// GET /players/{player_id}/event-stats/{id}/edit - Show edit modal
pub async fn event_stats_edit_form(
    Extension(t): Extension<TranslationContext>,
    State(state): State<AppState>,
    Path((player_id, stats_id)): Path<(i64, i64)>,
) -> impl IntoResponse {
    let player = match get_player_or_error(&state.db, player_id).await {
        Ok(p) => p,
        Err(err) => return err,
    };

    // Get event stats
    let all_stats = match players::get_player_event_stats(&state.db, player_id).await {
        Ok(stats) => stats,
        Err(e) => {
            tracing::error!("Failed to fetch event stats: {}", e);
            return Html(error_message("Failed to load statistics").into_string());
        }
    };

    let stats = match all_stats.iter().find(|s| s.id == stats_id) {
        Some(s) => s,
        None => return Html(error_message("Statistics not found").into_string()),
    };

    Html(event_stats_edit_modal(&t, &player, stats, None).into_string())
}

/// POST /players/{player_id}/event-stats/{id} - Update event stats
pub async fn event_stats_update(
    Extension(t): Extension<TranslationContext>,
    State(state): State<AppState>,
    Path((player_id, stats_id)): Path<(i64, i64)>,
    Form(form): Form<EventStatsUpdateForm>,
) -> impl IntoResponse {
    // Validation
    if form.goals_total < 0 || form.assists_total < 0 {
        // Get player and stats for error display
        let player = match get_player_or_error(&state.db, player_id).await {
            Ok(p) => p,
            Err(err) => return err,
        };

        let all_stats = match players::get_player_event_stats(&state.db, player_id).await {
            Ok(stats) => stats,
            Err(_) => return Html(error_message("Failed to load statistics").into_string()),
        };

        let stats = match all_stats.iter().find(|s| s.id == stats_id) {
            Some(s) => s,
            None => return Html(error_message("Statistics not found").into_string()),
        };

        return Html(
            event_stats_edit_modal(
                &t,
                &player,
                stats,
                Some("Goals and assists cannot be negative"),
            )
            .into_string(),
        );
    }

    // Update the totals
    match players::update_player_event_stats(
        &state.db,
        stats_id,
        form.goals_total,
        form.assists_total,
    )
    .await
    {
        Ok(_) => htmx_reload_page(),
        Err(e) => {
            tracing::error!("Failed to update event stats: {}", e);
            Html(error_message("Failed to save statistics").into_string())
        }
    }
}

/// POST /players/{player_id}/event-stats/{id}/delete - Delete event stats
pub async fn event_stats_delete(
    State(state): State<AppState>,
    Path((_player_id, stats_id)): Path<(i64, i64)>,
) -> impl IntoResponse {
    match players::delete_player_event_stats(&state.db, stats_id).await {
        Ok(_) => htmx_reload_page(),
        Err(e) => {
            tracing::error!("Failed to delete event stats: {}", e);
            Html(error_message("Failed to delete statistics").into_string())
        }
    }
}
