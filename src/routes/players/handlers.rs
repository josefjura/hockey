use axum::{
    extract::{Multipart, Path, Query, State},
    response::{Html, IntoResponse},
    Extension,
};
use serde::Deserialize;

use crate::app_state::AppState;
use crate::auth::Session;
use crate::i18n::TranslationContext;
use crate::service::players::{self, PlayerFilters, SortField, SortOrder};
use crate::views::{
    components::{
        error::error_message,
        htmx::{htmx_reload_page, htmx_reload_table},
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
                    crate::views::components::error::error_message(
                        &t,
                        t.messages.error_failed_to_load_players(),
                    ),
                )
                .into_string(),
            );
        }
    };

    // Get countries for filter
    let countries = match players::get_countries(&state.db).await {
        Ok(countries) => countries,
        Err(e) => {
            tracing::warn!("Failed to load countries for dropdown: {}", e);
            Vec::new()
        }
    };

    let content = players_page(
        &session,
        &t,
        &result,
        &filters,
        &sort_field,
        &sort_order,
        &countries,
    );
    Html(admin_layout("Players", &session, "/players", &t, content).into_string())
}

/// GET /players/list - HTMX endpoint for table updates
pub async fn players_list_partial(
    Extension(session): Extension<Session>,
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
                crate::views::components::error::error_message(
                    &t,
                    t.messages.error_failed_to_load_players(),
                )
                .into_string(),
            );
        }
    };

    Html(
        player_list_content(&session, &t, &result, &filters, &sort_field, &sort_order)
            .into_string(),
    )
}

/// GET /players/new - Show create modal
pub async fn player_create_form(
    Extension(session): Extension<Session>,
    Extension(t): Extension<TranslationContext>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let countries = match players::get_countries(&state.db).await {
        Ok(countries) => countries,
        Err(e) => {
            tracing::warn!("Failed to load countries for dropdown: {}", e);
            Vec::new()
        }
    };
    Html(player_create_modal(&session, &t, None, &countries).into_string())
}

/// POST /players - Create new player
pub async fn player_create(
    Extension(session): Extension<Session>,
    Extension(t): Extension<TranslationContext>,
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> impl IntoResponse {
    use axum::http::header::{HeaderMap, HeaderName};

    // Get countries for form re-render on error
    let countries = match players::get_countries(&state.db).await {
        Ok(countries) => countries,
        Err(e) => {
            tracing::warn!("Failed to load countries for dropdown: {}", e);
            Vec::new()
        }
    };

    // Parse multipart form data
    let form_data =
        match super::forms::parse_player_form(&mut multipart, "static/uploads/players", None).await
        {
            Ok(data) => data,
            Err(error_msg) => {
                return Html(
                    player_create_modal(&session, &t, Some(&error_msg), &countries).into_string(),
                )
                .into_response();
            }
        };

    // Validate CSRF token
    if let Err(response) = crate::auth::validate_csrf_token(&form_data.csrf_token, &session) {
        return response.into_response();
    }

    // Resolve final photo path (prefer uploaded file over URL)
    let final_photo_path = super::forms::resolve_photo_path(
        form_data.photo_path.clone(),
        form_data.photo_url.clone(),
        None,
    );

    // Validate and create player using business layer
    match crate::business::players::create_player_validated(&state.db, &form_data, final_photo_path)
        .await
    {
        Ok(_) => {
            // Return HTMX response to close modal and reload table
            // Trigger entity-created event for dashboard stats update
            let mut headers = HeaderMap::new();
            headers.insert(
                HeaderName::from_static("hx-trigger"),
                "entity-created".parse().unwrap(),
            );
            (headers, htmx_reload_table("/players/list", "players-table")).into_response()
        }
        Err(Ok(validation_error)) => {
            // Validation error
            Html(
                player_create_modal(&session, &t, Some(validation_error.message()), &countries)
                    .into_string(),
            )
            .into_response()
        }
        Err(Err(e)) => {
            // Database error
            tracing::error!("Failed to create player: {}", e);
            Html(
                player_create_modal(&session, &t, Some("Failed to create player"), &countries)
                    .into_string(),
            )
            .into_response()
        }
    }
}

/// GET /players/{id}/edit - Show edit modal
pub async fn player_edit_form(
    Extension(session): Extension<Session>,
    Extension(t): Extension<TranslationContext>,
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    let countries = match players::get_countries(&state.db).await {
        Ok(countries) => countries,
        Err(e) => {
            tracing::warn!("Failed to load countries for dropdown: {}", e);
            Vec::new()
        }
    };

    let player = match players::get_player_by_id(&state.db, id).await {
        Ok(Some(player)) => player,
        Ok(None) => {
            return Html(
                crate::views::components::error::error_message(
                    &t,
                    t.messages.error_player_not_found(),
                )
                .into_string(),
            );
        }
        Err(e) => {
            tracing::error!("Failed to fetch player: {}", e);
            return Html(
                crate::views::components::error::error_message(
                    &t,
                    t.messages.error_failed_to_load_player(),
                )
                .into_string(),
            );
        }
    };

    Html(player_edit_modal(&session, &t, &player, None, &countries).into_string())
}

/// POST /players/{id} - Update player
pub async fn player_update(
    Extension(session): Extension<Session>,
    Extension(t): Extension<TranslationContext>,
    State(state): State<AppState>,
    Path(id): Path<i64>,
    mut multipart: Multipart,
) -> impl IntoResponse {
    let countries = match players::get_countries(&state.db).await {
        Ok(countries) => countries,
        Err(e) => {
            tracing::warn!("Failed to load countries for dropdown: {}", e);
            Vec::new()
        }
    };

    // Get current player for fallback and old photo deletion
    let current_player = match players::get_player_by_id(&state.db, id).await {
        Ok(Some(player)) => player,
        Ok(None) => {
            return Html(error_message(&t, t.messages.error_player_not_found()).into_string())
                .into_response();
        }
        Err(e) => {
            tracing::error!("Failed to fetch player: {}", e);
            return Html(error_message(&t, t.messages.error_failed_to_load_player()).into_string())
                .into_response();
        }
    };

    // Parse multipart form data
    let form_data = match super::forms::parse_player_form(
        &mut multipart,
        "static/uploads/players",
        current_player.photo_path.as_deref(),
    )
    .await
    {
        Ok(data) => data,
        Err(error_msg) => {
            return Html(
                player_edit_modal(&session, &t, &current_player, Some(&error_msg), &countries)
                    .into_string(),
            )
            .into_response();
        }
    };

    // Validate CSRF token
    if let Err(response) = crate::auth::validate_csrf_token(&form_data.csrf_token, &session) {
        return response.into_response();
    }

    // Resolve final photo path
    // Priority: uploaded file > URL > existing photo
    let final_photo_path = super::forms::resolve_photo_path(
        form_data.photo_path.clone(),
        form_data.photo_url.clone(),
        current_player.photo_path.clone(),
    );

    // Validate and update player using business layer
    match crate::business::players::update_player_validated(
        &state.db,
        id,
        &form_data,
        final_photo_path,
    )
    .await
    {
        Ok(true) => {
            // Return HTMX response to close modal and reload page to show updated data
            htmx_reload_page().into_response()
        }
        Ok(false) => Html(
            player_edit_modal(
                &session,
                &t,
                &current_player,
                Some("Player not found"),
                &countries,
            )
            .into_string(),
        )
        .into_response(),
        Err(Ok(validation_error)) => {
            // Validation error
            Html(
                player_edit_modal(
                    &session,
                    &t,
                    &current_player,
                    Some(validation_error.message()),
                    &countries,
                )
                .into_string(),
            )
            .into_response()
        }
        Err(Err(e)) => {
            // Database error
            tracing::error!("Failed to update player: {}", e);
            Html(
                player_edit_modal(
                    &session,
                    &t,
                    &current_player,
                    Some("Failed to update player"),
                    &countries,
                )
                .into_string(),
            )
            .into_response()
        }
    }
}

/// Deserialize delete form
#[derive(Debug, Deserialize)]
pub struct DeletePlayerForm {
    csrf_token: String,
}

/// POST /players/{id}/delete - Delete player
pub async fn player_delete(
    Extension(session): Extension<Session>,
    Extension(t): Extension<TranslationContext>,
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Query(query): Query<PlayersQuery>,
    axum::Form(form): axum::Form<DeletePlayerForm>,
) -> impl IntoResponse {
    // Validate CSRF token
    if let Err(response) = crate::auth::validate_csrf_token(&form.csrf_token, &session) {
        return response.into_response();
    }
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
                        crate::views::components::error::error_message(
                            &t,
                            t.messages.error_failed_to_reload_players(),
                        )
                        .into_string(),
                    )
                    .into_response();
                }
            };

            Html(
                player_list_content(&session, &t, &result, &filters, &sort_field, &sort_order)
                    .into_string(),
            )
            .into_response()
        }
        Ok(false) => Html(
            crate::views::components::error::error_message(&t, t.messages.error_player_not_found())
                .into_string(),
        )
        .into_response(),
        Err(e) => {
            tracing::error!("Failed to delete player: {}", e);
            Html(
                crate::views::components::error::error_message(
                    &t,
                    t.messages.error_failed_to_delete_player(),
                )
                .into_string(),
            )
            .into_response()
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
    // Fetch all player detail page data from business layer
    let page_data = match crate::business::players::get_player_detail_page_data(&state.db, id).await
    {
        Ok(Some(data)) => data,
        Ok(None) => {
            return Html(
                admin_layout(
                    "Player Not Found",
                    &session,
                    "/players",
                    &t,
                    crate::views::components::error::error_message(
                        &t,
                        t.messages.error_player_not_found(),
                    ),
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
                    crate::views::components::error::error_message(
                        &t,
                        t.messages.error_failed_to_load_player(),
                    ),
                )
                .into_string(),
            );
        }
    };

    let content = player_detail_page(
        &t,
        &page_data.detail,
        &page_data.season_stats,
        &page_data.event_stats,
        &page_data.property_changes,
    );
    Html(admin_layout("Player Detail", &session, "/players", &t, content).into_string())
}
