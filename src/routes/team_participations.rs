use axum::{
    extract::{Query, State},
    response::{Html, IntoResponse, Response},
    Extension, Form,
};
use axum::http::{HeaderMap, HeaderName};
use serde::Deserialize;

use crate::app_state::AppState;
use crate::auth::Session;
use crate::i18n::TranslationContext;
use crate::service::team_participations::{self, CreateTeamParticipationEntity};
use crate::views::pages::team_participations::team_participation_create_modal;

#[derive(Debug, Deserialize)]
pub struct CreateFormQuery {
    /// Team ID to prefill (when opening from team detail)
    #[serde(default)]
    team_id: Option<i64>,
    /// Season ID to prefill (when opening from season detail)
    #[serde(default)]
    season_id: Option<i64>,
    /// Where to redirect after successful creation
    #[serde(default)]
    return_to: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateForm {
    team_id: i64,
    season_id: i64,
    #[serde(default)]
    return_to: Option<String>,
}

/// GET /team-participations/new - Show create modal
///
/// Query params:
/// - team_id: Prefill team selection (e.g., from team detail page)
/// - season_id: Prefill season selection (e.g., from season detail page)
/// - return_to: URL to redirect after success
pub async fn team_participation_create_form(
    Extension(_session): Extension<Session>,
    Extension(t): Extension<TranslationContext>,
    State(state): State<AppState>,
    Query(query): Query<CreateFormQuery>,
) -> impl IntoResponse {
    // Load available teams and seasons
    let (available_teams, available_seasons) = match tokio::try_join!(
        team_participations::get_all_teams_for_dropdown(&state.db),
        team_participations::get_all_seasons_for_dropdown(&state.db),
    ) {
        Ok((teams, seasons)) => (teams, seasons),
        Err(e) => {
            tracing::error!("Failed to load teams/seasons for form: {}", e);
            return Html(
                crate::views::components::error::error_message(
                    &t.messages.error_loading_form_data().to_string()
                ).into_string()
            );
        }
    };

    Html(team_participation_create_modal(
        &t,
        None,
        query.team_id,
        query.season_id,
        query.return_to.as_deref(),
        &available_teams,
        &available_seasons,
    ).into_string())
}

/// POST /team-participations - Create team participation
pub async fn team_participation_create(
    Extension(_session): Extension<Session>,
    Extension(t): Extension<TranslationContext>,
    State(state): State<AppState>,
    Form(form): Form<CreateForm>,
) -> Response {
    // Validate: check if team is already in this season
    match team_participations::is_team_in_season(&state.db, form.season_id, form.team_id).await {
        Ok(true) => {
            // Team already in season - show error in modal
            tracing::warn!(
                "Attempt to add team {} to season {} failed: already exists",
                form.team_id,
                form.season_id
            );

            // Reload form with error
            let (available_teams, available_seasons) = match tokio::try_join!(
                team_participations::get_all_teams_for_dropdown(&state.db),
                team_participations::get_all_seasons_for_dropdown(&state.db),
            ) {
                Ok((teams, seasons)) => (teams, seasons),
                Err(e) => {
                    tracing::error!("Failed to reload form data after error: {}", e);
                    return Html(
                        crate::views::components::error::error_message(
                            &t.messages.error_system().to_string()
                        ).into_string()
                    ).into_response();
                }
            };

            return Html(team_participation_create_modal(
                &t,
                Some(&t.messages.error_team_already_in_season().to_string()),
                Some(form.team_id),
                Some(form.season_id),
                form.return_to.as_deref(),
                &available_teams,
                &available_seasons,
            ).into_string()).into_response();
        }
        Ok(false) => {
            // Good to proceed
        }
        Err(e) => {
            tracing::error!("Database error checking team participation: {}", e);
            return Html(
                crate::views::components::error::error_message(
                    &t.messages.error_system().to_string()
                ).into_string()
            ).into_response();
        }
    }

    // Create the team participation
    match team_participations::add_team_to_season(
        &state.db,
        CreateTeamParticipationEntity {
            team_id: form.team_id,
            season_id: form.season_id,
        },
    ).await {
        Ok(_) => {
            tracing::info!(
                "Successfully added team {} to season {}",
                form.team_id,
                form.season_id
            );

            // Redirect to return_to URL or default to teams list
            let redirect_url = form.return_to.unwrap_or_else(|| "/team-participations".to_string());

            let mut headers = HeaderMap::new();
            headers.insert(
                HeaderName::from_static("hx-redirect"),
                redirect_url.parse().unwrap(),
            );
            (headers, Html(String::new())).into_response()
        }
        Err(e) => {
            tracing::error!("Failed to create team participation: {}", e);

            // Reload form with error
            let (available_teams, available_seasons) = match tokio::try_join!(
                team_participations::get_all_teams_for_dropdown(&state.db),
                team_participations::get_all_seasons_for_dropdown(&state.db),
            ) {
                Ok((teams, seasons)) => (teams, seasons),
                Err(e) => {
                    tracing::error!("Failed to reload form data after error: {}", e);
                    return Html(
                        crate::views::components::error::error_message(
                            &t.messages.error_system().to_string()
                        ).into_string()
                    ).into_response();
                }
            };

            Html(team_participation_create_modal(
                &t,
                Some(&t.messages.error_creating_participation().to_string()),
                Some(form.team_id),
                Some(form.season_id),
                form.return_to.as_deref(),
                &available_teams,
                &available_seasons,
            ).into_string()).into_response()
        }
    }
}
