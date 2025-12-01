use crate::{
    common::paging::{PagedResponse, Paging},
    errors::AppError,
    event::{
        service::{self, CreateEventEntity, EventFilters, UpdateEventEntity},
        Event,
    },
    http::ApiContext,
};

/// Business logic layer for events - pure domain operations
/// No HTTP concerns, only business rules and data transformations
pub struct EventBusinessLogic;

impl EventBusinessLogic {
    /// Get a single event by ID
    pub async fn get_event(ctx: &ApiContext, event_id: i64) -> Result<Event, AppError> {
        let event_data = service::get_event_by_id(&ctx.db, event_id).await?;

        let event = event_data.ok_or_else(|| AppError::event_not_found(event_id))?;

        Ok(Event {
            id: event.id,
            name: event.name,
            country_id: event.country_id,
            country_name: event.country_name,
            country_iso2_code: event.country_iso2_code,
            created_at: event.created_at,
            updated_at: event.updated_at,
        })
    }

    /// Create a new event with validation
    pub async fn create_event(
        ctx: &ApiContext,
        name: String,
        country_id: Option<i64>,
    ) -> Result<i64, AppError> {
        // Business validation rules
        if name.trim().is_empty() {
            return Err(AppError::invalid_input("Event name cannot be empty"));
        }
        if name.len() > 255 {
            return Err(AppError::invalid_input(
                "Event name cannot exceed 255 characters",
            ));
        }

        // Validate country_id if provided
        if let Some(country_id) = country_id {
            if country_id <= 0 {
                return Err(AppError::invalid_input("Country ID must be positive"));
            }
        }

        let event_id = service::create_event(
            &ctx.db,
            CreateEventEntity {
                name: name.trim().to_string(),
                country_id,
            },
        )
        .await?;

        Ok(event_id)
    }

    /// List events with filtering and pagination
    pub async fn list_events(
        ctx: &ApiContext,
        filters: EventFilters,
        paging: Paging,
    ) -> Result<PagedResponse<Event>, AppError> {
        let result = service::get_events(&ctx.db, &filters, Some(&paging)).await?;

        let events: Vec<Event> = result
            .items
            .into_iter()
            .map(|event| Event {
                id: event.id,
                name: event.name,
                country_id: event.country_id,
                country_name: event.country_name,
                country_iso2_code: event.country_iso2_code,
                created_at: event.created_at,
                updated_at: event.updated_at,
            })
            .collect();

        Ok(PagedResponse {
            items: events,
            total: result.total,
            page: result.page,
            page_size: result.page_size,
            total_pages: result.total_pages,
            has_next: result.has_next,
            has_previous: result.has_previous,
        })
    }

    /// Update an event with validation
    pub async fn update_event(
        ctx: &ApiContext,
        event_id: i64,
        name: String,
        country_id: Option<i64>,
    ) -> Result<bool, AppError> {
        // Check event exists first
        let _existing_event = Self::get_event(ctx, event_id).await?;

        // Same validation as create
        if name.trim().is_empty() {
            return Err(AppError::invalid_input("Event name cannot be empty"));
        }
        if name.len() > 255 {
            return Err(AppError::invalid_input(
                "Event name cannot exceed 255 characters",
            ));
        }

        if let Some(country_id) = country_id {
            if country_id <= 0 {
                return Err(AppError::invalid_input("Country ID must be positive"));
            }
        }

        let updated = service::update_event(
            &ctx.db,
            event_id,
            UpdateEventEntity {
                name: name.trim().to_string(),
                country_id,
            },
        )
        .await?;

        if !updated {
            return Err(AppError::event_not_found(event_id));
        }

        Ok(updated)
    }

    /// Delete an event
    pub async fn delete_event(ctx: &ApiContext, event_id: i64) -> Result<(), AppError> {
        let deleted = service::delete_event(&ctx.db, event_id).await?;

        if !deleted {
            return Err(AppError::event_not_found(event_id));
        }

        Ok(())
    }
}
