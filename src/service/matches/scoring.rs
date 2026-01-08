use sqlx::SqlitePool;

use super::entities::{CreateScoreEventEntity, ScoreEventEntity, UpdateScoreEventEntity};

/// Get a single score event by ID
pub async fn get_score_event_by_id(
    db: &SqlitePool,
    id: i64,
) -> Result<Option<ScoreEventEntity>, sqlx::Error> {
    let row = sqlx::query!(
        r#"
        SELECT
            se.id as "id!",
            se.match_id as "match_id!",
            se.team_id as "team_id!",
            t.name as "team_name!",
            se.scorer_id,
            scorer.name as scorer_name,
            se.assist1_id,
            assist1.name as assist1_name,
            se.assist2_id,
            assist2.name as assist2_name,
            se.period as "period!: i32",
            se.time_minutes as "time_minutes: i32",
            se.time_seconds as "time_seconds: i32",
            se.goal_type
        FROM score_event se
        INNER JOIN team t ON se.team_id = t.id
        LEFT JOIN player scorer ON se.scorer_id = scorer.id
        LEFT JOIN player assist1 ON se.assist1_id = assist1.id
        LEFT JOIN player assist2 ON se.assist2_id = assist2.id
        WHERE se.id = ?
        "#,
        id
    )
    .fetch_optional(db)
    .await?;

    Ok(row.map(|row| ScoreEventEntity {
        id: row.id,
        match_id: row.match_id,
        team_id: row.team_id,
        team_name: row.team_name,
        scorer_id: row.scorer_id,
        scorer_name: row.scorer_name,
        assist1_id: row.assist1_id,
        assist1_name: row.assist1_name,
        assist2_id: row.assist2_id,
        assist2_name: row.assist2_name,
        period: row.period,
        time_minutes: row.time_minutes,
        time_seconds: row.time_seconds,
        goal_type: row.goal_type,
    }))
}

/// Create a new score event and decrement unidentified goal count
pub async fn create_score_event(
    db: &SqlitePool,
    entity: CreateScoreEventEntity,
) -> Result<i64, sqlx::Error> {
    // Start a transaction to ensure both operations succeed or fail together
    let mut tx = db.begin().await?;

    // Insert the score event
    let result = sqlx::query!(
        "INSERT INTO score_event (match_id, team_id, scorer_id, assist1_id, assist2_id, period, time_minutes, time_seconds, goal_type) \
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
        entity.match_id,
        entity.team_id,
        entity.scorer_id,
        entity.assist1_id,
        entity.assist2_id,
        entity.period,
        entity.time_minutes,
        entity.time_seconds,
        entity.goal_type
    )
    .execute(&mut *tx)
    .await?;

    let score_event_id = result.last_insert_rowid();

    // Get the match to determine home/away team
    let match_row = sqlx::query!(
        "SELECT home_team_id, away_team_id, home_score_unidentified as \"home_score_unidentified!: i32\", away_score_unidentified as \"away_score_unidentified!: i32\" FROM match WHERE id = ?",
        entity.match_id
    )
    .fetch_one(&mut *tx)
    .await?;

    let home_team_id: i64 = match_row.home_team_id;
    let home_score_unidentified: i32 = match_row.home_score_unidentified;
    let away_score_unidentified: i32 = match_row.away_score_unidentified;

    // Decrement the appropriate unidentified score if it's greater than 0
    if entity.team_id == home_team_id && home_score_unidentified > 0 {
        sqlx::query!(
            "UPDATE match SET home_score_unidentified = home_score_unidentified - 1 WHERE id = ?",
            entity.match_id
        )
        .execute(&mut *tx)
        .await?;
    } else if entity.team_id != home_team_id && away_score_unidentified > 0 {
        sqlx::query!(
            "UPDATE match SET away_score_unidentified = away_score_unidentified - 1 WHERE id = ?",
            entity.match_id
        )
        .execute(&mut *tx)
        .await?;
    }

    // Commit the transaction
    tx.commit().await?;

    Ok(score_event_id)
}

/// Update an existing score event
pub async fn update_score_event(
    db: &SqlitePool,
    id: i64,
    entity: UpdateScoreEventEntity,
) -> Result<bool, sqlx::Error> {
    let result = sqlx::query!(
        "UPDATE score_event \
         SET team_id = ?, scorer_id = ?, assist1_id = ?, assist2_id = ?, \
             period = ?, time_minutes = ?, time_seconds = ?, goal_type = ? \
         WHERE id = ?",
        entity.team_id,
        entity.scorer_id,
        entity.assist1_id,
        entity.assist2_id,
        entity.period,
        entity.time_minutes,
        entity.time_seconds,
        entity.goal_type,
        id
    )
    .execute(db)
    .await?;

    Ok(result.rows_affected() > 0)
}

/// Delete a score event and increment unidentified goal count
pub async fn delete_score_event(db: &SqlitePool, id: i64) -> Result<bool, sqlx::Error> {
    // Start a transaction to ensure both operations succeed or fail together
    let mut tx = db.begin().await?;

    // Get the score event to determine match and team
    let score_event_row =
        sqlx::query!("SELECT match_id, team_id FROM score_event WHERE id = ?", id)
            .fetch_optional(&mut *tx)
            .await?;

    let (match_id, team_id) = match score_event_row {
        Some(row) => (row.match_id, row.team_id),
        None => return Ok(false), // Score event not found
    };

    // Delete the score event
    let result = sqlx::query!("DELETE FROM score_event WHERE id = ?", id)
        .execute(&mut *tx)
        .await?;

    if result.rows_affected() == 0 {
        return Ok(false);
    }

    // Get the match to determine home/away team
    let match_row = sqlx::query!("SELECT home_team_id FROM match WHERE id = ?", match_id)
        .fetch_one(&mut *tx)
        .await?;

    let home_team_id: i64 = match_row.home_team_id;

    // Increment the appropriate unidentified score
    if team_id == home_team_id {
        sqlx::query!(
            "UPDATE match SET home_score_unidentified = home_score_unidentified + 1 WHERE id = ?",
            match_id
        )
        .execute(&mut *tx)
        .await?;
    } else {
        sqlx::query!(
            "UPDATE match SET away_score_unidentified = away_score_unidentified + 1 WHERE id = ?",
            match_id
        )
        .execute(&mut *tx)
        .await?;
    }

    // Commit the transaction
    tx.commit().await?;

    Ok(true)
}
