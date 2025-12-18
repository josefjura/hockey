use axum::{
    extract::{Path, Query, State},
    response::{Html, IntoResponse},
    Extension, Form,
};
use serde::Deserialize;

use crate::app_state::AppState;
use crate::auth::Session;
use crate::i18n::Locale;
use crate::service::players::{
    self, CreatePlayerEntity, PlayerFilters, SortField, SortOrder, UpdatePlayerEntity,
};
use crate::views::{
    layout::admin_layout,
    pages::players::{player_create_modal, player_edit_modal, player_list_content, players_page},
};

#[derive(Debug, Deserialize)]
pub struct PlayersQuery {
    #[serde(default = "default_page")]
    page: usize,
    #[serde(default = "default_page_size")]
    page_size: usize,
    #[serde(default, deserialize_with = "crate::utils::empty_string_as_none")]
    name: Option<String>,
    #[serde(default, deserialize_with = "crate::utils::empty_string_as_none_i64")]
    country_id: Option<i64>,
    #[serde(default = "default_sort")]
    sort: String,
    #[serde(default = "default_sort_order")]
    order: String,
}

fn default_page() -> usize {
    1
}

fn default_page_size() -> usize {
    20
}

fn default_sort() -> String {
    "name".to_string()
}

fn default_sort_order() -> String {
    "asc".to_string()
}

#[derive(Debug, Deserialize)]
pub struct CreatePlayerForm {
    name: String,
    country_id: i64,
    photo_path: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePlayerForm {
    name: String,
    country_id: i64,
    photo_path: Option<String>,
}

/// GET /players - Players list page
pub async fn players_get(
    Extension(session): Extension<Session>,
    State(state): State<AppState>,
    Query(query): Query<PlayersQuery>,
) -> impl IntoResponse {
    let locale = Locale::English;

    // Build filters
    let filters = PlayerFilters {
        name: query.name.clone(),
        country_id: query.country_id,
    };

    // Parse sorting
    let sort_field = SortField::from_str(&query.sort);
    let sort_order = SortOrder::from_str(&query.order);

    // Get players
    let result = match players::get_players(
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
            tracing::error!("Failed to fetch players: {}", e);
            return Html(
                admin_layout(
                    "Players",
                    &session,
                    "/players",
                    &state.i18n,
                    locale,
                    crate::views::components::error::error_message("Failed to load players"),
                )
                .into_string(),
            );
        }
    };

    // Get countries for filter
    let countries = players::get_countries(&state.db).await.unwrap_or_default();

    let content = players_page(&result, &filters, &sort_field, &sort_order, &countries);
    Html(admin_layout("Players", &session, "/players", &state.i18n, locale, content).into_string())
}

/// GET /players/list - HTMX endpoint for table updates
pub async fn players_list_partial(
    State(state): State<AppState>,
    Query(query): Query<PlayersQuery>,
) -> impl IntoResponse {
    let filters = PlayerFilters {
        name: query.name.clone(),
        country_id: query.country_id,
    };

    // Parse sorting
    let sort_field = SortField::from_str(&query.sort);
    let sort_order = SortOrder::from_str(&query.order);

    let result = match players::get_players(
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
            tracing::error!("Failed to fetch players: {}", e);
            return Html(
                crate::views::components::error::error_message("Failed to load players")
                    .into_string(),
            );
        }
    };

    Html(player_list_content(&result, &filters, &sort_field, &sort_order).into_string())
}

/// GET /players/new - Show create modal
pub async fn player_create_form(State(state): State<AppState>) -> impl IntoResponse {
    let countries = players::get_countries(&state.db).await.unwrap_or_default();
    Html(player_create_modal(None, &countries).into_string())
}

/// POST /players - Create new player
pub async fn player_create(
    State(state): State<AppState>,
    Form(form): Form<CreatePlayerForm>,
) -> impl IntoResponse {
    // Get countries for form re-render on error
    let countries = players::get_countries(&state.db).await.unwrap_or_default();

    // Validation
    let name = form.name.trim();
    if name.is_empty() {
        return Html(player_create_modal(Some("Player name cannot be empty"), &countries).into_string());
    }

    if name.len() > 255 {
        return Html(
            player_create_modal(
                Some("Player name cannot exceed 255 characters"),
                &countries,
            )
            .into_string(),
        );
    }

    // Create player
    match players::create_player(
        &state.db,
        CreatePlayerEntity {
            name: name.to_string(),
            country_id: form.country_id,
            photo_path: form.photo_path,
        },
    )
    .await
    {
        Ok(_) => {
            // Return HTMX response to close modal and reload table
            Html("<div hx-get=\"/players/list\" hx-target=\"#players-table\" hx-trigger=\"load\" hx-swap=\"outerHTML\"></div>".to_string())
        }
        Err(e) => {
            tracing::error!("Failed to create player: {}", e);
            Html(player_create_modal(Some("Failed to create player"), &countries).into_string())
        }
    }
}

/// GET /players/{id}/edit - Show edit modal
pub async fn player_edit_form(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    let countries = players::get_countries(&state.db).await.unwrap_or_default();

    let player = match players::get_player_by_id(&state.db, id).await {
        Ok(Some(player)) => player,
        Ok(None) => {
            return Html(
                crate::views::components::error::error_message("Player not found").into_string(),
            );
        }
        Err(e) => {
            tracing::error!("Failed to fetch player: {}", e);
            return Html(
                crate::views::components::error::error_message("Failed to load player")
                    .into_string(),
            );
        }
    };

    Html(player_edit_modal(&player, None, &countries).into_string())
}

/// POST /players/{id} - Update player
pub async fn player_update(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Form(form): Form<UpdatePlayerForm>,
) -> impl IntoResponse {
    let countries = players::get_countries(&state.db).await.unwrap_or_default();

    // Validation
    let name = form.name.trim();
    if name.is_empty() {
        let player = players::get_player_by_id(&state.db, id).await.ok().flatten();
        return Html(
            player_edit_modal(
                &player.unwrap(),
                Some("Player name cannot be empty"),
                &countries,
            )
            .into_string(),
        );
    }

    if name.len() > 255 {
        let player = players::get_player_by_id(&state.db, id).await.ok().flatten();
        return Html(
            player_edit_modal(
                &player.unwrap(),
                Some("Player name cannot exceed 255 characters"),
                &countries,
            )
            .into_string(),
        );
    }

    // Update player
    match players::update_player(
        &state.db,
        id,
        UpdatePlayerEntity {
            name: name.to_string(),
            country_id: form.country_id,
            photo_path: form.photo_path,
        },
    )
    .await
    {
        Ok(true) => {
            // Return HTMX response to close modal and reload table
            Html("<div hx-get=\"/players/list\" hx-target=\"#players-table\" hx-trigger=\"load\" hx-swap=\"outerHTML\"></div>".to_string())
        }
        Ok(false) => {
            let player = players::get_player_by_id(&state.db, id).await.ok().flatten();
            Html(player_edit_modal(&player.unwrap(), Some("Player not found"), &countries).into_string())
        }
        Err(e) => {
            tracing::error!("Failed to update player: {}", e);
            let player = players::get_player_by_id(&state.db, id).await.ok().flatten();
            Html(
                player_edit_modal(&player.unwrap(), Some("Failed to update player"), &countries)
                    .into_string(),
            )
        }
    }
}

/// POST /players/{id}/delete - Delete player
pub async fn player_delete(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Query(query): Query<PlayersQuery>,
) -> impl IntoResponse {
    match players::delete_player(&state.db, id).await {
        Ok(true) => {
            // Reload the table content after successful delete
            let filters = PlayerFilters {
                name: query.name.clone(),
                country_id: query.country_id,
            };

            let sort_field = SortField::from_str(&query.sort);
            let sort_order = SortOrder::from_str(&query.order);

            let result = match players::get_players(
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
                    tracing::error!("Failed to fetch players after delete: {}", e);
                    return Html(
                        crate::views::components::error::error_message("Failed to reload players")
                            .into_string(),
                    );
                }
            };

            Html(player_list_content(&result, &filters, &sort_field, &sort_order).into_string())
        }
        Ok(false) => Html(
            crate::views::components::error::error_message("Player not found").into_string(),
        ),
        Err(e) => {
            tracing::error!("Failed to delete player: {}", e);
            Html(
                crate::views::components::error::error_message("Failed to delete player")
                    .into_string(),
            )
        }
    }
}
