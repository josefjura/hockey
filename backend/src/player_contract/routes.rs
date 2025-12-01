use aide::{
    axum::{
        routing::{get_with, post_with},
        ApiRouter, IntoApiResponse,
    },
    transform::TransformOperation,
};
use axum::{extract::Path, http::StatusCode, response::IntoResponse, Extension, Json};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::http::ApiContext;
use crate::player_contract::business::PlayerContractBusinessLogic;

use super::PlayerContract;

pub fn player_contract_routes() -> ApiRouter {
    ApiRouter::new()
        .api_route(
            "/",
            post_with(create_player_contract, create_player_contract_docs)
                .get_with(list_player_contracts, list_player_contracts_docs),
        )
        .api_route(
            "/{id}",
            get_with(get_player_contract, get_player_contract_docs)
                .delete_with(delete_player_contract, delete_player_contract_docs),
        )
}

/// New Player Contract details.
#[derive(Deserialize, JsonSchema)]
struct CreatePlayerContractRequest {
    /// The ID of the team.
    team_participation_id: i64,
    /// The ID of the season.
    player_id: i64,
}

/// New Player Contract details.
#[derive(Serialize, JsonSchema)]
struct PlayerContractCreateResponse {
    /// The ID of the new Player Contract.
    id: i64,
}

async fn create_player_contract(
    Extension(ctx): Extension<ApiContext>,
    Json(participation): Json<CreatePlayerContractRequest>,
) -> impl IntoApiResponse {
    match PlayerContractBusinessLogic::create_player_contract(
        &ctx,
        participation.team_participation_id,
        participation.player_id,
    )
    .await
    {
        Ok(result) => Ok((
            StatusCode::CREATED,
            Json(PlayerContractCreateResponse { id: result }),
        )),
        Err(app_error) => Err(app_error.into_response()),
    }
}

fn create_player_contract_docs(op: TransformOperation) -> TransformOperation {
    op.description("Create a new player contract.")
        .tag("player_contract")
        .response::<201, Json<PlayerContractCreateResponse>>()
}

async fn list_player_contracts(Extension(ctx): Extension<ApiContext>) -> impl IntoApiResponse {
    match PlayerContractBusinessLogic::list_player_contracts(&ctx).await {
        Ok(result) => Ok(Json(result)),
        Err(app_error) => Err(app_error.into_response()),
    }
}

fn list_player_contracts_docs(op: TransformOperation) -> TransformOperation {
    op.description("List all player contracts.")
        .tag("player_contract")
}

#[derive(Deserialize, JsonSchema)]
struct SelectPlayerContract {
    /// The ID of the Todo.
    id: i64,
}

async fn get_player_contract(
    Extension(ctx): Extension<ApiContext>,
    Path(contract): Path<SelectPlayerContract>,
) -> impl IntoApiResponse {
    match PlayerContractBusinessLogic::get_player_contract(&ctx, contract.id).await {
        Ok(result) => Ok(Json(result)),
        Err(app_error) => Err(app_error.into_response()),
    }
}

fn get_player_contract_docs(op: TransformOperation) -> TransformOperation {
    op.description("Get a single Player Contract.")
        .tag("player_contract")
        .response_with::<200, Json<PlayerContract>, _>(|res| {
            res.example(PlayerContract {
                id: 1,
                team_participation_id: 1,
                player_id: 1,
            })
        })
        .response_with::<404, (), _>(|res| res.description("player contract was not found"))
}

async fn delete_player_contract(
    Extension(ctx): Extension<ApiContext>,
    Path(contract): Path<SelectPlayerContract>,
) -> impl IntoApiResponse {
    match PlayerContractBusinessLogic::delete_player_contract(&ctx, contract.id).await {
        Ok(()) => Ok((StatusCode::NO_CONTENT, Json("Deleted".to_string()))),
        Err(app_error) => Err(app_error.into_response()),
    }
}

fn delete_player_contract_docs(op: TransformOperation) -> TransformOperation {
    op.description("Delete a Player Contract.")
        .tag("player_contract")
        .response_with::<204, (), _>(|res| res.description("The player contract has been deleted."))
        .response_with::<404, (), _>(|res| res.description("The player contract was not found"))
}
