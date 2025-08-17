use axum::{Extension, Json, Router, extract::Path, routing::get};
use axum_test::TestServer;
use serde_json::Value;
use sqlx::SqlitePool;

// Import necessary modules from the crate
use jura_hockey::{
    config::Config,
    errors::AppError,
    http::ApiContext,
    player::service::CreatePlayerEntity,
    player::{business::PlayerBusinessLogic, service},
};

/// Creates a test server with a simple endpoint for testing
async fn create_test_server(pool: SqlitePool) -> TestServer {
    let config = Config {
        database_url: "sqlite::memory:".to_string(),
        hmac_key: "test-hmac-key".to_string(),
    };

    let app = Router::new()
        .route("/api/players/{id}", get(test_get_player_endpoint))
        .layer(Extension(ApiContext::new(pool, config)));

    TestServer::new(app).unwrap()
}

/// Test endpoint that uses our business logic layer
async fn test_get_player_endpoint(
    Extension(ctx): Extension<ApiContext>,
    Path(id): Path<String>,
) -> Result<Json<Value>, AppError> {
    let player_id: i64 = id
        .parse()
        .map_err(|_| AppError::invalid_input("Invalid player ID format"))?;

    // Use the business logic layer directly
    match PlayerBusinessLogic::get_player(&ctx, player_id).await {
        Ok(player_data) => Ok(Json(serde_json::to_value(player_data).unwrap())),
        Err(err) => Err(err),
    }
}

// ========================================
// Player Business Logic Tests (Unit Tests)
// ========================================

#[sqlx::test]
async fn business_logic_get_nonexistent_player_returns_error(pool: SqlitePool) {
    let config = Config {
        database_url: "sqlite::memory:".to_string(),
        hmac_key: "test-key".to_string(),
    };
    let ctx = ApiContext::new(pool, config);

    // Test business logic directly - no HTTP involved
    let result = PlayerBusinessLogic::get_player(&ctx, 99999).await;

    assert!(result.is_err());
    match result.unwrap_err() {
        AppError::PlayerNotFound { id } => assert_eq!(id, 99999),
        _ => panic!("Expected PlayerNotFound error"),
    }
}

#[sqlx::test]
async fn business_logic_create_player_validation(pool: SqlitePool) {
    let config = Config {
        database_url: "sqlite::memory:".to_string(),
        hmac_key: "test-key".to_string(),
    };
    let ctx = ApiContext::new(pool, config);

    // Test invalid player name (empty)
    let result = PlayerBusinessLogic::create_player(&ctx, "".to_string(), 1, None).await;
    assert!(result.is_err());
    match result.unwrap_err() {
        AppError::InvalidInput { message } => {
            assert!(message.contains("Player name cannot be empty"))
        }
        _ => panic!("Expected InvalidInput error"),
    }

    // Test invalid player name (whitespace only)
    let result = PlayerBusinessLogic::create_player(&ctx, "   ".to_string(), 1, None).await;
    assert!(result.is_err());
    match result.unwrap_err() {
        AppError::InvalidInput { message } => {
            assert!(message.contains("Player name cannot be empty"))
        }
        _ => panic!("Expected InvalidInput error"),
    }

    // Test invalid player name (too long)
    let long_name = "A".repeat(101);
    let result = PlayerBusinessLogic::create_player(&ctx, long_name, 1, None).await;
    assert!(result.is_err());
    match result.unwrap_err() {
        AppError::InvalidInput { message } => assert!(message.contains("100 characters")),
        _ => panic!("Expected InvalidInput error"),
    }

    // Test invalid country ID (zero)
    let result = PlayerBusinessLogic::create_player(&ctx, "Test Player".to_string(), 0, None).await;
    assert!(result.is_err());
    match result.unwrap_err() {
        AppError::InvalidInput { message } => {
            assert!(message.contains("Country ID must be positive"))
        }
        _ => panic!("Expected InvalidInput error"),
    }

    // Test invalid country ID (negative)
    let result =
        PlayerBusinessLogic::create_player(&ctx, "Test Player".to_string(), -1, None).await;
    assert!(result.is_err());
    match result.unwrap_err() {
        AppError::InvalidInput { message } => {
            assert!(message.contains("Country ID must be positive"))
        }
        _ => panic!("Expected InvalidInput error"),
    }
}

#[sqlx::test]
async fn business_logic_photo_path_validation(pool: SqlitePool) {
    let config = Config {
        database_url: "sqlite::memory:".to_string(),
        hmac_key: "test-key".to_string(),
    };
    let ctx = ApiContext::new(pool, config);

    // Test empty photo path
    let result = PlayerBusinessLogic::create_player(
        &ctx,
        "Test Player".to_string(),
        1,
        Some("".to_string()),
    )
    .await;
    assert!(result.is_err());
    match result.unwrap_err() {
        AppError::InvalidInput { message } => {
            assert!(message.contains("Photo path cannot be empty"))
        }
        _ => panic!("Expected InvalidInput error"),
    }

    // Test whitespace-only photo path
    let result = PlayerBusinessLogic::create_player(
        &ctx,
        "Test Player".to_string(),
        1,
        Some("   ".to_string()),
    )
    .await;
    assert!(result.is_err());
    match result.unwrap_err() {
        AppError::InvalidInput { message } => {
            assert!(message.contains("Photo path cannot be empty"))
        }
        _ => panic!("Expected InvalidInput error"),
    }

    // Test too long photo path
    let long_path = "A".repeat(256);
    let result =
        PlayerBusinessLogic::create_player(&ctx, "Test Player".to_string(), 1, Some(long_path))
            .await;
    assert!(result.is_err());
    match result.unwrap_err() {
        AppError::InvalidInput { message } => assert!(message.contains("255 characters")),
        _ => panic!("Expected InvalidInput error"),
    }

    // Test invalid photo path format
    let result = PlayerBusinessLogic::create_player(
        &ctx,
        "Test Player".to_string(),
        1,
        Some("invalid-path".to_string()),
    )
    .await;
    assert!(result.is_err());
    match result.unwrap_err() {
        AppError::InvalidInput { message } => assert!(message.contains("valid path")),
        _ => panic!("Expected InvalidInput error"),
    }

    // Test valid photo paths
    let valid_paths = vec![
        "/images/player.jpg",
        "https://example.com/player.jpg",
        "http://example.com/player.jpg",
    ];

    for path in valid_paths {
        // We don't expect these to create successfully due to database constraints,
        // but they should pass validation and fail with a database error (foreign key constraint)
        let result = PlayerBusinessLogic::create_player(
            &ctx,
            "Test Player".to_string(),
            999999, // Use a country ID that definitely doesn't exist
            Some(path.to_string()),
        )
        .await;
        // Should fail with database error due to foreign key constraint, not validation error
        assert!(matches!(result, Err(AppError::Database(_))));
    }
}

#[sqlx::test]
async fn business_logic_update_player_validation(pool: SqlitePool) {
    let config = Config {
        database_url: "sqlite::memory:".to_string(),
        hmac_key: "test-key".to_string(),
    };
    let ctx = ApiContext::new(pool, config);

    // Test invalid player ID
    let result =
        PlayerBusinessLogic::update_player(&ctx, 0, "Test Player".to_string(), 1, None).await;
    assert!(result.is_err());
    match result.unwrap_err() {
        AppError::InvalidInput { message } => {
            assert!(message.contains("Player ID must be positive"))
        }
        _ => panic!("Expected InvalidInput error"),
    }

    // Test updating non-existent player
    let result =
        PlayerBusinessLogic::update_player(&ctx, 99999, "Test Player".to_string(), 1, None).await;
    assert!(result.is_err());
    match result.unwrap_err() {
        AppError::PlayerNotFound { id } => assert_eq!(id, 99999),
        _ => panic!("Expected PlayerNotFound error"),
    }
}

#[sqlx::test]
async fn business_logic_delete_player_validation(pool: SqlitePool) {
    let config = Config {
        database_url: "sqlite::memory:".to_string(),
        hmac_key: "test-key".to_string(),
    };
    let ctx = ApiContext::new(pool, config);

    // Test invalid player ID
    let result = PlayerBusinessLogic::delete_player(&ctx, 0).await;
    assert!(result.is_err());
    match result.unwrap_err() {
        AppError::InvalidInput { message } => {
            assert!(message.contains("Player ID must be positive"))
        }
        _ => panic!("Expected InvalidInput error"),
    }

    // Test deleting non-existent player
    let result = PlayerBusinessLogic::delete_player(&ctx, 99999).await;
    assert!(result.is_err());
    match result.unwrap_err() {
        AppError::PlayerNotFound { id } => assert_eq!(id, 99999),
        _ => panic!("Expected PlayerNotFound error"),
    }
}

#[sqlx::test]
async fn business_logic_list_players_validation(pool: SqlitePool) {
    let config = Config {
        database_url: "sqlite::memory:".to_string(),
        hmac_key: "test-key".to_string(),
    };
    let ctx = ApiContext::new(pool, config);

    // Test invalid country ID filter
    let result = PlayerBusinessLogic::list_players(&ctx, None, Some(0), None).await;
    assert!(result.is_err());
    match result.unwrap_err() {
        AppError::InvalidInput { message } => {
            assert!(message.contains("Country ID filter must be positive"))
        }
        _ => panic!("Expected InvalidInput error"),
    }

    // Test empty name filter
    let result = PlayerBusinessLogic::list_players(&ctx, Some("".to_string()), None, None).await;
    assert!(result.is_err());
    match result.unwrap_err() {
        AppError::InvalidInput { message } => {
            assert!(message.contains("Name filter cannot be empty"))
        }
        _ => panic!("Expected InvalidInput error"),
    }

    // Test whitespace-only name filter
    let result = PlayerBusinessLogic::list_players(&ctx, Some("   ".to_string()), None, None).await;
    assert!(result.is_err());
    match result.unwrap_err() {
        AppError::InvalidInput { message } => {
            assert!(message.contains("Name filter cannot be empty"))
        }
        _ => panic!("Expected InvalidInput error"),
    }
}

// ========================================
// Player HTTP Integration Tests (End-to-End)
// ========================================

#[sqlx::test]
async fn http_get_nonexistent_player_returns_404(pool: SqlitePool) {
    let server = create_test_server(pool).await;

    let response = server.get("/api/players/99999").await;

    // Test HTTP status code
    response.assert_status(axum::http::StatusCode::NOT_FOUND);

    // Test error response structure
    let error_body: Value = response.json();
    assert_eq!(error_body["error"], "Player not found with id: 99999");
    assert!(error_body["error_id"].is_string());
    assert!(error_body["error_details"].is_null());
}

#[sqlx::test]
async fn http_invalid_player_id_returns_400(pool: SqlitePool) {
    let server = create_test_server(pool).await;

    let response = server.get("/api/players/not-a-number").await;

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

    let response = server.get("/api/players/1").await;

    // Should return 500 Internal Server Error
    response.assert_status(axum::http::StatusCode::INTERNAL_SERVER_ERROR);

    let error_body: Value = response.json();
    assert_eq!(error_body["error"], "Internal server error");
    assert!(error_body["error_id"].is_string());
}

// ========================================
// Player Service Layer Tests (Data Access)
// ========================================

#[sqlx::test]
async fn service_layer_get_player_by_id(pool: SqlitePool) {
    // Test the service layer directly
    let result = service::get_player_by_id(&pool, 99999).await;

    // Should succeed but return None (no player found)
    assert!(result.is_ok());
    assert!(result.unwrap().is_none());
}

#[sqlx::test]
async fn service_layer_database_operations(pool: SqlitePool) {
    // Create the necessary tables based on migrations
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS country (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            iihf BOOLEAN DEFAULT FALSE,
            iocCode TEXT,
            iso2Code TEXT,
            isHistorical BOOLEAN DEFAULT FALSE,
            years TEXT,
            enabled BOOLEAN DEFAULT FALSE,
            created_at TEXT,
            updated_at TEXT
        )
    "#,
    )
    .execute(&pool)
    .await
    .expect("Failed to create country table");

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS player (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            country_id INTEGER NOT NULL,
            photo_path TEXT,
            created_at TEXT,
            updated_at TEXT,
            FOREIGN KEY (country_id) REFERENCES country (id)
        )
    "#,
    )
    .execute(&pool)
    .await
    .expect("Failed to create player table");

    // Insert test data
    sqlx::query(
        "INSERT OR IGNORE INTO country (id, name, iso2Code) VALUES (9999, 'Test Country', 'TC')",
    )
    .execute(&pool)
    .await
    .expect("Failed to insert test country");

    // Create a player
    let player_id = service::create_player(
        &pool,
        CreatePlayerEntity {
            name: "Test Player".to_string(),
            country_id: 9999,
            photo_path: Some("/images/test_player.jpg".to_string()),
        },
    )
    .await
    .expect("Failed to create player");

    // Retrieve the player
    let player_data = service::get_player_by_id(&pool, player_id)
        .await
        .expect("Failed to get player")
        .expect("Player should exist");

    assert_eq!(player_data.id, player_id);
    assert_eq!(player_data.name, "Test Player");
    assert_eq!(player_data.country_id, 9999);
    assert_eq!(player_data.country_name, "Test Country");
    assert_eq!(player_data.country_iso2_code, "TC");
    assert_eq!(
        player_data.photo_path,
        Some("/images/test_player.jpg".to_string())
    );
    assert!(!player_data.created_at.is_empty());
    assert!(!player_data.updated_at.is_empty());

    // Test update
    let update_entity = service::UpdatePlayerEntity {
        name: "Updated Player".to_string(),
        country_id: 9999,
        photo_path: None,
    };

    let updated = service::update_player(&pool, player_id, update_entity)
        .await
        .expect("Failed to update player");

    assert!(updated);

    // Verify update
    let updated_player = service::get_player_by_id(&pool, player_id)
        .await
        .expect("Failed to get updated player")
        .expect("Updated player should exist");

    assert_eq!(updated_player.name, "Updated Player");
    assert_eq!(updated_player.photo_path, None);

    // Test delete
    let deleted = service::delete_player(&pool, player_id)
        .await
        .expect("Failed to delete player");

    assert!(deleted);

    // Verify deletion
    let deleted_player = service::get_player_by_id(&pool, player_id)
        .await
        .expect("Failed to check deleted player");

    assert!(deleted_player.is_none());
}

#[sqlx::test]
async fn service_layer_list_players_with_filters(pool: SqlitePool) {
    // Create the necessary tables
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS country (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            iihf BOOLEAN DEFAULT FALSE,
            iocCode TEXT,
            iso2Code TEXT,
            isHistorical BOOLEAN DEFAULT FALSE,
            years TEXT,
            enabled BOOLEAN DEFAULT FALSE,
            created_at TEXT,
            updated_at TEXT
        )
    "#,
    )
    .execute(&pool)
    .await
    .expect("Failed to create country table");

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS player (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            country_id INTEGER NOT NULL,
            photo_path TEXT,
            created_at TEXT,
            updated_at TEXT,
            FOREIGN KEY (country_id) REFERENCES country (id)
        )
    "#,
    )
    .execute(&pool)
    .await
    .expect("Failed to create player table");

    // Insert test data
    sqlx::query(
        "INSERT OR IGNORE INTO country (id, name, iso2Code) VALUES (9998, 'Country A', 'CA')",
    )
    .execute(&pool)
    .await
    .expect("Failed to insert country A");

    sqlx::query(
        "INSERT OR IGNORE INTO country (id, name, iso2Code) VALUES (9997, 'Country B', 'CB')",
    )
    .execute(&pool)
    .await
    .expect("Failed to insert country B");

    // Create test players
    let _player1_id = service::create_player(
        &pool,
        CreatePlayerEntity {
            name: "John Smith".to_string(),
            country_id: 9998,
            photo_path: None,
        },
    )
    .await
    .expect("Failed to create player 1");

    let _player2_id = service::create_player(
        &pool,
        CreatePlayerEntity {
            name: "Jane Doe".to_string(),
            country_id: 9998,
            photo_path: None,
        },
    )
    .await
    .expect("Failed to create player 2");

    let _player3_id = service::create_player(
        &pool,
        CreatePlayerEntity {
            name: "Bob Johnson".to_string(),
            country_id: 9997,
            photo_path: None,
        },
    )
    .await
    .expect("Failed to create player 3");

    // Test listing all players
    let filters = service::PlayerFilters::default();
    let result = service::get_players(&pool, &filters, None)
        .await
        .expect("Failed to get all players");

    assert_eq!(result.total, 3);
    assert_eq!(result.items.len(), 3);

    // Test filtering by name
    let filters = service::PlayerFilters::new(Some("John".to_string()), None);
    let result = service::get_players(&pool, &filters, None)
        .await
        .expect("Failed to get players filtered by name");

    assert_eq!(result.total, 2); // John Smith and Bob Johnson
    assert_eq!(result.items.len(), 2);

    // Test filtering by country
    let filters = service::PlayerFilters::new(None, Some(9997));
    let result = service::get_players(&pool, &filters, None)
        .await
        .expect("Failed to get players filtered by country");

    assert_eq!(result.total, 1); // Only Bob Johnson
    assert_eq!(result.items.len(), 1);
    assert_eq!(result.items[0].name, "Bob Johnson");
    assert_eq!(result.items[0].country_name, "Country B");

    // Test combined filtering
    let filters = service::PlayerFilters::new(Some("Jane".to_string()), Some(9998));
    let result = service::get_players(&pool, &filters, None)
        .await
        .expect("Failed to get players with combined filters");

    assert_eq!(result.total, 1); // Only Jane Doe
    assert_eq!(result.items.len(), 1);
    assert_eq!(result.items[0].name, "Jane Doe");
}
