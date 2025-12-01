use axum::{extract::Path, routing::get, Extension, Json, Router};
use axum_test::TestServer;
use serde_json::Value;
use sqlx::SqlitePool;

// Import necessary modules from the crate
use jura_hockey::{
    config::Config,
    errors::AppError,
    http::ApiContext,
    team::{business::TeamBusinessLogic, service},
};

/// Creates a test server with a simple endpoint for testing
async fn create_test_server(pool: SqlitePool) -> TestServer {
    let config = Config::test_config();

    let app = Router::new()
        .route("/api/teams/{id}", get(test_get_team_endpoint))
        .layer(Extension(ApiContext::new(pool, config)));

    TestServer::new(app).unwrap()
}

/// Test endpoint that uses our business logic layer
async fn test_get_team_endpoint(
    Extension(ctx): Extension<ApiContext>,
    Path(id): Path<String>,
) -> Result<Json<Value>, AppError> {
    let team_id: i64 = id
        .parse()
        .map_err(|_| AppError::invalid_input("Invalid team ID format"))?;

    // Use the business logic layer directly
    match TeamBusinessLogic::get_team(&ctx, team_id).await {
        Ok(team) => Ok(Json(serde_json::to_value(team).unwrap())),
        Err(err) => Err(err),
    }
}

// ========================================
// Team Business Logic Tests (Unit Tests)
// ========================================

#[sqlx::test]
async fn business_logic_get_nonexistent_team_returns_error(pool: SqlitePool) {
    let config = Config::test_config();
    let ctx = ApiContext::new(pool, config);

    // Test business logic directly - no HTTP involved
    let result = TeamBusinessLogic::get_team(&ctx, 99999).await;

    assert!(result.is_err());
    match result.unwrap_err() {
        AppError::TeamNotFound { id } => assert_eq!(id, 99999),
        _ => panic!("Expected TeamNotFound error"),
    }
}

#[sqlx::test]
async fn business_logic_create_team_validation(pool: SqlitePool) {
    let config = Config::test_config();
    let ctx = ApiContext::new(pool, config);

    // Test empty name validation
    let result = TeamBusinessLogic::create_team(&ctx, Some("".to_string()), 1, None).await;
    assert!(result.is_err());
    match result.unwrap_err() {
        AppError::InvalidInput { message } => assert!(message.contains("cannot be empty")),
        _ => panic!("Expected InvalidInput error"),
    }

    // Test long name validation
    let long_name = "A".repeat(101);
    let result = TeamBusinessLogic::create_team(&ctx, Some(long_name), 1, None).await;
    assert!(result.is_err());
    match result.unwrap_err() {
        AppError::InvalidInput { message } => assert!(message.contains("100 characters")),
        _ => panic!("Expected InvalidInput error"),
    }

    // Test invalid country ID
    let result =
        TeamBusinessLogic::create_team(&ctx, Some("Valid Team".to_string()), -1, None).await;
    assert!(result.is_err());
    match result.unwrap_err() {
        AppError::InvalidInput { message } => assert!(message.contains("positive")),
        _ => panic!("Expected InvalidInput error"),
    }
}

#[sqlx::test]
async fn business_logic_logo_path_validation(pool: SqlitePool) {
    let config = Config::test_config();
    let ctx = ApiContext::new(pool, config);

    // Test invalid logo path
    let result = TeamBusinessLogic::create_team(
        &ctx,
        Some("Test Team".to_string()),
        1,
        Some("invalid-path".to_string()),
    )
    .await;

    assert!(result.is_err());
    match result.unwrap_err() {
        AppError::InvalidInput { message } => {
            assert!(message.contains("valid URL or absolute path"))
        }
        _ => panic!("Expected InvalidInput error"),
    }
}

// ========================================
// Team HTTP Integration Tests (End-to-End)
// ========================================

#[sqlx::test]
async fn http_get_nonexistent_team_returns_404(pool: SqlitePool) {
    let server = create_test_server(pool).await;

    let response = server.get("/api/teams/99999").await;

    // Test HTTP status code
    response.assert_status(axum::http::StatusCode::NOT_FOUND);

    // Test error response structure
    let error_body: Value = response.json();
    assert_eq!(error_body["error"], "Team not found with id: 99999");
    assert!(error_body["error_id"].is_string());
    assert!(error_body["error_details"].is_null());
}

#[sqlx::test]
async fn http_invalid_team_id_returns_400(pool: SqlitePool) {
    let server = create_test_server(pool).await;

    let response = server.get("/api/teams/not-a-number").await;

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

    let response = server.get("/api/teams/1").await;

    // Should return 500 Internal Server Error
    response.assert_status(axum::http::StatusCode::INTERNAL_SERVER_ERROR);

    let error_body: Value = response.json();
    assert_eq!(error_body["error"], "Internal server error");
    assert!(error_body["error_id"].is_string());
}

// ========================================
// Team Service Layer Tests (Data Access)
// ========================================

#[sqlx::test]
async fn service_layer_get_team_by_id(pool: SqlitePool) {
    // Test the service layer directly
    let result = service::get_team_by_id(&pool, 99999).await;

    // Should succeed but return None (no team found)
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
        CREATE TABLE IF NOT EXISTS team (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT,
            country_id INTEGER,
            logo_path TEXT,
            created_at TEXT,
            updated_at TEXT,
            FOREIGN KEY (country_id) REFERENCES country (id)
        )
    "#,
    )
    .execute(&pool)
    .await
    .expect("Failed to create team table");

    // Insert a country first (required for foreign key)
    sqlx::query(
        "INSERT OR IGNORE INTO country (id, name, iso2Code) VALUES (1, 'Test Country', 'TC')",
    )
    .execute(&pool)
    .await
    .expect("Failed to insert test country");

    // Create a team
    let team_id = service::create_team(
        &pool,
        service::CreateTeamEntity {
            name: Some("Test Team".to_string()),
            country_id: 1,
            logo_path: None,
        },
    )
    .await
    .expect("Failed to create team");

    // Retrieve the team
    let team = service::get_team_by_id(&pool, team_id)
        .await
        .expect("Failed to get team")
        .expect("Team should exist");

    assert_eq!(team.id, team_id);
    assert_eq!(team.name, Some("Test Team".to_string()));
    assert_eq!(team.country_id, 1);
}
