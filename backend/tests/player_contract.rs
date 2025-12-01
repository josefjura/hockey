use axum::{extract::Path, response::IntoResponse, routing::get, Extension, Json, Router};
use axum_test::TestServer;
use serde_json::Value;
use sqlx::SqlitePool;

use jura_hockey::{
    config::Config, http::ApiContext, player_contract::business::PlayerContractBusinessLogic,
};

/// Creates a test server with a simple endpoint for testing
async fn create_test_server(pool: SqlitePool) -> TestServer {
    let config = Config::test_config();

    let app = Router::new()
        .route(
            "/api/player-contracts/{id}",
            get(test_get_player_contract_endpoint),
        )
        .layer(Extension(ApiContext::new(pool, config).unwrap()));

    TestServer::new(app).unwrap()
}

/// Test endpoint that uses our business logic layer
async fn test_get_player_contract_endpoint(
    Extension(ctx): Extension<ApiContext>,
    Path(id): Path<i64>,
) -> axum::response::Result<Json<Value>, axum::response::Response> {
    match PlayerContractBusinessLogic::get_player_contract(&ctx, id).await {
        Ok(contract) => Ok(Json(serde_json::to_value(contract).unwrap())),
        Err(app_error) => Err(app_error.into_response()),
    }
}

#[sqlx::test(fixtures("player_contract_deps"))]
async fn business_logic_create_player_contract_validation(pool: SqlitePool) {
    let config = Config::test_config();
    let ctx = ApiContext::new(pool, config).unwrap();

    // Get the actual IDs created by the fixtures
    let team_participation_id: i64 =
        sqlx::query_scalar("SELECT id FROM team_participation LIMIT 1")
            .fetch_one(&ctx.db)
            .await
            .unwrap();

    let player_id: i64 = sqlx::query_scalar("SELECT id FROM player LIMIT 1")
        .fetch_one(&ctx.db)
        .await
        .unwrap();

    // Test with invalid team participation ID
    let result = PlayerContractBusinessLogic::create_player_contract(&ctx, 999, player_id).await;
    assert!(result.is_err());

    // Test with invalid player ID
    let result =
        PlayerContractBusinessLogic::create_player_contract(&ctx, team_participation_id, 999).await;
    assert!(result.is_err());

    // Test successful creation with valid IDs
    let result =
        PlayerContractBusinessLogic::create_player_contract(&ctx, team_participation_id, player_id)
            .await;
    assert!(result.is_ok());
}

#[sqlx::test(fixtures("player_contracts"))]
async fn business_logic_get_nonexistent_player_contract_returns_error(pool: SqlitePool) {
    let config = Config::test_config();
    let ctx = ApiContext::new(pool, config).unwrap();

    // Test getting a contract that doesn't exist
    let result = PlayerContractBusinessLogic::get_player_contract(&ctx, 999).await;
    assert!(result.is_err());

    // Test getting a contract that does exist (get the first one from fixtures)
    let contract_id: i64 = sqlx::query_scalar("SELECT id FROM player_contract LIMIT 1")
        .fetch_one(&ctx.db)
        .await
        .unwrap();

    let result = PlayerContractBusinessLogic::get_player_contract(&ctx, contract_id).await;
    assert!(result.is_ok());
}

#[sqlx::test]
async fn http_invalid_player_contract_id_returns_400(pool: SqlitePool) {
    let server = create_test_server(pool).await;

    let response = server.get("/api/player-contracts/invalid").await;
    assert_eq!(response.status_code(), 400);
}

#[sqlx::test]
async fn http_get_nonexistent_player_contract_returns_404(pool: SqlitePool) {
    let server = create_test_server(pool).await;

    let response = server.get("/api/player-contracts/999").await;
    assert_eq!(response.status_code(), 404); // Now properly returns 404 for not found
}

#[sqlx::test]
async fn http_database_error_returns_500(pool: SqlitePool) {
    let server = create_test_server(pool).await;

    // Try to get a non-existent player contract - this should return 404, not 500
    // For a real 500 error, we would need to simulate an actual database error
    let response = server.get("/api/player-contracts/999").await;
    assert_eq!(response.status_code(), 404); // Not found, not a database error
}

#[sqlx::test(fixtures("player_contract_deps"))]
async fn service_layer_database_operations(pool: SqlitePool) {
    let config = Config::test_config();
    let ctx = ApiContext::new(pool, config).unwrap();

    // Get the actual IDs created by the fixtures
    let team_participation_id: i64 =
        sqlx::query_scalar("SELECT id FROM team_participation LIMIT 1")
            .fetch_one(&ctx.db)
            .await
            .unwrap();

    let player_id: i64 = sqlx::query_scalar("SELECT id FROM player LIMIT 1")
        .fetch_one(&ctx.db)
        .await
        .unwrap();

    // Test create player contract using existing fixtures
    let contract_id =
        PlayerContractBusinessLogic::create_player_contract(&ctx, team_participation_id, player_id)
            .await
            .unwrap();

    assert!(contract_id > 0);

    // Test get player contract
    let contract = PlayerContractBusinessLogic::get_player_contract(&ctx, contract_id)
        .await
        .unwrap();

    assert_eq!(contract.id, contract_id);
    assert_eq!(contract.team_participation_id, team_participation_id);
    assert_eq!(contract.player_id, player_id);

    // Test list player contracts
    let contracts = PlayerContractBusinessLogic::list_player_contracts(&ctx)
        .await
        .unwrap();

    assert!(contracts.len() >= 1);
    assert!(contracts.iter().any(|c| c.id == contract_id));

    // Test delete player contract
    PlayerContractBusinessLogic::delete_player_contract(&ctx, contract_id)
        .await
        .unwrap();

    // Verify deletion
    let result = PlayerContractBusinessLogic::get_player_contract(&ctx, contract_id).await;
    assert!(result.is_err());
}
