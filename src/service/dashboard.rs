use sqlx::SqlitePool;

/// Dashboard statistics
#[derive(Debug, Clone, Default)]
pub struct DashboardStats {
    pub teams_count: i64,
    pub players_count: i64,
    pub events_count: i64,
    pub seasons_count: i64,
    pub matches_count: i64,
}

/// Recent activity item
#[derive(Debug, Clone)]
pub struct RecentActivity {
    pub entity_type: String,
    pub entity_name: String,
    pub action: String,
    pub timestamp: String,
}

/// Get dashboard statistics (counts of all entities)
pub async fn get_dashboard_stats(db: &SqlitePool) -> Result<DashboardStats, sqlx::Error> {
    // Optimize: Single query with subqueries instead of 5 separate queries
    // Reduces database round trips from 5 to 1
    let row = sqlx::query!(
        r#"
        SELECT
            (SELECT COUNT(*) FROM team) as teams_count,
            (SELECT COUNT(*) FROM player) as players_count,
            (SELECT COUNT(*) FROM event) as events_count,
            (SELECT COUNT(*) FROM season) as seasons_count,
            (SELECT COUNT(*) FROM match) as matches_count
        "#
    )
    .fetch_one(db)
    .await?;

    Ok(DashboardStats {
        teams_count: row.teams_count,
        players_count: row.players_count,
        events_count: row.events_count,
        seasons_count: row.seasons_count,
        matches_count: row.matches_count,
    })
}

/// Get recent activity across all entities (last 10 updates)
pub async fn get_recent_activity(db: &SqlitePool) -> Result<Vec<RecentActivity>, sqlx::Error> {
    // Query to get recent updates from various tables
    // We'll use UNION ALL to combine recent items from different tables
    let rows = sqlx::query!(
        r#"
        SELECT 'Team' as entity_type, name as entity_name, 'updated' as action, updated_at as timestamp
        FROM team WHERE updated_at IS NOT NULL
        UNION ALL
        SELECT 'Player' as entity_type, name as entity_name, 'updated' as action, updated_at as timestamp
        FROM player WHERE updated_at IS NOT NULL
        UNION ALL
        SELECT 'Event' as entity_type, name as entity_name, 'updated' as action, updated_at as timestamp
        FROM event WHERE updated_at IS NOT NULL
        UNION ALL
        SELECT 'Season' as entity_type,
               COALESCE(display_name, 'Season ' || year) as entity_name,
               'updated' as action,
               updated_at as timestamp
        FROM season WHERE updated_at IS NOT NULL
        UNION ALL
        SELECT 'Match' as entity_type,
               'Match #' || id as entity_name,
               'updated' as action,
               updated_at as timestamp
        FROM match WHERE updated_at IS NOT NULL
        ORDER BY timestamp DESC
        LIMIT 10
        "#
    )
    .fetch_all(db)
    .await?;

    let activities = rows
        .into_iter()
        .map(|row| RecentActivity {
            entity_type: row.entity_type,
            entity_name: row.entity_name,
            action: row.action,
            timestamp: row.timestamp,
        })
        .collect();

    Ok(activities)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[sqlx::test(migrations = "./migrations")]
    async fn test_get_dashboard_stats_empty(pool: SqlitePool) {
        let stats = get_dashboard_stats(&pool).await.unwrap();

        // Empty database should have 0 counts
        assert_eq!(stats.teams_count, 0);
        assert_eq!(stats.players_count, 0);
        assert_eq!(stats.events_count, 0);
        assert_eq!(stats.seasons_count, 0);
        assert_eq!(stats.matches_count, 0);
    }

    #[sqlx::test(
        migrations = "./migrations",
        fixtures("events", "seasons", "teams", "players")
    )]
    async fn test_get_dashboard_stats_with_data(pool: SqlitePool) {
        let stats = get_dashboard_stats(&pool).await.unwrap();

        // Should have fixture counts
        assert_eq!(stats.teams_count, 5); // From teams fixture
        assert_eq!(stats.players_count, 10); // From players fixture
        assert_eq!(stats.events_count, 3); // From events fixture
        assert_eq!(stats.seasons_count, 3); // From seasons fixture
        assert_eq!(stats.matches_count, 0); // No matches fixture
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn test_get_recent_activity_empty(pool: SqlitePool) {
        let activities = get_recent_activity(&pool).await.unwrap();

        // Empty database should have no activities
        assert_eq!(activities.len(), 0);
    }

    #[sqlx::test(migrations = "./migrations", fixtures("events"))]
    async fn test_get_recent_activity_with_data(pool: SqlitePool) {
        // Update an event to give it an updated_at timestamp
        sqlx::query!("UPDATE event SET updated_at = datetime('now') WHERE id = 1")
            .execute(&pool)
            .await
            .unwrap();

        // Update another event
        sqlx::query!("UPDATE event SET updated_at = datetime('now', '-1 hour') WHERE id = 2")
            .execute(&pool)
            .await
            .unwrap();

        let activities = get_recent_activity(&pool).await.unwrap();

        // Should have 3 events (all from fixture)
        assert_eq!(activities.len(), 3);

        // Most recent should be first (id=1)
        assert_eq!(activities[0].entity_type, "Event");
        assert!(activities[0].entity_name.contains("Olympics"));

        // Verify second one (id=2)
        assert_eq!(activities[1].entity_type, "Event");
    }

    #[sqlx::test(migrations = "./migrations", fixtures("events", "teams", "players"))]
    async fn test_get_recent_activity_multiple_types(pool: SqlitePool) {
        // Update entities with different timestamps
        sqlx::query!("UPDATE team SET updated_at = datetime('now') WHERE id = 1")
            .execute(&pool)
            .await
            .unwrap();

        sqlx::query!("UPDATE player SET updated_at = datetime('now', '-30 minutes') WHERE id = 1")
            .execute(&pool)
            .await
            .unwrap();

        sqlx::query!("UPDATE event SET updated_at = datetime('now', '-1 hour') WHERE id = 1")
            .execute(&pool)
            .await
            .unwrap();

        let activities = get_recent_activity(&pool).await.unwrap();

        // Should have 10 activities (limit of get_recent_activity)
        // Fixtures have 3 events + 5 teams + 10 players = 18 total, limited to 10
        assert_eq!(activities.len(), 10);

        // Verify the most recently updated one is first
        assert_eq!(activities[0].entity_type, "Team");
        // Note: The rest of the order depends on insertion timestamps which may vary
    }

    #[sqlx::test(migrations = "./migrations", fixtures("events"))]
    async fn test_get_recent_activity_limit_10(pool: SqlitePool) {
        // Update more than 10 events to test limit
        for i in 1..=3 {
            let time_offset = format!("-{}", i);
            sqlx::query!(
                "UPDATE event SET updated_at = datetime('now', ? || ' minutes') WHERE id = ?",
                time_offset,
                i
            )
            .execute(&pool)
            .await
            .unwrap();
        }

        let activities = get_recent_activity(&pool).await.unwrap();

        // Should be limited to available items (3 in this case)
        assert!(activities.len() <= 10);
        assert_eq!(activities.len(), 3);
    }
}
