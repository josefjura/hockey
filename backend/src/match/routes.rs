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
use crate::r#match::business::MatchBusinessLogic;
use crate::r#match::service::{self, CreateScoreEventEntity, MatchFilters};
use crate::r#match::{Match, MatchWithStats, ScoreEvent};

// Use generic PagedResponse for matches
type PagedMatchesResponse = PagedResponse<Match>;

/// Query parameters for filtering matches
#[derive(Deserialize, Serialize, JsonSchema)]
struct MatchQueryParams {
    /// Filter by season ID
    season_id: Option<i64>,
    /// Filter by team ID (either home or away)
    team_id: Option<i64>,
    /// Filter by match status
    status: Option<String>,
    /// Filter matches from this date (ISO format)
    date_from: Option<String>,
    /// Filter matches to this date (ISO format)
    date_to: Option<String>,
    /// Page number (1-based)
    page: Option<usize>,
    /// Number of items per page
    page_size: Option<usize>,
}

pub fn match_routes() -> ApiRouter {
    ApiRouter::new()
        .api_route(
            "/",
            post_with(create_match, create_match_docs).get_with(list_matches, list_matches_docs),
        )
        .api_route(
            "/{id}",
            get_with(get_match, get_match_docs)
                .put_with(update_match, update_match_docs)
                .delete_with(delete_match, delete_match_docs),
        )
        .api_route(
            "/{id}/stats",
            get_with(get_match_with_stats, get_match_with_stats_docs),
        )
        .api_route(
            "/{id}/score-events",
            post_with(create_score_event, create_score_event_docs)
                .get_with(list_score_events, list_score_events_docs),
        )
        .api_route(
            "/{match_id}/score-events/{event_id}",
            get_with(get_score_event, get_score_event_docs)
                .delete_with(delete_score_event, delete_score_event_docs),
        )
        .api_route(
            "/{id}/identify-goal",
            post_with(identify_goal, identify_goal_docs),
        )
}

/// New Match details.
#[derive(Deserialize, JsonSchema)]
struct CreateMatchRequest {
    /// The season this match belongs to.
    season_id: i64,
    /// The home team ID.
    home_team_id: i64,
    /// The away team ID.
    away_team_id: i64,
    /// Number of unidentified goals for the home team.
    #[serde(default)]
    home_score_unidentified: i32,
    /// Number of unidentified goals for the away team.
    #[serde(default)]
    away_score_unidentified: i32,
    /// The date and time of the match (ISO 8601 format).
    match_date: Option<String>,
    /// The current status of the match.
    status: Option<String>,
    /// The venue where the match is played.
    venue: Option<String>,
}

/// New Match response.
#[derive(Serialize, JsonSchema)]
struct MatchCreateResponse {
    /// The ID of the new Match.
    id: i64,
}

async fn create_match(
    Extension(ctx): Extension<ApiContext>,
    Json(match_data): Json<CreateMatchRequest>,
) -> impl IntoApiResponse {
    match MatchBusinessLogic::create_match(
        &ctx,
        match_data.season_id,
        match_data.home_team_id,
        match_data.away_team_id,
        match_data.home_score_unidentified,
        match_data.away_score_unidentified,
        match_data.match_date,
        match_data.status,
        match_data.venue,
    )
    .await
    {
        Ok(result) => Ok((
            StatusCode::CREATED,
            Json(MatchCreateResponse { id: result }),
        )),
        Err(app_error) => Err(app_error.into_response()),
    }
}

fn create_match_docs(op: TransformOperation) -> TransformOperation {
    op.description("Create a new match.")
        .tag("match")
        .response::<201, Json<MatchCreateResponse>>()
}

async fn list_matches(
    Extension(ctx): Extension<ApiContext>,
    Query(params): Query<MatchQueryParams>,
) -> impl IntoApiResponse {
    let filters = MatchFilters::new(
        params.season_id,
        params.team_id,
        params.status,
        params.date_from,
        params.date_to,
    );
    let paging = Paging::new(params.page.unwrap_or(1), params.page_size.unwrap_or(15));

    match MatchBusinessLogic::list_matches(&ctx, filters, paging).await {
        Ok(response) => Ok(Json(response)),
        Err(app_error) => Err(app_error.into_response()),
    }
}

fn list_matches_docs(op: TransformOperation) -> TransformOperation {
    op.description("List matches with optional filtering and pagination.")
        .tag("match")
        .response::<200, Json<PagedMatchesResponse>>()
}

#[derive(Deserialize, JsonSchema)]
struct SelectMatch {
    /// The ID of the Match.
    id: i64,
}

async fn get_match(
    Extension(ctx): Extension<ApiContext>,
    Path(match_): Path<SelectMatch>,
) -> impl IntoApiResponse {
    match MatchBusinessLogic::get_match(&ctx, match_.id).await {
        Ok(result) => Ok(Json(result)),
        Err(app_error) => Err(app_error.into_response()),
    }
}

fn get_match_docs(op: TransformOperation) -> TransformOperation {
    op.description("Get a single Match.")
        .tag("match")
        .response_with::<200, Json<Match>, _>(|res| {
            res.example(Match {
                id: 1,
                season_id: 1,
                home_team_id: 1,
                away_team_id: 2,
                home_score_unidentified: 2,
                away_score_unidentified: 1,
                home_score_total: 3,
                away_score_total: 2,
                match_date: Some("2024-03-15T19:00:00Z".to_string()),
                status: "finished".to_string(),
                venue: Some("Hockey Arena".to_string()),
                season_name: "2023-2024".to_string(),
                home_team_name: "Team A".to_string(),
                away_team_name: "Team B".to_string(),
            })
        })
        .response_with::<404, (), _>(|res| res.description("match was not found"))
}

/// Update Match details.
#[derive(Deserialize, JsonSchema)]
struct UpdateMatchRequest {
    /// The season this match belongs to.
    season_id: Option<i64>,
    /// The home team ID.
    home_team_id: Option<i64>,
    /// The away team ID.
    away_team_id: Option<i64>,
    /// Number of unidentified goals for the home team.
    home_score_unidentified: Option<i32>,
    /// Number of unidentified goals for the away team.
    away_score_unidentified: Option<i32>,
    /// The date and time of the match (ISO 8601 format).
    match_date: Option<String>,
    /// The current status of the match.
    status: Option<String>,
    /// The venue where the match is played.
    venue: Option<String>,
}

async fn update_match(
    Extension(ctx): Extension<ApiContext>,
    Path(match_): Path<SelectMatch>,
    Json(update_data): Json<UpdateMatchRequest>,
) -> impl IntoApiResponse {
    match MatchBusinessLogic::update_match(
        &ctx,
        match_.id,
        update_data.season_id,
        update_data.home_team_id,
        update_data.away_team_id,
        update_data.home_score_unidentified,
        update_data.away_score_unidentified,
        update_data.match_date,
        update_data.status,
        update_data.venue,
    )
    .await
    {
        Ok(_) => Ok((StatusCode::OK, Json("Updated".to_string()))),
        Err(app_error) => Err(app_error.into_response()),
    }
}

fn update_match_docs(op: TransformOperation) -> TransformOperation {
    op.description("Update a Match.")
        .tag("match")
        .response_with::<200, (), _>(|res| res.description("The match has been updated."))
        .response_with::<404, (), _>(|res| res.description("The match was not found"))
}

async fn delete_match(
    Extension(ctx): Extension<ApiContext>,
    Path(match_): Path<SelectMatch>,
) -> impl IntoApiResponse {
    match MatchBusinessLogic::delete_match(&ctx, match_.id).await {
        Ok(()) => Ok((StatusCode::NO_CONTENT, Json("Deleted".to_string()))),
        Err(app_error) => Err(app_error.into_response()),
    }
}

fn delete_match_docs(op: TransformOperation) -> TransformOperation {
    op.description("Delete a Match.")
        .tag("match")
        .response_with::<204, (), _>(|res| res.description("The match has been deleted."))
        .response_with::<404, (), _>(|res| res.description("The match was not found"))
}

async fn get_match_with_stats(
    Extension(ctx): Extension<ApiContext>,
    Path(match_): Path<SelectMatch>,
) -> impl IntoApiResponse {
    match MatchBusinessLogic::get_match_with_stats(&ctx, match_.id).await {
        Ok(result) => Ok(Json(result)),
        Err(app_error) => Err(app_error.into_response()),
    }
}

fn get_match_with_stats_docs(op: TransformOperation) -> TransformOperation {
    op.description("Get a match with calculated statistics.")
        .tag("match")
        .response::<200, Json<MatchWithStats>>()
        .response_with::<404, (), _>(|res| res.description("match was not found"))
}

// Score Event endpoints

/// New Score Event details.
#[derive(Deserialize, JsonSchema)]
struct CreateScoreEventRequest {
    /// The team that scored.
    team_id: i64,
    /// The player who scored (can be null for unknown scorer).
    scorer_id: Option<i64>,
    /// The first assist (can be null).
    assist1_id: Option<i64>,
    /// The second assist (can be null).
    assist2_id: Option<i64>,
    /// The period when the goal was scored (1, 2, 3, 4=OT, 5=SO).
    period: Option<i32>,
    /// The minute within the period.
    time_minutes: Option<i32>,
    /// The seconds within the minute.
    time_seconds: Option<i32>,
    /// The type of goal (even_strength, power_play, etc.).
    goal_type: Option<String>,
}

/// New Score Event response.
#[derive(Serialize, JsonSchema)]
struct ScoreEventCreateResponse {
    /// The ID of the new Score Event.
    id: i64,
}

async fn create_score_event(
    Extension(ctx): Extension<ApiContext>,
    Path(match_): Path<SelectMatch>,
    Json(event_data): Json<CreateScoreEventRequest>,
) -> impl IntoApiResponse {
    match MatchBusinessLogic::create_score_event(
        &ctx,
        match_.id,
        event_data.team_id,
        event_data.scorer_id,
        event_data.assist1_id,
        event_data.assist2_id,
        event_data.period,
        event_data.time_minutes,
        event_data.time_seconds,
        event_data.goal_type,
    )
    .await
    {
        Ok(result) => Ok((
            StatusCode::CREATED,
            Json(ScoreEventCreateResponse { id: result }),
        )),
        Err(app_error) => Err(app_error.into_response()),
    }
}

fn create_score_event_docs(op: TransformOperation) -> TransformOperation {
    op.description("Create a new score event for a match.")
        .tag("match")
        .response::<201, Json<ScoreEventCreateResponse>>()
}

async fn list_score_events(
    Extension(ctx): Extension<ApiContext>,
    Path(match_): Path<SelectMatch>,
) -> impl IntoApiResponse {
    match MatchBusinessLogic::get_score_events(&ctx, match_.id).await {
        Ok(events) => Ok(Json(events)),
        Err(app_error) => Err(app_error.into_response()),
    }
}

fn list_score_events_docs(op: TransformOperation) -> TransformOperation {
    op.description("List all score events for a match.")
        .tag("match")
        .response::<200, Json<Vec<ScoreEvent>>>()
}

#[derive(Deserialize, JsonSchema)]
struct SelectScoreEvent {
    /// The ID of the Match.
    #[allow(dead_code)]
    match_id: i64,
    /// The ID of the Score Event.
    event_id: i64,
}

async fn get_score_event(
    Extension(ctx): Extension<ApiContext>,
    Path(params): Path<SelectScoreEvent>,
) -> impl IntoApiResponse {
    match MatchBusinessLogic::get_score_event(&ctx, params.event_id).await {
        Ok(result) => Ok(Json(result)),
        Err(app_error) => Err(app_error.into_response()),
    }
}

fn get_score_event_docs(op: TransformOperation) -> TransformOperation {
    op.description("Get a single Score Event.")
        .tag("match")
        .response::<200, Json<ScoreEvent>>()
        .response_with::<404, (), _>(|res| res.description("score event was not found"))
}

async fn delete_score_event(
    Extension(ctx): Extension<ApiContext>,
    Path(params): Path<SelectScoreEvent>,
) -> impl IntoApiResponse {
    match MatchBusinessLogic::delete_score_event(&ctx, params.match_id, params.event_id).await {
        Ok(()) => Ok((StatusCode::NO_CONTENT, Json("Deleted".to_string()))),
        Err(app_error) => Err(app_error.into_response()),
    }
}

fn delete_score_event_docs(op: TransformOperation) -> TransformOperation {
    op.description("Delete a Score Event.")
        .tag("match")
        .response_with::<204, (), _>(|res| res.description("The score event has been deleted."))
        .response_with::<404, (), _>(|res| res.description("The score event was not found"))
}

/// Identify Goal request.
#[derive(Deserialize, JsonSchema)]
struct IdentifyGoalRequest {
    /// The team that scored.
    team_id: i64,
    /// The player who scored (can be null for unknown scorer).
    scorer_id: Option<i64>,
    /// The first assist (can be null).
    assist1_id: Option<i64>,
    /// The second assist (can be null).
    assist2_id: Option<i64>,
    /// The period when the goal was scored (1, 2, 3, 4=OT, 5=SO).
    period: Option<i32>,
    /// The minute within the period.
    time_minutes: Option<i32>,
    /// The seconds within the minute.
    time_seconds: Option<i32>,
    /// The type of goal (even_strength, power_play, etc.).
    goal_type: Option<String>,
}

async fn identify_goal(
    Extension(ctx): Extension<ApiContext>,
    Path(match_): Path<SelectMatch>,
    Json(goal_data): Json<IdentifyGoalRequest>,
) -> impl IntoApiResponse {
    match service::identify_goal(
        &ctx.db,
        match_.id,
        goal_data.team_id,
        CreateScoreEventEntity {
            match_id: match_.id,
            team_id: goal_data.team_id,
            scorer_id: goal_data.scorer_id,
            assist1_id: goal_data.assist1_id,
            assist2_id: goal_data.assist2_id,
            period: goal_data.period,
            time_minutes: goal_data.time_minutes,
            time_seconds: goal_data.time_seconds,
            goal_type: goal_data.goal_type,
        },
    )
    .await
    {
        Ok(result) => Ok((
            StatusCode::CREATED,
            Json(ScoreEventCreateResponse { id: result }),
        )),
        Err(sqlx::Error::ColumnNotFound(_)) => Err((
            StatusCode::BAD_REQUEST,
            Json("No unidentified goals available for this team".into()),
        )),
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(err.to_string()))),
    }
}

fn identify_goal_docs(op: TransformOperation) -> TransformOperation {
    op.description("Convert an unidentified goal to a detailed score event.")
        .tag("match")
        .response::<201, Json<ScoreEventCreateResponse>>()
        .response_with::<400, (), _>(|res| {
            res.description("No unidentified goals available for this team")
        })
}
