use sqlx::{Row, SqlitePool};

/// Player event statistics entity (career totals for a player in a specific event/competition)
#[derive(Debug, Clone)]
pub struct PlayerEventStatsEntity {
    pub id: i64,
    #[allow(dead_code)] // Loaded from DB but not currently used in views
    pub event_id: i64,
    pub event_name: String,
    pub goals_total: i32,
    pub assists_total: i32,
    pub points_total: i32,       // Calculated: goals + assists
    pub goals_identified: i32,   // Calculated from score_event
    pub assists_identified: i32, // Calculated from score_event
    pub points_identified: i32,  // Calculated: goals_identified + assists_identified
}

/// Get all event stats for a player (with identified counts)
pub async fn get_player_event_stats(
    db: &SqlitePool,
    player_id: i64,
) -> Result<Vec<PlayerEventStatsEntity>, sqlx::Error> {
    let rows = sqlx::query(
        r#"
        SELECT
            pes.id,
            pes.player_id,
            pes.event_id,
            e.name as event_name,
            pes.goals_total,
            pes.assists_total,
            COALESCE(SUM(CASE WHEN se.scorer_id = ? THEN 1 ELSE 0 END), 0) as goals_identified,
            COALESCE(SUM(CASE WHEN se.assist1_id = ? OR se.assist2_id = ? THEN 1 ELSE 0 END), 0) as assists_identified
        FROM player_event_stats pes
        INNER JOIN event e ON pes.event_id = e.id
        LEFT JOIN season s ON s.event_id = e.id
        LEFT JOIN match m ON m.season_id = s.id
        LEFT JOIN score_event se ON se.match_id = m.id
        WHERE pes.player_id = ?
        GROUP BY pes.id, pes.player_id, pes.event_id, e.name, pes.goals_total, pes.assists_total
        ORDER BY e.name ASC
        "#,
    )
    .bind(player_id)
    .bind(player_id)
    .bind(player_id)
    .bind(player_id)
    .fetch_all(db)
    .await?;

    Ok(rows
        .into_iter()
        .map(|row| PlayerEventStatsEntity {
            id: row.get("id"),
            event_id: row.get("event_id"),
            event_name: row.get("event_name"),
            goals_total: row.get("goals_total"),
            assists_total: row.get("assists_total"),
            points_total: row.get::<i32, _>("goals_total") + row.get::<i32, _>("assists_total"),
            goals_identified: row.get("goals_identified"),
            assists_identified: row.get("assists_identified"),
            points_identified: row.get::<i32, _>("goals_identified")
                + row.get::<i32, _>("assists_identified"),
        })
        .collect())
}

/// Get or create event stats for a player + event combination
pub async fn get_or_create_player_event_stats(
    db: &SqlitePool,
    player_id: i64,
    event_id: i64,
) -> Result<i64, sqlx::Error> {
    // Try to get existing
    let existing =
        sqlx::query("SELECT id FROM player_event_stats WHERE player_id = ? AND event_id = ?")
            .bind(player_id)
            .bind(event_id)
            .fetch_optional(db)
            .await?;

    if let Some(row) = existing {
        return Ok(row.get("id"));
    }

    // Create new
    let result = sqlx::query(
        "INSERT INTO player_event_stats (player_id, event_id, goals_total, assists_total) VALUES (?, ?, 0, 0)"
    )
    .bind(player_id)
    .bind(event_id)
    .execute(db)
    .await?;

    Ok(result.last_insert_rowid())
}

/// Update event stats totals
pub async fn update_player_event_stats(
    db: &SqlitePool,
    id: i64,
    goals_total: i32,
    assists_total: i32,
) -> Result<bool, sqlx::Error> {
    let result = sqlx::query(
        "UPDATE player_event_stats SET goals_total = ?, assists_total = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?"
    )
    .bind(goals_total)
    .bind(assists_total)
    .bind(id)
    .execute(db)
    .await?;

    Ok(result.rows_affected() > 0)
}

/// Delete event stats
pub async fn delete_player_event_stats(db: &SqlitePool, id: i64) -> Result<bool, sqlx::Error> {
    let result = sqlx::query("DELETE FROM player_event_stats WHERE id = ?")
        .bind(id)
        .execute(db)
        .await?;

    Ok(result.rows_affected() > 0)
}

/// Get all events (for dropdowns when adding stats)
pub async fn get_all_events(db: &SqlitePool) -> Result<Vec<(i64, String)>, sqlx::Error> {
    let rows = sqlx::query("SELECT id, name FROM event ORDER BY name ASC")
        .fetch_all(db)
        .await?;

    Ok(rows
        .into_iter()
        .map(|row| (row.get("id"), row.get("name")))
        .collect())
}
