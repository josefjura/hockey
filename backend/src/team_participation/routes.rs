use aide::{
    axum::{
        ApiRouter, IntoApiResponse,
        routing::{get_with, post_with},
    },
    transform::TransformOperation,
};
use axum::{Extension, Json, extract::Path, http::StatusCode, response::IntoResponse};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::http::ApiContext;
use crate::team_participation::business::TeamParticipationBusinessLogic;

use super::TeamParticipation;

pub fn team_participation_routes() -> ApiRouter {
    ApiRouter::new()
        .api_route(
            "/",
            post_with(create_team_participation, create_team_participation_docs)
                .get_with(list_team_participation, list_team_participation_docs),
        )
        .api_route(
            "/find-or-create",
            post_with(
                find_or_create_team_participation_route,
                find_or_create_team_participation_docs,
            ),
        )
        .api_route(
            "/{id}",
            get_with(get_team_participation, get_team_participation_docs)
                .delete_with(delete_team_participation, delete_team_participation_docs),
        )
}

/// New Team Participation details.
#[derive(Deserialize, JsonSchema)]
struct CreateTeamParticipationRequest {
    /// The ID of the team.
    pub team_id: i64,
    /// The ID of the season.
    pub season_id: i64,
}

/// New Team Participation details.
#[derive(Serialize, JsonSchema)]
struct TeamParticipationCreateResponse {
    /// The ID of the new Team Participation.
    id: i64,
}

async fn create_team_participation(
    Extension(ctx): Extension<ApiContext>,
    Json(participation): Json<CreateTeamParticipationRequest>,
) -> impl IntoApiResponse {
    match TeamParticipationBusinessLogic::create_team_participation(
        &ctx,
        participation.team_id,
        participation.season_id,
    )
    .await
    {
        Ok(result) => Ok((
            StatusCode::CREATED,
            Json(TeamParticipationCreateResponse { id: result }),
        )),
        Err(app_error) => Err(app_error.into_response()),
    }
}

fn create_team_participation_docs(op: TransformOperation) -> TransformOperation {
    op.description("Create a new team participation.")
        .tag("team_participation")
        .response::<201, Json<TeamParticipationCreateResponse>>()
}

async fn list_team_participation(Extension(ctx): Extension<ApiContext>) -> impl IntoApiResponse {
    match TeamParticipationBusinessLogic::list_team_participations(&ctx).await {
        Ok(result) => Ok(Json(result)),
        Err(app_error) => Err(app_error.into_response()),
    }
}

fn list_team_participation_docs(op: TransformOperation) -> TransformOperation {
    op.description("List all team participations.")
        .tag("team_participation")
}

#[derive(Deserialize, JsonSchema)]
struct SelectTeamParticipation {
    /// The ID of the Todo.
    id: i64,
}

async fn get_team_participation(
    Extension(ctx): Extension<ApiContext>,
    Path(participation): Path<SelectTeamParticipation>,
) -> impl IntoApiResponse {
    match TeamParticipationBusinessLogic::get_team_participation(&ctx, participation.id).await {
        Ok(result) => Ok(Json(result)),
        Err(app_error) => Err(app_error.into_response()),
    }
}

fn get_team_participation_docs(op: TransformOperation) -> TransformOperation {
    op.description("Get a single Team Participation.")
        .tag("team_participation")
        .response_with::<200, Json<TeamParticipation>, _>(|res| {
            res.example(TeamParticipation {
                id: 1,
                team_id: 1,
                season_id: 2023,
            })
        })
        .response_with::<404, (), _>(|res| res.description("season was not found"))
}

async fn delete_team_participation(
    Extension(ctx): Extension<ApiContext>,
    Path(participation): Path<SelectTeamParticipation>,
) -> impl IntoApiResponse {
    match TeamParticipationBusinessLogic::delete_team_participation(&ctx, participation.id).await {
        Ok(()) => Ok((StatusCode::NO_CONTENT, Json("Deleted".to_string()))),
        Err(app_error) => Err(app_error.into_response()),
    }
}

fn delete_team_participation_docs(op: TransformOperation) -> TransformOperation {
    op.description("Delete a Team Participation.")
        .tag("team_participation")
        .response_with::<204, (), _>(|res| {
            res.description("The team participation has been deleted.")
        })
        .response_with::<404, (), _>(|res| res.description("The team participation was not found"))
}

async fn find_or_create_team_participation_route(
    Extension(ctx): Extension<ApiContext>,
    Json(participation): Json<CreateTeamParticipationRequest>,
) -> impl IntoApiResponse {
    match TeamParticipationBusinessLogic::find_or_create_team_participation(
        &ctx,
        participation.team_id,
        participation.season_id,
    )
    .await
    {
        Ok(result) => Ok((
            StatusCode::OK,
            Json(TeamParticipationCreateResponse { id: result }),
        )),
        Err(app_error) => Err(app_error.into_response()),
    }
}

fn find_or_create_team_participation_docs(op: TransformOperation) -> TransformOperation {
    op.description("Find existing team participation or create new one for a season and team.")
        .tag("team_participation")
        .response::<200, Json<TeamParticipationCreateResponse>>()
}
