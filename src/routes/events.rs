use axum::{
    extract::{Path, Query, State},
    response::{Html, IntoResponse},
    Extension, Form,
};
use serde::Deserialize;

use crate::app_state::AppState;
use crate::auth::Session;
use crate::i18n::TranslationContext;
use crate::service::events::{self, CreateEventEntity, EventFilters, UpdateEventEntity};
use crate::validation::validate_name;
use crate::views::{
    components::{error::error_message, htmx::htmx_reload_table},
    layout::admin_layout,
    pages::event_detail::event_detail_page,
    pages::events::{event_create_modal, event_edit_modal, event_list_content, events_page},
};

#[derive(Debug, Deserialize)]
pub struct EventsQuery {
    #[serde(default = "default_page")]
    page: usize,
    #[serde(default = "default_page_size")]
    page_size: usize,
    #[serde(default, deserialize_with = "crate::utils::empty_string_as_none")]
    name: Option<String>,
    #[serde(default, deserialize_with = "crate::utils::empty_string_as_none_i64")]
    country_id: Option<i64>,
}

fn default_page() -> usize {
    1
}

fn default_page_size() -> usize {
    20
}

#[derive(Debug, Deserialize)]
pub struct CreateEventForm {
    name: String,
    #[serde(default, deserialize_with = "crate::utils::empty_string_as_none_i64")]
    country_id: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateEventForm {
    name: String,
    #[serde(default, deserialize_with = "crate::utils::empty_string_as_none_i64")]
    country_id: Option<i64>,
}

/// GET /events - Events list page
pub async fn events_get(
    Extension(session): Extension<Session>,
    Extension(t): Extension<TranslationContext>,
    State(state): State<AppState>,
    Query(query): Query<EventsQuery>,
) -> impl IntoResponse {
    // Build filters
    let filters = EventFilters {
        name: query.name.clone(),
        country_id: query.country_id,
    };

    // Get events
    let result = match events::get_events(&state.db, &filters, query.page, query.page_size).await {
        Ok(result) => result,
        Err(e) => {
            tracing::error!("Failed to fetch events: {}", e);
            return Html(
                admin_layout(
                    "Events",
                    &session,
                    "/events",
                    &t,
                    crate::views::components::error::error_message("Failed to load events"),
                )
                .into_string(),
            );
        }
    };

    // Get countries for filter
    let countries = events::get_countries(&state.db).await.unwrap_or_default();

    let content = events_page(&t, &result, &filters, &countries);
    Html(admin_layout("Events", &session, "/events", &t, content).into_string())
}

/// GET /events/list - HTMX endpoint for table updates
pub async fn events_list_partial(
    Extension(t): Extension<TranslationContext>,
    State(state): State<AppState>,
    Query(query): Query<EventsQuery>,
) -> impl IntoResponse {
    let filters = EventFilters {
        name: query.name.clone(),
        country_id: query.country_id,
    };

    let result = match events::get_events(&state.db, &filters, query.page, query.page_size).await {
        Ok(result) => result,
        Err(e) => {
            tracing::error!("Failed to fetch events: {}", e);
            return Html(
                crate::views::components::error::error_message("Failed to load events")
                    .into_string(),
            );
        }
    };

    Html(event_list_content(&t, &result, &filters).into_string())
}

/// GET /events/{id} - Event detail page
pub async fn event_detail(
    Extension(session): Extension<Session>,
    Extension(t): Extension<TranslationContext>,
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    let detail = match events::get_event_detail(&state.db, id).await {
        Ok(Some(detail)) => detail,
        Ok(None) => {
            return Html(
                admin_layout(
                    "Event Not Found",
                    &session,
                    "/events",
                    &t,
                    crate::views::components::error::error_message("Event not found"),
                )
                .into_string(),
            );
        }
        Err(e) => {
            tracing::error!("Failed to fetch event detail: {}", e);
            return Html(
                admin_layout(
                    "Error",
                    &session,
                    "/events",
                    &t,
                    crate::views::components::error::error_message("Failed to load event"),
                )
                .into_string(),
            );
        }
    };

    let content = event_detail_page(&t, &detail);
    Html(admin_layout("Event Detail", &session, "/events", &t, content).into_string())
}

/// GET /events/new - Show create modal
pub async fn event_create_form(
    Extension(t): Extension<TranslationContext>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let countries = events::get_countries(&state.db).await.unwrap_or_default();
    Html(event_create_modal(&t, &countries, None).into_string())
}

/// POST /events - Create new event
pub async fn event_create(
    Extension(t): Extension<TranslationContext>,
    State(state): State<AppState>,
    Form(form): Form<CreateEventForm>,
) -> impl IntoResponse {
    // Validation
    let name = match validate_name(&form.name) {
        Ok(n) => n,
        Err(error) => {
            let countries = events::get_countries(&state.db).await.unwrap_or_default();
            return Html(event_create_modal(&t, &countries, Some(error)).into_string())
                .into_response();
        }
    };

    // Create event
    match events::create_event(
        &state.db,
        CreateEventEntity {
            name: name.to_string(),
            country_id: form.country_id,
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
            (headers, htmx_reload_table("/events/list", "events-table")).into_response()
        }
        Err(e) => {
            tracing::error!("Failed to create event: {}", e);
            let countries = events::get_countries(&state.db).await.unwrap_or_default();
            Html(event_create_modal(&t, &countries, Some("Failed to create event")).into_string())
                .into_response()
        }
    }
}

/// GET /events/{id}/edit - Show edit modal
pub async fn event_edit_form(
    Extension(t): Extension<TranslationContext>,
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    let event = match events::get_event_by_id(&state.db, id).await {
        Ok(Some(event)) => event,
        Ok(None) => {
            return Html(
                crate::views::components::error::error_message("Event not found").into_string(),
            );
        }
        Err(e) => {
            tracing::error!("Failed to fetch event: {}", e);
            return Html(
                crate::views::components::error::error_message("Failed to load event")
                    .into_string(),
            );
        }
    };

    let countries = events::get_countries(&state.db).await.unwrap_or_default();
    Html(event_edit_modal(&t, &event, &countries, None).into_string())
}

/// POST /events/{id} - Update event
pub async fn event_update(
    Extension(t): Extension<TranslationContext>,
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Form(form): Form<UpdateEventForm>,
) -> impl IntoResponse {
    // Validation
    let name = match validate_name(&form.name) {
        Ok(n) => n,
        Err(error) => {
            let event = events::get_event_by_id(&state.db, id).await.ok().flatten();
            let countries = events::get_countries(&state.db).await.unwrap_or_default();
            let Some(event) = event else {
                return Html(error_message("Event not found").into_string());
            };
            return Html(event_edit_modal(&t, &event, &countries, Some(error)).into_string());
        }
    };

    // Update event
    match events::update_event(
        &state.db,
        id,
        UpdateEventEntity {
            name: name.to_string(),
            country_id: form.country_id,
        },
    )
    .await
    {
        Ok(true) => {
            // Return HTMX response to close modal and reload table
            htmx_reload_table("/events/list", "events-table")
        }
        Ok(false) => Html(error_message("Event not found").into_string()),
        Err(e) => {
            tracing::error!("Failed to update event: {}", e);
            let event = events::get_event_by_id(&state.db, id).await.ok().flatten();
            let countries = events::get_countries(&state.db).await.unwrap_or_default();
            let Some(event) = event else {
                return Html(error_message("Event not found").into_string());
            };
            Html(
                event_edit_modal(&t, &event, &countries, Some("Failed to update event"))
                    .into_string(),
            )
        }
    }
}

/// POST /events/{id}/delete - Delete event
pub async fn event_delete(State(state): State<AppState>, Path(id): Path<i64>) -> impl IntoResponse {
    match events::delete_event(&state.db, id).await {
        Ok(true) => {
            // Return HTMX response to reload table
            htmx_reload_table("/events/list", "events-table")
        }
        Ok(false) => {
            Html(crate::views::components::error::error_message("Event not found").into_string())
        }
        Err(e) => {
            tracing::error!("Failed to delete event: {}", e);
            Html(
                crate::views::components::error::error_message("Failed to delete event")
                    .into_string(),
            )
        }
    }
}
