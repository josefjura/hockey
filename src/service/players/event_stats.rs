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
    // Use INSERT OR IGNORE to handle concurrent inserts safely
    sqlx::query(
        "INSERT OR IGNORE INTO player_event_stats (player_id, event_id, goals_total, assists_total)
         VALUES (?, ?, 0, 0)",
    )
    .bind(player_id)
    .bind(event_id)
    .execute(db)
    .await?;

    // Now fetch the ID (will work whether we inserted or it already existed)
    let row = sqlx::query("SELECT id FROM player_event_stats WHERE player_id = ? AND event_id = ?")
        .bind(player_id)
        .bind(event_id)
        .fetch_one(db)
        .await?;

    Ok(row.get("id"))
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

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::SqlitePool;

    #[sqlx::test(migrations = "./migrations", fixtures("players", "events"))]
    async fn test_get_or_create_player_event_stats_creates_new(pool: SqlitePool) {
        let player_id = 1; // Wayne Gretzky
        let event_id = 1; // Winter Olympics

        let id = get_or_create_player_event_stats(&pool, player_id, event_id)
            .await
            .unwrap();
        assert!(id > 0);

        // Verify it was created with zeros
        let row =
            sqlx::query("SELECT goals_total, assists_total FROM player_event_stats WHERE id = ?")
                .bind(id)
                .fetch_one(&pool)
                .await
                .unwrap();
        assert_eq!(row.get::<i32, _>("goals_total"), 0);
        assert_eq!(row.get::<i32, _>("assists_total"), 0);
    }

    #[sqlx::test(migrations = "./migrations", fixtures("players", "events"))]
    async fn test_get_or_create_player_event_stats_returns_existing(pool: SqlitePool) {
        let player_id = 1;
        let event_id = 1;

        // Create first time
        let id1 = get_or_create_player_event_stats(&pool, player_id, event_id)
            .await
            .unwrap();

        // Second call should return same ID
        let id2 = get_or_create_player_event_stats(&pool, player_id, event_id)
            .await
            .unwrap();

        assert_eq!(id1, id2);
    }

    #[sqlx::test(migrations = "./migrations", fixtures("players", "events"))]
    async fn test_get_or_create_concurrent_requests(pool: SqlitePool) {
        let player_id = 1;
        let event_id = 1;

        // Simulate concurrent requests
        let pool1 = pool.clone();
        let pool2 = pool.clone();
        let pool3 = pool.clone();

        let handle1 = tokio::spawn(async move {
            get_or_create_player_event_stats(&pool1, player_id, event_id).await
        });
        let handle2 = tokio::spawn(async move {
            get_or_create_player_event_stats(&pool2, player_id, event_id).await
        });
        let handle3 = tokio::spawn(async move {
            get_or_create_player_event_stats(&pool3, player_id, event_id).await
        });

        // All should succeed
        let id1 = handle1.await.unwrap().unwrap();
        let id2 = handle2.await.unwrap().unwrap();
        let id3 = handle3.await.unwrap().unwrap();

        // All should return the same ID
        assert_eq!(id1, id2);
        assert_eq!(id2, id3);

        // Verify only one record exists
        let count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM player_event_stats WHERE player_id = ? AND event_id = ?",
        )
        .bind(player_id)
        .bind(event_id)
        .fetch_one(&pool)
        .await
        .unwrap();
        assert_eq!(count, 1);
    }

    #[sqlx::test(migrations = "./migrations", fixtures("players", "events"))]
    async fn test_update_player_event_stats(pool: SqlitePool) {
        let player_id = 1;
        let event_id = 1;

        let id = get_or_create_player_event_stats(&pool, player_id, event_id)
            .await
            .unwrap();

        // Update with new values
        let updated = update_player_event_stats(&pool, id, 10, 15).await.unwrap();
        assert!(updated);

        // Verify values were updated
        let row =
            sqlx::query("SELECT goals_total, assists_total FROM player_event_stats WHERE id = ?")
                .bind(id)
                .fetch_one(&pool)
                .await
                .unwrap();
        assert_eq!(row.get::<i32, _>("goals_total"), 10);
        assert_eq!(row.get::<i32, _>("assists_total"), 15);
    }

    #[sqlx::test(migrations = "./migrations", fixtures("players", "events"))]
    async fn test_update_player_event_stats_not_found(pool: SqlitePool) {
        let updated = update_player_event_stats(&pool, 99999, 10, 15)
            .await
            .unwrap();
        assert!(!updated);
    }

    #[sqlx::test(migrations = "./migrations", fixtures("players", "events"))]
    async fn test_delete_player_event_stats(pool: SqlitePool) {
        let player_id = 1;
        let event_id = 1;

        let id = get_or_create_player_event_stats(&pool, player_id, event_id)
            .await
            .unwrap();

        let deleted = delete_player_event_stats(&pool, id).await.unwrap();
        assert!(deleted);

        // Verify it was deleted
        let row = sqlx::query("SELECT id FROM player_event_stats WHERE id = ?")
            .bind(id)
            .fetch_optional(&pool)
            .await
            .unwrap();
        assert!(row.is_none());
    }

    #[sqlx::test(migrations = "./migrations", fixtures("players", "events"))]
    async fn test_delete_player_event_stats_not_found(pool: SqlitePool) {
        let deleted = delete_player_event_stats(&pool, 99999).await.unwrap();
        assert!(!deleted);
    }

    #[sqlx::test(migrations = "./migrations", fixtures("players", "events"))]
    async fn test_get_all_events(pool: SqlitePool) {
        let events = get_all_events(&pool).await.unwrap();

        assert!(events.len() >= 3); // At least the fixture events
        assert!(events.iter().any(|(_, name)| name == "Winter Olympics"));
        assert!(events.iter().any(|(_, name)| name == "World Championship"));
    }
}
