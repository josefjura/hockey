use sqlx::SqlitePool;

use crate::service::matches::{self, CreateMatchEntity, UpdateMatchEntity};

/// Business logic validation errors for match operations
#[derive(Debug, Clone)]
pub enum MatchValidationError {
    /// Home and away teams are the same
    SameTeams,
    /// Score values are negative
    NegativeScores,
    /// Teams don't participate in the selected season
    TeamsNotInSeason,
    /// Database error during validation
    DatabaseError,
}

impl MatchValidationError {
    /// Get user-friendly error message
    pub fn message(&self) -> &'static str {
        match self {
            MatchValidationError::SameTeams => "Home and away teams must be different",
            MatchValidationError::NegativeScores => "Scores cannot be negative",
            MatchValidationError::TeamsNotInSeason => {
                "Both teams must participate in the selected season"
            }
            MatchValidationError::DatabaseError => "Failed to validate team participation",
        }
    }
}

/// Validates match form data
///
/// # Arguments
/// * `db` - Database connection pool
/// * `season_id` - Season ID
/// * `home_team_id` - Home team ID
/// * `away_team_id` - Away team ID
/// * `home_score_unidentified` - Home team score
/// * `away_score_unidentified` - Away team score
///
/// # Returns
/// * `Ok(())` - If validation passes
/// * `Err(MatchValidationError)` - If validation fails
async fn validate_match_form(
    db: &SqlitePool,
    season_id: i64,
    home_team_id: i64,
    away_team_id: i64,
    home_score_unidentified: i32,
    away_score_unidentified: i32,
) -> Result<(), MatchValidationError> {
    // Validate teams are different
    if home_team_id == away_team_id {
        return Err(MatchValidationError::SameTeams);
    }

    // Validate scores are non-negative
    if home_score_unidentified < 0 || away_score_unidentified < 0 {
        return Err(MatchValidationError::NegativeScores);
    }

    // Validate teams participate in season
    match matches::validate_teams_in_season(db, season_id, home_team_id, away_team_id).await {
        Ok(true) => Ok(()),
        Ok(false) => Err(MatchValidationError::TeamsNotInSeason),
        Err(e) => {
            tracing::error!("Failed to validate teams in season: {}", e);
            Err(MatchValidationError::DatabaseError)
        }
    }
}

/// Creates a new match with validation
///
/// # Arguments
/// * `db` - Database connection pool
/// * `entity` - Match entity to create
///
/// # Returns
/// * `Ok(i64)` - ID of created match
/// * `Err(MatchValidationError)` - If validation fails
/// * `Err(sqlx::Error)` - If database operation fails
pub async fn create_match_validated(
    db: &SqlitePool,
    entity: CreateMatchEntity,
) -> Result<i64, Result<MatchValidationError, sqlx::Error>> {
    // Validate match data
    validate_match_form(
        db,
        entity.season_id,
        entity.home_team_id,
        entity.away_team_id,
        entity.home_score_unidentified,
        entity.away_score_unidentified,
    )
    .await
    .map_err(Ok)?;

    // Create match
    matches::create_match(db, entity).await.map_err(Err)
}

/// Updates an existing match with validation
///
/// # Arguments
/// * `db` - Database connection pool
/// * `id` - Match ID to update
/// * `entity` - Updated match entity
///
/// # Returns
/// * `Ok(bool)` - true if match was updated, false if not found
/// * `Err(MatchValidationError)` - If validation fails
/// * `Err(sqlx::Error)` - If database operation fails
pub async fn update_match_validated(
    db: &SqlitePool,
    id: i64,
    entity: UpdateMatchEntity,
) -> Result<bool, Result<MatchValidationError, sqlx::Error>> {
    // Validate match data
    validate_match_form(
        db,
        entity.season_id,
        entity.home_team_id,
        entity.away_team_id,
        entity.home_score_unidentified,
        entity.away_score_unidentified,
    )
    .await
    .map_err(Ok)?;

    // Update match
    matches::update_match(db, id, entity).await.map_err(Err)
}
