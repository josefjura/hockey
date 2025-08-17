use axum::{Extension, Json, Router, extract::Path, routing::get};
use axum_test::TestServer;
use serde_json::Value;
use sqlx::SqlitePool;

// Import necessary modules from the crate
use jura_hockey::{
    config::Config,
    errors::AppError,
    event::{business::EventBusinessLogic, service as event_service},
    http::ApiContext,
};

/// Creates a test server with a simple endpoint for testing events
async fn create_test_server(pool: SqlitePool) -> TestServer {
    let config = Config {
        database_url: "sqlite::memory:".to_string(),
        hmac_key: "test-hmac-key".to_string(),
    };

    let app = Router::new()
        .route("/api/events/{id}", get(test_get_event_endpoint))
        .layer(Extension(ApiContext::new(pool, config)));

    TestServer::new(app).unwrap()
}

/// Test endpoint that uses our business logic layer
async fn test_get_event_endpoint(
    Extension(ctx): Extension<ApiContext>,
    Path(id): Path<String>,
) -> Result<Json<Value>, AppError> {
    let event_id: i64 = id
        .parse()
        .map_err(|_| AppError::invalid_input("Invalid event ID format"))?;

    // Use the business logic layer directly
    match EventBusinessLogic::get_event(&ctx, event_id).await {
        Ok(event) => Ok(Json(serde_json::to_value(event).unwrap())),
        Err(err) => Err(err),
    }
}

// ========================================
// Event Business Logic Tests
// ========================================

#[sqlx::test(fixtures("events"))]
async fn business_logic_get_nonexistent_event_returns_error(pool: SqlitePool) {
    let config = Config {
        database_url: "sqlite::memory:".to_string(),
        hmac_key: "test-key".to_string(),
    };
    let ctx = ApiContext::new(pool, config);

    // Test business logic directly - no HTTP involved
    let result = EventBusinessLogic::get_event(&ctx, 99999).await;

    assert!(result.is_err());
    match result.unwrap_err() {
        AppError::EventNotFound { id } => assert_eq!(id, 99999),
        _ => panic!("Expected EventNotFound error"),
    }
}

#[sqlx::test]
async fn business_logic_create_event_validation(pool: SqlitePool) {
    let config = Config {
        database_url: "sqlite::memory:".to_string(),
        hmac_key: "test-key".to_string(),
    };
    let ctx = ApiContext::new(pool, config);

    // Test empty name validation
    let result = EventBusinessLogic::create_event(&ctx, "".to_string(), None).await;
    assert!(result.is_err());
    match result.unwrap_err() {
        AppError::InvalidInput { message } => assert!(message.contains("cannot be empty")),
        _ => panic!("Expected InvalidInput error"),
    }

    // Test long name validation
    let long_name = "A".repeat(256);
    let result = EventBusinessLogic::create_event(&ctx, long_name, None).await;
    assert!(result.is_err());
    match result.unwrap_err() {
        AppError::InvalidInput { message } => assert!(message.contains("255 characters")),
        _ => panic!("Expected InvalidInput error"),
    }

    // Test invalid country ID
    let result = EventBusinessLogic::create_event(&ctx, "Valid Event".to_string(), Some(-1)).await;
    assert!(result.is_err());
    match result.unwrap_err() {
        AppError::InvalidInput { message } => assert!(message.contains("positive")),
        _ => panic!("Expected InvalidInput error"),
    }
}

#[sqlx::test]
async fn business_logic_successful_creation(pool: SqlitePool) {
    let config = Config {
        database_url: "sqlite::memory:".to_string(),
        hmac_key: "test-key".to_string(),
    };
    let ctx = ApiContext::new(pool, config);

    // Test successful creation
    let result =
        EventBusinessLogic::create_event(&ctx, "World Championship".to_string(), None).await;
    assert!(result.is_ok());

    let event_id = result.unwrap();
    assert!(event_id > 0);

    // Verify we can retrieve the created event
    let event = EventBusinessLogic::get_event(&ctx, event_id).await.unwrap();
    assert_eq!(event.name, "World Championship");
    assert_eq!(event.country_id, None);
}

// ========================================
// Event HTTP Integration Tests
// ========================================

#[sqlx::test]
async fn http_get_nonexistent_event_returns_404(pool: SqlitePool) {
    let server = create_test_server(pool).await;

    let response = server.get("/api/events/99999").await;

    // Test HTTP status code
    response.assert_status(axum::http::StatusCode::NOT_FOUND);

    // Test error response structure
    let error_body: Value = response.json();
    assert_eq!(error_body["error"], "Event not found with id: 99999");
    assert!(error_body["error_id"].is_string());
    assert!(error_body["error_details"].is_null());
}

#[sqlx::test]
async fn http_invalid_event_id_returns_400(pool: SqlitePool) {
    let server = create_test_server(pool).await;

    let response = server.get("/api/events/not-a-number").await;

    // Test HTTP status code
    response.assert_status(axum::http::StatusCode::BAD_REQUEST);

    // Test error response structure
    let error_body: Value = response.json();
    assert_eq!(error_body["error"], "Invalid input");
    assert!(error_body["error_id"].is_string());
}

// ========================================
// Event Service Layer Tests
// ========================================

#[sqlx::test]
async fn service_layer_database_operations(pool: SqlitePool) {
    // Create an event
    let event_id = event_service::create_event(
        &pool,
        event_service::CreateEventEntity {
            name: "Test Event".to_string(),
            country_id: None,
        },
    )
    .await
    .expect("Failed to create event");

    // Retrieve the event
    let event = event_service::get_event_by_id(&pool, event_id)
        .await
        .expect("Failed to get event")
        .expect("Event should exist");

    assert_eq!(event.id, event_id);
    assert_eq!(event.name, "Test Event");
    assert_eq!(event.country_id, None);

    // Update the event
    let updated = event_service::update_event(
        &pool,
        event_id,
        event_service::UpdateEventEntity {
            name: "Updated Event".to_string(),
            country_id: Some(1),
        },
    )
    .await
    .expect("Failed to update event");

    assert!(updated);

    // Delete the event
    let deleted = event_service::delete_event(&pool, event_id)
        .await
        .expect("Failed to delete event");

    assert!(deleted);

    // Verify deletion
    let result = event_service::get_event_by_id(&pool, event_id).await;
    assert!(result.is_ok());
    assert!(result.unwrap().is_none());
}
