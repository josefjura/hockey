use sqlx::SqlitePool;

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
    let rows = sqlx::query_as!(
        PlayerEventStatsEntity,
        r#"
        SELECT
            pes.id as "id!",
            pes.event_id as "event_id!",
            e.name as event_name,
            CAST(pes.goals_total AS INTEGER) as "goals_total!: i32",
            CAST(pes.assists_total AS INTEGER) as "assists_total!: i32",
            CAST(pes.goals_total + pes.assists_total AS INTEGER) as "points_total!: i32",
            CAST(COALESCE(SUM(CASE WHEN se.scorer_id = ? THEN 1 ELSE 0 END), 0) AS INTEGER) as "goals_identified!: i32",
            CAST(COALESCE(SUM(CASE WHEN se.assist1_id = ? OR se.assist2_id = ? THEN 1 ELSE 0 END), 0) AS INTEGER) as "assists_identified!: i32",
            CAST(COALESCE(SUM(CASE WHEN se.scorer_id = ? THEN 1 ELSE 0 END), 0) +
            COALESCE(SUM(CASE WHEN se.assist1_id = ? OR se.assist2_id = ? THEN 1 ELSE 0 END), 0) AS INTEGER) as "points_identified!: i32"
        FROM player_event_stats pes
        INNER JOIN event e ON pes.event_id = e.id
        LEFT JOIN season s ON s.event_id = e.id
        LEFT JOIN match m ON m.season_id = s.id
        LEFT JOIN score_event se ON se.match_id = m.id
        WHERE pes.player_id = ?
        GROUP BY pes.id, pes.event_id, e.name, pes.goals_total, pes.assists_total
        ORDER BY e.name ASC
        "#,
        player_id,
        player_id,
        player_id,
        player_id,
        player_id,
        player_id,
        player_id
    )
    .fetch_all(db)
    .await?;

    Ok(rows)
}

/// Create or update event stats atomically (for create operations with initial values)
/// This function uses UPSERT to ensure atomicity - no orphaned zero-value records on failure
pub async fn create_or_update_player_event_stats(
    db: &SqlitePool,
    player_id: i64,
    event_id: i64,
    goals_total: i32,
    assists_total: i32,
) -> Result<i64, sqlx::Error> {
    // Use UPSERT pattern: INSERT with ON CONFLICT DO UPDATE
    // This is atomic - either both insert and values happen, or neither
    let result = sqlx::query!(
        r#"INSERT INTO player_event_stats (player_id, event_id, goals_total, assists_total)
         VALUES (?, ?, ?, ?)
         ON CONFLICT(player_id, event_id)
         DO UPDATE SET
           goals_total = excluded.goals_total,
           assists_total = excluded.assists_total,
           updated_at = CURRENT_TIMESTAMP
         RETURNING id"#,
        player_id,
        event_id,
        goals_total,
        assists_total
    )
    .fetch_one(db)
    .await?;

    Ok(result.id)
}

/// Update event stats totals
pub async fn update_player_event_stats(
    db: &SqlitePool,
    id: i64,
    goals_total: i32,
    assists_total: i32,
) -> Result<bool, sqlx::Error> {
    let result = sqlx::query!(
        r#"UPDATE player_event_stats SET goals_total = ?, assists_total = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?"#,
        goals_total,
        assists_total,
        id
    )
    .execute(db)
    .await?;

    Ok(result.rows_affected() > 0)
}

/// Delete event stats
pub async fn delete_player_event_stats(db: &SqlitePool, id: i64) -> Result<bool, sqlx::Error> {
    let result = sqlx::query!(r#"DELETE FROM player_event_stats WHERE id = ?"#, id)
        .execute(db)
        .await?;

    Ok(result.rows_affected() > 0)
}

/// Get all events (for dropdowns when adding stats)
pub async fn get_all_events(db: &SqlitePool) -> Result<Vec<(i64, String)>, sqlx::Error> {
    let rows = sqlx::query!(r#"SELECT id, name FROM event ORDER BY name ASC"#)
        .fetch_all(db)
        .await?;

    Ok(rows.into_iter().map(|row| (row.id, row.name)).collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::{Row, SqlitePool};

    #[sqlx::test(migrations = "./migrations", fixtures("players", "events"))]
    async fn test_update_player_event_stats(pool: SqlitePool) {
        let player_id = 1;
        let event_id = 1;

        // Create initial record with zeros
        let id = create_or_update_player_event_stats(&pool, player_id, event_id, 0, 0)
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

        let id = create_or_update_player_event_stats(&pool, player_id, event_id, 5, 10)
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

    #[sqlx::test(migrations = "./migrations", fixtures("players", "events"))]
    async fn test_create_or_update_creates_new_with_values(pool: SqlitePool) {
        let player_id = 1; // Wayne Gretzky
        let event_id = 1; // Winter Olympics

        let id = create_or_update_player_event_stats(&pool, player_id, event_id, 10, 15)
            .await
            .unwrap();
        assert!(id > 0);

        // Verify it was created with the provided values (not zeros)
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
    async fn test_create_or_update_updates_existing(pool: SqlitePool) {
        let player_id = 1;
        let event_id = 1;

        // Create with initial values
        let id1 = create_or_update_player_event_stats(&pool, player_id, event_id, 5, 8)
            .await
            .unwrap();

        // Update with new values
        let id2 = create_or_update_player_event_stats(&pool, player_id, event_id, 10, 15)
            .await
            .unwrap();

        // Should return same ID
        assert_eq!(id1, id2);

        // Verify values were updated
        let row =
            sqlx::query("SELECT goals_total, assists_total FROM player_event_stats WHERE id = ?")
                .bind(id1)
                .fetch_one(&pool)
                .await
                .unwrap();
        assert_eq!(row.get::<i32, _>("goals_total"), 10);
        assert_eq!(row.get::<i32, _>("assists_total"), 15);
    }

    #[sqlx::test(migrations = "./migrations", fixtures("players", "events"))]
    async fn test_create_or_update_concurrent_requests(pool: SqlitePool) {
        let player_id = 1;
        let event_id = 1;

        // Simulate concurrent requests with different values
        let pool1 = pool.clone();
        let pool2 = pool.clone();
        let pool3 = pool.clone();

        let handle1 = tokio::spawn(async move {
            create_or_update_player_event_stats(&pool1, player_id, event_id, 10, 15).await
        });
        let handle2 = tokio::spawn(async move {
            create_or_update_player_event_stats(&pool2, player_id, event_id, 20, 25).await
        });
        let handle3 = tokio::spawn(async move {
            create_or_update_player_event_stats(&pool3, player_id, event_id, 30, 35).await
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

        // Verify final values are one of the concurrent updates (not zeros)
        let row =
            sqlx::query("SELECT goals_total, assists_total FROM player_event_stats WHERE id = ?")
                .bind(id1)
                .fetch_one(&pool)
                .await
                .unwrap();
        let goals = row.get::<i32, _>("goals_total");
        let assists = row.get::<i32, _>("assists_total");

        // Should be one of the three sets of values, never zeros
        assert!(
            (goals == 10 && assists == 15)
                || (goals == 20 && assists == 25)
                || (goals == 30 && assists == 35)
        );
    }

    #[sqlx::test(migrations = "./migrations", fixtures("players", "events"))]
    async fn test_create_or_update_no_orphaned_zeros(pool: SqlitePool) {
        let player_id = 1;
        let event_id = 1;

        // Create with non-zero values
        let id = create_or_update_player_event_stats(&pool, player_id, event_id, 5, 10)
            .await
            .unwrap();

        // Verify no zero-value records exist for this player+event
        let row =
            sqlx::query("SELECT goals_total, assists_total FROM player_event_stats WHERE id = ?")
                .bind(id)
                .fetch_one(&pool)
                .await
                .unwrap();

        let goals = row.get::<i32, _>("goals_total");
        let assists = row.get::<i32, _>("assists_total");

        // Should have the values we set, not zeros
        assert_eq!(goals, 5);
        assert_eq!(assists, 10);

        // Verify only one record exists for this player+event combination
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
}
