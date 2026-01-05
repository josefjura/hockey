use sqlx::SqlitePool;

use crate::common::pagination::{PagedResult, SortOrder};
use crate::routes::players::forms::PlayerFormData;
use crate::service::players::{
    self, CreatePlayerEntity, PlayerDetailEntity, PlayerEntity, PlayerEventStatsEntity,
    PlayerScoringEventEntity, PlayerScoringFilters, PlayerSeasonStats, PropertyChangeEntity,
    ScoringEventSortField, UpdatePlayerEntity,
};
use crate::validation::validate_name;

/// Bundled player detail page data
///
/// This struct encapsulates all the data needed to render a player detail page,
/// fetched from multiple sources in the database.
#[derive(Debug)]
pub struct PlayerDetailPageData {
    /// Player basic information and contracts
    pub detail: PlayerDetailEntity,
    /// Season-by-season statistics
    pub season_stats: Vec<PlayerSeasonStats>,
    /// Event-aggregated career statistics
    pub event_stats: Vec<PlayerEventStatsEntity>,
    /// Property changes (career timeline)
    pub property_changes: Vec<PropertyChangeEntity>,
}

/// Fetches all data needed for the player detail page
///
/// This function bundles three separate data fetching operations:
/// 1. Player basic info and contracts
/// 2. Season-by-season statistics
/// 3. Event-aggregated career statistics
///
/// # Arguments
/// * `db` - Database connection pool
/// * `player_id` - ID of the player to fetch data for
///
/// # Returns
/// * `Ok(Some(PlayerDetailPageData))` - If player exists and data is fetched successfully
/// * `Ok(None)` - If player doesn't exist
/// * `Err(sqlx::Error)` - If database operation fails
pub async fn get_player_detail_page_data(
    db: &SqlitePool,
    player_id: i64,
) -> Result<Option<PlayerDetailPageData>, sqlx::Error> {
    // Fetch player detail
    let detail = match players::get_player_detail(db, player_id).await? {
        Some(detail) => detail,
        None => return Ok(None),
    };

    // Fetch season statistics (return empty vec on error to maintain partial functionality)
    let season_stats = players::get_player_season_stats(db, player_id)
        .await
        .unwrap_or_default();

    // Fetch event statistics (return empty vec on error to maintain partial functionality)
    let event_stats = players::get_player_event_stats(db, player_id)
        .await
        .unwrap_or_default();

    // Fetch property changes (return empty vec on error to maintain partial functionality)
    let property_changes = players::get_player_property_changes(db, player_id)
        .await
        .unwrap_or_default();

    Ok(Some(PlayerDetailPageData {
        detail,
        season_stats,
        event_stats,
        property_changes,
    }))
}

/// Bundled player scoring page data
///
/// This struct encapsulates all the data needed to render a player scoring page,
/// fetched from multiple sources in the database.
#[derive(Debug)]
pub struct PlayerScoringPageData {
    /// Player basic information
    pub player: PlayerEntity,
    /// Season-by-season statistics
    pub season_stats: Vec<PlayerSeasonStats>,
    /// Scoring events (paginated, filtered, sorted)
    pub scoring_events: PagedResult<PlayerScoringEventEntity>,
    /// Available seasons for filter dropdown
    pub seasons: Vec<(i64, String)>,
    /// Available teams for filter dropdown
    pub teams: Vec<(i64, String)>,
}

/// Fetches all data needed for the player scoring page
///
/// This function bundles five separate data fetching operations:
/// 1. Player basic info
/// 2. Season-by-season statistics
/// 3. Scoring events (with filters, sorting, pagination)
/// 4. Available seasons for filter dropdown
/// 5. Available teams for filter dropdown
///
/// # Arguments
/// * `db` - Database connection pool
/// * `player_id` - ID of the player to fetch data for
/// * `filters` - Filters to apply to scoring events
/// * `sort_field` - Field to sort scoring events by
/// * `sort_order` - Sort order (ascending/descending)
/// * `page` - Page number for pagination
/// * `page_size` - Number of items per page
///
/// # Returns
/// * `Ok(Some(PlayerScoringPageData))` - If player exists and data is fetched successfully
/// * `Ok(None)` - If player doesn't exist
/// * `Err(sqlx::Error)` - If database operation fails
pub async fn get_player_scoring_page_data(
    db: &SqlitePool,
    player_id: i64,
    filters: &PlayerScoringFilters,
    sort_field: &ScoringEventSortField,
    sort_order: &SortOrder,
    page: usize,
    page_size: usize,
) -> Result<Option<PlayerScoringPageData>, sqlx::Error> {
    // Fetch player basic info
    let player = match players::get_player_by_id(db, player_id).await? {
        Some(player) => player,
        None => return Ok(None),
    };

    // Fetch season statistics
    let season_stats = players::get_player_season_stats(db, player_id).await?;

    // Fetch scoring events with filters, sorting, and pagination
    let scoring_events = players::get_player_scoring_events(
        db, player_id, filters, sort_field, sort_order, page, page_size,
    )
    .await?;

    // Fetch filter dropdown data (return empty vec on error to maintain partial functionality)
    let seasons = players::get_player_seasons(db, player_id)
        .await
        .unwrap_or_default();
    let teams = players::get_player_teams(db, player_id)
        .await
        .unwrap_or_default();

    Ok(Some(PlayerScoringPageData {
        player,
        season_stats,
        scoring_events,
        seasons,
        teams,
    }))
}

/// Validation error for player operations
#[derive(Debug, Clone)]
pub enum PlayerValidationError {
    /// Player name validation failed
    InvalidName(&'static str),
    /// Country ID is required but not provided
    MissingCountryId,
}

impl PlayerValidationError {
    /// Get user-friendly error message
    pub fn message(&self) -> &'static str {
        match self {
            PlayerValidationError::InvalidName(msg) => msg,
            PlayerValidationError::MissingCountryId => "Please select a country",
        }
    }
}

/// Validates and creates a new player
///
/// This function validates the player data and creates the player in the database.
///
/// # Arguments
/// * `db` - Database connection pool
/// * `form_data` - Parsed player form data
/// * `photo_path` - Resolved photo path (from file upload or URL)
///
/// # Returns
/// * `Ok(i64)` - ID of created player
/// * `Err(PlayerValidationError)` - If validation fails
/// * `Err(sqlx::Error)` - If database operation fails
pub async fn create_player_validated(
    db: &SqlitePool,
    form_data: &PlayerFormData,
    photo_path: Option<String>,
) -> Result<i64, Result<PlayerValidationError, sqlx::Error>> {
    // Validate name
    let validated_name = validate_name(&form_data.name)
        .map_err(|msg| Ok(PlayerValidationError::InvalidName(msg)))?;

    // Validate country_id is provided
    let country_id = form_data
        .country_id
        .ok_or(Ok(PlayerValidationError::MissingCountryId))?;

    // Create player
    players::create_player(
        db,
        CreatePlayerEntity {
            name: validated_name,
            country_id,
            photo_path,
            birth_date: form_data.birth_date.clone(),
            birth_place: form_data.birth_place.clone(),
            height_cm: form_data.height_cm,
            weight_kg: form_data.weight_kg,
            position: form_data.position.clone(),
            shoots: form_data.shoots.clone(),
        },
    )
    .await
    .map_err(Err)
}

/// Validates and updates an existing player
///
/// This function validates the player data and updates the player in the database.
///
/// # Arguments
/// * `db` - Database connection pool
/// * `id` - Player ID to update
/// * `form_data` - Parsed player form data
/// * `photo_path` - Resolved photo path (from file upload or URL)
///
/// # Returns
/// * `Ok(bool)` - true if player was updated, false if not found
/// * `Err(PlayerValidationError)` - If validation fails
/// * `Err(sqlx::Error)` - If database operation fails
pub async fn update_player_validated(
    db: &SqlitePool,
    id: i64,
    form_data: &PlayerFormData,
    photo_path: Option<String>,
) -> Result<bool, Result<PlayerValidationError, sqlx::Error>> {
    // Validate name
    let validated_name = validate_name(&form_data.name)
        .map_err(|msg| Ok(PlayerValidationError::InvalidName(msg)))?;

    // Validate country_id is provided
    let country_id = form_data
        .country_id
        .ok_or(Ok(PlayerValidationError::MissingCountryId))?;

    // Update player
    players::update_player(
        db,
        id,
        UpdatePlayerEntity {
            name: validated_name,
            country_id,
            photo_path,
            birth_date: form_data.birth_date.clone(),
            birth_place: form_data.birth_place.clone(),
            height_cm: form_data.height_cm,
            weight_kg: form_data.weight_kg,
            position: form_data.position.clone(),
            shoots: form_data.shoots.clone(),
        },
    )
    .await
    .map_err(Err)
}

/// Validates property change data
///
/// # Arguments
/// * `change_date` - Date in ISO 8601 format (YYYY-MM-DD)
/// * `property_type` - Type of property change (must be in allowed list)
/// * `description` - Description of the change
///
/// # Returns
/// * `Ok(())` - If validation passes
/// * `Err(&str)` - Error message if validation fails
pub fn validate_property_change(
    change_date: &str,
    property_type: &str,
    description: &str,
) -> Result<(), &'static str> {
    // Validate date format (basic ISO 8601 check: YYYY-MM-DD)
    if change_date.len() != 10 {
        return Err("Invalid date format. Use YYYY-MM-DD");
    }
    if change_date.chars().nth(4) != Some('-') || change_date.chars().nth(7) != Some('-') {
        return Err("Invalid date format. Use YYYY-MM-DD");
    }

    // Validate property type (must be in predefined list)
    const VALID_TYPES: [&str; 7] = [
        "Position",
        "Trade",
        "Role",
        "JerseyNumber",
        "Status",
        "Retirement",
        "Other",
    ];
    if !VALID_TYPES.contains(&property_type) {
        return Err("Invalid property type");
    }

    // Validate description (similar to validate_name)
    let trimmed = description.trim();
    if trimmed.is_empty() {
        return Err("Description cannot be empty");
    }
    if trimmed.len() > 500 {
        return Err("Description cannot exceed 500 characters");
    }

    Ok(())
}
