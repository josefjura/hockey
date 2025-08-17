use sqlx::Row;
use sqlx::SqlitePool;

#[cfg(test)]
pub mod test;

pub struct CreatePlayerContractEntity {
    pub team_participation_id: i64,
    pub player_id: i64,
}

pub struct PlayerContractEntity {
    pub id: i64,
    pub team_participation_id: i64,
    pub player_id: i64,
}

pub async fn create_player_contract(
    db: &SqlitePool,
    contract: CreatePlayerContractEntity,
) -> Result<i64, sqlx::Error> {
    let result =
        sqlx::query("INSERT INTO player_contract (team_participation_id, player_id) VALUES (?, ?)")
            .bind(contract.team_participation_id)
            .bind(contract.player_id)
            .execute(db)
            .await?;

    Ok(result.last_insert_rowid())
}

pub async fn get_player_contracts(
    db: &SqlitePool,
) -> Result<Vec<PlayerContractEntity>, sqlx::Error> {
    let rows = sqlx::query("SELECT id, team_participation_id, player_id FROM player_contract")
        .fetch_all(db)
        .await?;

    Ok(rows
        .into_iter()
        .map(|row| PlayerContractEntity {
            id: row.get("id"),
            team_participation_id: row.get("team_participation_id"),
            player_id: row.get("player_id"),
        })
        .collect())
}

pub async fn get_player_contract_by_id(
    db: &SqlitePool,
    id: i64,
) -> Result<Option<PlayerContractEntity>, sqlx::Error> {
    let row = sqlx::query(
        "SELECT id, team_participation_id, player_id FROM player_contract WHERE id = ?",
    )
    .bind(id)
    .fetch_optional(db)
    .await?;

    Ok(row.map(|row| PlayerContractEntity {
        id: row.get("id"),
        team_participation_id: row.get("team_participation_id"),
        player_id: row.get("player_id"),
    }))
}

pub async fn delete_player_contract(db: &SqlitePool, id: i64) -> Result<bool, sqlx::Error> {
    let result = sqlx::query("DELETE FROM player_contract WHERE id = ?")
        .bind(id)
        .execute(db)
        .await?;
    Ok(result.rows_affected() > 0)
}
