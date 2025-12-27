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
    let teams_count: i64 = sqlx::query!("SELECT COUNT(*) as count FROM team")
        .fetch_one(db)
        .await?
        .count;

    let players_count: i64 = sqlx::query!("SELECT COUNT(*) as count FROM player")
        .fetch_one(db)
        .await?
        .count;

    let events_count: i64 = sqlx::query!("SELECT COUNT(*) as count FROM event")
        .fetch_one(db)
        .await?
        .count;

    let seasons_count: i64 = sqlx::query!("SELECT COUNT(*) as count FROM season")
        .fetch_one(db)
        .await?
        .count;

    let matches_count: i64 = sqlx::query!("SELECT COUNT(*) as count FROM match")
        .fetch_one(db)
        .await?
        .count;

    Ok(DashboardStats {
        teams_count,
        players_count,
        events_count,
        seasons_count,
        matches_count,
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
