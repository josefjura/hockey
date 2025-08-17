use crate::{
    common::paging::{PagedResult, Paging},
    errors::AppError,
    http::ApiContext,
    season::{PlayerDropdown, Season, SeasonList, service},
};

/// Business logic layer for season operations
///
/// This layer implements domain validation rules and business logic before
/// delegating to the service layer for persistence operations.
pub struct SeasonBusinessLogic;

impl SeasonBusinessLogic {
    /// Get a season by ID with full validation
    ///
    /// # Arguments
    /// * `ctx` - API context containing database connection
    /// * `season_id` - The ID of the season to retrieve
    ///
    /// # Returns
    /// * `Ok(Season)` - The season with event information
    /// * `Err(AppError::SeasonNotFound)` - If no season exists with the given ID
    /// * `Err(AppError::Database)` - If there's a database error
    pub async fn get_season(ctx: &ApiContext, season_id: i64) -> Result<Season, AppError> {
        // Validate input
        if season_id <= 0 {
            return Err(AppError::invalid_input("Season ID must be positive"));
        }

        // Call service layer
        match service::get_season_by_id(&ctx.db, season_id).await {
            Ok(Some(season_entity)) => {
                // Convert service entity to domain model
                Ok(Season {
                    id: season_entity.id,
                    year: season_entity.year,
                    display_name: season_entity.display_name,
                    event_id: season_entity.event_id,
                    event_name: season_entity.event_name,
                    created_at: season_entity.created_at,
                    updated_at: season_entity.updated_at,
                })
            }
            Ok(None) => Err(AppError::season_not_found(season_id)),
            Err(e) => Err(AppError::Database(e)),
        }
    }

    /// Create a new season with validation
    ///
    /// # Arguments
    /// * `ctx` - API context containing database connection
    /// * `year` - Season year (must be between 1900 and 2100)
    /// * `display_name` - Optional display name (if provided, must not be empty)
    /// * `event_id` - Event ID (must be positive)
    ///
    /// # Returns
    /// * `Ok(i64)` - The ID of the created season
    /// * `Err(AppError::InvalidInput)` - If validation fails
    /// * `Err(AppError::Database)` - If there's a database error
    pub async fn create_season(
        ctx: &ApiContext,
        year: i64,
        display_name: Option<String>,
        event_id: i64,
    ) -> Result<i64, AppError> {
        // Validate input
        Self::validate_season_data(year, &display_name, event_id)?;

        // Call service layer
        let create_entity = service::CreateSeasonEntity {
            year,
            display_name,
            event_id,
        };

        match service::create_season(&ctx.db, create_entity).await {
            Ok(season_id) => Ok(season_id),
            Err(e) => Err(AppError::Database(e)),
        }
    }

    /// List seasons with filtering and pagination
    ///
    /// # Arguments
    /// * `ctx` - API context containing database connection
    /// * `year_filter` - Optional year filter
    /// * `event_id_filter` - Optional event ID filter
    /// * `paging` - Optional paging parameters
    ///
    /// # Returns
    /// * `Ok(PagedResult<Season>)` - Paginated list of seasons
    /// * `Err(AppError::Database)` - If there's a database error
    pub async fn list_seasons(
        ctx: &ApiContext,
        year_filter: Option<i64>,
        event_id_filter: Option<i64>,
        paging: Option<&Paging>,
    ) -> Result<PagedResult<Season>, AppError> {
        // Validate filters
        if let Some(year) = year_filter {
            if year < 1900 || year > 2100 {
                return Err(AppError::invalid_input(
                    "Year filter must be between 1900 and 2100",
                ));
            }
        }

        if let Some(event_id) = event_id_filter {
            if event_id <= 0 {
                return Err(AppError::invalid_input("Event ID filter must be positive"));
            }
        }

        // Build filters
        let filters = service::SeasonFilters::new(year_filter, event_id_filter);

        // Call service layer
        match service::get_seasons(&ctx.db, &filters, paging).await {
            Ok(paged_result) => {
                // Convert service entities to domain models
                let seasons = paged_result
                    .items
                    .into_iter()
                    .map(|season_entity| Season {
                        id: season_entity.id,
                        year: season_entity.year,
                        display_name: season_entity.display_name,
                        event_id: season_entity.event_id,
                        event_name: season_entity.event_name,
                        created_at: season_entity.created_at,
                        updated_at: season_entity.updated_at,
                    })
                    .collect();

                Ok(PagedResult {
                    items: seasons,
                    total: paged_result.total,
                    page: paged_result.page,
                    page_size: paged_result.page_size,
                    total_pages: paged_result.total_pages,
                    has_next: paged_result.has_next,
                    has_previous: paged_result.has_previous,
                })
            }
            Err(e) => Err(AppError::Database(e)),
        }
    }

    /// Update an existing season with validation
    ///
    /// # Arguments
    /// * `ctx` - API context containing database connection
    /// * `season_id` - The ID of the season to update
    /// * `year` - Updated season year (must be between 1900 and 2100)
    /// * `display_name` - Updated display name (if provided, must not be empty)
    /// * `event_id` - Updated event ID (must be positive)
    ///
    /// # Returns
    /// * `Ok(Season)` - The updated season
    /// * `Err(AppError::SeasonNotFound)` - If no season exists with the given ID
    /// * `Err(AppError::InvalidInput)` - If validation fails
    /// * `Err(AppError::Database)` - If there's a database error
    pub async fn update_season(
        ctx: &ApiContext,
        season_id: i64,
        year: i64,
        display_name: Option<String>,
        event_id: i64,
    ) -> Result<Season, AppError> {
        // Validate season ID
        if season_id <= 0 {
            return Err(AppError::invalid_input("Season ID must be positive"));
        }

        // Validate season data
        Self::validate_season_data(year, &display_name, event_id)?;

        // Check if season exists
        Self::get_season(ctx, season_id).await?;

        // Call service layer
        let update_entity = service::UpdateSeasonEntity {
            year,
            display_name,
            event_id,
        };

        match service::update_season(&ctx.db, season_id, update_entity).await {
            Ok(true) => {
                // Return updated season
                Self::get_season(ctx, season_id).await
            }
            Ok(false) => Err(AppError::season_not_found(season_id)),
            Err(e) => Err(AppError::Database(e)),
        }
    }

    /// Delete a season by ID
    ///
    /// # Arguments
    /// * `ctx` - API context containing database connection
    /// * `season_id` - The ID of the season to delete
    ///
    /// # Returns
    /// * `Ok(())` - If the season was successfully deleted
    /// * `Err(AppError::SeasonNotFound)` - If no season exists with the given ID
    /// * `Err(AppError::Database)` - If there's a database error
    pub async fn delete_season(ctx: &ApiContext, season_id: i64) -> Result<(), AppError> {
        // Validate season ID
        if season_id <= 0 {
            return Err(AppError::invalid_input("Season ID must be positive"));
        }

        // Check if season exists
        Self::get_season(ctx, season_id).await?;

        // Call service layer
        match service::delete_season(&ctx.db, season_id).await {
            Ok(true) => Ok(()),
            Ok(false) => Err(AppError::season_not_found(season_id)),
            Err(e) => Err(AppError::Database(e)),
        }
    }

    /// Get seasons list for dropdowns and UI components
    ///
    /// # Arguments
    /// * `ctx` - API context containing database connection
    ///
    /// # Returns
    /// * `Ok(Vec<SeasonList>)` - List of seasons for UI components
    /// * `Err(AppError::Database)` - If there's a database error
    pub async fn get_seasons_list(ctx: &ApiContext) -> Result<Vec<SeasonList>, AppError> {
        match service::get_seasons_list(&ctx.db).await {
            Ok(season_items) => {
                let seasons = season_items
                    .into_iter()
                    .map(|item| SeasonList {
                        id: item.id,
                        name: item.name,
                        year: item.year,
                        event_name: item.event_name,
                    })
                    .collect();
                Ok(seasons)
            }
            Err(e) => Err(AppError::Database(e)),
        }
    }

    /// Get players for a specific team in a season
    ///
    /// # Arguments
    /// * `ctx` - API context containing database connection
    /// * `season_id` - The season ID (must be positive)
    /// * `team_id` - The team ID (must be positive)
    ///
    /// # Returns
    /// * `Ok(Vec<PlayerDropdown>)` - List of players for the team in the season
    /// * `Err(AppError::InvalidInput)` - If validation fails
    /// * `Err(AppError::Database)` - If there's a database error
    pub async fn get_players_for_team_in_season(
        ctx: &ApiContext,
        season_id: i64,
        team_id: i64,
    ) -> Result<Vec<PlayerDropdown>, AppError> {
        // Validate inputs
        if season_id <= 0 {
            return Err(AppError::invalid_input("Season ID must be positive"));
        }

        if team_id <= 0 {
            return Err(AppError::invalid_input("Team ID must be positive"));
        }

        // Call service layer
        match service::get_players_for_team_in_season(&ctx.db, season_id, team_id).await {
            Ok(player_items) => {
                let players = player_items
                    .into_iter()
                    .map(|item| PlayerDropdown {
                        id: item.id,
                        name: item.name,
                        nationality: item.nationality,
                    })
                    .collect();
                Ok(players)
            }
            Err(e) => Err(AppError::Database(e)),
        }
    }

    /// Validate season data according to business rules
    ///
    /// # Arguments
    /// * `year` - Season year to validate
    /// * `display_name` - Optional display name to validate
    /// * `event_id` - Event ID to validate
    ///
    /// # Returns
    /// * `Ok(())` - If all validation passes
    /// * `Err(AppError::InvalidInput)` - If any validation fails
    fn validate_season_data(
        year: i64,
        display_name: &Option<String>,
        event_id: i64,
    ) -> Result<(), AppError> {
        // Validate year
        if year < 1900 || year > 2100 {
            return Err(AppError::invalid_input(
                "Season year must be between 1900 and 2100",
            ));
        }

        // Validate display name if provided
        if let Some(display_name) = display_name {
            if display_name.trim().is_empty() {
                return Err(AppError::invalid_input(
                    "Display name cannot be empty if provided",
                ));
            }

            if display_name.len() > 100 {
                return Err(AppError::invalid_input(
                    "Display name cannot exceed 100 characters",
                ));
            }
        }

        // Validate event ID
        if event_id <= 0 {
            return Err(AppError::invalid_input("Event ID must be positive"));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_season_data_success() {
        assert!(
            SeasonBusinessLogic::validate_season_data(2024, &Some("Test Season".to_string()), 1)
                .is_ok()
        );

        assert!(SeasonBusinessLogic::validate_season_data(2024, &None, 1).is_ok());

        assert!(SeasonBusinessLogic::validate_season_data(1900, &None, 1).is_ok());
        assert!(SeasonBusinessLogic::validate_season_data(2100, &None, 1).is_ok());
    }

    #[test]
    fn test_validate_season_data_failures() {
        // Invalid year (too early)
        assert!(SeasonBusinessLogic::validate_season_data(1899, &None, 1).is_err());

        // Invalid year (too late)
        assert!(SeasonBusinessLogic::validate_season_data(2101, &None, 1).is_err());

        // Empty display name
        assert!(SeasonBusinessLogic::validate_season_data(2024, &Some("".to_string()), 1).is_err());
        assert!(
            SeasonBusinessLogic::validate_season_data(2024, &Some("   ".to_string()), 1).is_err()
        );

        // Long display name
        let long_name = "A".repeat(101);
        assert!(SeasonBusinessLogic::validate_season_data(2024, &Some(long_name), 1).is_err());

        // Invalid event ID
        assert!(SeasonBusinessLogic::validate_season_data(2024, &None, 0).is_err());
        assert!(SeasonBusinessLogic::validate_season_data(2024, &None, -1).is_err());
    }
}
