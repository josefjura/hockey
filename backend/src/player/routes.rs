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
use crate::http::ApiContext;
use crate::player::business::PlayerBusinessLogic;

use super::Player;

// Use generic PagedResponse for players
type PagedPlayersResponse = PagedResponse<Player>;

/// Query parameters for filtering players
#[derive(Deserialize, Serialize, JsonSchema)]
struct PlayerQueryParams {
    /// Filter by player name (partial match)
    name: Option<String>,
    /// Filter by country ID
    country_id: Option<i64>,
    /// Page number (1-based)
    page: Option<usize>,
    /// Number of items per page
    page_size: Option<usize>,
}

pub fn player_routes() -> ApiRouter {
    ApiRouter::new()
        .api_route(
            "/",
            post_with(create_player, create_player_docs).get_with(list_players, list_players_docs),
        )
        .api_route(
            "/{id}",
            get_with(get_player, get_player_docs)
                .put_with(update_player, update_player_docs)
                .delete_with(delete_player, delete_player_docs),
        )
}

/// New Player details.
#[derive(Deserialize, JsonSchema)]
struct CreatePlayerRequest {
    /// The name for the new Player.
    name: String,
    /// The country this player represents.
    country_id: i64,
    /// URL path to the player photo image.
    photo_path: Option<String>,
}

/// New Player details.
#[derive(Serialize, JsonSchema)]
struct PlayerCreateResponse {
    /// The ID of the new Player.
    id: i64,
}

async fn create_player(
    Extension(ctx): Extension<ApiContext>,
    Json(player): Json<CreatePlayerRequest>,
) -> impl IntoApiResponse {
    match PlayerBusinessLogic::create_player(
        &ctx,
        player.name,
        player.country_id,
        player.photo_path,
    )
    .await
    {
        Ok(result) => Ok((
            StatusCode::CREATED,
            Json(PlayerCreateResponse { id: result }),
        )),
        Err(app_error) => Err(app_error.into_response()),
    }
}

fn create_player_docs(op: TransformOperation) -> TransformOperation {
    op.description("Create a new player.")
        .tag("player")
        .response::<201, Json<PlayerCreateResponse>>()
}

async fn list_players(
    Extension(ctx): Extension<ApiContext>,
    Query(params): Query<PlayerQueryParams>,
) -> impl IntoApiResponse {
    let paging = Some(Paging::new(
        params.page.unwrap_or(1),
        params.page_size.unwrap_or(15),
    ));

    match PlayerBusinessLogic::list_players(&ctx, params.name, params.country_id, paging.as_ref())
        .await
    {
        Ok(result) => {
            let response = PagedPlayersResponse::from_result(result.clone(), result.items);
            Ok(Json(response))
        }
        Err(app_error) => Err(app_error.into_response()),
    }
}

fn list_players_docs(op: TransformOperation) -> TransformOperation {
    op.description("List players with optional filtering and pagination.")
        .tag("player")
        .response::<200, Json<PagedPlayersResponse>>()
}

#[derive(Deserialize, JsonSchema)]
struct SelectPlayer {
    /// The ID of the Player.
    id: i64,
}

async fn get_player(
    Extension(ctx): Extension<ApiContext>,
    Path(player): Path<SelectPlayer>,
) -> impl IntoApiResponse {
    match PlayerBusinessLogic::get_player(&ctx, player.id).await {
        Ok(result) => Ok(Json(result)),
        Err(app_error) => Err(app_error.into_response()),
    }
}

fn get_player_docs(op: TransformOperation) -> TransformOperation {
    op.description("Get a single Player.")
        .tag("player")
        .response_with::<200, Json<Player>, _>(|res| {
            res.example(Player {
                name: "Example Player".to_string(),
                id: 1,
                country_id: 1,
                country_name: "Example Country".to_string(),
                country_iso2_code: "EX".to_string(),
                photo_path: Some(
                    "http://localhost:9000/hockey-uploads/player-photo.jpg".to_string(),
                ),
                created_at: "2024-01-01T00:00:00Z".to_string(),
                updated_at: "2024-01-01T00:00:00Z".to_string(),
            })
        })
        .response_with::<404, (), _>(|res| res.description("player was not found"))
}

/// Update Player details.
#[derive(Deserialize, JsonSchema)]
struct UpdatePlayerRequest {
    /// The name of the player.
    name: String,
    /// The country this player represents.
    country_id: i64,
    /// URL path to the player photo image.
    photo_path: Option<String>,
}

async fn update_player(
    Extension(ctx): Extension<ApiContext>,
    Path(player): Path<SelectPlayer>,
    Json(update_data): Json<UpdatePlayerRequest>,
) -> impl IntoApiResponse {
    match PlayerBusinessLogic::update_player(
        &ctx,
        player.id,
        update_data.name,
        update_data.country_id,
        update_data.photo_path,
    )
    .await
    {
        Ok(updated_player) => Ok(Json(updated_player)),
        Err(app_error) => Err(app_error.into_response()),
    }
}

fn update_player_docs(op: TransformOperation) -> TransformOperation {
    op.description("Update a Player.")
        .tag("player")
        .response_with::<200, (), _>(|res| res.description("The player has been updated."))
        .response_with::<404, (), _>(|res| res.description("The player was not found"))
}

async fn delete_player(
    Extension(ctx): Extension<ApiContext>,
    Path(player): Path<SelectPlayer>,
) -> impl IntoApiResponse {
    match PlayerBusinessLogic::delete_player(&ctx, player.id).await {
        Ok(()) => Ok((StatusCode::NO_CONTENT, Json("Deleted".to_string()))),
        Err(app_error) => Err(app_error.into_response()),
    }
}

fn delete_player_docs(op: TransformOperation) -> TransformOperation {
    op.description("Delete a Player.")
        .tag("player")
        .response_with::<204, (), _>(|res| res.description("The player has been deleted."))
        .response_with::<404, (), _>(|res| res.description("The player was not found"))
}
