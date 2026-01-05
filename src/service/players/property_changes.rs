use sqlx::{Row, SqlitePool};

/// Property change entity (full data from database)
#[derive(Debug, Clone)]
pub struct PropertyChangeEntity {
    pub id: i64,
    #[allow(dead_code)] // Used in Maud templates
    pub player_id: i64,
    pub change_date: String, // ISO 8601 format
    pub property_type: String,
    pub old_value: Option<String>,
    pub new_value: Option<String>,
    pub description: String,
    pub season_id: Option<i64>,
    pub season_year: Option<i64>,
    #[allow(dead_code)] // Used in Maud templates
    pub season_display_name: Option<String>,
    pub event_name: Option<String>,
}

/// Create property change entity (for inserts)
#[derive(Debug, Clone)]
pub struct CreatePropertyChangeEntity {
    pub player_id: i64,
    pub change_date: String,
    pub property_type: String,
    pub old_value: Option<String>,
    pub new_value: Option<String>,
    pub description: String,
    pub season_id: Option<i64>,
}

/// Update property change entity (for updates)
#[derive(Debug, Clone)]
pub struct UpdatePropertyChangeEntity {
    pub change_date: String,
    pub property_type: String,
    pub old_value: Option<String>,
    pub new_value: Option<String>,
    pub description: String,
    pub season_id: Option<i64>,
}

/// Get all property changes for a player (ordered by date DESC)
pub async fn get_player_property_changes(
    db: &SqlitePool,
    player_id: i64,
) -> Result<Vec<PropertyChangeEntity>, sqlx::Error> {
    let rows = sqlx::query(
        r#"
        SELECT
            pc.id,
            pc.player_id,
            pc.change_date,
            pc.property_type,
            pc.old_value,
            pc.new_value,
            pc.description,
            pc.season_id,
            s.year as season_year,
            s.display_name as season_display_name,
            e.name as event_name
        FROM player_property_change pc
        LEFT JOIN season s ON pc.season_id = s.id
        LEFT JOIN event e ON s.event_id = e.id
        WHERE pc.player_id = ?
        ORDER BY pc.change_date DESC, pc.id DESC
        "#,
    )
    .bind(player_id)
    .fetch_all(db)
    .await?;

    Ok(rows
        .into_iter()
        .map(|row| PropertyChangeEntity {
            id: row.get("id"),
            player_id: row.get("player_id"),
            change_date: row.get("change_date"),
            property_type: row.get("property_type"),
            old_value: row.get("old_value"),
            new_value: row.get("new_value"),
            description: row.get("description"),
            season_id: row.get("season_id"),
            season_year: row.get("season_year"),
            season_display_name: row.get("season_display_name"),
            event_name: row.get("event_name"),
        })
        .collect())
}

/// Create a new property change
pub async fn create_property_change(
    db: &SqlitePool,
    entity: CreatePropertyChangeEntity,
) -> Result<i64, sqlx::Error> {
    let result = sqlx::query(
        r#"
        INSERT INTO player_property_change
        (player_id, change_date, property_type, old_value, new_value, description, season_id)
        VALUES (?, ?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(entity.player_id)
    .bind(entity.change_date)
    .bind(entity.property_type)
    .bind(entity.old_value)
    .bind(entity.new_value)
    .bind(entity.description)
    .bind(entity.season_id)
    .execute(db)
    .await?;

    Ok(result.last_insert_rowid())
}

/// Update a property change
pub async fn update_property_change(
    db: &SqlitePool,
    id: i64,
    entity: UpdatePropertyChangeEntity,
) -> Result<bool, sqlx::Error> {
    let result = sqlx::query(
        r#"
        UPDATE player_property_change
        SET change_date = ?, property_type = ?, old_value = ?, new_value = ?,
            description = ?, season_id = ?, updated_at = CURRENT_TIMESTAMP
        WHERE id = ?
        "#,
    )
    .bind(entity.change_date)
    .bind(entity.property_type)
    .bind(entity.old_value)
    .bind(entity.new_value)
    .bind(entity.description)
    .bind(entity.season_id)
    .bind(id)
    .execute(db)
    .await?;

    Ok(result.rows_affected() > 0)
}

/// Delete a property change
pub async fn delete_property_change(db: &SqlitePool, id: i64) -> Result<bool, sqlx::Error> {
    let result = sqlx::query("DELETE FROM player_property_change WHERE id = ?")
        .bind(id)
        .execute(db)
        .await?;

    Ok(result.rows_affected() > 0)
}

/// Get all seasons for a player (for dropdown filtering)
pub async fn get_player_seasons_for_changes(
    db: &SqlitePool,
    player_id: i64,
) -> Result<Vec<(i64, String)>, sqlx::Error> {
    let rows = sqlx::query(
        r#"
        SELECT DISTINCT s.id, s.year, s.display_name, e.name as event_name
        FROM season s
        INNER JOIN event e ON s.event_id = e.id
        INNER JOIN team_participation tp ON tp.season_id = s.id
        INNER JOIN player_contract pc ON pc.team_participation_id = tp.id
        WHERE pc.player_id = ?
        ORDER BY s.year DESC
        "#,
    )
    .bind(player_id)
    .fetch_all(db)
    .await?;

    Ok(rows
        .into_iter()
        .map(|row| {
            let year: i64 = row.get("year");
            let display_name: Option<String> = row.get("display_name");
            let event_name: String = row.get("event_name");
            let id: i64 = row.get("id");

            let label = if let Some(display) = display_name {
                format!("{} {} ({})", event_name, display, year)
            } else {
                format!("{} {}", event_name, year)
            };

            (id, label)
        })
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[sqlx::test(migrations = "./migrations", fixtures("players"))]
    async fn test_create_property_change(pool: SqlitePool) {
        let entity = CreatePropertyChangeEntity {
            player_id: 1,
            change_date: "2023-09-15".to_string(),
            property_type: "Position".to_string(),
            old_value: Some("C".to_string()),
            new_value: Some("LW".to_string()),
            description: "Moved to left wing due to team needs".to_string(),
            season_id: None,
        };

        let id = create_property_change(&pool, entity).await.unwrap();
        assert!(id > 0);

        // Verify it was created
        let changes = get_player_property_changes(&pool, 1).await.unwrap();
        assert_eq!(changes.len(), 1);
        assert_eq!(changes[0].property_type, "Position");
        assert_eq!(changes[0].old_value, Some("C".to_string()));
        assert_eq!(changes[0].new_value, Some("LW".to_string()));
    }

    #[sqlx::test(migrations = "./migrations", fixtures("players"))]
    async fn test_create_property_change_without_season(pool: SqlitePool) {
        // Test retirement (no season)
        let entity = CreatePropertyChangeEntity {
            player_id: 1,
            change_date: "2024-05-20".to_string(),
            property_type: "Retirement".to_string(),
            old_value: None,
            new_value: None,
            description: "Announced retirement from professional hockey".to_string(),
            season_id: None,
        };

        let id = create_property_change(&pool, entity).await.unwrap();
        assert!(id > 0);

        let changes = get_player_property_changes(&pool, 1).await.unwrap();
        assert_eq!(changes.len(), 1);
        assert_eq!(changes[0].property_type, "Retirement");
        assert!(changes[0].season_id.is_none());
    }

    #[sqlx::test(migrations = "./migrations", fixtures("players"))]
    async fn test_get_player_property_changes_ordered_by_date(pool: SqlitePool) {
        // Create multiple changes
        create_property_change(
            &pool,
            CreatePropertyChangeEntity {
                player_id: 1,
                change_date: "2023-01-01".to_string(),
                property_type: "Position".to_string(),
                old_value: None,
                new_value: Some("C".to_string()),
                description: "First change".to_string(),
                season_id: None,
            },
        )
        .await
        .unwrap();

        create_property_change(
            &pool,
            CreatePropertyChangeEntity {
                player_id: 1,
                change_date: "2024-06-15".to_string(),
                property_type: "Trade".to_string(),
                old_value: Some("Team A".to_string()),
                new_value: Some("Team B".to_string()),
                description: "Third change".to_string(),
                season_id: None,
            },
        )
        .await
        .unwrap();

        create_property_change(
            &pool,
            CreatePropertyChangeEntity {
                player_id: 1,
                change_date: "2023-09-01".to_string(),
                property_type: "JerseyNumber".to_string(),
                old_value: Some("9".to_string()),
                new_value: Some("97".to_string()),
                description: "Second change".to_string(),
                season_id: None,
            },
        )
        .await
        .unwrap();

        let changes = get_player_property_changes(&pool, 1).await.unwrap();
        assert_eq!(changes.len(), 3);

        // Verify descending order by date
        assert_eq!(changes[0].change_date, "2024-06-15"); // Latest first
        assert_eq!(changes[1].change_date, "2023-09-01");
        assert_eq!(changes[2].change_date, "2023-01-01"); // Oldest last
    }

    #[sqlx::test(migrations = "./migrations", fixtures("players"))]
    async fn test_update_property_change(pool: SqlitePool) {
        let id = create_property_change(
            &pool,
            CreatePropertyChangeEntity {
                player_id: 1,
                change_date: "2023-05-01".to_string(),
                property_type: "Position".to_string(),
                old_value: Some("C".to_string()),
                new_value: Some("LW".to_string()),
                description: "Original description".to_string(),
                season_id: None,
            },
        )
        .await
        .unwrap();

        let updated = update_property_change(
            &pool,
            id,
            UpdatePropertyChangeEntity {
                change_date: "2023-05-15".to_string(),
                property_type: "Position".to_string(),
                old_value: Some("C".to_string()),
                new_value: Some("RW".to_string()),
                description: "Updated description".to_string(),
                season_id: None,
            },
        )
        .await
        .unwrap();

        assert!(updated);

        let changes = get_player_property_changes(&pool, 1).await.unwrap();
        assert_eq!(changes[0].change_date, "2023-05-15");
        assert_eq!(changes[0].new_value, Some("RW".to_string()));
        assert_eq!(changes[0].description, "Updated description");
    }

    #[sqlx::test(migrations = "./migrations", fixtures("players"))]
    async fn test_delete_property_change(pool: SqlitePool) {
        let id = create_property_change(
            &pool,
            CreatePropertyChangeEntity {
                player_id: 1,
                change_date: "2023-01-01".to_string(),
                property_type: "Trade".to_string(),
                old_value: None,
                new_value: None,
                description: "Test trade".to_string(),
                season_id: None,
            },
        )
        .await
        .unwrap();

        let deleted = delete_property_change(&pool, id).await.unwrap();
        assert!(deleted);

        let changes = get_player_property_changes(&pool, 1).await.unwrap();
        assert_eq!(changes.len(), 0);
    }

    #[sqlx::test(migrations = "./migrations", fixtures("players"))]
    async fn test_delete_nonexistent_property_change(pool: SqlitePool) {
        let deleted = delete_property_change(&pool, 999).await.unwrap();
        assert!(!deleted);
    }

    #[sqlx::test(migrations = "./migrations", fixtures("players"))]
    async fn test_unique_constraint_prevents_duplicates(pool: SqlitePool) {
        let entity = CreatePropertyChangeEntity {
            player_id: 1,
            change_date: "2023-01-01".to_string(),
            property_type: "Position".to_string(),
            old_value: Some("C".to_string()),
            new_value: Some("LW".to_string()),
            description: "Same change".to_string(),
            season_id: None,
        };

        create_property_change(&pool, entity.clone()).await.unwrap();

        // Second attempt should fail due to UNIQUE constraint
        let result = create_property_change(&pool, entity).await;
        assert!(result.is_err());
    }

    #[sqlx::test(
        migrations = "./migrations",
        fixtures("events", "seasons", "teams", "team_participations", "players")
    )]
    async fn test_get_player_seasons_for_changes(pool: SqlitePool) {
        // Add a player contract for player 1 so they have seasons
        sqlx::query!(
            "INSERT INTO player_contract (player_id, team_participation_id) VALUES (?, ?)",
            1,
            1
        )
        .execute(&pool)
        .await
        .unwrap();

        let seasons = get_player_seasons_for_changes(&pool, 1).await.unwrap();

        // Should have at least one season from the fixtures
        assert!(!seasons.is_empty());

        // Verify format
        let (id, label) = &seasons[0];
        assert!(*id > 0);
        assert!(label.contains("2022") || label.contains("2023") || label.contains("2024"));
    }

    #[sqlx::test(migrations = "./migrations", fixtures("players"))]
    async fn test_get_property_changes_empty_for_player_without_changes(pool: SqlitePool) {
        let changes = get_player_property_changes(&pool, 1).await.unwrap();
        assert_eq!(changes.len(), 0);
    }

    #[sqlx::test(migrations = "./migrations", fixtures("players"))]
    async fn test_get_property_changes_for_nonexistent_player(pool: SqlitePool) {
        let changes = get_player_property_changes(&pool, 999).await.unwrap();
        assert_eq!(changes.len(), 0);
    }
}
