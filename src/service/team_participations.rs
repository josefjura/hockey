use sqlx::SqlitePool;

#[derive(Debug, Clone)]
pub struct TeamParticipationEntity {
    pub id: i64,
    #[allow(dead_code)]
    pub team_id: i64,
    pub team_name: String,
    #[allow(dead_code)]
    pub country_id: Option<i64>,
    pub country_iso2_code: Option<String>,
    #[allow(dead_code)]
    pub season_id: i64,
}

#[derive(Debug, Clone)]
pub struct CreateTeamParticipationEntity {
    pub team_id: i64,
    pub season_id: i64,
    pub event_id: i64,
}

#[derive(Debug, Clone)]
pub struct TeamParticipationNameEntity {
    pub id: i64,
    pub name: String,
}

/// Get all teams participating in a season with team details
pub async fn get_teams_for_season(
    db: &SqlitePool,
    season_id: i64,
) -> Result<Vec<TeamParticipationEntity>, sqlx::Error> {
    let teams = sqlx::query_as!(
        TeamParticipationEntity,
        r#"
        SELECT
            tp.id as "id!",
            tp.team_id as team_id,
            t.name as team_name,
            t.country_id,
            c.iso2Code as country_iso2_code,
            tp.season_id as season_id
        FROM team_participation tp
        INNER JOIN team t ON tp.team_id = t.id
        LEFT JOIN country c ON t.country_id = c.id
        WHERE tp.season_id = ?
        ORDER BY t.name ASC
        "#,
        season_id
    )
    .fetch_all(db)
    .await?;

    Ok(teams)
}

/// Get teams that are not yet participating in a season (for dropdown)
pub async fn get_available_teams_for_season(
    db: &SqlitePool,
    season_id: i64,
) -> Result<Vec<(i64, String)>, sqlx::Error> {
    let rows = sqlx::query_as!(
        TeamParticipationNameEntity,
        r#"
        SELECT t.id, t.name
        FROM team t
        WHERE t.id NOT IN (
            SELECT team_id FROM team_participation WHERE season_id = ?
        )
        ORDER BY t.name ASC
        "#,
        season_id
    )
    .fetch_all(db)
    .await?;

    Ok(rows.into_iter().map(|row| (row.id, row.name)).collect())
}

/// Get seasons where a team is not yet participating (for dropdown)
#[allow(dead_code)]
pub async fn get_available_seasons_for_team(
    db: &SqlitePool,
    team_id: i64,
) -> Result<Vec<(i64, String)>, sqlx::Error> {
    let rows = sqlx::query!(
        r#"
        SELECT s.id as "id!", s.display_name, s.year, e.name as event_name
        FROM season s
        INNER JOIN event e ON s.event_id = e.id
        WHERE s.id NOT IN (
            SELECT season_id FROM team_participation WHERE team_id = ?
        )
        ORDER BY s.year DESC, e.name ASC
        "#,
        team_id
    )
    .fetch_all(db)
    .await?;

    Ok(rows
        .into_iter()
        .map(|row| {
            let year: i64 = row.year;
            let event_name: String = row.event_name;
            let display_name: Option<String> = row.display_name;

            let label = if let Some(dn) = display_name {
                format!("{} {} ({})", event_name, dn, year)
            } else {
                format!("{} {}", event_name, year)
            };

            (row.id, label)
        })
        .collect())
}

/// Check if a team is already participating in a season
pub async fn is_team_in_season(
    db: &SqlitePool,
    season_id: i64,
    team_id: i64,
) -> Result<bool, sqlx::Error> {
    let row = sqlx::query!(
        r#"
        SELECT COUNT(*) as count
        FROM team_participation
        WHERE season_id = ? AND team_id = ?
        "#,
        season_id,
        team_id
    )
    .fetch_one(db)
    .await?;

    Ok(row.count > 0)
}

/// Add a team to a season (create team participation)
pub async fn add_team_to_season(
    db: &SqlitePool,
    entity: CreateTeamParticipationEntity,
) -> Result<i64, sqlx::Error> {
    let result = sqlx::query!(
        "INSERT INTO team_participation (team_id, season_id, event_id) VALUES (?, ?, ?)",
        entity.team_id,
        entity.season_id,
        entity.event_id
    )
    .execute(db)
    .await?;

    Ok(result.last_insert_rowid())
}

/// Remove a team from a season (delete team participation)
pub async fn remove_team_from_season(db: &SqlitePool, id: i64) -> Result<bool, sqlx::Error> {
    let result = sqlx::query!("DELETE FROM team_participation WHERE id = ?", id)
        .execute(db)
        .await?;

    Ok(result.rows_affected() > 0)
}

/// Get the season_id for a team participation (useful for redirects after delete)
pub async fn get_season_id_for_participation(
    db: &SqlitePool,
    id: i64,
) -> Result<Option<i64>, sqlx::Error> {
    let row = sqlx::query!(
        r#"
        SELECT season_id
        FROM team_participation
        WHERE id = ?
        "#,
        id
    )
    .fetch_optional(db)
    .await?;

    Ok(row.map(|r| r.season_id))
}

/// Get all teams for dropdown (not filtered)
pub async fn get_all_teams_for_dropdown(
    db: &SqlitePool,
) -> Result<Vec<(i64, String)>, sqlx::Error> {
    let rows = sqlx::query!(
        r#"
        SELECT t.id, t.name, c.iso2Code as country_code
        FROM team t
        LEFT JOIN country c ON t.country_id = c.id
        ORDER BY t.name ASC
        "#
    )
    .fetch_all(db)
    .await?;

    Ok(rows
        .into_iter()
        .map(|row| {
            let name: String = row.name;
            let country_code: Option<String> = row.country_code;

            let display = if let Some(code) = country_code {
                format!("{} ({})", name, code)
            } else {
                name
            };

            (row.id, display)
        })
        .collect())
}

/// Get all seasons for dropdown (not filtered)
pub async fn get_all_seasons_for_dropdown(
    db: &SqlitePool,
) -> Result<Vec<(i64, String)>, sqlx::Error> {
    let rows = sqlx::query!(
        r#"
        SELECT s.id as "id!", s.display_name, s.year, e.name as event_name
        FROM season s
        INNER JOIN event e ON s.event_id = e.id
        ORDER BY s.year DESC, e.name ASC
        "#
    )
    .fetch_all(db)
    .await?;

    Ok(rows
        .into_iter()
        .map(|row| {
            let year: i64 = row.year;
            let event_name: String = row.event_name;
            let display_name: Option<String> = row.display_name;

            let label = if let Some(dn) = display_name {
                format!("{} {} ({})", event_name, dn, year)
            } else {
                format!("{} {}", event_name, year)
            };

            (row.id, label)
        })
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[sqlx::test(
        migrations = "./migrations",
        fixtures("events", "seasons", "teams", "team_participations")
    )]
    async fn test_get_teams_for_season(pool: SqlitePool) {
        // Season 1 (2022 Olympics) has Team Canada and Team USA
        let teams = get_teams_for_season(&pool, 1).await.unwrap();

        assert_eq!(teams.len(), 2);
        assert!(teams.iter().any(|t| t.team_name == "Team Canada"));
        assert!(teams.iter().any(|t| t.team_name == "Team USA"));

        // Verify teams are sorted by name
        assert!(teams[0].team_name <= teams[1].team_name);
    }

    #[sqlx::test(
        migrations = "./migrations",
        fixtures("events", "seasons", "teams", "team_participations")
    )]
    async fn test_get_teams_for_season_empty(pool: SqlitePool) {
        // Season 3 (2024 World Cup) has no teams
        let teams = get_teams_for_season(&pool, 3).await.unwrap();

        assert_eq!(teams.len(), 0);
    }

    #[sqlx::test(
        migrations = "./migrations",
        fixtures("events", "seasons", "teams", "team_participations")
    )]
    async fn test_get_available_teams_for_season(pool: SqlitePool) {
        // Season 1 has Canada and USA, so available should be Russia, Finland, Sweden
        let available = get_available_teams_for_season(&pool, 1).await.unwrap();

        assert_eq!(available.len(), 3);
        assert!(available.iter().any(|(_, name)| name == "Team Russia"));
        assert!(available.iter().any(|(_, name)| name == "Team Finland"));
        assert!(available.iter().any(|(_, name)| name == "Team Sweden"));

        // Should not include teams already in season
        assert!(!available.iter().any(|(_, name)| name == "Team Canada"));
        assert!(!available.iter().any(|(_, name)| name == "Team USA"));
    }

    #[sqlx::test(
        migrations = "./migrations",
        fixtures("events", "seasons", "teams", "team_participations")
    )]
    async fn test_get_available_teams_for_season_all_available(pool: SqlitePool) {
        // Season 3 has no teams, so all should be available
        let available = get_available_teams_for_season(&pool, 3).await.unwrap();

        assert_eq!(available.len(), 5); // All 5 fixture teams
    }

    #[sqlx::test(
        migrations = "./migrations",
        fixtures("events", "seasons", "teams", "team_participations")
    )]
    async fn test_get_available_seasons_for_team(pool: SqlitePool) {
        // Team Canada (id=1) is in season 1, so seasons 2 and 3 should be available
        let available = get_available_seasons_for_team(&pool, 1).await.unwrap();

        assert_eq!(available.len(), 2);

        // Verify labels contain event name and year
        assert!(available.iter().any(|(id, label)| {
            *id == 2 && label.contains("World Championship") && label.contains("2023")
        }));
        assert!(available.iter().any(|(id, label)| {
            *id == 3 && label.contains("World Cup") && label.contains("2024")
        }));
    }

    #[sqlx::test(
        migrations = "./migrations",
        fixtures("events", "seasons", "teams", "team_participations")
    )]
    async fn test_get_available_seasons_for_team_all_available(pool: SqlitePool) {
        // Team Sweden (id=5) is not in any season
        let available = get_available_seasons_for_team(&pool, 5).await.unwrap();

        assert_eq!(available.len(), 3); // All 3 seasons available
    }

    #[sqlx::test(
        migrations = "./migrations",
        fixtures("events", "seasons", "teams", "team_participations")
    )]
    async fn test_is_team_in_season_true(pool: SqlitePool) {
        // Team Canada is in season 1
        let result = is_team_in_season(&pool, 1, 1).await.unwrap();

        assert!(result);
    }

    #[sqlx::test(
        migrations = "./migrations",
        fixtures("events", "seasons", "teams", "team_participations")
    )]
    async fn test_is_team_in_season_false(pool: SqlitePool) {
        // Team Sweden is not in season 1
        let result = is_team_in_season(&pool, 1, 5).await.unwrap();

        assert!(!result);
    }

    #[sqlx::test(
        migrations = "./migrations",
        fixtures("events", "seasons", "teams", "team_participations")
    )]
    async fn test_add_team_to_season(pool: SqlitePool) {
        // Add Team Sweden to season 3 (World Cup event)
        let entity = CreateTeamParticipationEntity {
            team_id: 5,
            season_id: 3,
            event_id: 3,
        };

        let id = add_team_to_season(&pool, entity).await.unwrap();
        assert!(id > 0);

        // Verify team was added
        let is_in_season = is_team_in_season(&pool, 3, 5).await.unwrap();
        assert!(is_in_season);

        // Verify it appears in the season's teams list
        let teams = get_teams_for_season(&pool, 3).await.unwrap();
        assert_eq!(teams.len(), 1);
        assert_eq!(teams[0].team_name, "Team Sweden");
    }

    #[sqlx::test(
        migrations = "./migrations",
        fixtures("events", "seasons", "teams", "team_participations")
    )]
    async fn test_remove_team_from_season(pool: SqlitePool) {
        // Remove team participation id=1 (Team Canada from season 1)
        let removed = remove_team_from_season(&pool, 1).await.unwrap();
        assert!(removed);

        // Verify team was removed
        let is_in_season = is_team_in_season(&pool, 1, 1).await.unwrap();
        assert!(!is_in_season);

        // Verify it no longer appears in the season's teams list
        let teams = get_teams_for_season(&pool, 1).await.unwrap();
        assert_eq!(teams.len(), 1); // Only Team USA remains
        assert_eq!(teams[0].team_name, "Team USA");
    }

    #[sqlx::test(
        migrations = "./migrations",
        fixtures("events", "seasons", "teams", "team_participations")
    )]
    async fn test_remove_team_from_season_not_found(pool: SqlitePool) {
        // Try to remove non-existent participation
        let removed = remove_team_from_season(&pool, 999).await.unwrap();
        assert!(!removed);
    }

    #[sqlx::test(
        migrations = "./migrations",
        fixtures("events", "seasons", "teams", "team_participations")
    )]
    async fn test_get_season_id_for_participation(pool: SqlitePool) {
        // Get season_id for participation id=1 (should be season 1)
        let season_id = get_season_id_for_participation(&pool, 1).await.unwrap();

        assert_eq!(season_id, Some(1));
    }

    #[sqlx::test(
        migrations = "./migrations",
        fixtures("events", "seasons", "teams", "team_participations")
    )]
    async fn test_get_season_id_for_participation_not_found(pool: SqlitePool) {
        // Try to get season_id for non-existent participation
        let season_id = get_season_id_for_participation(&pool, 999).await.unwrap();

        assert_eq!(season_id, None);
    }

    #[sqlx::test(migrations = "./migrations", fixtures("events", "seasons", "teams"))]
    async fn test_get_all_teams_for_dropdown(pool: SqlitePool) {
        let teams = get_all_teams_for_dropdown(&pool).await.unwrap();

        assert_eq!(teams.len(), 5);

        // Verify teams are sorted by name (alphabetically)
        for i in 0..teams.len() - 1 {
            assert!(
                teams[i].1 <= teams[i + 1].1,
                "Teams should be sorted by name"
            );
        }

        // Verify all teams have proper formatting with country code in parentheses
        assert!(teams.iter().all(|(_, name)| {
            name.contains("Team") && name.contains('(') && name.contains(')')
        }));

        // Verify specific teams exist
        assert!(teams
            .iter()
            .any(|(_, name)| name.starts_with("Team Canada")));
        assert!(teams.iter().any(|(_, name)| name.starts_with("Team USA")));
    }

    #[sqlx::test(migrations = "./migrations", fixtures("events", "seasons"))]
    async fn test_get_all_seasons_for_dropdown(pool: SqlitePool) {
        let seasons = get_all_seasons_for_dropdown(&pool).await.unwrap();

        assert_eq!(seasons.len(), 3);

        // Verify seasons are sorted by year DESC (most recent first)
        assert!(seasons[0].1.contains("2024"));
        assert!(seasons[1].1.contains("2023"));
        assert!(seasons[2].1.contains("2022"));

        // Verify label format includes event name and year
        assert!(seasons.iter().all(|(_, label)| {
            (label.contains("Olympics")
                || label.contains("Championship")
                || label.contains("World Cup"))
                && (label.contains("2022") || label.contains("2023") || label.contains("2024"))
        }));
    }
}
