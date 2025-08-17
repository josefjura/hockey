use axum::{Extension, Json, Router, extract::Path, routing::get};
use axum_test::TestServer;
use serde_json::Value;
use sqlx::SqlitePool;

// Import necessary modules from the crate
use jura_hockey::{
    config::Config,
    errors::AppError,
    http::ApiContext,
    r#match::service::CreateMatchEntity,
    r#match::{Match, business::MatchBusinessLogic, service},
};

/// Creates a test server with a simple endpoint for testing
async fn create_test_server(pool: SqlitePool) -> TestServer {
    let config = Config {
        database_url: "sqlite::memory:".to_string(),
        hmac_key: "test-hmac-key".to_string(),
    };

    let app = Router::new()
        .route("/api/matches/{id}", get(test_get_match_endpoint))
        .layer(Extension(ApiContext::new(pool, config)));

    TestServer::new(app).unwrap()
}

/// Test endpoint that uses our business logic layer
async fn test_get_match_endpoint(
    Extension(ctx): Extension<ApiContext>,
    Path(id): Path<String>,
) -> Result<Json<Value>, AppError> {
    let match_id: i64 = id
        .parse()
        .map_err(|_| AppError::invalid_input("Invalid match ID format"))?;

    // Use the business logic layer directly
    match MatchBusinessLogic::get_match(&ctx, match_id).await {
        Ok(match_data) => Ok(Json(serde_json::to_value(match_data).unwrap())),
        Err(err) => Err(err),
    }
}

// ========================================
// Match Business Logic Tests (Unit Tests)
// ========================================

#[sqlx::test]
async fn business_logic_get_nonexistent_match_returns_error(pool: SqlitePool) {
    let config = Config {
        database_url: "sqlite::memory:".to_string(),
        hmac_key: "test-key".to_string(),
    };
    let ctx = ApiContext::new(pool, config);

    // Test business logic directly - no HTTP involved
    let result = MatchBusinessLogic::get_match(&ctx, 99999).await;

    assert!(result.is_err());
    match result.unwrap_err() {
        AppError::MatchNotFound { id } => assert_eq!(id, 99999),
        _ => panic!("Expected MatchNotFound error"),
    }
}

#[sqlx::test]
async fn business_logic_create_match_validation(pool: SqlitePool) {
    let config = Config {
        database_url: "sqlite::memory:".to_string(),
        hmac_key: "test-key".to_string(),
    };
    let ctx = ApiContext::new(pool, config);

    // Test invalid season ID
    let result = MatchBusinessLogic::create_match(&ctx, -1, 1, 2, 0, 0, None, None, None).await;
    assert!(result.is_err());
    match result.unwrap_err() {
        AppError::InvalidInput { message } => {
            assert!(message.contains("Season ID must be positive"))
        }
        _ => panic!("Expected InvalidInput error"),
    }

    // Test invalid home team ID
    let result = MatchBusinessLogic::create_match(&ctx, 1, -1, 2, 0, 0, None, None, None).await;
    assert!(result.is_err());
    match result.unwrap_err() {
        AppError::InvalidInput { message } => {
            assert!(message.contains("Home team ID must be positive"))
        }
        _ => panic!("Expected InvalidInput error"),
    }

    // Test invalid away team ID
    let result = MatchBusinessLogic::create_match(&ctx, 1, 1, -1, 0, 0, None, None, None).await;
    assert!(result.is_err());
    match result.unwrap_err() {
        AppError::InvalidInput { message } => {
            assert!(message.contains("Away team ID must be positive"))
        }
        _ => panic!("Expected InvalidInput error"),
    }

    // Test same team for home and away
    let result = MatchBusinessLogic::create_match(&ctx, 1, 1, 1, 0, 0, None, None, None).await;
    assert!(result.is_err());
    match result.unwrap_err() {
        AppError::InvalidInput { message } => {
            assert!(message.contains("Home team and away team must be different"))
        }
        _ => panic!("Expected InvalidInput error"),
    }

    // Test negative scores
    let result = MatchBusinessLogic::create_match(&ctx, 1, 1, 2, -1, 0, None, None, None).await;
    assert!(result.is_err());
    match result.unwrap_err() {
        AppError::InvalidInput { message } => {
            assert!(message.contains("Home score cannot be negative"))
        }
        _ => panic!("Expected InvalidInput error"),
    }

    let result = MatchBusinessLogic::create_match(&ctx, 1, 1, 2, 0, -1, None, None, None).await;
    assert!(result.is_err());
    match result.unwrap_err() {
        AppError::InvalidInput { message } => {
            assert!(message.contains("Away score cannot be negative"))
        }
        _ => panic!("Expected InvalidInput error"),
    }
}

#[sqlx::test(fixtures("match_deps"))]
async fn business_logic_match_date_validation(pool: SqlitePool) {
    let config = Config {
        database_url: "sqlite::memory:".to_string(),
        hmac_key: "test-key".to_string(),
    };
    let ctx = ApiContext::new(pool, config);

    // Get the actual IDs created by the fixtures
    let season_id: i64 = sqlx::query_scalar("SELECT id FROM season LIMIT 1")
        .fetch_one(&ctx.db)
        .await
        .unwrap();

    let home_team_id: i64 = sqlx::query_scalar("SELECT id FROM team WHERE name = 'Home Team'")
        .fetch_one(&ctx.db)
        .await
        .unwrap();

    let away_team_id: i64 = sqlx::query_scalar("SELECT id FROM team WHERE name = 'Away Team'")
        .fetch_one(&ctx.db)
        .await
        .unwrap();

    // Test empty match date
    let result = MatchBusinessLogic::create_match(
        &ctx,
        season_id,
        home_team_id,
        away_team_id,
        0,
        0,
        Some("".to_string()),
        None,
        None,
    )
    .await;
    assert!(result.is_err());
    match result.unwrap_err() {
        AppError::InvalidInput { message } => {
            assert!(message.contains("Match date cannot be empty"))
        }
        _ => panic!("Expected InvalidInput error"),
    }

    // Test invalid date format (too short)
    let result = MatchBusinessLogic::create_match(
        &ctx,
        season_id,
        home_team_id,
        away_team_id,
        0,
        0,
        Some("invalid".to_string()),
        None,
        None,
    )
    .await;
    assert!(result.is_err());
    match result.unwrap_err() {
        AppError::InvalidInput { message } => assert!(message.contains("ISO 8601 format")),
        _ => panic!("Expected InvalidInput error"),
    }
}

#[sqlx::test]
async fn business_logic_status_validation(pool: SqlitePool) {
    let config = Config {
        database_url: "sqlite::memory:".to_string(),
        hmac_key: "test-key".to_string(),
    };
    let ctx = ApiContext::new(pool, config);

    // Test empty status
    let result =
        MatchBusinessLogic::create_match(&ctx, 1, 1, 2, 0, 0, None, Some("".to_string()), None)
            .await;
    assert!(result.is_err());
    match result.unwrap_err() {
        AppError::InvalidInput { message } => {
            assert!(message.contains("Match status cannot be empty"))
        }
        _ => panic!("Expected InvalidInput error"),
    }

    // Test invalid status
    let result = MatchBusinessLogic::create_match(
        &ctx,
        1,
        1,
        2,
        0,
        0,
        None,
        Some("invalid_status".to_string()),
        None,
    )
    .await;
    assert!(result.is_err());
    match result.unwrap_err() {
        AppError::InvalidInput { message } => assert!(message.contains("Invalid match status")),
        _ => panic!("Expected InvalidInput error"),
    }
}

#[sqlx::test]
async fn business_logic_venue_validation(pool: SqlitePool) {
    let config = Config {
        database_url: "sqlite::memory:".to_string(),
        hmac_key: "test-key".to_string(),
    };
    let ctx = ApiContext::new(pool, config);

    // Test empty venue
    let result =
        MatchBusinessLogic::create_match(&ctx, 1, 1, 2, 0, 0, None, None, Some("".to_string()))
            .await;
    assert!(result.is_err());
    match result.unwrap_err() {
        AppError::InvalidInput { message } => assert!(message.contains("Venue cannot be empty")),
        _ => panic!("Expected InvalidInput error"),
    }

    // Test long venue name
    let long_venue = "A".repeat(201);
    let result =
        MatchBusinessLogic::create_match(&ctx, 1, 1, 2, 0, 0, None, None, Some(long_venue)).await;
    assert!(result.is_err());
    match result.unwrap_err() {
        AppError::InvalidInput { message } => assert!(message.contains("200 characters")),
        _ => panic!("Expected InvalidInput error"),
    }
}

#[sqlx::test]
async fn business_logic_score_event_validation(pool: SqlitePool) {
    let config = Config {
        database_url: "sqlite::memory:".to_string(),
        hmac_key: "test-key".to_string(),
    };
    let ctx = ApiContext::new(pool, config);

    // Test invalid team ID (without checking match existence first)
    // This tests validation logic independent of database state

    // Test invalid period
    let result = MatchBusinessLogic::create_score_event(
        &ctx,
        1,
        1,
        None,
        None,
        None,
        Some(0),
        None,
        None,
        None,
    )
    .await;
    assert!(result.is_err());
    // Should return MatchNotFound first since we validate match existence before other validation
    match result.unwrap_err() {
        AppError::MatchNotFound { id } => assert_eq!(id, 1),
        _ => panic!("Expected MatchNotFound error"),
    }
}

#[sqlx::test]
async fn business_logic_score_event_validation_with_match(pool: SqlitePool) {
    let config = Config {
        database_url: "sqlite::memory:".to_string(),
        hmac_key: "test-key".to_string(),
    };
    let ctx = ApiContext::new(pool.clone(), config);

    // Create the necessary tables and data first
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS event (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL
        )
    "#,
    )
    .execute(&pool)
    .await
    .expect("Failed to create event table");

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS season (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            year INTEGER NOT NULL,
            display_name TEXT,
            event_id INTEGER NOT NULL,
            FOREIGN KEY (event_id) REFERENCES event (id)
        )
    "#,
    )
    .execute(&pool)
    .await
    .expect("Failed to create season table");

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS country (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL
        )
    "#,
    )
    .execute(&pool)
    .await
    .expect("Failed to create country table");

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS team (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT,
            country_id INTEGER,
            FOREIGN KEY (country_id) REFERENCES country (id)
        )
    "#,
    )
    .execute(&pool)
    .await
    .expect("Failed to create team table");

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS match (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            season_id INTEGER NOT NULL,
            home_team_id INTEGER NOT NULL,
            away_team_id INTEGER NOT NULL,
            home_score_unidentified INTEGER DEFAULT 0,
            away_score_unidentified INTEGER DEFAULT 0,
            match_date TEXT,
            status TEXT DEFAULT 'scheduled',
            venue TEXT,
            created_at TEXT DEFAULT CURRENT_TIMESTAMP,
            updated_at TEXT DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (season_id) REFERENCES season (id),
            FOREIGN KEY (home_team_id) REFERENCES team (id),
            FOREIGN KEY (away_team_id) REFERENCES team (id)
        )
    "#,
    )
    .execute(&pool)
    .await
    .expect("Failed to create match table");

    // Insert test data
    sqlx::query("INSERT OR IGNORE INTO event (id, name) VALUES (1, 'Test Event')")
        .execute(&pool)
        .await
        .expect("Failed to insert test event");

    sqlx::query("INSERT OR IGNORE INTO season (id, year, display_name, event_id) VALUES (1, 2024, 'Test Season', 1)")
        .execute(&pool)
        .await
        .expect("Failed to insert test season");

    sqlx::query("INSERT OR IGNORE INTO country (id, name) VALUES (1, 'Test Country')")
        .execute(&pool)
        .await
        .expect("Failed to insert test country");

    sqlx::query(
        "INSERT OR IGNORE INTO team (id, name, country_id) VALUES (1, 'Test Home Team', 1)",
    )
    .execute(&pool)
    .await
    .expect("Failed to insert test home team");

    sqlx::query(
        "INSERT OR IGNORE INTO team (id, name, country_id) VALUES (2, 'Test Away Team', 1)",
    )
    .execute(&pool)
    .await
    .expect("Failed to insert test away team");

    // Create a match
    let match_id = service::create_match(
        &pool,
        CreateMatchEntity {
            season_id: 1,
            home_team_id: 1,
            away_team_id: 2,
            home_score_unidentified: 0,
            away_score_unidentified: 0,
            match_date: None,
            status: Some("scheduled".to_string()),
            venue: None,
        },
    )
    .await
    .expect("Failed to create match");

    // Now test validation with an existing match

    // Test invalid team ID
    let result = MatchBusinessLogic::create_score_event(
        &ctx, match_id, -1, None, None, None, None, None, None, None,
    )
    .await;
    assert!(result.is_err());
    match result.unwrap_err() {
        AppError::InvalidInput { message } => assert!(message.contains("Team ID must be positive")),
        _ => panic!("Expected InvalidInput error"),
    }

    // Test invalid period
    let result = MatchBusinessLogic::create_score_event(
        &ctx,
        match_id,
        1,
        None,
        None,
        None,
        Some(0),
        None,
        None,
        None,
    )
    .await;
    assert!(result.is_err());
    match result.unwrap_err() {
        AppError::InvalidInput { message } => {
            assert!(message.contains("Period must be between 1 and 5"))
        }
        _ => panic!("Expected InvalidInput error"),
    }

    let result = MatchBusinessLogic::create_score_event(
        &ctx,
        match_id,
        1,
        None,
        None,
        None,
        Some(6),
        None,
        None,
        None,
    )
    .await;
    assert!(result.is_err());
    match result.unwrap_err() {
        AppError::InvalidInput { message } => {
            assert!(message.contains("Period must be between 1 and 5"))
        }
        _ => panic!("Expected InvalidInput error"),
    }

    // Test invalid time
    let result = MatchBusinessLogic::create_score_event(
        &ctx,
        match_id,
        1,
        None,
        None,
        None,
        None,
        Some(-1),
        None,
        None,
    )
    .await;
    assert!(result.is_err());
    match result.unwrap_err() {
        AppError::InvalidInput { message } => {
            assert!(message.contains("Time minutes must be between 0 and 60"))
        }
        _ => panic!("Expected InvalidInput error"),
    }

    let result = MatchBusinessLogic::create_score_event(
        &ctx,
        match_id,
        1,
        None,
        None,
        None,
        None,
        None,
        Some(60),
        None,
    )
    .await;
    assert!(result.is_err());
    match result.unwrap_err() {
        AppError::InvalidInput { message } => {
            assert!(message.contains("Time seconds must be between 0 and 59"))
        }
        _ => panic!("Expected InvalidInput error"),
    }

    // Test invalid goal type
    let result = MatchBusinessLogic::create_score_event(
        &ctx,
        match_id,
        1,
        None,
        None,
        None,
        None,
        None,
        None,
        Some("invalid_type".to_string()),
    )
    .await;
    assert!(result.is_err());
    match result.unwrap_err() {
        AppError::InvalidInput { message } => assert!(message.contains("Invalid goal type")),
        _ => panic!("Expected InvalidInput error"),
    }

    // Test same player for scorer and assist
    let result = MatchBusinessLogic::create_score_event(
        &ctx,
        match_id,
        1,
        Some(1),
        Some(1),
        None,
        None,
        None,
        None,
        None,
    )
    .await;
    assert!(result.is_err());
    match result.unwrap_err() {
        AppError::InvalidInput { message } => {
            assert!(message.contains("Scorer and first assist cannot be the same player"))
        }
        _ => panic!("Expected InvalidInput error"),
    }

    let result = MatchBusinessLogic::create_score_event(
        &ctx,
        match_id,
        1,
        Some(1),
        None,
        Some(1),
        None,
        None,
        None,
        None,
    )
    .await;
    assert!(result.is_err());
    match result.unwrap_err() {
        AppError::InvalidInput { message } => {
            assert!(message.contains("Scorer and second assist cannot be the same player"))
        }
        _ => panic!("Expected InvalidInput error"),
    }

    let result = MatchBusinessLogic::create_score_event(
        &ctx,
        match_id,
        1,
        None,
        Some(1),
        Some(1),
        None,
        None,
        None,
        None,
    )
    .await;
    assert!(result.is_err());
    match result.unwrap_err() {
        AppError::InvalidInput { message } => {
            assert!(message.contains("First and second assist cannot be the same player"))
        }
        _ => panic!("Expected InvalidInput error"),
    }
}

// ========================================
// Match HTTP Integration Tests (End-to-End)
// ========================================

#[sqlx::test]
async fn http_get_nonexistent_match_returns_404(pool: SqlitePool) {
    let server = create_test_server(pool).await;

    let response = server.get("/api/matches/99999").await;

    // Test HTTP status code
    response.assert_status(axum::http::StatusCode::NOT_FOUND);

    // Test error response structure
    let error_body: Value = response.json();
    assert_eq!(error_body["error"], "Match not found with id: 99999");
    assert!(error_body["error_id"].is_string());
    assert!(error_body["error_details"].is_null());
}

#[sqlx::test]
async fn http_invalid_match_id_returns_400(pool: SqlitePool) {
    let server = create_test_server(pool).await;

    let response = server.get("/api/matches/not-a-number").await;

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

    let response = server.get("/api/matches/1").await;

    // Should return 500 Internal Server Error
    response.assert_status(axum::http::StatusCode::INTERNAL_SERVER_ERROR);

    let error_body: Value = response.json();
    assert_eq!(error_body["error"], "Internal server error");
    assert!(error_body["error_id"].is_string());
}

// ========================================
// Match Service Layer Tests (Data Access)
// ========================================

#[sqlx::test]
async fn service_layer_get_match_by_id(pool: SqlitePool) {
    // Test the service layer directly
    let result = service::get_match_by_id(&pool, 99999).await;

    // Should succeed but return None (no match found)
    assert!(result.is_ok());
    assert!(result.unwrap().is_none());
}

#[sqlx::test(fixtures("match_deps"))]
async fn service_layer_database_operations(pool: SqlitePool) {
    // Get the actual IDs created by the fixtures
    let season_id: i64 = sqlx::query_scalar("SELECT id FROM season LIMIT 1")
        .fetch_one(&pool)
        .await
        .unwrap();

    let home_team_id: i64 = sqlx::query_scalar("SELECT id FROM team WHERE name = 'Home Team'")
        .fetch_one(&pool)
        .await
        .unwrap();

    let away_team_id: i64 = sqlx::query_scalar("SELECT id FROM team WHERE name = 'Away Team'")
        .fetch_one(&pool)
        .await
        .unwrap();

    // Create a match
    let match_id = service::create_match(
        &pool,
        CreateMatchEntity {
            season_id,
            home_team_id,
            away_team_id,
            home_score_unidentified: 2,
            away_score_unidentified: 1,
            match_date: Some("2024-01-15T20:00:00".to_string()),
            status: Some("finished".to_string()),
            venue: Some("Test Arena".to_string()),
        },
    )
    .await
    .expect("Failed to create match");

    // Retrieve the match
    let match_data = service::get_match_by_id(&pool, match_id)
        .await
        .expect("Failed to get match")
        .expect("Match should exist");

    assert_eq!(match_data.id, match_id);
    assert_eq!(match_data.season_id, season_id);
    assert_eq!(match_data.home_team_id, home_team_id);
    assert_eq!(match_data.away_team_id, away_team_id);
    assert_eq!(match_data.home_score_unidentified, 2);
    assert_eq!(match_data.away_score_unidentified, 1);
    assert_eq!(
        match_data.match_date,
        Some("2024-01-15T20:00:00".to_string())
    );
    assert_eq!(match_data.status, "finished");
    assert_eq!(match_data.venue, Some("Test Arena".to_string()));
    assert_eq!(match_data.season_name, "2023 Czech Season");
    assert_eq!(match_data.home_team_name, "Home Team");
    assert_eq!(match_data.away_team_name, "Away Team");
}
