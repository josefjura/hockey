use axum::{
    extract::{Multipart, Path, Query, State},
    response::{Html, IntoResponse},
    Extension,
};
use serde::Deserialize;

use crate::app_state::AppState;
use crate::auth::Session;
use crate::i18n::TranslationContext;
use crate::service::players::{
    self, CreatePlayerEntity, PlayerFilters, SortField, SortOrder, UpdatePlayerEntity,
};
use crate::views::{
    components::{
        error::error_message,
        htmx::{htmx_reload_table, htmx_reload_table_with_stats},
    },
    layout::admin_layout,
    pages::player_detail::player_detail_page,
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

/// GET /players - Players list page
pub async fn players_get(
    Extension(session): Extension<Session>,
    Extension(t): Extension<TranslationContext>,
    State(state): State<AppState>,
    Query(query): Query<PlayersQuery>,
) -> impl IntoResponse {
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
                    &t,
                    crate::views::components::error::error_message("Failed to load players"),
                )
                .into_string(),
            );
        }
    };

    // Get countries for filter
    let countries = players::get_countries(&state.db).await.unwrap_or_default();

    let content = players_page(&t, &result, &filters, &sort_field, &sort_order, &countries);
    Html(admin_layout("Players", &session, "/players", &t, content).into_string())
}

/// GET /players/list - HTMX endpoint for table updates
pub async fn players_list_partial(
    Extension(t): Extension<TranslationContext>,
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

    Html(player_list_content(&t, &result, &filters, &sort_field, &sort_order).into_string())
}

/// GET /players/new - Show create modal
pub async fn player_create_form(
    Extension(t): Extension<TranslationContext>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let countries = players::get_countries(&state.db).await.unwrap_or_default();
    Html(player_create_modal(&t, None, &countries).into_string())
}

/// POST /players - Create new player
pub async fn player_create(
    Extension(t): Extension<TranslationContext>,
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> impl IntoResponse {
    // Get countries for form re-render on error
    let countries = players::get_countries(&state.db).await.unwrap_or_default();

    // Parse multipart form data
    let mut name = String::new();
    let mut country_id: Option<i64> = None;
    let mut photo_path: Option<String> = None;
    let mut photo_url: Option<String> = None;
    let mut birth_date: Option<String> = None;
    let mut birth_place: Option<String> = None;
    let mut height_cm: Option<i64> = None;
    let mut weight_kg: Option<i64> = None;
    let mut position: Option<String> = None;
    let mut shoots: Option<String> = None;

    while let Ok(Some(field)) = multipart.next_field().await {
        let field_name = field.name().unwrap_or("").to_string();

        match field_name.as_str() {
            "name" => {
                name = field.text().await.unwrap_or_default();
            }
            "country_id" => {
                let text = field.text().await.unwrap_or_default();
                country_id = text.parse().ok();
            }
            "photo_url" => {
                photo_url = Some(field.text().await.unwrap_or_default());
            }
            "birth_date" => {
                let text = field.text().await.unwrap_or_default();
                birth_date = if text.trim().is_empty() { None } else { Some(text) };
            }
            "birth_place" => {
                let text = field.text().await.unwrap_or_default();
                birth_place = if text.trim().is_empty() { None } else { Some(text) };
            }
            "height_cm" => {
                let text = field.text().await.unwrap_or_default();
                height_cm = text.parse().ok();
            }
            "weight_kg" => {
                let text = field.text().await.unwrap_or_default();
                weight_kg = text.parse().ok();
            }
            "position" => {
                let text = field.text().await.unwrap_or_default();
                position = if text.trim().is_empty() { None } else { Some(text) };
            }
            "shoots" => {
                let text = field.text().await.unwrap_or_default();
                shoots = if text.trim().is_empty() { None } else { Some(text) };
            }
            "photo_file" => {
                // Handle file upload
                let filename = field.file_name().unwrap_or("photo.jpg").to_string();
                let data = field.bytes().await.unwrap_or_default();

                if !data.is_empty() {
                    match crate::utils::save_uploaded_file(
                        &data,
                        &filename,
                        "static/uploads/players",
                    )
                    .await
                    {
                        Ok(path) => photo_path = Some(path),
                        Err(e) => {
                            tracing::error!("Failed to save uploaded file: {}", e);
                            return Html(player_create_modal(&t, Some("Failed to save photo. Only image files (jpg, png, gif, webp) are allowed."), &countries).into_string());
                        }
                    }
                }
            }
            _ => {}
        }
    }

    // Validation
    let name = name.trim();
    if name.is_empty() {
        return Html(
            player_create_modal(&t, Some("Player name cannot be empty"), &countries).into_string(),
        );
    }

    if name.len() > 255 {
        return Html(
            player_create_modal(
                &t,
                Some("Player name cannot exceed 255 characters"),
                &countries,
            )
            .into_string(),
        );
    }

    let country_id = match country_id {
        Some(id) => id,
        None => {
            return Html(
                player_create_modal(&t, Some("Please select a country"), &countries).into_string(),
            );
        }
    };

    // Prefer uploaded file over URL
    let final_photo_path = if photo_path.is_some() {
        photo_path
    } else {
        photo_url.filter(|url| !url.trim().is_empty())
    };

    // Create player
    match players::create_player(
        &state.db,
        CreatePlayerEntity {
            name: name.to_string(),
            country_id,
            photo_path: final_photo_path,
            birth_date,
            birth_place,
            height_cm,
            weight_kg,
            position,
            shoots,
        },
    )
    .await
    {
        Ok(_) => {
            // Fetch updated dashboard stats
            let stats = crate::service::dashboard::get_dashboard_stats(&state.db)
                .await
                .unwrap_or_else(|e| {
                    tracing::warn!(
                        "Failed to fetch dashboard stats after player creation: {}",
                        e
                    );
                    crate::service::dashboard::DashboardStats::default()
                });

            // Return HTMX response to close modal, reload table, and update dashboard stats
            htmx_reload_table_with_stats("/players/list", "players-table", &t, &stats)
        }
        Err(e) => {
            tracing::error!("Failed to create player: {}", e);
            Html(player_create_modal(&t, Some("Failed to create player"), &countries).into_string())
        }
    }
}

/// GET /players/{id}/edit - Show edit modal
pub async fn player_edit_form(
    Extension(t): Extension<TranslationContext>,
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

    Html(player_edit_modal(&t, &player, None, &countries).into_string())
}

/// POST /players/{id} - Update player
pub async fn player_update(
    Extension(t): Extension<TranslationContext>,
    State(state): State<AppState>,
    Path(id): Path<i64>,
    mut multipart: Multipart,
) -> impl IntoResponse {
    let countries = players::get_countries(&state.db).await.unwrap_or_default();

    // Get current player for fallback
    let current_player = match players::get_player_by_id(&state.db, id).await {
        Ok(Some(player)) => player,
        Ok(None) => {
            return Html(error_message("Player not found").into_string());
        }
        Err(e) => {
            tracing::error!("Failed to fetch player: {}", e);
            return Html(error_message("Failed to load player").into_string());
        }
    };

    // Parse multipart form data
    let mut name = String::new();
    let mut country_id: Option<i64> = None;
    let mut photo_path: Option<String> = current_player.photo_path.clone();
    let mut photo_url: Option<String> = None;
    let mut new_photo_uploaded = false;
    let mut birth_date: Option<String> = current_player.birth_date.clone();
    let mut birth_place: Option<String> = current_player.birth_place.clone();
    let mut height_cm: Option<i64> = current_player.height_cm;
    let mut weight_kg: Option<i64> = current_player.weight_kg;
    let mut position: Option<String> = current_player.position.clone();
    let mut shoots: Option<String> = current_player.shoots.clone();

    while let Ok(Some(field)) = multipart.next_field().await {
        let field_name = field.name().unwrap_or("").to_string();

        match field_name.as_str() {
            "name" => {
                name = field.text().await.unwrap_or_default();
            }
            "country_id" => {
                let text = field.text().await.unwrap_or_default();
                country_id = text.parse().ok();
            }
            "photo_url" => {
                photo_url = Some(field.text().await.unwrap_or_default());
            }
            "birth_date" => {
                let text = field.text().await.unwrap_or_default();
                birth_date = if text.trim().is_empty() { None } else { Some(text) };
            }
            "birth_place" => {
                let text = field.text().await.unwrap_or_default();
                birth_place = if text.trim().is_empty() { None } else { Some(text) };
            }
            "height_cm" => {
                let text = field.text().await.unwrap_or_default();
                height_cm = if text.trim().is_empty() { None } else { text.parse().ok() };
            }
            "weight_kg" => {
                let text = field.text().await.unwrap_or_default();
                weight_kg = if text.trim().is_empty() { None } else { text.parse().ok() };
            }
            "position" => {
                let text = field.text().await.unwrap_or_default();
                position = if text.trim().is_empty() { None } else { Some(text) };
            }
            "shoots" => {
                let text = field.text().await.unwrap_or_default();
                shoots = if text.trim().is_empty() { None } else { Some(text) };
            }
            "photo_file" => {
                // Handle file upload
                let filename = field.file_name().unwrap_or("photo.jpg").to_string();
                let data = field.bytes().await.unwrap_or_default();

                if !data.is_empty() {
                    match crate::utils::save_uploaded_file(
                        &data,
                        &filename,
                        "static/uploads/players",
                    )
                    .await
                    {
                        Ok(path) => {
                            // Delete old photo if it exists and is an uploaded file
                            if let Some(old_path) = &current_player.photo_path {
                                if old_path.starts_with("/static/uploads/") {
                                    let _ = crate::utils::delete_uploaded_file(old_path).await;
                                }
                            }
                            photo_path = Some(path);
                            new_photo_uploaded = true;
                        }
                        Err(e) => {
                            tracing::error!("Failed to save uploaded file: {}", e);
                            return Html(player_edit_modal(&t, &current_player, Some("Failed to save photo. Only image files (jpg, png, gif, webp) are allowed."), &countries).into_string());
                        }
                    }
                }
            }
            _ => {}
        }
    }

    // Validation
    let name = name.trim();
    if name.is_empty() {
        return Html(
            player_edit_modal(
                &t,
                &current_player,
                Some("Player name cannot be empty"),
                &countries,
            )
            .into_string(),
        );
    }

    if name.len() > 255 {
        return Html(
            player_edit_modal(
                &t,
                &current_player,
                Some("Player name cannot exceed 255 characters"),
                &countries,
            )
            .into_string(),
        );
    }

    let country_id = match country_id {
        Some(id) => id,
        None => {
            return Html(
                player_edit_modal(
                    &t,
                    &current_player,
                    Some("Please select a country"),
                    &countries,
                )
                .into_string(),
            );
        }
    };

    // If no new photo was uploaded, use the URL if provided
    let final_photo_path = if new_photo_uploaded {
        photo_path
    } else if let Some(url) = photo_url {
        if url.trim().is_empty() {
            None
        } else {
            Some(url)
        }
    } else {
        photo_path
    };

    // Update player
    match players::update_player(
        &state.db,
        id,
        UpdatePlayerEntity {
            name: name.to_string(),
            country_id,
            photo_path: final_photo_path,
            birth_date,
            birth_place,
            height_cm,
            weight_kg,
            position,
            shoots,
        },
    )
    .await
    {
        Ok(true) => {
            // Return HTMX response to close modal and reload table
            htmx_reload_table("/players/list", "players-table")
        }
        Ok(false) => Html(
            player_edit_modal(&t, &current_player, Some("Player not found"), &countries)
                .into_string(),
        ),
        Err(e) => {
            tracing::error!("Failed to update player: {}", e);
            Html(
                player_edit_modal(
                    &t,
                    &current_player,
                    Some("Failed to update player"),
                    &countries,
                )
                .into_string(),
            )
        }
    }
}

/// POST /players/{id}/delete - Delete player
pub async fn player_delete(
    Extension(t): Extension<TranslationContext>,
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

            Html(player_list_content(&t, &result, &filters, &sort_field, &sort_order).into_string())
        }
        Ok(false) => {
            Html(crate::views::components::error::error_message("Player not found").into_string())
        }
        Err(e) => {
            tracing::error!("Failed to delete player: {}", e);
            Html(
                crate::views::components::error::error_message("Failed to delete player")
                    .into_string(),
            )
        }
    }
}

/// GET /players/{id} - Player detail page
pub async fn player_detail(
    Extension(session): Extension<Session>,
    Extension(t): Extension<TranslationContext>,
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    let detail = match players::get_player_detail(&state.db, id).await {
        Ok(Some(detail)) => detail,
        Ok(None) => {
            return Html(
                admin_layout(
                    "Player Not Found",
                    &session,
                    "/players",
                    &t,
                    crate::views::components::error::error_message("Player not found"),
                )
                .into_string(),
            );
        }
        Err(e) => {
            tracing::error!("Failed to fetch player detail: {}", e);
            return Html(
                admin_layout(
                    "Error",
                    &session,
                    "/players",
                    &t,
                    crate::views::components::error::error_message("Failed to load player"),
                )
                .into_string(),
            );
        }
    };

    // Get season statistics for the player
    let season_stats = players::get_player_season_stats(&state.db, id)
        .await
        .unwrap_or_default();

    // Get event-specific career statistics
    let event_stats = players::get_player_event_stats(&state.db, id)
        .await
        .unwrap_or_default();

    let content = player_detail_page(&t, &detail, &season_stats, &event_stats);
    Html(admin_layout("Player Detail", &session, "/players", &t, content).into_string())
}
