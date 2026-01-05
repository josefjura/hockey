use axum::{
    extract::{Path, State},
    response::{Html, IntoResponse},
    Extension, Form,
};
use serde::Deserialize;

use crate::app_state::AppState;
use crate::auth::session::Session;
use crate::business::players::validate_property_change;
use crate::i18n::TranslationContext;
use crate::service::players::{self, PlayerEntity};
use crate::views::{
    components::htmx::htmx_reload_page,
    pages::player_property_changes::{property_change_create_modal, property_change_edit_modal},
};

#[derive(Debug, Deserialize)]
pub struct PropertyChangeForm {
    change_date: String,
    property_type: String,
    #[serde(default)]
    old_value: Option<String>,
    #[serde(default)]
    new_value: Option<String>,
    description: String,
    #[serde(default, deserialize_with = "crate::utils::empty_string_as_none_i64")]
    season_id: Option<i64>,
}

/// Helper function to fetch player by ID with consistent error handling
async fn get_player_or_404(
    db: &sqlx::SqlitePool,
    player_id: i64,
) -> Result<PlayerEntity, Html<String>> {
    match players::get_player_by_id(db, player_id).await {
        Ok(Some(player)) => Ok(player),
        Ok(None) => Err(Html("<div>Player not found</div>".to_string())),
        Err(e) => {
            tracing::error!("Failed to fetch player: {}", e);
            Err(Html("<div>Failed to load player</div>".to_string()))
        }
    }
}

/// GET /players/{id}/property-changes/new - Show create modal
pub async fn property_change_create_form(
    Extension(_session): Extension<Session>,
    Extension(t): Extension<TranslationContext>,
    State(state): State<AppState>,
    Path(player_id): Path<i64>,
) -> impl IntoResponse {
    let player = match get_player_or_404(&state.db, player_id).await {
        Ok(p) => p,
        Err(err) => return err,
    };

    // Get seasons for dropdown
    let seasons = players::get_player_seasons_for_changes(&state.db, player_id)
        .await
        .unwrap_or_default();

    Html(property_change_create_modal(&t, &player, &seasons, None).into_string())
}

/// POST /players/{id}/property-changes - Create new property change
pub async fn property_change_create(
    Extension(_session): Extension<Session>,
    Extension(t): Extension<TranslationContext>,
    State(state): State<AppState>,
    Path(player_id): Path<i64>,
    Form(form): Form<PropertyChangeForm>,
) -> impl IntoResponse {
    let player = match get_player_or_404(&state.db, player_id).await {
        Ok(p) => p,
        Err(err) => return err,
    };

    // Validation
    if let Err(error_msg) =
        validate_property_change(&form.change_date, &form.property_type, &form.description)
    {
        let seasons = players::get_player_seasons_for_changes(&state.db, player_id)
            .await
            .unwrap_or_default();
        return Html(
            property_change_create_modal(&t, &player, &seasons, Some(error_msg)).into_string(),
        );
    }

    // Create property change
    match players::create_property_change(
        &state.db,
        players::CreatePropertyChangeEntity {
            player_id,
            change_date: form.change_date,
            property_type: form.property_type,
            old_value: form.old_value.filter(|s| !s.trim().is_empty()),
            new_value: form.new_value.filter(|s| !s.trim().is_empty()),
            description: form.description,
            season_id: form.season_id,
        },
    )
    .await
    {
        Ok(_) => htmx_reload_page(),
        Err(e) => {
            tracing::error!("Failed to create property change: {}", e);
            let seasons = players::get_player_seasons_for_changes(&state.db, player_id)
                .await
                .unwrap_or_default();

            // Check if it's a UNIQUE constraint violation
            let error_message = if e.to_string().contains("UNIQUE constraint") {
                "A similar property change already exists for this player"
            } else {
                "Failed to save property change"
            };

            Html(
                property_change_create_modal(&t, &player, &seasons, Some(error_message))
                    .into_string(),
            )
        }
    }
}

/// GET /players/{player_id}/property-changes/{id}/edit - Show edit modal
pub async fn property_change_edit_form(
    Extension(_session): Extension<Session>,
    Extension(t): Extension<TranslationContext>,
    State(state): State<AppState>,
    Path((player_id, change_id)): Path<(i64, i64)>,
) -> impl IntoResponse {
    let player = match get_player_or_404(&state.db, player_id).await {
        Ok(p) => p,
        Err(err) => return err,
    };

    // Get property changes
    let all_changes = match players::get_player_property_changes(&state.db, player_id).await {
        Ok(changes) => changes,
        Err(e) => {
            tracing::error!("Failed to fetch property changes: {}", e);
            return Html("<div>Failed to load property changes</div>".to_string());
        }
    };

    let change = match all_changes.iter().find(|c| c.id == change_id) {
        Some(c) => c,
        None => return Html("<div>Property change not found</div>".to_string()),
    };

    let seasons = players::get_player_seasons_for_changes(&state.db, player_id)
        .await
        .unwrap_or_default();

    Html(property_change_edit_modal(&t, &player, change, &seasons, None).into_string())
}

/// POST /players/{player_id}/property-changes/{id} - Update property change
pub async fn property_change_update(
    Extension(_session): Extension<Session>,
    Extension(t): Extension<TranslationContext>,
    State(state): State<AppState>,
    Path((player_id, change_id)): Path<(i64, i64)>,
    Form(form): Form<PropertyChangeForm>,
) -> impl IntoResponse {
    // Validation
    if let Err(error_msg) =
        validate_property_change(&form.change_date, &form.property_type, &form.description)
    {
        let player = match get_player_or_404(&state.db, player_id).await {
            Ok(p) => p,
            Err(err) => return err,
        };

        let all_changes = match players::get_player_property_changes(&state.db, player_id).await {
            Ok(changes) => changes,
            Err(_) => return Html("<div>Failed to load property changes</div>".to_string()),
        };

        let change = match all_changes.iter().find(|c| c.id == change_id) {
            Some(c) => c,
            None => return Html("<div>Property change not found</div>".to_string()),
        };

        let seasons = players::get_player_seasons_for_changes(&state.db, player_id)
            .await
            .unwrap_or_default();

        return Html(
            property_change_edit_modal(&t, &player, change, &seasons, Some(error_msg))
                .into_string(),
        );
    }

    // Update the property change
    match players::update_property_change(
        &state.db,
        change_id,
        players::UpdatePropertyChangeEntity {
            change_date: form.change_date,
            property_type: form.property_type,
            old_value: form.old_value.filter(|s| !s.trim().is_empty()),
            new_value: form.new_value.filter(|s| !s.trim().is_empty()),
            description: form.description,
            season_id: form.season_id,
        },
    )
    .await
    {
        Ok(_) => htmx_reload_page(),
        Err(e) => {
            tracing::error!("Failed to update property change: {}", e);

            // Check if it's a UNIQUE constraint violation
            let error_message = if e.to_string().contains("UNIQUE constraint") {
                "A similar property change already exists for this player"
            } else {
                "Failed to save property change"
            };

            Html(format!("<div>{}</div>", error_message))
        }
    }
}

/// POST /players/{player_id}/property-changes/{id}/delete - Delete property change
pub async fn property_change_delete(
    Extension(_session): Extension<Session>,
    State(state): State<AppState>,
    Path((_player_id, change_id)): Path<(i64, i64)>,
) -> impl IntoResponse {
    match players::delete_property_change(&state.db, change_id).await {
        Ok(_) => htmx_reload_page(),
        Err(e) => {
            tracing::error!("Failed to delete property change: {}", e);
            Html("<div>Failed to delete property change</div>".to_string())
        }
    }
}
