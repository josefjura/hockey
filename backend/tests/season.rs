use axum::{Extension, Json, Router, extract::Path, routing::get};
use axum_test::TestServer;
use serde_json::Value;
use sqlx::SqlitePool;

// Import necessary modules from the crate
use jura_hockey::{
    config::Config,
    errors::AppError,
    http::ApiContext,
    season::service::CreateSeasonEntity,
    season::{business::SeasonBusinessLogic, service},
};

/// Creates a test server with a simple endpoint for testing
async fn create_test_server(pool: SqlitePool) -> TestServer {
    let config = Config {
        database_url: "sqlite::memory:".to_string(),
        hmac_key: "test-hmac-key".to_string(),
    };

    let app = Router::new()
        .route("/api/seasons/{id}", get(test_get_season_endpoint))
        .layer(Extension(ApiContext::new(pool, config)));

    TestServer::new(app).unwrap()
}

/// Test endpoint that uses our business logic layer
async fn test_get_season_endpoint(
    Extension(ctx): Extension<ApiContext>,
    Path(id): Path<String>,
) -> Result<Json<Value>, AppError> {
    let season_id: i64 = id
        .parse()
        .map_err(|_| AppError::invalid_input("Invalid season ID format"))?;

    // Use the business logic layer directly
    match SeasonBusinessLogic::get_season(&ctx, season_id).await {
        Ok(season_data) => Ok(Json(serde_json::to_value(season_data).unwrap())),
        Err(err) => Err(err),
    }
}

// ========================================
// Season Business Logic Tests (Unit Tests)
// ========================================

#[sqlx::test]
async fn business_logic_get_nonexistent_season_returns_error(pool: SqlitePool) {
    let config = Config {
        database_url: "sqlite::memory:".to_string(),
        hmac_key: "test-key".to_string(),
    };
    let ctx = ApiContext::new(pool, config);

    // Test business logic directly - no HTTP involved
    let result = SeasonBusinessLogic::get_season(&ctx, 99999).await;

    assert!(result.is_err());
    match result.unwrap_err() {
        AppError::SeasonNotFound { id } => assert_eq!(id, 99999),
        _ => panic!("Expected SeasonNotFound error"),
    }
}

#[sqlx::test]
async fn business_logic_create_season_validation(pool: SqlitePool) {
    let config = Config {
        database_url: "sqlite::memory:".to_string(),
        hmac_key: "test-key".to_string(),
    };
    let ctx = ApiContext::new(pool, config);

    // Test invalid year (too early)
    let result = SeasonBusinessLogic::create_season(&ctx, 1899, None, 1).await;
    assert!(result.is_err());
    match result.unwrap_err() {
        AppError::InvalidInput { message } => {
            assert!(message.contains("Season year must be between 1900 and 2100"))
        }
        _ => panic!("Expected InvalidInput error"),
    }

    // Test invalid year (too late)
    let result = SeasonBusinessLogic::create_season(&ctx, 2101, None, 1).await;
    assert!(result.is_err());
    match result.unwrap_err() {
        AppError::InvalidInput { message } => {
            assert!(message.contains("Season year must be between 1900 and 2100"))
        }
        _ => panic!("Expected InvalidInput error"),
    }

    // Test invalid event ID (zero)
    let result = SeasonBusinessLogic::create_season(&ctx, 2024, None, 0).await;
    assert!(result.is_err());
    match result.unwrap_err() {
        AppError::InvalidInput { message } => {
            assert!(message.contains("Event ID must be positive"))
        }
        _ => panic!("Expected InvalidInput error"),
    }

    // Test invalid event ID (negative)
    let result = SeasonBusinessLogic::create_season(&ctx, 2024, None, -1).await;
    assert!(result.is_err());
    match result.unwrap_err() {
        AppError::InvalidInput { message } => {
            assert!(message.contains("Event ID must be positive"))
        }
        _ => panic!("Expected InvalidInput error"),
    }
}

#[sqlx::test]
async fn business_logic_display_name_validation(pool: SqlitePool) {
    let config = Config {
        database_url: "sqlite::memory:".to_string(),
        hmac_key: "test-key".to_string(),
    };
    let ctx = ApiContext::new(pool, config);

    // Test empty display name
    let result = SeasonBusinessLogic::create_season(&ctx, 2024, Some("".to_string()), 1).await;
    assert!(result.is_err());
    match result.unwrap_err() {
        AppError::InvalidInput { message } => {
            assert!(message.contains("Display name cannot be empty"))
        }
        _ => panic!("Expected InvalidInput error"),
    }

    // Test whitespace-only display name
    let result = SeasonBusinessLogic::create_season(&ctx, 2024, Some("   ".to_string()), 1).await;
    assert!(result.is_err());
    match result.unwrap_err() {
        AppError::InvalidInput { message } => {
            assert!(message.contains("Display name cannot be empty"))
        }
        _ => panic!("Expected InvalidInput error"),
    }

    // Test too long display name
    let long_name = "A".repeat(101);
    let result = SeasonBusinessLogic::create_season(&ctx, 2024, Some(long_name), 1).await;
    assert!(result.is_err());
    match result.unwrap_err() {
        AppError::InvalidInput { message } => assert!(message.contains("100 characters")),
        _ => panic!("Expected InvalidInput error"),
    }
}

#[sqlx::test]
async fn business_logic_update_season_validation(pool: SqlitePool) {
    let config = Config {
        database_url: "sqlite::memory:".to_string(),
        hmac_key: "test-key".to_string(),
    };
    let ctx = ApiContext::new(pool, config);

    // Test invalid season ID
    let result = SeasonBusinessLogic::update_season(&ctx, 0, 2024, None, 1).await;
    assert!(result.is_err());
    match result.unwrap_err() {
        AppError::InvalidInput { message } => {
            assert!(message.contains("Season ID must be positive"))
        }
        _ => panic!("Expected InvalidInput error"),
    }

    // Test updating non-existent season
    let result = SeasonBusinessLogic::update_season(&ctx, 99999, 2024, None, 1).await;
    assert!(result.is_err());
    match result.unwrap_err() {
        AppError::SeasonNotFound { id } => assert_eq!(id, 99999),
        _ => panic!("Expected SeasonNotFound error"),
    }
}

#[sqlx::test]
async fn business_logic_delete_season_validation(pool: SqlitePool) {
    let config = Config {
        database_url: "sqlite::memory:".to_string(),
        hmac_key: "test-key".to_string(),
    };
    let ctx = ApiContext::new(pool, config);

    // Test invalid season ID
    let result = SeasonBusinessLogic::delete_season(&ctx, 0).await;
    assert!(result.is_err());
    match result.unwrap_err() {
        AppError::InvalidInput { message } => {
            assert!(message.contains("Season ID must be positive"))
        }
        _ => panic!("Expected InvalidInput error"),
    }

    // Test deleting non-existent season
    let result = SeasonBusinessLogic::delete_season(&ctx, 99999).await;
    assert!(result.is_err());
    match result.unwrap_err() {
        AppError::SeasonNotFound { id } => assert_eq!(id, 99999),
        _ => panic!("Expected SeasonNotFound error"),
    }
}

#[sqlx::test]
async fn business_logic_list_seasons_validation(pool: SqlitePool) {
    let config = Config {
        database_url: "sqlite::memory:".to_string(),
        hmac_key: "test-key".to_string(),
    };
    let ctx = ApiContext::new(pool, config);

    // Test invalid year filter (too early)
    let result = SeasonBusinessLogic::list_seasons(&ctx, Some(1899), None, None).await;
    assert!(result.is_err());
    match result.unwrap_err() {
        AppError::InvalidInput { message } => {
            assert!(message.contains("Year filter must be between 1900 and 2100"))
        }
        _ => panic!("Expected InvalidInput error"),
    }

    // Test invalid year filter (too late)
    let result = SeasonBusinessLogic::list_seasons(&ctx, Some(2101), None, None).await;
    assert!(result.is_err());
    match result.unwrap_err() {
        AppError::InvalidInput { message } => {
            assert!(message.contains("Year filter must be between 1900 and 2100"))
        }
        _ => panic!("Expected InvalidInput error"),
    }

    // Test invalid event ID filter
    let result = SeasonBusinessLogic::list_seasons(&ctx, None, Some(0), None).await;
    assert!(result.is_err());
    match result.unwrap_err() {
        AppError::InvalidInput { message } => {
            assert!(message.contains("Event ID filter must be positive"))
        }
        _ => panic!("Expected InvalidInput error"),
    }
}

#[sqlx::test]
async fn business_logic_get_players_for_team_validation(pool: SqlitePool) {
    let config = Config {
        database_url: "sqlite::memory:".to_string(),
        hmac_key: "test-key".to_string(),
    };
    let ctx = ApiContext::new(pool, config);

    // Test invalid season ID
    let result = SeasonBusinessLogic::get_players_for_team_in_season(&ctx, 0, 1).await;
    assert!(result.is_err());
    match result.unwrap_err() {
        AppError::InvalidInput { message } => {
            assert!(message.contains("Season ID must be positive"))
        }
        _ => panic!("Expected InvalidInput error"),
    }

    // Test invalid team ID
    let result = SeasonBusinessLogic::get_players_for_team_in_season(&ctx, 1, 0).await;
    assert!(result.is_err());
    match result.unwrap_err() {
        AppError::InvalidInput { message } => assert!(message.contains("Team ID must be positive")),
        _ => panic!("Expected InvalidInput error"),
    }
}

// ========================================
// Season HTTP Integration Tests (End-to-End)
// ========================================

#[sqlx::test]
async fn http_get_nonexistent_season_returns_404(pool: SqlitePool) {
    let server = create_test_server(pool).await;

    let response = server.get("/api/seasons/99999").await;

    // Test HTTP status code
    response.assert_status(axum::http::StatusCode::NOT_FOUND);

    // Test error response structure
    let error_body: Value = response.json();
    assert_eq!(error_body["error"], "Season not found with id: 99999");
    assert!(error_body["error_id"].is_string());
    assert!(error_body["error_details"].is_null());
}

#[sqlx::test]
async fn http_invalid_season_id_returns_400(pool: SqlitePool) {
    let server = create_test_server(pool).await;

    let response = server.get("/api/seasons/not-a-number").await;

    // Test HTTP status code
    response.assert_status(axum::http::StatusCode::BAD_REQUEST);

    // Test error response structure
    let error_body: Value = response.json();
    assert_eq!(error_body["error"], "Invalid input");
    assert!(error_body["error_id"].is_string());
}

#[sqlx::test]
async fn http_database_error_returns_500(pool: SqlitePool) {
    let pool_clone = pool.clone();
    let server = create_test_server(pool).await;

    // Close the connection pool to simulate database failure
    pool_clone.close().await;

    let response = server.get("/api/seasons/1").await;

    // Should return 500 Internal Server Error
    response.assert_status(axum::http::StatusCode::INTERNAL_SERVER_ERROR);

    let error_body: Value = response.json();
    assert_eq!(error_body["error"], "Internal server error");
    assert!(error_body["error_id"].is_string());
}

// ========================================
// Season Service Layer Tests (Data Access)
// ========================================

#[sqlx::test]
async fn service_layer_get_season_by_id(pool: SqlitePool) {
    // Test the service layer directly
    let result = service::get_season_by_id(&pool, 99999).await;

    // Should succeed but return None (no season found)
    assert!(result.is_ok());
    assert!(result.unwrap().is_none());
}

#[sqlx::test(fixtures("season_deps"))]
async fn service_layer_database_operations(pool: SqlitePool) {
    // Get the actual event ID created by the fixtures
    let event_id: i64 = sqlx::query_scalar("SELECT id FROM event LIMIT 1")
        .fetch_one(&pool)
        .await
        .unwrap();

    // Create a season
    let season_id = service::create_season(
        &pool,
        CreateSeasonEntity {
            year: 2024,
            display_name: Some("Test Season 2024".to_string()),
            event_id,
        },
    )
    .await
    .expect("Failed to create season");

    // Retrieve the season
    let season_data = service::get_season_by_id(&pool, season_id)
        .await
        .expect("Failed to get season")
        .expect("Season should exist");

    assert_eq!(season_data.id, season_id);
    assert_eq!(season_data.year, 2024);
    assert_eq!(
        season_data.display_name,
        Some("Test Season 2024".to_string())
    );
    assert_eq!(season_data.event_id, event_id);
    assert_eq!(season_data.event_name, "World Championship");
    assert!(!season_data.created_at.is_empty());
    assert!(!season_data.updated_at.is_empty());

    // Test update
    let update_entity = service::UpdateSeasonEntity {
        year: 2025,
        display_name: Some("Updated Season 2025".to_string()),
        event_id,
    };

    let updated = service::update_season(&pool, season_id, update_entity)
        .await
        .expect("Failed to update season");

    assert!(updated);

    // Verify update
    let updated_season = service::get_season_by_id(&pool, season_id)
        .await
        .expect("Failed to get updated season")
        .expect("Updated season should exist");

    assert_eq!(updated_season.year, 2025);
    assert_eq!(
        updated_season.display_name,
        Some("Updated Season 2025".to_string())
    );

    // Test delete
    let deleted = service::delete_season(&pool, season_id)
        .await
        .expect("Failed to delete season");

    assert!(deleted);

    // Verify deletion
    let deleted_season = service::get_season_by_id(&pool, season_id)
        .await
        .expect("Failed to check deleted season");

    assert!(deleted_season.is_none());
}

#[sqlx::test(fixtures("season_deps"))]
async fn service_layer_list_seasons_with_filters(pool: SqlitePool) {
    // Get event IDs from fixtures
    let events: Vec<(i64, String)> = sqlx::query_as("SELECT id, name FROM event ORDER BY id")
        .fetch_all(&pool)
        .await
        .unwrap();

    let event_a_id = events[0].0; // First event from fixtures
    let event_b_id = events[1].0; // Second event from fixtures

    // Create test seasons using fixture event IDs
    let _season1_id = service::create_season(
        &pool,
        CreateSeasonEntity {
            year: 2023,
            display_name: Some("Season 2023".to_string()),
            event_id: event_a_id,
        },
    )
    .await
    .expect("Failed to create season 1");

    let _season2_id = service::create_season(
        &pool,
        CreateSeasonEntity {
            year: 2024,
            display_name: Some("Season 2024".to_string()),
            event_id: event_a_id,
        },
    )
    .await
    .expect("Failed to create season 2");

    let _season3_id = service::create_season(
        &pool,
        CreateSeasonEntity {
            year: 2024,
            display_name: Some("Season 2024 B".to_string()),
            event_id: event_b_id,
        },
    )
    .await
    .expect("Failed to create season 3");

    // Test listing all seasons
    let filters = service::SeasonFilters::default();
    let result = service::get_seasons(&pool, &filters, None)
        .await
        .expect("Failed to get all seasons");

    assert_eq!(result.total, 3);
    assert_eq!(result.items.len(), 3);

    // Test filtering by year
    let filters = service::SeasonFilters::new(Some(2024), None);
    let result = service::get_seasons(&pool, &filters, None)
        .await
        .expect("Failed to get seasons filtered by year");

    assert_eq!(result.total, 2); // Both 2024 seasons
    assert_eq!(result.items.len(), 2);

    // Test filtering by event
    let filters = service::SeasonFilters::new(None, Some(event_b_id));
    let result = service::get_seasons(&pool, &filters, None)
        .await
        .expect("Failed to get seasons filtered by event");

    assert_eq!(result.total, 1); // Only Season 2024 B
    assert_eq!(result.items.len(), 1);
    assert_eq!(result.items[0].event_name, "European Championship"); // Updated to match fixture

    // Test combined filtering
    let filters = service::SeasonFilters::new(Some(2024), Some(event_a_id));
    let result = service::get_seasons(&pool, &filters, None)
        .await
        .expect("Failed to get seasons with combined filters");

    assert_eq!(result.total, 1); // Only Season 2024 from Event A
    assert_eq!(result.items.len(), 1);
    assert_eq!(
        result.items[0].display_name,
        Some("Season 2024".to_string())
    );
}

#[sqlx::test(fixtures("season_deps"))]
async fn service_layer_seasons_list_functionality(pool: SqlitePool) {
    // Get the event ID from fixtures
    let event_id: i64 = sqlx::query_scalar("SELECT id FROM event LIMIT 1")
        .fetch_one(&pool)
        .await
        .unwrap();

    // Create test seasons
    let _season1_id = service::create_season(
        &pool,
        CreateSeasonEntity {
            year: 2023,
            display_name: Some("Championship 2023".to_string()),
            event_id,
        },
    )
    .await
    .expect("Failed to create season 1");

    let _season2_id = service::create_season(
        &pool,
        CreateSeasonEntity {
            year: 2024,
            display_name: None, // Test season without display name
            event_id,
        },
    )
    .await
    .expect("Failed to create season 2");

    // Test seasons list functionality
    let seasons_list = service::get_seasons_list(&pool)
        .await
        .expect("Failed to get seasons list");

    assert_eq!(seasons_list.len(), 2);

    // Verify seasons are ordered by year DESC (most recent first)
    assert_eq!(seasons_list[0].year, 2024);
    assert_eq!(seasons_list[1].year, 2023);

    // Verify first season (2024)
    assert_eq!(seasons_list[0].name, None);
    assert_eq!(seasons_list[0].event_name, "World Championship");

    // Verify second season (2023)
    assert_eq!(seasons_list[1].name, Some("Championship 2023".to_string()));
    assert_eq!(seasons_list[1].event_name, "World Championship");
}
