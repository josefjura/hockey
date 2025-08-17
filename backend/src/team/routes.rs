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
use crate::errors::AppError;
use crate::http::ApiContext;
use crate::team::business::TeamBusinessLogic;
use crate::team::service::TeamFilters;

use super::{Team, TeamDetail, TeamList};

// Use generic PagedResponse for teams
type PagedTeamsResponse = PagedResponse<Team>;

/// Query parameters for filtering teams
#[derive(Deserialize, Serialize, JsonSchema)]
struct TeamQueryParams {
    /// Filter by team name (partial match)
    name: Option<String>,
    /// Filter by country ID
    country_id: Option<i64>,
    /// Page number (1-based)
    page: Option<usize>,
    /// Number of items per page
    page_size: Option<usize>,
}

pub fn team_routes() -> ApiRouter {
    ApiRouter::new()
        .api_route(
            "/",
            post_with(create_team, create_team_docs).get_with(list_teams, list_teams_docs),
        )
        .api_route("/list", get_with(list_teams_simple, list_teams_simple_docs))
        .api_route(
            "/{id}",
            get_with(get_team, get_team_docs)
                .put_with(update_team, update_team_docs)
                .delete_with(delete_team, delete_team_docs),
        )
        .api_route(
            "/{id}/detail",
            get_with(get_team_detail, get_team_detail_docs),
        )
}

/// New Team details.
#[derive(Deserialize, JsonSchema)]
struct CreateTeamRequest {
    /// The name of the new Team.
    name: Option<String>,
    country_id: i64, // The ID of the country the team belongs to
    /// URL path to the team logo image.
    logo_path: Option<String>,
}

/// New Team details.
#[derive(Serialize, JsonSchema)]
struct TeamCreateResponse {
    /// The ID of the new Team.
    id: i64,
}

async fn create_team(
    Extension(ctx): Extension<ApiContext>,
    Json(request): Json<CreateTeamRequest>,
) -> impl IntoApiResponse {
    // 🎯 Route Handler: Only HTTP concerns
    match TeamBusinessLogic::create_team(&ctx, request.name, request.country_id, request.logo_path)
        .await
    {
        Ok(team_id) => Ok((
            StatusCode::CREATED,
            Json(TeamCreateResponse { id: team_id }),
        )),
        Err(app_error) => Err(app_error.into_response()),
    }
}

fn create_team_docs(op: TransformOperation) -> TransformOperation {
    op.description("Create a new team.")
        .tag("team")
        .response::<201, Json<TeamCreateResponse>>()
}

async fn list_teams(
    Extension(ctx): Extension<ApiContext>,
    Query(params): Query<TeamQueryParams>,
) -> impl IntoApiResponse {
    let filters = TeamFilters::new(params.name, params.country_id);
    let paging = Paging::new(params.page.unwrap_or(1), params.page_size.unwrap_or(15));

    match TeamBusinessLogic::list_teams(&ctx, filters, paging).await {
        Ok(response) => Ok(Json(response)),
        Err(app_error) => Err(app_error.into_response()),
    }
}

fn list_teams_docs(op: TransformOperation) -> TransformOperation {
    op.description("List teams with optional filtering and pagination.")
        .tag("team")
        .response::<200, Json<PagedTeamsResponse>>()
}

#[derive(Deserialize, JsonSchema)]
struct SelectTeam {
    /// The ID of the Team.
    id: i64,
}

async fn get_team(
    Extension(ctx): Extension<ApiContext>,
    Path(team): Path<SelectTeam>,
) -> impl IntoApiResponse {
    // 🎯 Route Handler: Only HTTP concerns
    match TeamBusinessLogic::get_team(&ctx, team.id).await {
        Ok(team) => Ok(Json(team)),
        Err(app_error) => Err(app_error.into_response()),
    }
}

fn get_team_docs(op: TransformOperation) -> TransformOperation {
    op.description("Get a single Team.")
        .tag("team")
        .response_with::<200, Json<Team>, _>(|res| {
            res.example(Team {
                name: Some("Example Team".to_string()),
                id: 1,
                country_id: 1,
                country_name: "Example Country".to_string(),
                country_iso2_code: "EX".to_string(),
                logo_path: Some("http://localhost:9000/hockey-uploads/team-logo.jpg".to_string()),
                created_at: "2024-01-01T00:00:00Z".to_string(),
                updated_at: "2024-01-01T00:00:00Z".to_string(),
            })
        })
        .response_with::<404, (), _>(|res| res.description("team was not found"))
}

/// Update Team details.
#[derive(Deserialize, JsonSchema)]
struct UpdateTeamRequest {
    /// The name of the team.
    name: Option<String>,
    /// The country this team represents.
    country_id: i64,
    /// URL path to the team logo image.
    logo_path: Option<String>,
}

async fn update_team(
    Extension(ctx): Extension<ApiContext>,
    Path(team): Path<SelectTeam>,
    Json(update_data): Json<UpdateTeamRequest>,
) -> impl IntoApiResponse {
    match TeamBusinessLogic::update_team(
        &ctx,
        team.id,
        update_data.name,
        update_data.country_id,
        update_data.logo_path,
    )
    .await
    {
        Ok(_) => Ok((StatusCode::OK, Json("Updated".to_string()))),
        Err(app_error) => Err(app_error.into_response()),
    }
}

fn update_team_docs(op: TransformOperation) -> TransformOperation {
    op.description("Update a Team.")
        .tag("team")
        .response_with::<200, (), _>(|res| res.description("The team has been updated."))
        .response_with::<404, (), _>(|res| res.description("The team was not found"))
}

async fn delete_team(
    Extension(ctx): Extension<ApiContext>,
    Path(team): Path<SelectTeam>,
) -> impl IntoApiResponse {
    match TeamBusinessLogic::delete_team(&ctx, team.id).await {
        Ok(()) => Ok((StatusCode::OK, Json("Deleted".to_string()))),
        Err(err) => Err(err.into_response()),
    }
}

fn delete_team_docs(op: TransformOperation) -> TransformOperation {
    op.description("Delete a Team.")
        .tag("team")
        .response_with::<204, (), _>(|res| res.description("The team has been deleted."))
        .response_with::<404, (), _>(|res| res.description("The team was not found"))
}

async fn list_teams_simple(Extension(ctx): Extension<ApiContext>) -> impl IntoApiResponse {
    match TeamBusinessLogic::get_teams_list(&ctx).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => Err(err.into_response()),
    }
}

fn list_teams_simple_docs(op: TransformOperation) -> TransformOperation {
    op.description("List all teams in a simple format for dropdowns.")
        .tag("team")
        .response::<200, Json<Vec<TeamList>>>()
}

/// Get team detail with participations and roster
async fn get_team_detail(
    Extension(ctx): Extension<ApiContext>,
    Path(id): Path<i64>,
) -> impl IntoApiResponse {
    match TeamBusinessLogic::get_team_detail(&ctx, id).await {
        Ok(Some(team_detail)) => Ok(Json(team_detail)),
        Ok(None) => {
            let not_found_error = AppError::TeamNotFound { id };
            Err(not_found_error.into_response())
        }
        Err(app_error) => Err(app_error.into_response()),
    }
}

fn get_team_detail_docs(op: TransformOperation) -> TransformOperation {
    op.description("Get team detail with participations and roster by season.")
        .tag("team")
        .response::<200, Json<TeamDetail>>()
        .response::<404, Json<String>>()
}
