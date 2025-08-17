use sqlx::Row;
use sqlx::SqlitePool;

#[cfg(test)]
pub mod test;

pub struct CreateTeamParticipationEntity {
    pub team_id: i64,
    pub season_id: i64,
}

pub struct TeamParticipationEntity {
    pub id: i64,
    pub team_id: i64,
    pub season_id: i64,
}

pub async fn create_team_participation(
    db: &SqlitePool,
    participation: CreateTeamParticipationEntity,
) -> Result<i64, sqlx::Error> {
    let result = sqlx::query("INSERT INTO team_participation (team_id, season_id) VALUES (?, ?)")
        .bind(participation.team_id)
        .bind(participation.season_id)
        .execute(db)
        .await?;

    Ok(result.last_insert_rowid())
}

pub async fn get_team_participation(
    db: &SqlitePool,
) -> Result<Vec<TeamParticipationEntity>, sqlx::Error> {
    let rows = sqlx::query("SELECT id, team_id, season_id FROM team_participation")
        .fetch_all(db)
        .await?;

    Ok(rows
        .into_iter()
        .map(|row| TeamParticipationEntity {
            id: row.get("id"),
            team_id: row.get("team_id"),
            season_id: row.get("season_id"),
        })
        .collect())
}

pub async fn get_team_participation_by_id(
    db: &SqlitePool,
    id: i64,
) -> Result<Option<TeamParticipationEntity>, sqlx::Error> {
    let row = sqlx::query("SELECT id, team_id, season_id FROM team_participation WHERE id = ?")
        .bind(id)
        .fetch_optional(db)
        .await?;

    Ok(row.map(|row| TeamParticipationEntity {
        id: row.get("id"),
        team_id: row.get("team_id"),
        season_id: row.get("season_id"),
    }))
}

pub async fn get_team_participation_by_season_and_team(
    db: &SqlitePool,
    season_id: i64,
    team_id: i64,
) -> Result<Option<TeamParticipationEntity>, sqlx::Error> {
    let row = sqlx::query(
        "SELECT id, team_id, season_id FROM team_participation WHERE season_id = ? AND team_id = ?",
    )
    .bind(season_id)
    .bind(team_id)
    .fetch_optional(db)
    .await?;

    Ok(row.map(|row| TeamParticipationEntity {
        id: row.get("id"),
        team_id: row.get("team_id"),
        season_id: row.get("season_id"),
    }))
}

pub async fn find_or_create_team_participation(
    db: &SqlitePool,
    season_id: i64,
    team_id: i64,
) -> Result<i64, sqlx::Error> {
    // First try to find existing
    if let Some(existing) =
        get_team_participation_by_season_and_team(db, season_id, team_id).await?
    {
        return Ok(existing.id);
    }

    // If not found, create new
    create_team_participation(db, CreateTeamParticipationEntity { team_id, season_id }).await
}

pub async fn delete_team_participation(db: &SqlitePool, id: i64) -> Result<bool, sqlx::Error> {
    let result = sqlx::query("DELETE FROM team_participation WHERE id = ?")
        .bind(id)
        .execute(db)
        .await?;
    Ok(result.rows_affected() > 0)
}
