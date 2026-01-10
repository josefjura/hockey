use axum::{
    extract::{Path, State},
    http::{HeaderMap, HeaderName},
    response::{Html, IntoResponse},
    Extension, Form,
};
use serde::Deserialize;

use crate::app_state::AppState;
use crate::auth::Session;
use crate::i18n::TranslationContext;
use crate::service::player_contracts;
use crate::views::{
    layout::admin_layout,
    pages::roster::{add_player_modal, roster_page},
};

#[derive(Debug, Deserialize)]
pub struct AddPlayerForm {
    player_id: i64,
}

/// GET /team-participations/{id}/roster - Roster management page
pub async fn roster_get(
    Extension(session): Extension<Session>,
    Extension(t): Extension<TranslationContext>,
    State(state): State<AppState>,
    Path(team_participation_id): Path<i64>,
) -> impl IntoResponse {
    // Get team participation context
    let context =
        match player_contracts::get_team_participation_context(&state.db, team_participation_id)
            .await
        {
            Ok(Some(context)) => context,
            Ok(None) => {
                return Html(
                    admin_layout(
                        "Team Participation Not Found",
                        &session,
                        "/seasons",
                        &t,
                        crate::views::components::error::error_message(
                            &t,
                            t.messages.error_team_participation_not_found(),
                        ),
                    )
                    .into_string(),
                );
            }
            Err(e) => {
                tracing::error!(
                    "Failed to fetch team participation context for {}: {}",
                    team_participation_id,
                    e
                );
                return Html(
                    admin_layout(
                        "Error",
                        &session,
                        "/seasons",
                        &t,
                        crate::views::components::error::error_message(
                            &t,
                            t.messages.error_team_participation_not_found(),
                        ),
                    )
                    .into_string(),
                );
            }
        };

    // Get current roster
    let roster = match player_contracts::get_roster(&state.db, team_participation_id).await {
        Ok(roster) => roster,
        Err(e) => {
            tracing::error!(
                "Failed to fetch roster for team participation {}: {}",
                team_participation_id,
                e
            );
            return Html(
                admin_layout(
                    "Error",
                    &session,
                    &format!("/seasons/{}", context.season_id),
                    &t,
                    crate::views::components::error::error_message(
                        &t,
                        t.messages.error_failed_to_load_roster(),
                    ),
                )
                .into_string(),
            );
        }
    };

    let content = roster_page(&t, &context, &roster);
    Html(admin_layout("Roster Management", &session, "/seasons", &t, content).into_string())
}

/// GET /team-participations/{id}/roster/add-player - Form/modal to add player
pub async fn roster_add_player_form(
    Extension(t): Extension<TranslationContext>,
    State(state): State<AppState>,
    Path(team_participation_id): Path<i64>,
) -> impl IntoResponse {
    let available_players =
        player_contracts::get_available_players(&state.db, team_participation_id)
            .await
            .unwrap_or_default();

    Html(add_player_modal(&t, team_participation_id, None, &available_players).into_string())
}

/// POST /team-participations/{id}/roster - Add player to roster
pub async fn roster_add_player(
    Extension(t): Extension<TranslationContext>,
    State(state): State<AppState>,
    Path(team_participation_id): Path<i64>,
    Form(form): Form<AddPlayerForm>,
) -> axum::response::Response {
    // Get available players for form re-render on error
    let available_players =
        player_contracts::get_available_players(&state.db, team_participation_id)
            .await
            .unwrap_or_default();

    // Check if player is already in roster
    match player_contracts::is_player_in_roster(&state.db, team_participation_id, form.player_id)
        .await
    {
        Ok(true) => {
            return Html(
                add_player_modal(
                    &t,
                    team_participation_id,
                    Some("This player is already in the roster"),
                    &available_players,
                )
                .into_string(),
            )
            .into_response();
        }
        Err(e) => {
            tracing::error!("Failed to check player in roster: {}", e);
            return Html(
                add_player_modal(
                    &t,
                    team_participation_id,
                    Some("Failed to check player status"),
                    &available_players,
                )
                .into_string(),
            )
            .into_response();
        }
        _ => {}
    }

    // Add player to roster
    match player_contracts::add_player_to_roster(&state.db, team_participation_id, form.player_id)
        .await
    {
        Ok(_) => {
            // Return HTMX redirect to roster page
            let mut headers = HeaderMap::new();
            headers.insert(
                HeaderName::from_static("hx-redirect"),
                format!("/team-participations/{}/roster", team_participation_id)
                    .parse()
                    .expect("Valid redirect URL should parse"),
            );
            (headers, Html("".to_string())).into_response()
        }
        Err(e) => {
            tracing::error!("Failed to add player to roster: {}", e);
            Html(
                add_player_modal(
                    &t,
                    team_participation_id,
                    Some("Failed to add player. Please try again."),
                    &available_players,
                )
                .into_string(),
            )
            .into_response()
        }
    }
}

/// POST /player-contracts/{id}/delete - Remove player from roster
pub async fn player_contract_delete(
    Extension(t): Extension<TranslationContext>,
    State(state): State<AppState>,
    Path(player_contract_id): Path<i64>,
) -> axum::response::Response {
    // Get team_participation_id before deleting for redirect
    let team_participation_id = match player_contracts::get_team_participation_id_for_contract(
        &state.db,
        player_contract_id,
    )
    .await
    {
        Ok(Some(id)) => id,
        Ok(None) => {
            return Html(
                crate::views::components::error::error_message(
                    &t,
                    t.messages.error_player_contract_not_found(),
                )
                .into_string(),
            )
            .into_response();
        }
        Err(e) => {
            tracing::error!("Failed to fetch player contract: {}", e);
            return Html(
                crate::views::components::error::error_message(
                    &t,
                    t.messages.error_failed_to_load_roster(),
                )
                .into_string(),
            )
            .into_response();
        }
    };

    match player_contracts::remove_player_from_roster(&state.db, player_contract_id).await {
        Ok(true) => {
            // Return HTMX redirect to roster page
            let mut headers = HeaderMap::new();
            headers.insert(
                HeaderName::from_static("hx-redirect"),
                format!("/team-participations/{}/roster", team_participation_id)
                    .parse()
                    .expect("Valid redirect URL should parse"),
            );
            (headers, Html("".to_string())).into_response()
        }
        Ok(false) => Html(
            crate::views::components::error::error_message(
                &t,
                t.messages.error_player_contract_not_found(),
            )
            .into_string(),
        )
        .into_response(),
        Err(e) => {
            tracing::error!("Failed to remove player from roster: {}", e);
            Html(
                crate::views::components::error::error_message(
                    &t,
                    t.messages.error_failed_to_load_roster(),
                )
                .into_string(),
            )
            .into_response()
        }
    }
}
