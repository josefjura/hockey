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
            entity_name: row.entity_name.unwrap_or_else(|| String::from("")),
            action: row.action,
            timestamp: row.timestamp.unwrap_or_else(|| String::from("")),
        })
        .collect();

    Ok(activities)
}
