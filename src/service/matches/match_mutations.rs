use sqlx::SqlitePool;

use super::entities::{CreateMatchEntity, UpdateMatchEntity};

/// Create a new match
pub async fn create_match(db: &SqlitePool, entity: CreateMatchEntity) -> Result<i64, sqlx::Error> {
    let result = sqlx::query!(
        "INSERT INTO match (season_id, home_team_id, away_team_id, home_score_unidentified, away_score_unidentified, match_date, status, venue) \
         VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
        entity.season_id,
        entity.home_team_id,
        entity.away_team_id,
        entity.home_score_unidentified,
        entity.away_score_unidentified,
        entity.match_date,
        entity.status,
        entity.venue
    )
    .execute(db)
    .await?;

    Ok(result.last_insert_rowid())
}

/// Update an existing match
pub async fn update_match(
    db: &SqlitePool,
    id: i64,
    entity: UpdateMatchEntity,
) -> Result<bool, sqlx::Error> {
    let result = sqlx::query!(
        "UPDATE match \
         SET season_id = ?, home_team_id = ?, away_team_id = ?, \
             home_score_unidentified = ?, away_score_unidentified = ?, \
             match_date = ?, status = ?, venue = ?, \
             updated_at = CURRENT_TIMESTAMP \
         WHERE id = ?",
        entity.season_id,
        entity.home_team_id,
        entity.away_team_id,
        entity.home_score_unidentified,
        entity.away_score_unidentified,
        entity.match_date,
        entity.status,
        entity.venue,
        id
    )
    .execute(db)
    .await?;

    Ok(result.rows_affected() > 0)
}

/// Delete a match (cascades to score events)
pub async fn delete_match(db: &SqlitePool, id: i64) -> Result<bool, sqlx::Error> {
    let result = sqlx::query!("DELETE FROM match WHERE id = ?", id)
        .execute(db)
        .await?;

    Ok(result.rows_affected() > 0)
}
