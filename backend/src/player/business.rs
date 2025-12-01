use crate::{
    common::paging::{PagedResult, Paging},
    errors::AppError,
    http::ApiContext,
    player::{service, Player},
};

/// Business logic layer for player operations
///
/// This layer implements domain validation rules and business logic before
/// delegating to the service layer for persistence operations.
pub struct PlayerBusinessLogic;

impl PlayerBusinessLogic {
    /// Get a player by ID with full validation
    ///
    /// # Arguments
    /// * `ctx` - API context containing database connection
    /// * `player_id` - The ID of the player to retrieve
    ///
    /// # Returns
    /// * `Ok(Player)` - The player with country information
    /// * `Err(AppError::PlayerNotFound)` - If no player exists with the given ID
    /// * `Err(AppError::Database)` - If there's a database error
    pub async fn get_player(ctx: &ApiContext, player_id: i64) -> Result<Player, AppError> {
        // Validate input
        if player_id <= 0 {
            return Err(AppError::invalid_input("Player ID must be positive"));
        }

        // Call service layer
        match service::get_player_by_id(&ctx.db, player_id).await {
            Ok(Some(player_entity)) => {
                // Convert service entity to domain model
                Ok(Player {
                    id: player_entity.id,
                    name: player_entity.name,
                    country_id: player_entity.country_id,
                    country_name: player_entity.country_name,
                    country_iso2_code: player_entity.country_iso2_code,
                    photo_path: player_entity.photo_path,
                    created_at: player_entity.created_at,
                    updated_at: player_entity.updated_at,
                })
            }
            Ok(None) => Err(AppError::player_not_found(player_id)),
            Err(e) => Err(AppError::Database(e)),
        }
    }

    /// Create a new player with validation
    ///
    /// # Arguments
    /// * `ctx` - API context containing database connection
    /// * `name` - Player name (required, must not be empty)
    /// * `country_id` - Country ID (must be positive)
    /// * `photo_path` - Optional photo path (if provided, must not be empty)
    ///
    /// # Returns
    /// * `Ok(i64)` - The ID of the created player
    /// * `Err(AppError::InvalidInput)` - If validation fails
    /// * `Err(AppError::Database)` - If there's a database error
    pub async fn create_player(
        ctx: &ApiContext,
        name: String,
        country_id: i64,
        photo_path: Option<String>,
    ) -> Result<i64, AppError> {
        // Validate input
        Self::validate_player_data(&name, country_id, &photo_path)?;

        // Call service layer
        let create_entity = service::CreatePlayerEntity {
            name,
            country_id,
            photo_path,
        };

        match service::create_player(&ctx.db, create_entity).await {
            Ok(player_id) => Ok(player_id),
            Err(e) => Err(AppError::Database(e)),
        }
    }

    /// List players with filtering and pagination
    ///
    /// # Arguments
    /// * `ctx` - API context containing database connection
    /// * `name_filter` - Optional name filter (partial match)
    /// * `country_id_filter` - Optional country ID filter
    /// * `paging` - Optional paging parameters
    ///
    /// # Returns
    /// * `Ok(PagedResult<Player>)` - Paginated list of players
    /// * `Err(AppError::Database)` - If there's a database error
    pub async fn list_players(
        ctx: &ApiContext,
        name_filter: Option<String>,
        country_id_filter: Option<i64>,
        paging: Option<&Paging>,
    ) -> Result<PagedResult<Player>, AppError> {
        // Validate filters
        if let Some(country_id) = country_id_filter {
            if country_id <= 0 {
                return Err(AppError::invalid_input(
                    "Country ID filter must be positive",
                ));
            }
        }

        if let Some(ref name) = name_filter {
            if name.trim().is_empty() {
                return Err(AppError::invalid_input("Name filter cannot be empty"));
            }
        }

        // Build filters
        let filters = service::PlayerFilters::new(name_filter, country_id_filter);

        // Call service layer
        match service::get_players(&ctx.db, &filters, paging).await {
            Ok(paged_result) => {
                // Convert service entities to domain models
                let players = paged_result
                    .items
                    .into_iter()
                    .map(|player_entity| Player {
                        id: player_entity.id,
                        name: player_entity.name,
                        country_id: player_entity.country_id,
                        country_name: player_entity.country_name,
                        country_iso2_code: player_entity.country_iso2_code,
                        photo_path: player_entity.photo_path,
                        created_at: player_entity.created_at,
                        updated_at: player_entity.updated_at,
                    })
                    .collect();

                Ok(PagedResult {
                    items: players,
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

    /// Update an existing player with validation
    ///
    /// # Arguments
    /// * `ctx` - API context containing database connection
    /// * `player_id` - The ID of the player to update
    /// * `name` - Updated player name (required, must not be empty)
    /// * `country_id` - Updated country ID (must be positive)
    /// * `photo_path` - Updated photo path (if provided, must not be empty)
    ///
    /// # Returns
    /// * `Ok(Player)` - The updated player
    /// * `Err(AppError::PlayerNotFound)` - If no player exists with the given ID
    /// * `Err(AppError::InvalidInput)` - If validation fails
    /// * `Err(AppError::Database)` - If there's a database error
    pub async fn update_player(
        ctx: &ApiContext,
        player_id: i64,
        name: String,
        country_id: i64,
        photo_path: Option<String>,
    ) -> Result<Player, AppError> {
        // Validate player ID
        if player_id <= 0 {
            return Err(AppError::invalid_input("Player ID must be positive"));
        }

        // Validate player data
        Self::validate_player_data(&name, country_id, &photo_path)?;

        // Check if player exists
        Self::get_player(ctx, player_id).await?;

        // Call service layer
        let update_entity = service::UpdatePlayerEntity {
            name,
            country_id,
            photo_path,
        };

        match service::update_player(&ctx.db, player_id, update_entity).await {
            Ok(true) => {
                // Return updated player
                Self::get_player(ctx, player_id).await
            }
            Ok(false) => Err(AppError::player_not_found(player_id)),
            Err(e) => Err(AppError::Database(e)),
        }
    }

    /// Delete a player by ID
    ///
    /// # Arguments
    /// * `ctx` - API context containing database connection
    /// * `player_id` - The ID of the player to delete
    ///
    /// # Returns
    /// * `Ok(())` - If the player was successfully deleted
    /// * `Err(AppError::PlayerNotFound)` - If no player exists with the given ID
    /// * `Err(AppError::Database)` - If there's a database error
    pub async fn delete_player(ctx: &ApiContext, player_id: i64) -> Result<(), AppError> {
        // Validate player ID
        if player_id <= 0 {
            return Err(AppError::invalid_input("Player ID must be positive"));
        }

        // Check if player exists
        Self::get_player(ctx, player_id).await?;

        // Call service layer
        match service::delete_player(&ctx.db, player_id).await {
            Ok(true) => Ok(()),
            Ok(false) => Err(AppError::player_not_found(player_id)),
            Err(e) => Err(AppError::Database(e)),
        }
    }

    /// Validate player data according to business rules
    ///
    /// # Arguments
    /// * `name` - Player name to validate
    /// * `country_id` - Country ID to validate
    /// * `photo_path` - Optional photo path to validate
    ///
    /// # Returns
    /// * `Ok(())` - If all validation passes
    /// * `Err(AppError::InvalidInput)` - If any validation fails
    fn validate_player_data(
        name: &str,
        country_id: i64,
        photo_path: &Option<String>,
    ) -> Result<(), AppError> {
        // Validate name
        if name.trim().is_empty() {
            return Err(AppError::invalid_input("Player name cannot be empty"));
        }

        if name.len() > 100 {
            return Err(AppError::invalid_input(
                "Player name cannot exceed 100 characters",
            ));
        }

        // Validate country ID
        if country_id <= 0 {
            return Err(AppError::invalid_input("Country ID must be positive"));
        }

        // Validate photo path if provided
        if let Some(photo_path) = photo_path {
            if photo_path.trim().is_empty() {
                return Err(AppError::invalid_input(
                    "Photo path cannot be empty if provided",
                ));
            }

            if photo_path.len() > 255 {
                return Err(AppError::invalid_input(
                    "Photo path cannot exceed 255 characters",
                ));
            }

            // Basic URL validation - should start with / or http(s)://
            if !photo_path.starts_with('/')
                && !photo_path.starts_with("http://")
                && !photo_path.starts_with("https://")
            {
                return Err(AppError::invalid_input(
                    "Photo path must be a valid path starting with '/' or a valid URL starting with 'http://' or 'https://'",
                ));
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_player_data_success() {
        assert!(PlayerBusinessLogic::validate_player_data(
            "Test Player",
            1,
            &Some("/images/player.jpg".to_string())
        )
        .is_ok());

        assert!(PlayerBusinessLogic::validate_player_data("Test Player", 1, &None).is_ok());

        assert!(PlayerBusinessLogic::validate_player_data(
            "Test Player",
            1,
            &Some("https://example.com/player.jpg".to_string())
        )
        .is_ok());
    }

    #[test]
    fn test_validate_player_data_failures() {
        // Empty name
        assert!(PlayerBusinessLogic::validate_player_data("", 1, &None).is_err());
        assert!(PlayerBusinessLogic::validate_player_data("   ", 1, &None).is_err());

        // Long name
        let long_name = "A".repeat(101);
        assert!(PlayerBusinessLogic::validate_player_data(&long_name, 1, &None).is_err());

        // Invalid country ID
        assert!(PlayerBusinessLogic::validate_player_data("Test Player", 0, &None).is_err());
        assert!(PlayerBusinessLogic::validate_player_data("Test Player", -1, &None).is_err());

        // Empty photo path
        assert!(
            PlayerBusinessLogic::validate_player_data("Test Player", 1, &Some("".to_string()))
                .is_err()
        );
        assert!(PlayerBusinessLogic::validate_player_data(
            "Test Player",
            1,
            &Some("   ".to_string())
        )
        .is_err());

        // Long photo path
        let long_path = "A".repeat(256);
        assert!(
            PlayerBusinessLogic::validate_player_data("Test Player", 1, &Some(long_path)).is_err()
        );

        // Invalid photo path format
        assert!(PlayerBusinessLogic::validate_player_data(
            "Test Player",
            1,
            &Some("invalid-path".to_string())
        )
        .is_err());
    }
}
