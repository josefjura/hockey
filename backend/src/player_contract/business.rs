use crate::{
    errors::AppError,
    http::ApiContext,
    player_contract::{
        PlayerContract,
        service::{self, CreatePlayerContractEntity},
    },
};

pub struct PlayerContractBusinessLogic;

impl PlayerContractBusinessLogic {
    /// Create a new player contract with validation
    pub async fn create_player_contract(
        ctx: &ApiContext,
        team_participation_id: i64,
        player_id: i64,
    ) -> Result<i64, AppError> {
        // Validate team participation exists
        let _team_participation = crate::team_participation::service::get_team_participation_by_id(
            &ctx.db,
            team_participation_id,
        )
        .await?;
        if _team_participation.is_none() {
            return Err(AppError::InvalidInput {
                message: format!(
                    "Team participation with id {} not found",
                    team_participation_id
                ),
            });
        }

        // Validate player exists
        let _player = crate::player::service::get_player_by_id(&ctx.db, player_id).await?;
        if _player.is_none() {
            return Err(AppError::PlayerNotFound { id: player_id });
        }

        // Create the player contract
        let contract_id = service::create_player_contract(
            &ctx.db,
            CreatePlayerContractEntity {
                team_participation_id,
                player_id,
            },
        )
        .await?;

        Ok(contract_id)
    }

    /// Get a single player contract by ID
    pub async fn get_player_contract(
        ctx: &ApiContext,
        contract_id: i64,
    ) -> Result<PlayerContract, AppError> {
        let contract = service::get_player_contract_by_id(&ctx.db, contract_id).await?;

        match contract {
            Some(contract) => Ok(PlayerContract {
                id: contract.id,
                team_participation_id: contract.team_participation_id,
                player_id: contract.player_id,
            }),
            None => Err(AppError::PlayerContractNotFound { id: contract_id }),
        }
    }

    /// List all player contracts
    pub async fn list_player_contracts(ctx: &ApiContext) -> Result<Vec<PlayerContract>, AppError> {
        let contracts = service::get_player_contracts(&ctx.db).await?;

        let player_contracts: Vec<PlayerContract> = contracts
            .into_iter()
            .map(|contract| PlayerContract {
                id: contract.id,
                team_participation_id: contract.team_participation_id,
                player_id: contract.player_id,
            })
            .collect();

        Ok(player_contracts)
    }

    /// Delete a player contract
    pub async fn delete_player_contract(
        ctx: &ApiContext,
        contract_id: i64,
    ) -> Result<(), AppError> {
        let deleted = service::delete_player_contract(&ctx.db, contract_id).await?;

        if !deleted {
            return Err(AppError::InvalidInput {
                message: format!("Player contract with id {} not found", contract_id),
            });
        }

        Ok(())
    }
}
