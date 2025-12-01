use crate::{
    errors::AppError,
    http::ApiContext,
    team_participation::{
        service::{self, CreateTeamParticipationEntity},
        TeamParticipation,
    },
};

pub struct TeamParticipationBusinessLogic;

impl TeamParticipationBusinessLogic {
    /// Create a new team participation with validation
    pub async fn create_team_participation(
        ctx: &ApiContext,
        team_id: i64,
        season_id: i64,
    ) -> Result<i64, AppError> {
        // Validate team exists
        let _team = crate::team::service::get_team_by_id(&ctx.db, team_id).await?;
        if _team.is_none() {
            return Err(AppError::TeamNotFound { id: team_id });
        }

        // Validate season exists
        let _season = crate::season::service::get_season_by_id(&ctx.db, season_id).await?;
        if _season.is_none() {
            return Err(AppError::SeasonNotFound { id: season_id });
        }

        // Create the team participation
        let participation_id = service::create_team_participation(
            &ctx.db,
            CreateTeamParticipationEntity { team_id, season_id },
        )
        .await?;

        Ok(participation_id)
    }

    /// Get a single team participation by ID
    pub async fn get_team_participation(
        ctx: &ApiContext,
        participation_id: i64,
    ) -> Result<TeamParticipation, AppError> {
        let participation =
            service::get_team_participation_by_id(&ctx.db, participation_id).await?;

        match participation {
            Some(participation) => Ok(TeamParticipation {
                id: participation.id,
                team_id: participation.team_id,
                season_id: participation.season_id,
            }),
            None => Err(AppError::TeamParticipationNotFound {
                id: participation_id,
            }),
        }
    }

    /// List all team participations
    pub async fn list_team_participations(
        ctx: &ApiContext,
    ) -> Result<Vec<TeamParticipation>, AppError> {
        let participations = service::get_team_participation(&ctx.db).await?;

        let team_participations: Vec<TeamParticipation> = participations
            .into_iter()
            .map(|participation| TeamParticipation {
                id: participation.id,
                team_id: participation.team_id,
                season_id: participation.season_id,
            })
            .collect();

        Ok(team_participations)
    }

    /// Find or create a team participation (special business logic)
    pub async fn find_or_create_team_participation(
        ctx: &ApiContext,
        team_id: i64,
        season_id: i64,
    ) -> Result<i64, AppError> {
        // Validate team exists
        let _team = crate::team::service::get_team_by_id(&ctx.db, team_id).await?;
        if _team.is_none() {
            return Err(AppError::TeamNotFound { id: team_id });
        }

        // Validate season exists
        let _season = crate::season::service::get_season_by_id(&ctx.db, season_id).await?;
        if _season.is_none() {
            return Err(AppError::SeasonNotFound { id: season_id });
        }

        // Find or create the participation
        let participation_id =
            service::find_or_create_team_participation(&ctx.db, season_id, team_id).await?;

        Ok(participation_id)
    }

    /// Delete a team participation
    pub async fn delete_team_participation(
        ctx: &ApiContext,
        participation_id: i64,
    ) -> Result<(), AppError> {
        let deleted = service::delete_team_participation(&ctx.db, participation_id).await?;

        if !deleted {
            return Err(AppError::InvalidInput {
                message: format!("Team participation with id {} not found", participation_id),
            });
        }

        Ok(())
    }
}
