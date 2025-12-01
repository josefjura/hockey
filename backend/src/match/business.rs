use crate::{
    common::paging::{PagedResponse, Paging},
    errors::AppError,
    http::ApiContext,
    r#match::{
        service::{
            self, CreateMatchEntity, CreateScoreEventEntity, MatchFilters, UpdateMatchEntity,
        },
        Match, MatchWithStats, ScoreEvent,
    },
};

/// Business logic layer for match operations
/// No HTTP concerns, only business rules and data transformations
pub struct MatchBusinessLogic;

impl MatchBusinessLogic {
    /// Get a single match by ID
    pub async fn get_match(ctx: &ApiContext, match_id: i64) -> Result<Match, AppError> {
        let match_data = service::get_match_by_id(&ctx.db, match_id).await?;

        let match_entity = match_data.ok_or_else(|| AppError::MatchNotFound { id: match_id })?;

        Ok(Match {
            id: match_entity.id,
            season_id: match_entity.season_id,
            home_team_id: match_entity.home_team_id,
            away_team_id: match_entity.away_team_id,
            home_score_unidentified: match_entity.home_score_unidentified,
            away_score_unidentified: match_entity.away_score_unidentified,
            home_score_total: match_entity.home_score_total,
            away_score_total: match_entity.away_score_total,
            match_date: match_entity.match_date,
            status: match_entity.status,
            venue: match_entity.venue,
            season_name: match_entity.season_name,
            home_team_name: match_entity.home_team_name,
            away_team_name: match_entity.away_team_name,
        })
    }

    /// Create a new match with validation
    pub async fn create_match(
        ctx: &ApiContext,
        season_id: i64,
        home_team_id: i64,
        away_team_id: i64,
        home_score_unidentified: i32,
        away_score_unidentified: i32,
        match_date: Option<String>,
        status: Option<String>,
        venue: Option<String>,
    ) -> Result<i64, AppError> {
        // Business validation rules
        if season_id <= 0 {
            return Err(AppError::invalid_input("Season ID must be positive"));
        }

        if home_team_id <= 0 {
            return Err(AppError::invalid_input("Home team ID must be positive"));
        }

        if away_team_id <= 0 {
            return Err(AppError::invalid_input("Away team ID must be positive"));
        }

        if home_team_id == away_team_id {
            return Err(AppError::invalid_input(
                "Home team and away team must be different",
            ));
        }

        if home_score_unidentified < 0 {
            return Err(AppError::invalid_input("Home score cannot be negative"));
        }

        if away_score_unidentified < 0 {
            return Err(AppError::invalid_input("Away score cannot be negative"));
        }

        // Match date validation (basic ISO format check)
        if let Some(ref date) = match_date {
            if date.trim().is_empty() {
                return Err(AppError::invalid_input("Match date cannot be empty"));
            }
            // Basic ISO date format validation (YYYY-MM-DD or YYYY-MM-DDTHH:MM:SS)
            if !date.contains('-') || date.len() < 10 {
                return Err(AppError::invalid_input(
                    "Match date must be in ISO 8601 format",
                ));
            }
        }

        // Status validation
        let validated_status = match status.as_deref() {
            Some("") => return Err(AppError::invalid_input("Match status cannot be empty")),
            Some(s)
                if ![
                    "scheduled",
                    "in_progress",
                    "finished",
                    "cancelled",
                    "postponed",
                ]
                .contains(&s) =>
            {
                return Err(AppError::invalid_input("Invalid match status"));
            }
            Some(s) => s.to_string(),
            None => "scheduled".to_string(), // Default status
        };

        // Venue validation
        if let Some(ref v) = venue {
            if v.trim().is_empty() {
                return Err(AppError::invalid_input("Venue cannot be empty"));
            }
            if v.len() > 200 {
                return Err(AppError::invalid_input(
                    "Venue name cannot exceed 200 characters",
                ));
            }
        }

        let match_id = service::create_match(
            &ctx.db,
            CreateMatchEntity {
                season_id,
                home_team_id,
                away_team_id,
                home_score_unidentified,
                away_score_unidentified,
                match_date,
                status: Some(validated_status),
                venue,
            },
        )
        .await?;

        Ok(match_id)
    }

    /// List matches with filtering and pagination
    pub async fn list_matches(
        ctx: &ApiContext,
        filters: MatchFilters,
        paging: Paging,
    ) -> Result<PagedResponse<Match>, AppError> {
        let result = service::get_matches(&ctx.db, &filters, Some(&paging)).await?;

        let matches: Vec<Match> = result
            .items
            .into_iter()
            .map(|match_entity| Match {
                id: match_entity.id,
                season_id: match_entity.season_id,
                home_team_id: match_entity.home_team_id,
                away_team_id: match_entity.away_team_id,
                home_score_unidentified: match_entity.home_score_unidentified,
                away_score_unidentified: match_entity.away_score_unidentified,
                home_score_total: match_entity.home_score_total,
                away_score_total: match_entity.away_score_total,
                match_date: match_entity.match_date,
                status: match_entity.status,
                venue: match_entity.venue,
                season_name: match_entity.season_name,
                home_team_name: match_entity.home_team_name,
                away_team_name: match_entity.away_team_name,
            })
            .collect();

        Ok(PagedResponse {
            items: matches,
            total: result.total,
            page: result.page,
            page_size: result.page_size,
            total_pages: result.total_pages,
            has_next: result.has_next,
            has_previous: result.has_previous,
        })
    }

    /// Update a match with validation
    pub async fn update_match(
        ctx: &ApiContext,
        match_id: i64,
        season_id: Option<i64>,
        home_team_id: Option<i64>,
        away_team_id: Option<i64>,
        home_score_unidentified: Option<i32>,
        away_score_unidentified: Option<i32>,
        match_date: Option<String>,
        status: Option<String>,
        venue: Option<String>,
    ) -> Result<bool, AppError> {
        // Check match exists first
        let _existing_match = Self::get_match(ctx, match_id).await?;

        // Validate input if provided
        if let Some(sid) = season_id {
            if sid <= 0 {
                return Err(AppError::invalid_input("Season ID must be positive"));
            }
        }

        if let Some(htid) = home_team_id {
            if htid <= 0 {
                return Err(AppError::invalid_input("Home team ID must be positive"));
            }
        }

        if let Some(atid) = away_team_id {
            if atid <= 0 {
                return Err(AppError::invalid_input("Away team ID must be positive"));
            }
        }

        // Check that home and away teams are different (if both are provided)
        if let (Some(htid), Some(atid)) = (home_team_id, away_team_id) {
            if htid == atid {
                return Err(AppError::invalid_input(
                    "Home team and away team must be different",
                ));
            }
        }

        if let Some(score) = home_score_unidentified {
            if score < 0 {
                return Err(AppError::invalid_input("Home score cannot be negative"));
            }
        }

        if let Some(score) = away_score_unidentified {
            if score < 0 {
                return Err(AppError::invalid_input("Away score cannot be negative"));
            }
        }

        // Status validation
        if let Some(ref s) = status {
            if s.trim().is_empty() {
                return Err(AppError::invalid_input("Match status cannot be empty"));
            }
            if ![
                "scheduled",
                "in_progress",
                "finished",
                "cancelled",
                "postponed",
            ]
            .contains(&s.as_str())
            {
                return Err(AppError::invalid_input("Invalid match status"));
            }
        }

        // Venue validation
        if let Some(ref v) = venue {
            if v.trim().is_empty() {
                return Err(AppError::invalid_input("Venue cannot be empty"));
            }
            if v.len() > 200 {
                return Err(AppError::invalid_input(
                    "Venue name cannot exceed 200 characters",
                ));
            }
        }

        let updated = service::update_match(
            &ctx.db,
            match_id,
            UpdateMatchEntity {
                season_id,
                home_team_id,
                away_team_id,
                home_score_unidentified,
                away_score_unidentified,
                match_date,
                status,
                venue,
            },
        )
        .await?;

        if !updated {
            return Err(AppError::MatchNotFound { id: match_id });
        }

        Ok(updated)
    }

    /// Delete a match
    pub async fn delete_match(ctx: &ApiContext, match_id: i64) -> Result<(), AppError> {
        let deleted = service::delete_match(&ctx.db, match_id).await?;

        if !deleted {
            return Err(AppError::MatchNotFound { id: match_id });
        }

        Ok(())
    }

    /// Get match with calculated statistics
    pub async fn get_match_with_stats(
        ctx: &ApiContext,
        match_id: i64,
    ) -> Result<MatchWithStats, AppError> {
        let match_stats = service::get_match_with_stats(&ctx.db, match_id).await?;

        let (match_data, home_total, away_total, home_detailed, away_detailed) =
            match_stats.ok_or_else(|| AppError::MatchNotFound { id: match_id })?;

        Ok(MatchWithStats {
            match_info: Match {
                id: match_data.id,
                season_id: match_data.season_id,
                home_team_id: match_data.home_team_id,
                away_team_id: match_data.away_team_id,
                home_score_unidentified: match_data.home_score_unidentified,
                away_score_unidentified: match_data.away_score_unidentified,
                home_score_total: match_data.home_score_total,
                away_score_total: match_data.away_score_total,
                match_date: match_data.match_date,
                status: match_data.status,
                venue: match_data.venue,
                season_name: match_data.season_name,
                home_team_name: match_data.home_team_name,
                away_team_name: match_data.away_team_name,
            },
            home_total_score: home_total,
            away_total_score: away_total,
            home_detailed_goals: home_detailed,
            away_detailed_goals: away_detailed,
        })
    }

    /// Create a score event with validation
    pub async fn create_score_event(
        ctx: &ApiContext,
        match_id: i64,
        team_id: i64,
        scorer_id: Option<i64>,
        assist1_id: Option<i64>,
        assist2_id: Option<i64>,
        period: Option<i32>,
        time_minutes: Option<i32>,
        time_seconds: Option<i32>,
        goal_type: Option<String>,
    ) -> Result<i64, AppError> {
        // Validate match exists
        let _match = Self::get_match(ctx, match_id).await?;

        if team_id <= 0 {
            return Err(AppError::invalid_input("Team ID must be positive"));
        }

        // Period validation
        if let Some(p) = period {
            if p < 1 || p > 5 {
                return Err(AppError::invalid_input(
                    "Period must be between 1 and 5 (1-3 regular, 4=OT, 5=SO)",
                ));
            }
        }

        // Time validation
        if let Some(minutes) = time_minutes {
            if minutes < 0 || minutes > 60 {
                return Err(AppError::invalid_input(
                    "Time minutes must be between 0 and 60",
                ));
            }
        }

        if let Some(seconds) = time_seconds {
            if seconds < 0 || seconds >= 60 {
                return Err(AppError::invalid_input(
                    "Time seconds must be between 0 and 59",
                ));
            }
        }

        // Goal type validation
        if let Some(ref gt) = goal_type {
            let valid_types = [
                "even_strength",
                "power_play",
                "short_handed",
                "penalty_shot",
                "empty_net",
            ];
            if !valid_types.contains(&gt.as_str()) {
                return Err(AppError::invalid_input("Invalid goal type"));
            }
        }

        // Validate that assist players are different from scorer and each other
        if let (Some(scorer), Some(assist1)) = (scorer_id, assist1_id) {
            if scorer == assist1 {
                return Err(AppError::invalid_input(
                    "Scorer and first assist cannot be the same player",
                ));
            }
        }

        if let (Some(scorer), Some(assist2)) = (scorer_id, assist2_id) {
            if scorer == assist2 {
                return Err(AppError::invalid_input(
                    "Scorer and second assist cannot be the same player",
                ));
            }
        }

        if let (Some(assist1), Some(assist2)) = (assist1_id, assist2_id) {
            if assist1 == assist2 {
                return Err(AppError::invalid_input(
                    "First and second assist cannot be the same player",
                ));
            }
        }

        let event_id = service::create_score_event(
            &ctx.db,
            CreateScoreEventEntity {
                match_id,
                team_id,
                scorer_id,
                assist1_id,
                assist2_id,
                period,
                time_minutes,
                time_seconds,
                goal_type,
            },
        )
        .await?;

        Ok(event_id)
    }

    /// Get score events for a match
    pub async fn get_score_events(
        ctx: &ApiContext,
        match_id: i64,
    ) -> Result<Vec<ScoreEvent>, AppError> {
        // Validate match exists
        let _match = Self::get_match(ctx, match_id).await?;

        let events = service::get_score_events_for_match(&ctx.db, match_id).await?;

        let score_events: Vec<ScoreEvent> = events
            .into_iter()
            .map(|event| ScoreEvent {
                id: event.id,
                match_id: event.match_id,
                team_id: event.team_id,
                scorer_id: event.scorer_id,
                assist1_id: event.assist1_id,
                assist2_id: event.assist2_id,
                period: event.period,
                time_minutes: event.time_minutes,
                time_seconds: event.time_seconds,
                goal_type: event.goal_type,
                scorer_name: event.scorer_name,
                assist1_name: event.assist1_name,
                assist2_name: event.assist2_name,
            })
            .collect();

        Ok(score_events)
    }

    /// Get a single score event by ID
    pub async fn get_score_event(ctx: &ApiContext, event_id: i64) -> Result<ScoreEvent, AppError> {
        let event = service::get_score_event_by_id(&ctx.db, event_id).await?;

        match event {
            Some(event) => Ok(ScoreEvent {
                id: event.id,
                match_id: event.match_id,
                team_id: event.team_id,
                scorer_id: event.scorer_id,
                assist1_id: event.assist1_id,
                assist2_id: event.assist2_id,
                period: event.period,
                time_minutes: event.time_minutes,
                time_seconds: event.time_seconds,
                goal_type: event.goal_type,
                scorer_name: event.scorer_name,
                assist1_name: event.assist1_name,
                assist2_name: event.assist2_name,
            }),
            None => Err(AppError::ScoreEventNotFound { id: event_id }),
        }
    }

    /// Delete a score event
    pub async fn delete_score_event(
        ctx: &ApiContext,
        match_id: i64,
        event_id: i64,
    ) -> Result<(), AppError> {
        // Validate match exists
        let _match = Self::get_match(ctx, match_id).await?;

        let deleted = service::delete_score_event(&ctx.db, event_id).await?;

        if !deleted {
            return Err(AppError::ScoreEventNotFound { id: event_id });
        }

        Ok(())
    }
}
