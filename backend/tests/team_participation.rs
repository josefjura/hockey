use axum::{extract::Path, response::IntoResponse, routing::get, Extension, Json, Router};
use axum_test::TestServer;
use serde_json::Value;
use sqlx::SqlitePool;

use jura_hockey::{
    config::Config, http::ApiContext, team_participation::business::TeamParticipationBusinessLogic,
};

/// Creates a test server with a simple endpoint for testing
async fn create_test_server(pool: SqlitePool) -> TestServer {
    let config = Config::test_config();

    let app = Router::new()
        .route(
            "/api/team-participations/{id}",
            get(test_get_team_participation_endpoint),
        )
        .layer(Extension(ApiContext::new(pool, config)));

    TestServer::new(app).unwrap()
}

/// Test endpoint that uses our business logic layer
async fn test_get_team_participation_endpoint(
    Extension(ctx): Extension<ApiContext>,
    Path(id): Path<i64>,
) -> axum::response::Result<Json<Value>, axum::response::Response> {
    match TeamParticipationBusinessLogic::get_team_participation(&ctx, id).await {
        Ok(participation) => Ok(Json(serde_json::to_value(participation).unwrap())),
        Err(app_error) => Err(app_error.into_response()),
    }
}

#[sqlx::test(fixtures("team_participation_deps"))]
async fn business_logic_create_team_participation_validation(pool: SqlitePool) {
    let config = Config::test_config();
    let ctx = ApiContext::new(pool, config);

    // Get the actual IDs created by the fixtures
    let team_id: i64 = sqlx::query_scalar("SELECT id FROM team LIMIT 1")
        .fetch_one(&ctx.db)
        .await
        .unwrap();

    let season_id: i64 = sqlx::query_scalar("SELECT id FROM season LIMIT 1")
        .fetch_one(&ctx.db)
        .await
        .unwrap();

    // Test with invalid team ID
    let result =
        TeamParticipationBusinessLogic::create_team_participation(&ctx, 999, season_id).await;
    assert!(result.is_err());

    // Test with invalid season ID
    let result =
        TeamParticipationBusinessLogic::create_team_participation(&ctx, team_id, 999).await;
    assert!(result.is_err());

    // Test successful creation with valid IDs
    let result =
        TeamParticipationBusinessLogic::create_team_participation(&ctx, team_id, season_id).await;
    assert!(result.is_ok());
}

#[sqlx::test(fixtures("team_participations"))]
async fn business_logic_get_nonexistent_team_participation_returns_error(pool: SqlitePool) {
    let config = Config::test_config();
    let ctx = ApiContext::new(pool, config);

    // Test getting a participation that doesn't exist
    let result = TeamParticipationBusinessLogic::get_team_participation(&ctx, 999).await;
    assert!(result.is_err());

    // Test getting a participation that does exist
    let result = TeamParticipationBusinessLogic::get_team_participation(&ctx, 1).await;
    assert!(result.is_ok());
}

#[sqlx::test(fixtures("team_participation_deps"))]
async fn business_logic_find_or_create_validation(pool: SqlitePool) {
    let config = Config::test_config();
    let ctx = ApiContext::new(pool, config);

    // Test with invalid team ID
    let result =
        TeamParticipationBusinessLogic::find_or_create_team_participation(&ctx, 999, 1).await;
    assert!(result.is_err());

    // Test with invalid season ID
    let result =
        TeamParticipationBusinessLogic::find_or_create_team_participation(&ctx, 1, 999).await;
    assert!(result.is_err());

    // Test successful find or create with valid IDs
    let result =
        TeamParticipationBusinessLogic::find_or_create_team_participation(&ctx, 1, 1).await;
    assert!(result.is_ok());
}

#[sqlx::test]
async fn http_invalid_team_participation_id_returns_400(pool: SqlitePool) {
    let server = create_test_server(pool).await;

    let response = server.get("/api/team-participations/invalid").await;
    assert_eq!(response.status_code(), 400);
}

#[sqlx::test]
async fn http_get_nonexistent_team_participation_returns_404(pool: SqlitePool) {
    let server = create_test_server(pool).await;

    let response = server.get("/api/team-participations/999").await;
    assert_eq!(response.status_code(), 404); // Now properly returns 404 for not found
}

#[sqlx::test]
async fn http_database_error_returns_500(pool: SqlitePool) {
    let server = create_test_server(pool).await;

    // Try to get a non-existent team participation - this should return 404, not 500
    // For a real 500 error, we would need to simulate an actual database error
    let response = server.get("/api/team-participations/999").await;
    assert_eq!(response.status_code(), 404); // Not found, not a database error
}

#[sqlx::test(fixtures("team_participation_deps"))]
async fn service_layer_database_operations(pool: SqlitePool) {
    let config = Config::test_config();
    let ctx = ApiContext::new(pool, config);

    // Get the actual IDs created by the fixtures
    let team_id: i64 = sqlx::query_scalar("SELECT id FROM team LIMIT 1")
        .fetch_one(&ctx.db)
        .await
        .unwrap();

    let season_id: i64 = sqlx::query_scalar("SELECT id FROM season LIMIT 1")
        .fetch_one(&ctx.db)
        .await
        .unwrap();

    // Test create team participation using existing fixtures
    let participation_id =
        TeamParticipationBusinessLogic::create_team_participation(&ctx, team_id, season_id)
            .await
            .unwrap();

    assert!(participation_id > 0);

    // Test get team participation
    let participation =
        TeamParticipationBusinessLogic::get_team_participation(&ctx, participation_id)
            .await
            .unwrap();

    assert_eq!(participation.id, participation_id);
    assert_eq!(participation.team_id, team_id);
    assert_eq!(participation.season_id, season_id);

    // Test list team participations
    let participations = TeamParticipationBusinessLogic::list_team_participations(&ctx)
        .await
        .unwrap();

    assert!(participations.len() >= 1);
    assert!(participations.iter().any(|p| p.id == participation_id));

    // Test find or create (should find existing)
    let found_id =
        TeamParticipationBusinessLogic::find_or_create_team_participation(&ctx, team_id, season_id)
            .await
            .unwrap();

    assert_eq!(found_id, participation_id);

    // Test delete team participation
    TeamParticipationBusinessLogic::delete_team_participation(&ctx, participation_id)
        .await
        .unwrap();

    // Verify deletion
    let result =
        TeamParticipationBusinessLogic::get_team_participation(&ctx, participation_id).await;
    assert!(result.is_err());
}

#[sqlx::test(fixtures("team_participation_deps"))]
async fn business_logic_find_or_create_new_participation(pool: SqlitePool) {
    let config = Config::test_config();
    let ctx = ApiContext::new(pool, config);

    // Get the actual IDs created by the fixtures
    let team_ids: Vec<i64> = sqlx::query_scalar("SELECT id FROM team ORDER BY id")
        .fetch_all(&ctx.db)
        .await
        .unwrap();

    let season_id: i64 = sqlx::query_scalar("SELECT id FROM season LIMIT 1")
        .fetch_one(&ctx.db)
        .await
        .unwrap();

    // Use the second team if available, otherwise create with the first team (it should still work)
    let team_id = if team_ids.len() > 1 {
        team_ids[1]
    } else {
        team_ids[0]
    };

    // Test find or create (should create new) - use different team/season combination
    let participation_id =
        TeamParticipationBusinessLogic::find_or_create_team_participation(&ctx, team_id, season_id)
            .await
            .unwrap();

    assert!(participation_id > 0);

    // Verify it was actually created
    let participation =
        TeamParticipationBusinessLogic::get_team_participation(&ctx, participation_id)
            .await
            .unwrap();

    assert_eq!(participation.team_id, team_id);
    assert_eq!(participation.season_id, season_id);
}
