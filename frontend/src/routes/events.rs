use axum::{
    extract::{Path, Query, State},
    response::{Html, IntoResponse},
    Extension, Form,
};
use serde::Deserialize;

use crate::app_state::AppState;
use crate::auth::Session;
use crate::i18n::Locale;
use crate::service::events::{self, CreateEventEntity, EventFilters, UpdateEventEntity};
use crate::views::{
    layout::admin_layout,
    pages::events::{event_create_modal, event_edit_modal, event_list_content, events_page},
};

#[derive(Debug, Deserialize)]
pub struct EventsQuery {
    #[serde(default = "default_page")]
    page: usize,
    #[serde(default = "default_page_size")]
    page_size: usize,
    name: Option<String>,
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
    country_id: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateEventForm {
    name: String,
    country_id: Option<i64>,
}

/// GET /events - Events list page
pub async fn events_get(
    Extension(session): Extension<Session>,
    State(state): State<AppState>,
    Query(query): Query<EventsQuery>,
) -> impl IntoResponse {
    let locale = Locale::English;

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
                    &state.i18n,
                    locale,
                    crate::views::components::error::error_message("Failed to load events"),
                )
                .into_string(),
            );
        }
    };

    // Get countries for filter
    let countries = events::get_countries(&state.db).await.unwrap_or_default();

    let content = events_page(&result, &filters, &countries);
    Html(admin_layout("Events", &session, "/events", &state.i18n, locale, content).into_string())
}

/// GET /events/list - HTMX endpoint for table updates
pub async fn events_list_partial(
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

    Html(event_list_content(&result, &filters).into_string())
}

/// GET /events/new - Show create modal
pub async fn event_create_form(State(state): State<AppState>) -> impl IntoResponse {
    let countries = events::get_countries(&state.db).await.unwrap_or_default();
    Html(event_create_modal(&countries, None).into_string())
}

/// POST /events - Create new event
pub async fn event_create(
    State(state): State<AppState>,
    Form(form): Form<CreateEventForm>,
) -> impl IntoResponse {
    // Validation
    let name = form.name.trim();
    if name.is_empty() {
        let countries = events::get_countries(&state.db).await.unwrap_or_default();
        return Html(
            event_create_modal(&countries, Some("Event name cannot be empty")).into_string(),
        );
    }

    if name.len() > 255 {
        let countries = events::get_countries(&state.db).await.unwrap_or_default();
        return Html(
            event_create_modal(&countries, Some("Event name cannot exceed 255 characters"))
                .into_string(),
        );
    }

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
            // Return HTMX response to close modal and reload table
            Html("<div hx-get=\"/events/list\" hx-target=\"#events-table\" hx-trigger=\"load\" hx-swap=\"outerHTML\"></div>".to_string())
        }
        Err(e) => {
            tracing::error!("Failed to create event: {}", e);
            let countries = events::get_countries(&state.db).await.unwrap_or_default();
            Html(event_create_modal(&countries, Some("Failed to create event")).into_string())
        }
    }
}

/// GET /events/{id}/edit - Show edit modal
pub async fn event_edit_form(
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
    Html(event_edit_modal(&event, &countries, None).into_string())
}

/// POST /events/{id} - Update event
pub async fn event_update(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Form(form): Form<UpdateEventForm>,
) -> impl IntoResponse {
    // Validation
    let name = form.name.trim();
    if name.is_empty() {
        let event = events::get_event_by_id(&state.db, id).await.ok().flatten();
        let countries = events::get_countries(&state.db).await.unwrap_or_default();
        return Html(
            event_edit_modal(
                &event.unwrap(),
                &countries,
                Some("Event name cannot be empty"),
            )
            .into_string(),
        );
    }

    if name.len() > 255 {
        let event = events::get_event_by_id(&state.db, id).await.ok().flatten();
        let countries = events::get_countries(&state.db).await.unwrap_or_default();
        return Html(
            event_edit_modal(
                &event.unwrap(),
                &countries,
                Some("Event name cannot exceed 255 characters"),
            )
            .into_string(),
        );
    }

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
            Html("<div hx-get=\"/events/list\" hx-target=\"#events-table\" hx-trigger=\"load\" hx-swap=\"outerHTML\"></div>".to_string())
        }
        Ok(false) => {
            let event = events::get_event_by_id(&state.db, id).await.ok().flatten();
            let countries = events::get_countries(&state.db).await.unwrap_or_default();
            Html(
                event_edit_modal(&event.unwrap(), &countries, Some("Event not found"))
                    .into_string(),
            )
        }
        Err(e) => {
            tracing::error!("Failed to update event: {}", e);
            let event = events::get_event_by_id(&state.db, id).await.ok().flatten();
            let countries = events::get_countries(&state.db).await.unwrap_or_default();
            Html(
                event_edit_modal(&event.unwrap(), &countries, Some("Failed to update event"))
                    .into_string(),
            )
        }
    }
}

/// POST /events/{id}/delete - Delete event
pub async fn event_delete(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    match events::delete_event(&state.db, id).await {
        Ok(true) => {
            // Return HTMX response to reload table
            Html("<div hx-get=\"/events/list\" hx-target=\"#events-table\" hx-trigger=\"load\" hx-swap=\"outerHTML\"></div>".to_string())
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
