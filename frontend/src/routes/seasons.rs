use axum::{
    extract::{Path, Query, State},
    response::{Html, IntoResponse},
    Extension, Form,
};
use axum_extra::extract::CookieJar;
use serde::Deserialize;

use crate::app_state::AppState;
use crate::auth::Session;
use crate::routes::locale::get_locale_from_cookies;
use crate::service::seasons::{
    self, CreateSeasonEntity, SeasonFilters, SortField, SortOrder, UpdateSeasonEntity,
};
use crate::views::{
    layout::admin_layout,
    pages::seasons::{season_create_modal, season_edit_modal, season_list_content, seasons_page},
};

#[derive(Debug, Deserialize)]
pub struct SeasonsQuery {
    #[serde(default = "default_page")]
    page: usize,
    #[serde(default = "default_page_size")]
    page_size: usize,
    #[serde(default, deserialize_with = "crate::utils::empty_string_as_none_i64")]
    event_id: Option<i64>,
    #[serde(default, deserialize_with = "crate::utils::empty_string_as_none_i64")]
    year: Option<i64>,
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
    "year".to_string()
}

fn default_sort_order() -> String {
    "desc".to_string()
}

#[derive(Debug, Deserialize)]
pub struct CreateSeasonForm {
    year: i64,
    display_name: Option<String>,
    event_id: i64,
}

#[derive(Debug, Deserialize)]
pub struct UpdateSeasonForm {
    year: i64,
    display_name: Option<String>,
    event_id: i64,
}

/// GET /seasons - Seasons list page
pub async fn seasons_get(
    Extension(session): Extension<Session>,
    State(state): State<AppState>,
    jar: CookieJar,
    Query(query): Query<SeasonsQuery>,
) -> impl IntoResponse {
    let locale = get_locale_from_cookies(&jar);

    // Build filters
    let filters = SeasonFilters {
        event_id: query.event_id,
        year: query.year,
    };

    // Parse sorting
    let sort_field = SortField::from_str(&query.sort);
    let sort_order = SortOrder::from_str(&query.order);

    // Get seasons
    let result = match seasons::get_seasons(
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
            tracing::error!("Failed to fetch seasons: {}", e);
            return Html(
                admin_layout(
                    "Seasons",
                    &session,
                    "/seasons",
                    &state.i18n,
                    locale,
                    crate::views::components::error::error_message("Failed to load seasons"),
                )
                .into_string(),
            );
        }
    };

    // Get events for filter
    let events = seasons::get_events(&state.db).await.unwrap_or_default();

    let content = seasons_page(
        &state.i18n,
        locale,
        &result,
        &filters,
        &sort_field,
        &sort_order,
        &events,
    );
    Html(
        admin_layout(
            "Seasons",
            &session,
            "/seasons",
            &state.i18n,
            locale,
            content,
        )
        .into_string(),
    )
}

/// GET /seasons/list - HTMX endpoint for table updates
pub async fn seasons_list_partial(
    State(state): State<AppState>,
    jar: CookieJar,
    Query(query): Query<SeasonsQuery>,
) -> impl IntoResponse {
    let locale = get_locale_from_cookies(&jar);

    let filters = SeasonFilters {
        event_id: query.event_id,
        year: query.year,
    };

    // Parse sorting
    let sort_field = SortField::from_str(&query.sort);
    let sort_order = SortOrder::from_str(&query.order);

    let result = match seasons::get_seasons(
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
            tracing::error!("Failed to fetch seasons: {}", e);
            return Html(
                crate::views::components::error::error_message("Failed to load seasons")
                    .into_string(),
            );
        }
    };

    Html(
        season_list_content(
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

/// GET /seasons/new - Show create modal
pub async fn season_create_form(
    State(state): State<AppState>,
    jar: CookieJar,
) -> impl IntoResponse {
    let locale = get_locale_from_cookies(&jar);
    let events = seasons::get_events(&state.db).await.unwrap_or_default();
    Html(season_create_modal(&state.i18n, locale, None, &events).into_string())
}

/// POST /seasons - Create new season
pub async fn season_create(
    State(state): State<AppState>,
    jar: CookieJar,
    Form(form): Form<CreateSeasonForm>,
) -> impl IntoResponse {
    let locale = get_locale_from_cookies(&jar);

    // Get events for form re-render on error
    let events = seasons::get_events(&state.db).await.unwrap_or_default();

    // Validation
    if form.year < 1900 || form.year > 2100 {
        return Html(
            season_create_modal(
                &state.i18n,
                locale,
                Some("Year must be between 1900 and 2100"),
                &events,
            )
            .into_string(),
        );
    }

    if let Some(display_name) = &form.display_name {
        let trimmed = display_name.trim();
        if !trimmed.is_empty() && trimmed.len() > 255 {
            return Html(
                season_create_modal(
                    &state.i18n,
                    locale,
                    Some("Display name cannot exceed 255 characters"),
                    &events,
                )
                .into_string(),
            );
        }
    }

    // Create season
    match seasons::create_season(
        &state.db,
        CreateSeasonEntity {
            year: form.year,
            display_name: form.display_name.and_then(|s| {
                let trimmed = s.trim();
                if trimmed.is_empty() {
                    None
                } else {
                    Some(trimmed.to_string())
                }
            }),
            event_id: form.event_id,
        },
    )
    .await
    {
        Ok(_) => {
            // Return HTMX response to close modal and reload table
            Html("<div hx-get=\"/seasons/list\" hx-target=\"#seasons-table\" hx-trigger=\"load\" hx-swap=\"outerHTML\"></div>".to_string())
        }
        Err(e) => {
            tracing::error!("Failed to create season: {}", e);
            Html(
                season_create_modal(
                    &state.i18n,
                    locale,
                    Some("Failed to create season"),
                    &events,
                )
                .into_string(),
            )
        }
    }
}

/// GET /seasons/{id}/edit - Show edit modal
pub async fn season_edit_form(
    State(state): State<AppState>,
    jar: CookieJar,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    let locale = get_locale_from_cookies(&jar);
    let events = seasons::get_events(&state.db).await.unwrap_or_default();

    let season = match seasons::get_season_by_id(&state.db, id).await {
        Ok(Some(season)) => season,
        Ok(None) => {
            return Html(
                crate::views::components::error::error_message("Season not found").into_string(),
            );
        }
        Err(e) => {
            tracing::error!("Failed to fetch season: {}", e);
            return Html(
                crate::views::components::error::error_message("Failed to load season")
                    .into_string(),
            );
        }
    };

    Html(season_edit_modal(&state.i18n, locale, &season, None, &events).into_string())
}

/// POST /seasons/{id} - Update season
pub async fn season_update(
    State(state): State<AppState>,
    jar: CookieJar,
    Path(id): Path<i64>,
    Form(form): Form<UpdateSeasonForm>,
) -> impl IntoResponse {
    let locale = get_locale_from_cookies(&jar);
    let events = seasons::get_events(&state.db).await.unwrap_or_default();

    // Validation
    if form.year < 1900 || form.year > 2100 {
        let season = seasons::get_season_by_id(&state.db, id)
            .await
            .ok()
            .flatten();
        return Html(
            season_edit_modal(
                &state.i18n,
                locale,
                &season.unwrap(),
                Some("Year must be between 1900 and 2100"),
                &events,
            )
            .into_string(),
        );
    }

    if let Some(display_name) = &form.display_name {
        let trimmed = display_name.trim();
        if !trimmed.is_empty() && trimmed.len() > 255 {
            let season = seasons::get_season_by_id(&state.db, id)
                .await
                .ok()
                .flatten();
            return Html(
                season_edit_modal(
                    &state.i18n,
                    locale,
                    &season.unwrap(),
                    Some("Display name cannot exceed 255 characters"),
                    &events,
                )
                .into_string(),
            );
        }
    }

    // Update season
    match seasons::update_season(
        &state.db,
        id,
        UpdateSeasonEntity {
            year: form.year,
            display_name: form.display_name.and_then(|s| {
                let trimmed = s.trim();
                if trimmed.is_empty() {
                    None
                } else {
                    Some(trimmed.to_string())
                }
            }),
            event_id: form.event_id,
        },
    )
    .await
    {
        Ok(true) => {
            // Return HTMX response to close modal and reload table
            Html("<div hx-get=\"/seasons/list\" hx-target=\"#seasons-table\" hx-trigger=\"load\" hx-swap=\"outerHTML\"></div>".to_string())
        }
        Ok(false) => {
            let season = seasons::get_season_by_id(&state.db, id)
                .await
                .ok()
                .flatten();
            Html(
                season_edit_modal(
                    &state.i18n,
                    locale,
                    &season.unwrap(),
                    Some("Season not found"),
                    &events,
                )
                .into_string(),
            )
        }
        Err(e) => {
            tracing::error!("Failed to update season: {}", e);
            let season = seasons::get_season_by_id(&state.db, id)
                .await
                .ok()
                .flatten();
            Html(
                season_edit_modal(
                    &state.i18n,
                    locale,
                    &season.unwrap(),
                    Some("Failed to update season"),
                    &events,
                )
                .into_string(),
            )
        }
    }
}

/// POST /seasons/{id}/delete - Delete season
pub async fn season_delete(
    State(state): State<AppState>,
    jar: CookieJar,
    Path(id): Path<i64>,
    Query(query): Query<SeasonsQuery>,
) -> impl IntoResponse {
    let locale = get_locale_from_cookies(&jar);

    match seasons::delete_season(&state.db, id).await {
        Ok(true) => {
            // Reload the table content after successful delete
            let filters = SeasonFilters {
                event_id: query.event_id,
                year: query.year,
            };

            let sort_field = SortField::from_str(&query.sort);
            let sort_order = SortOrder::from_str(&query.order);

            let result = match seasons::get_seasons(
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
                    tracing::error!("Failed to fetch seasons after delete: {}", e);
                    return Html(
                        crate::views::components::error::error_message("Failed to reload seasons")
                            .into_string(),
                    );
                }
            };

            Html(
                season_list_content(
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
        Ok(false) => {
            Html(crate::views::components::error::error_message("Season not found").into_string())
        }
        Err(e) => {
            tracing::error!("Failed to delete season: {}", e);
            Html(
                crate::views::components::error::error_message("Failed to delete season")
                    .into_string(),
            )
        }
    }
}
