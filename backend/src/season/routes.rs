use aide::{
    axum::{
        routing::{get_with, post_with},
        ApiRouter, IntoApiResponse,
    },
    transform::TransformOperation,
};
use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::IntoResponse,
    Extension, Json,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::common::paging::{PagedResponse, Paging};
use crate::http::ApiContext;
use crate::season::business::SeasonBusinessLogic;

use super::{PlayerDropdown, Season, SeasonList};

// Use generic PagedResponse for seasons
type PagedSeasonsResponse = PagedResponse<Season>;

/// Query parameters for filtering seasons
#[derive(Deserialize, Serialize, JsonSchema)]
struct SeasonQueryParams {
    /// Filter by year
    year: Option<i64>,
    /// Filter by event ID
    event_id: Option<i64>,
    /// Page number (1-based)
    page: Option<usize>,
    /// Number of items per page
    page_size: Option<usize>,
}

pub fn season_routes() -> ApiRouter {
    ApiRouter::new()
        .api_route(
            "/",
            post_with(create_season, create_season_docs).get_with(list_seasons, list_seasons_docs),
        )
        .api_route(
            "/list",
            get_with(list_seasons_simple, list_seasons_simple_docs),
        )
        .api_route(
            "/{season_id}/team/{team_id}/players",
            get_with(get_season_team_players, get_season_team_players_docs),
        )
        .api_route(
            "/{id}",
            get_with(get_season, get_season_docs)
                .put_with(update_season, update_season_docs)
                .delete_with(delete_season, delete_season_docs),
        )
}

/// New Season details.
#[derive(Deserialize, JsonSchema)]
struct CreateSeasonRequest {
    /// The year of the season.
    year: i64,
    /// The display name of the season.
    display_name: Option<String>,
    /// The event this season belongs to.
    event_id: i64,
}

/// New Season details.
#[derive(Serialize, JsonSchema)]
struct SeasonCreateResponse {
    /// The ID of the new Season.
    id: i64,
}

async fn create_season(
    Extension(ctx): Extension<ApiContext>,
    Json(season): Json<CreateSeasonRequest>,
) -> impl IntoApiResponse {
    match SeasonBusinessLogic::create_season(
        &ctx,
        season.year,
        season.display_name,
        season.event_id,
    )
    .await
    {
        Ok(result) => Ok((
            StatusCode::CREATED,
            Json(SeasonCreateResponse { id: result }),
        )),
        Err(app_error) => Err(app_error.into_response()),
    }
}

fn create_season_docs(op: TransformOperation) -> TransformOperation {
    op.description("Create a new season.")
        .tag("season")
        .response::<201, Json<SeasonCreateResponse>>()
}

async fn list_seasons(
    Extension(ctx): Extension<ApiContext>,
    Query(params): Query<SeasonQueryParams>,
) -> impl IntoApiResponse {
    let paging = Some(Paging::new(
        params.page.unwrap_or(1),
        params.page_size.unwrap_or(15),
    ));

    match SeasonBusinessLogic::list_seasons(&ctx, params.year, params.event_id, paging.as_ref())
        .await
    {
        Ok(result) => {
            let response = PagedSeasonsResponse::from_result(result.clone(), result.items);
            Ok(Json(response))
        }
        Err(app_error) => Err(app_error.into_response()),
    }
}

fn list_seasons_docs(op: TransformOperation) -> TransformOperation {
    op.description("List seasons with optional filtering and pagination.")
        .tag("season")
        .response::<200, Json<PagedSeasonsResponse>>()
}

#[derive(Deserialize, JsonSchema)]
struct SelectSeason {
    /// The ID of the Season.
    id: i64,
}

async fn get_season(
    Extension(ctx): Extension<ApiContext>,
    Path(season): Path<SelectSeason>,
) -> impl IntoApiResponse {
    match SeasonBusinessLogic::get_season(&ctx, season.id).await {
        Ok(result) => Ok(Json(result)),
        Err(app_error) => Err(app_error.into_response()),
    }
}

fn get_season_docs(op: TransformOperation) -> TransformOperation {
    op.description("Get a single Season.")
        .tag("season")
        .response_with::<200, Json<Season>, _>(|res| {
            res.example(Season {
                id: 1,
                year: 2023,
                display_name: Some("Example Season".to_string()),
                event_id: 1,
                event_name: "Example Event".to_string(),
                created_at: "2024-01-01T00:00:00Z".to_string(),
                updated_at: "2024-01-01T00:00:00Z".to_string(),
            })
        })
        .response_with::<404, (), _>(|res| res.description("season was not found"))
}

/// Update Season details.
#[derive(Deserialize, JsonSchema)]
struct UpdateSeasonRequest {
    /// The year of the season.
    year: i64,
    /// The display name of the season.
    display_name: Option<String>,
    /// The event this season belongs to.
    event_id: i64,
}

async fn update_season(
    Extension(ctx): Extension<ApiContext>,
    Path(season): Path<SelectSeason>,
    Json(update_data): Json<UpdateSeasonRequest>,
) -> impl IntoApiResponse {
    match SeasonBusinessLogic::update_season(
        &ctx,
        season.id,
        update_data.year,
        update_data.display_name,
        update_data.event_id,
    )
    .await
    {
        Ok(updated_season) => Ok(Json(updated_season)),
        Err(app_error) => Err(app_error.into_response()),
    }
}

fn update_season_docs(op: TransformOperation) -> TransformOperation {
    op.description("Update a Season.")
        .tag("season")
        .response_with::<200, (), _>(|res| res.description("The season has been updated."))
        .response_with::<404, (), _>(|res| res.description("The season was not found"))
}

async fn delete_season(
    Extension(ctx): Extension<ApiContext>,
    Path(season): Path<SelectSeason>,
) -> impl IntoApiResponse {
    match SeasonBusinessLogic::delete_season(&ctx, season.id).await {
        Ok(()) => Ok((StatusCode::NO_CONTENT, Json("Deleted".to_string()))),
        Err(app_error) => Err(app_error.into_response()),
    }
}

fn delete_season_docs(op: TransformOperation) -> TransformOperation {
    op.description("Delete a Season.")
        .tag("season")
        .response_with::<204, (), _>(|res| res.description("The season has been deleted."))
        .response_with::<404, (), _>(|res| res.description("The season was not found"))
}

async fn list_seasons_simple(Extension(ctx): Extension<ApiContext>) -> impl IntoApiResponse {
    match SeasonBusinessLogic::get_seasons_list(&ctx).await {
        Ok(result) => Ok(Json(result)),
        Err(app_error) => Err(app_error.into_response()),
    }
}

fn list_seasons_simple_docs(op: TransformOperation) -> TransformOperation {
    op.description("List all seasons in a simple format for dropdowns.")
        .tag("season")
        .response::<200, Json<Vec<SeasonList>>>()
}

#[derive(Deserialize, JsonSchema)]
struct SeasonTeamParams {
    /// The ID of the season.
    season_id: i64,
    /// The ID of the team.
    team_id: i64,
}

async fn get_season_team_players(
    Extension(ctx): Extension<ApiContext>,
    Path(params): Path<SeasonTeamParams>,
) -> impl IntoApiResponse {
    match SeasonBusinessLogic::get_players_for_team_in_season(
        &ctx,
        params.season_id,
        params.team_id,
    )
    .await
    {
        Ok(result) => Ok(Json(result)),
        Err(app_error) => Err(app_error.into_response()),
    }
}

fn get_season_team_players_docs(op: TransformOperation) -> TransformOperation {
    op.description("Get players for a specific team in a specific season.")
        .tag("season")
        .response::<200, Json<Vec<PlayerDropdown>>>()
}
