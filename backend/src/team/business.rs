use crate::{
    common::paging::{PagedResponse, Paging},
    errors::AppError,
    http::ApiContext,
    team::{
        Team, TeamDetail, TeamList,
        service::{self, CreateTeamEntity, TeamFilters, UpdateTeamEntity},
    },
};

/// Business logic layer - pure domain operations
/// No HTTP concerns, only business rules and data transformations
pub struct TeamBusinessLogic;

impl TeamBusinessLogic {
    /// Get a single team by ID
    pub async fn get_team(ctx: &ApiContext, team_id: i64) -> Result<Team, AppError> {
        let team_data = service::get_team_by_id(&ctx.db, team_id).await?;

        let team = team_data.ok_or_else(|| AppError::team_not_found(team_id))?;

        Ok(Team {
            id: team.id,
            name: team.name,
            country_id: team.country_id,
            country_name: team.country_name,
            country_iso2_code: team.country_iso2_code,
            logo_path: team.logo_path,
            created_at: team.created_at,
            updated_at: team.updated_at,
        })
    }

    /// Create a new team with validation
    pub async fn create_team(
        ctx: &ApiContext,
        name: Option<String>,
        country_id: i64,
        logo_path: Option<String>,
    ) -> Result<i64, AppError> {
        // Business validation rules
        if let Some(ref name) = name {
            if name.trim().is_empty() {
                return Err(AppError::invalid_input("Team name cannot be empty"));
            }
            if name.len() > 100 {
                return Err(AppError::invalid_input(
                    "Team name cannot exceed 100 characters",
                ));
            }
        }

        // Check if country exists (example business rule)
        if country_id <= 0 {
            return Err(AppError::invalid_input("Country ID must be positive"));
        }

        // Logo path validation
        if let Some(ref logo) = logo_path {
            if !logo.starts_with("http://")
                && !logo.starts_with("https://")
                && !logo.starts_with("/")
            {
                return Err(AppError::invalid_input(
                    "Logo path must be a valid URL or absolute path",
                ));
            }
        }

        let team_id = service::create_team(
            &ctx.db,
            CreateTeamEntity {
                name,
                country_id,
                logo_path,
            },
        )
        .await?;

        Ok(team_id)
    }

    /// List teams with filtering and pagination
    pub async fn list_teams(
        ctx: &ApiContext,
        filters: TeamFilters,
        paging: Paging,
    ) -> Result<PagedResponse<Team>, AppError> {
        let result = service::get_teams(&ctx.db, &filters, Some(&paging)).await?;

        let teams: Vec<Team> = result
            .items
            .into_iter()
            .map(|team| Team {
                id: team.id,
                name: team.name,
                country_id: team.country_id,
                country_name: team.country_name,
                country_iso2_code: team.country_iso2_code,
                logo_path: team.logo_path,
                created_at: team.created_at,
                updated_at: team.updated_at,
            })
            .collect();

        Ok(PagedResponse {
            items: teams,
            total: result.total,
            page: result.page,
            page_size: result.page_size,
            total_pages: result.total_pages,
            has_next: result.has_next,
            has_previous: result.has_previous,
        })
    }

    /// Update a team with validation
    pub async fn update_team(
        ctx: &ApiContext,
        team_id: i64,
        name: Option<String>,
        country_id: i64,
        logo_path: Option<String>,
    ) -> Result<bool, AppError> {
        // Check team exists first
        let _existing_team = Self::get_team(ctx, team_id).await?;

        // Same validation as create
        if let Some(ref name) = name {
            if name.trim().is_empty() {
                return Err(AppError::invalid_input("Team name cannot be empty"));
            }
            if name.len() > 100 {
                return Err(AppError::invalid_input(
                    "Team name cannot exceed 100 characters",
                ));
            }
        }

        if country_id <= 0 {
            return Err(AppError::invalid_input("Country ID must be positive"));
        }

        let updated = service::update_team(
            &ctx.db,
            team_id,
            UpdateTeamEntity {
                name,
                country_id,
                logo_path,
            },
        )
        .await?;

        if !updated {
            return Err(AppError::team_not_found(team_id));
        }

        Ok(updated)
    }

    /// Delete a team
    pub async fn delete_team(ctx: &ApiContext, team_id: i64) -> Result<(), AppError> {
        let deleted = service::delete_team(&ctx.db, team_id).await?;

        if !deleted {
            return Err(AppError::team_not_found(team_id));
        }

        Ok(())
    }

    /// Get simplified team list for dropdowns
    pub async fn get_teams_list(ctx: &ApiContext) -> Result<Vec<TeamList>, AppError> {
        let teams = service::get_teams_list(&ctx.db).await?;

        let team_list: Vec<TeamList> = teams
            .into_iter()
            .map(|team| TeamList {
                id: team.id,
                name: team.name,
            })
            .collect();

        Ok(team_list)
    }

    pub async fn get_team_detail(
        ctx: &ApiContext,
        team_id: i64,
    ) -> Result<Option<TeamDetail>, AppError> {
        let team_detail = service::get_team_detail(&ctx.db, team_id).await?;
        Ok(team_detail)
    }
}
