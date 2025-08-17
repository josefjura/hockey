use aide::{
    axum::{
        ApiRouter, IntoApiResponse,
        routing::{get_with, post_with},
    },
    transform::TransformOperation,
};
use axum::{
    Extension, Json,
    extract::{Path, Query},
    http::StatusCode,
    response::IntoResponse,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::common::paging::{PagedResponse, Paging};
use crate::event::business::EventBusinessLogic;
use crate::event::service::EventFilters;
use crate::http::ApiContext;

use super::Event;

// Use generic PagedResponse for events
type PagedEventsResponse = PagedResponse<Event>;

pub fn event_routes() -> ApiRouter {
    ApiRouter::new()
        .api_route(
            "/",
            post_with(create_event, create_event_docs).get_with(list_events, list_events_docs),
        )
        .api_route(
            "/{id}",
            get_with(get_event, get_event_docs)
                .put_with(update_event, update_event_docs)
                .delete_with(delete_event, delete_event_docs),
        )
}

#[derive(Deserialize, JsonSchema)]
struct CreateEventRequest {
    /// The name for the new Event.
    name: String,
    /// The country ID for the new Event.
    country_id: Option<i64>,
}

#[derive(Serialize, JsonSchema)]
struct EventCreateResponse {
    /// The ID of the new Event.
    id: i64,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct EventQueryParams {
    pub name: Option<String>,
    pub country_id: Option<i64>,
    pub page: Option<usize>,
    pub page_size: Option<usize>,
}

#[derive(Deserialize, JsonSchema)]
struct UpdateEventRequest {
    /// The updated name for the Event.
    name: String,
    /// The updated country ID for the Event.
    country_id: Option<i64>,
}

async fn create_event(
    Extension(ctx): Extension<ApiContext>,
    Json(request): Json<CreateEventRequest>,
) -> impl IntoApiResponse {
    // 🎯 Route Handler: Only HTTP concerns
    match EventBusinessLogic::create_event(&ctx, request.name, request.country_id).await {
        Ok(event_id) => Ok((
            StatusCode::CREATED,
            Json(EventCreateResponse { id: event_id }),
        )),
        Err(app_error) => Err(app_error.into_response()),
    }
}

fn create_event_docs(op: TransformOperation) -> TransformOperation {
    op.description("Create a new event.")
        .tag("event")
        .response::<201, Json<EventCreateResponse>>()
}

async fn list_events(
    Query(params): Query<EventQueryParams>,
    Extension(ctx): Extension<ApiContext>,
) -> impl IntoApiResponse {
    let filters = EventFilters::new(params.name, params.country_id);
    let paging = Paging::new(
        params.page.unwrap_or(1),
        params.page_size.unwrap_or(15),
    );

    // 🎯 Route Handler: Only HTTP concerns
    match EventBusinessLogic::list_events(&ctx, filters, paging).await {
        Ok(response) => Ok(Json(response)),
        Err(app_error) => Err(app_error.into_response()),
    }
}

fn list_events_docs(op: TransformOperation) -> TransformOperation {
    op.description("List events with filtering and paging.")
        .tag("event")
        .response_with::<200, Json<PagedEventsResponse>, _>(|res| {
            res.example(PagedEventsResponse::new(
                vec![Event {
                    id: 1,
                    name: "World Championship".to_string(),
                    country_id: Some(1),
                    country_name: Some("Canada".to_string()),
                    country_iso2_code: Some("CA".to_string()),
                    created_at: "2024-01-01T00:00:00Z".to_string(),
                    updated_at: "2024-01-01T00:00:00Z".to_string(),
                }],
                25,
                1,
                20,
                2,
                true,
                false,
            ))
        })
}

#[derive(Deserialize, JsonSchema)]
struct SelectEvent {
    /// The ID of the Event.
    id: i64,
}

async fn get_event(
    Extension(ctx): Extension<ApiContext>,
    Path(event): Path<SelectEvent>,
) -> impl IntoApiResponse {
    // 🎯 Route Handler: Only HTTP concerns
    match EventBusinessLogic::get_event(&ctx, event.id).await {
        Ok(event) => Ok(Json(event)),
        Err(app_error) => Err(app_error.into_response()),
    }
}

fn get_event_docs(op: TransformOperation) -> TransformOperation {
    op.description("Get a single Event.")
        .tag("event")
        .response_with::<200, Json<Event>, _>(|res| {
            res.example(Event {
                name: "Example Event".to_string(),
                id: 1,
                country_id: Some(1),
                country_name: Some("Example Country".to_string()),
                country_iso2_code: Some("EX".to_string()),
                created_at: "2024-01-01T00:00:00Z".to_string(),
                updated_at: "2024-01-01T00:00:00Z".to_string(),
            })
        })
        .response_with::<404, (), _>(|res| res.description("event was not found"))
}

async fn update_event(
    Extension(ctx): Extension<ApiContext>,
    Path(event_id): Path<i64>,
    Json(update_request): Json<UpdateEventRequest>,
) -> impl IntoApiResponse {
    // 🎯 Route Handler: Only HTTP concerns
    match EventBusinessLogic::update_event(&ctx, event_id, update_request.name, update_request.country_id).await {
        Ok(_) => Ok((
            StatusCode::OK,
            Json("Event updated successfully".to_string()),
        )),
        Err(app_error) => Err(app_error.into_response()),
    }
}

fn update_event_docs(op: TransformOperation) -> TransformOperation {
    op.description("Update an event.")
        .tag("event")
        .response_with::<200, Json<String>, _>(|res| {
            res.example("Event updated successfully".to_string())
        })
        .response_with::<404, (), _>(|res| res.description("Event not found"))
        .response_with::<500, (), _>(|res| res.description("Internal server error"))
}

async fn delete_event(
    Extension(ctx): Extension<ApiContext>,
    Path(event): Path<SelectEvent>,
) -> impl IntoApiResponse {
    // 🎯 Route Handler: Only HTTP concerns
    match EventBusinessLogic::delete_event(&ctx, event.id).await {
        Ok(_) => Ok((StatusCode::OK, Json("Deleted".to_string()))),
        Err(app_error) => Err(app_error.into_response()),
    }
}

fn delete_event_docs(op: TransformOperation) -> TransformOperation {
    op.description("Delete an Event item.")
        .tag("event")
        .response_with::<204, (), _>(|res| res.description("The event has been deleted."))
        .response_with::<404, (), _>(|res| res.description("The event was not found"))
}
