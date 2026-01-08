use sqlx::SqlitePool;

/// Player in a roster with additional details
#[derive(Debug, Clone)]
pub struct PlayerInRoster {
    pub player_contract_id: i64, // ID of the player_contract record
    #[allow(dead_code)]
    pub player_id: i64,
    pub player_name: String,
    #[allow(dead_code)]
    pub country_id: i64,
    pub country_name: String,
    pub country_iso2_code: String,
    pub photo_path: Option<String>,
}

/// Team participation context for roster page header
#[derive(Debug, Clone)]
pub struct TeamParticipationContext {
    pub team_participation_id: i64,
    #[allow(dead_code)]
    pub team_id: i64,
    pub team_name: String,
    pub country_iso2_code: Option<String>,
    pub season_id: i64,
    pub season_year: i64,
    pub season_display_name: Option<String>,
    #[allow(dead_code)]
    pub event_id: i64,
    pub event_name: String,
}

/// Get all players in a roster for a team participation
pub async fn get_roster(
    db: &SqlitePool,
    team_participation_id: i64,
) -> Result<Vec<PlayerInRoster>, sqlx::Error> {
    let players = sqlx::query_as!(
        PlayerInRoster,
        r#"
        SELECT
            pc.id as "player_contract_id!",
            p.id as "player_id!",
            p.name as player_name,
            p.photo_path,
            c.id as "country_id!",
            c.name as country_name,
            c.iso2Code as "country_iso2_code!"
        FROM player_contract pc
        INNER JOIN player p ON pc.player_id = p.id
        INNER JOIN country c ON p.country_id = c.id
        WHERE pc.team_participation_id = ?
        ORDER BY p.name ASC
        "#,
        team_participation_id
    )
    .fetch_all(db)
    .await?;

    Ok(players)
}

/// Get team participation context (team, event, season info)
pub async fn get_team_participation_context(
    db: &SqlitePool,
    team_participation_id: i64,
) -> Result<Option<TeamParticipationContext>, sqlx::Error> {
    let row = sqlx::query_as!(
        TeamParticipationContext,
        r#"
        SELECT
            tp.id as team_participation_id,
            t.id as team_id,
            t.name as team_name,
            c.iso2Code as country_iso2_code,
            s.id as season_id,
            s.year as season_year,
            s.display_name as season_display_name,
            e.id as event_id,
            e.name as event_name
        FROM team_participation tp
        INNER JOIN team t ON tp.team_id = t.id
        LEFT JOIN country c ON t.country_id = c.id
        INNER JOIN season s ON tp.season_id = s.id
        INNER JOIN event e ON s.event_id = e.id
        WHERE tp.id = ?
        "#,
        team_participation_id
    )
    .fetch_optional(db)
    .await?;

    Ok(row)
}

/// Get players not yet in the roster (available to add)
pub async fn get_available_players(
    db: &SqlitePool,
    team_participation_id: i64,
) -> Result<Vec<(i64, String, String)>, sqlx::Error> {
    let rows = sqlx::query!(
        r#"
        SELECT p.id, p.name, c.name as country_name
        FROM player p
        INNER JOIN country c ON p.country_id = c.id
        WHERE p.id NOT IN (
            SELECT player_id FROM player_contract WHERE team_participation_id = ?
        )
        ORDER BY p.name ASC
        "#,
        team_participation_id
    )
    .fetch_all(db)
    .await?;

    Ok(rows
        .into_iter()
        .map(|row| (row.id, row.name, row.country_name))
        .collect())
}

/// Check if a player is already in a roster
pub async fn is_player_in_roster(
    db: &SqlitePool,
    team_participation_id: i64,
    player_id: i64,
) -> Result<bool, sqlx::Error> {
    let row = sqlx::query!(
        r#"
        SELECT COUNT(*) as count
        FROM player_contract
        WHERE team_participation_id = ? AND player_id = ?
        "#,
        team_participation_id,
        player_id
    )
    .fetch_one(db)
    .await?;

    Ok(row.count > 0)
}

/// Add a player to a roster (create player_contract)
pub async fn add_player_to_roster(
    db: &SqlitePool,
    team_participation_id: i64,
    player_id: i64,
) -> Result<i64, sqlx::Error> {
    let result = sqlx::query!(
        "INSERT INTO player_contract (team_participation_id, player_id) VALUES (?, ?)",
        team_participation_id,
        player_id
    )
    .execute(db)
    .await?;

    Ok(result.last_insert_rowid())
}

/// Remove a player from a roster (delete player_contract)
pub async fn remove_player_from_roster(
    db: &SqlitePool,
    player_contract_id: i64,
) -> Result<bool, sqlx::Error> {
    let result = sqlx::query!(
        "DELETE FROM player_contract WHERE id = ?",
        player_contract_id
    )
    .execute(db)
    .await?;

    Ok(result.rows_affected() > 0)
}

/// Get team_participation_id for a player_contract (useful for redirects after delete)
pub async fn get_team_participation_id_for_contract(
    db: &SqlitePool,
    player_contract_id: i64,
) -> Result<Option<i64>, sqlx::Error> {
    let row = sqlx::query!(
        r#"
        SELECT team_participation_id
        FROM player_contract
        WHERE id = ?
        "#,
        player_contract_id
    )
    .fetch_optional(db)
    .await?;

    Ok(row.map(|r| r.team_participation_id))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[sqlx::test(migrations = "./migrations")]
    async fn test_get_roster_empty(pool: SqlitePool) {
        // Create minimal test data
        let country_id = sqlx::query_scalar::<_, i64>(
            "INSERT INTO country (name, iihf) VALUES ('Canada', 1) RETURNING id",
        )
        .fetch_one(&pool)
        .await
        .unwrap();

        let team_id = sqlx::query_scalar::<_, i64>(
            "INSERT INTO team (name, country_id) VALUES ('Team Canada', ?) RETURNING id",
        )
        .bind(country_id)
        .fetch_one(&pool)
        .await
        .unwrap();

        let event_id = sqlx::query_scalar::<_, i64>(
            "INSERT INTO event (name, country_id) VALUES ('Olympics', ?) RETURNING id",
        )
        .bind(country_id)
        .fetch_one(&pool)
        .await
        .unwrap();

        let season_id = sqlx::query_scalar::<_, i64>(
            "INSERT INTO season (year, event_id) VALUES (2024, ?) RETURNING id",
        )
        .bind(event_id)
        .fetch_one(&pool)
        .await
        .unwrap();

        let participation_id = sqlx::query_scalar::<_, i64>(
            "INSERT INTO team_participation (team_id, season_id) VALUES (?, ?) RETURNING id",
        )
        .bind(team_id)
        .bind(season_id)
        .fetch_one(&pool)
        .await
        .unwrap();

        let roster = get_roster(&pool, participation_id).await.unwrap();
        assert_eq!(roster.len(), 0);
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn test_get_roster_with_players(pool: SqlitePool) {
        // Create test data
        let country_id = sqlx::query_scalar::<_, i64>(
            "INSERT INTO country (name, iihf, iso2Code) VALUES ('Canada', 1, 'CA') RETURNING id",
        )
        .fetch_one(&pool)
        .await
        .unwrap();

        let team_id = sqlx::query_scalar::<_, i64>(
            "INSERT INTO team (name, country_id) VALUES ('Team Canada', ?) RETURNING id",
        )
        .bind(country_id)
        .fetch_one(&pool)
        .await
        .unwrap();

        let event_id = sqlx::query_scalar::<_, i64>(
            "INSERT INTO event (name, country_id) VALUES ('Olympics', ?) RETURNING id",
        )
        .bind(country_id)
        .fetch_one(&pool)
        .await
        .unwrap();

        let season_id = sqlx::query_scalar::<_, i64>(
            "INSERT INTO season (year, event_id) VALUES (2024, ?) RETURNING id",
        )
        .bind(event_id)
        .fetch_one(&pool)
        .await
        .unwrap();

        let participation_id = sqlx::query_scalar::<_, i64>(
            "INSERT INTO team_participation (team_id, season_id) VALUES (?, ?) RETURNING id",
        )
        .bind(team_id)
        .bind(season_id)
        .fetch_one(&pool)
        .await
        .unwrap();

        // Add players
        let player1_id = sqlx::query_scalar::<_, i64>(
            "INSERT INTO player (name, country_id) VALUES ('Wayne Gretzky', ?) RETURNING id",
        )
        .bind(country_id)
        .fetch_one(&pool)
        .await
        .unwrap();

        let player2_id = sqlx::query_scalar::<_, i64>(
            "INSERT INTO player (name, country_id) VALUES ('Mario Lemieux', ?) RETURNING id",
        )
        .bind(country_id)
        .fetch_one(&pool)
        .await
        .unwrap();

        // Add to roster
        add_player_to_roster(&pool, participation_id, player1_id)
            .await
            .unwrap();
        add_player_to_roster(&pool, participation_id, player2_id)
            .await
            .unwrap();

        let roster = get_roster(&pool, participation_id).await.unwrap();
        assert_eq!(roster.len(), 2);
        assert_eq!(roster[0].player_name, "Mario Lemieux"); // Sorted alphabetically
        assert_eq!(roster[1].player_name, "Wayne Gretzky");
        assert_eq!(roster[0].country_name, "Canada");
        assert_eq!(roster[0].country_iso2_code, "CA");
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn test_get_team_participation_context(pool: SqlitePool) {
        // Create test data
        let country_id = sqlx::query_scalar::<_, i64>(
            "INSERT INTO country (name, iihf, iso2Code) VALUES ('Canada', 1, 'CA') RETURNING id",
        )
        .fetch_one(&pool)
        .await
        .unwrap();

        let team_id = sqlx::query_scalar::<_, i64>(
            "INSERT INTO team (name, country_id) VALUES ('Team Canada', ?) RETURNING id",
        )
        .bind(country_id)
        .fetch_one(&pool)
        .await
        .unwrap();

        let event_id = sqlx::query_scalar::<_, i64>(
            "INSERT INTO event (name, country_id) VALUES ('Olympics', ?) RETURNING id",
        )
        .bind(country_id)
        .fetch_one(&pool)
        .await
        .unwrap();

        let season_id = sqlx::query_scalar::<_, i64>(
            "INSERT INTO season (year, event_id, display_name) VALUES (2024, ?, 'Olympics 2024') RETURNING id",
        )
        .bind(event_id)
        .fetch_one(&pool)
        .await
        .unwrap();

        let participation_id = sqlx::query_scalar::<_, i64>(
            "INSERT INTO team_participation (team_id, season_id) VALUES (?, ?) RETURNING id",
        )
        .bind(team_id)
        .bind(season_id)
        .fetch_one(&pool)
        .await
        .unwrap();

        let context = get_team_participation_context(&pool, participation_id)
            .await
            .unwrap();

        assert!(context.is_some());
        let ctx = context.unwrap();
        assert_eq!(ctx.team_name, "Team Canada");
        assert_eq!(ctx.event_name, "Olympics");
        assert_eq!(ctx.season_year, 2024);
        assert_eq!(ctx.season_display_name, Some("Olympics 2024".to_string()));
        assert_eq!(ctx.country_iso2_code, Some("CA".to_string()));
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn test_get_team_participation_context_not_found(pool: SqlitePool) {
        let context = get_team_participation_context(&pool, 999).await.unwrap();
        assert!(context.is_none());
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn test_get_available_players(pool: SqlitePool) {
        // Create test data
        let country_id = sqlx::query_scalar::<_, i64>(
            "INSERT INTO country (name, iihf) VALUES ('Canada', 1) RETURNING id",
        )
        .fetch_one(&pool)
        .await
        .unwrap();

        let team_id = sqlx::query_scalar::<_, i64>(
            "INSERT INTO team (name, country_id) VALUES ('Team Canada', ?) RETURNING id",
        )
        .bind(country_id)
        .fetch_one(&pool)
        .await
        .unwrap();

        let event_id = sqlx::query_scalar::<_, i64>(
            "INSERT INTO event (name, country_id) VALUES ('Olympics', ?) RETURNING id",
        )
        .bind(country_id)
        .fetch_one(&pool)
        .await
        .unwrap();

        let season_id = sqlx::query_scalar::<_, i64>(
            "INSERT INTO season (year, event_id) VALUES (2024, ?) RETURNING id",
        )
        .bind(event_id)
        .fetch_one(&pool)
        .await
        .unwrap();

        let participation_id = sqlx::query_scalar::<_, i64>(
            "INSERT INTO team_participation (team_id, season_id) VALUES (?, ?) RETURNING id",
        )
        .bind(team_id)
        .bind(season_id)
        .fetch_one(&pool)
        .await
        .unwrap();

        // Create 3 players
        let player1_id = sqlx::query_scalar::<_, i64>(
            "INSERT INTO player (name, country_id) VALUES ('Player 1', ?) RETURNING id",
        )
        .bind(country_id)
        .fetch_one(&pool)
        .await
        .unwrap();

        sqlx::query("INSERT INTO player (name, country_id) VALUES ('Player 2', ?)")
            .bind(country_id)
            .execute(&pool)
            .await
            .unwrap();

        sqlx::query("INSERT INTO player (name, country_id) VALUES ('Player 3', ?)")
            .bind(country_id)
            .execute(&pool)
            .await
            .unwrap();

        // Add player1 to roster
        add_player_to_roster(&pool, participation_id, player1_id)
            .await
            .unwrap();

        // Get available players (should be 2)
        let available = get_available_players(&pool, participation_id)
            .await
            .unwrap();
        assert_eq!(available.len(), 2);
        // Should not include Player 1
        assert!(!available.iter().any(|(_, name, _)| name == "Player 1"));
        assert!(available.iter().any(|(_, name, _)| name == "Player 2"));
        assert!(available.iter().any(|(_, name, _)| name == "Player 3"));
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn test_is_player_in_roster(pool: SqlitePool) {
        // Create test data
        let country_id = sqlx::query_scalar::<_, i64>(
            "INSERT INTO country (name, iihf) VALUES ('Canada', 1) RETURNING id",
        )
        .fetch_one(&pool)
        .await
        .unwrap();

        let team_id = sqlx::query_scalar::<_, i64>(
            "INSERT INTO team (name, country_id) VALUES ('Team Canada', ?) RETURNING id",
        )
        .bind(country_id)
        .fetch_one(&pool)
        .await
        .unwrap();

        let event_id = sqlx::query_scalar::<_, i64>(
            "INSERT INTO event (name, country_id) VALUES ('Olympics', ?) RETURNING id",
        )
        .bind(country_id)
        .fetch_one(&pool)
        .await
        .unwrap();

        let season_id = sqlx::query_scalar::<_, i64>(
            "INSERT INTO season (year, event_id) VALUES (2024, ?) RETURNING id",
        )
        .bind(event_id)
        .fetch_one(&pool)
        .await
        .unwrap();

        let participation_id = sqlx::query_scalar::<_, i64>(
            "INSERT INTO team_participation (team_id, season_id) VALUES (?, ?) RETURNING id",
        )
        .bind(team_id)
        .bind(season_id)
        .fetch_one(&pool)
        .await
        .unwrap();

        let player1_id = sqlx::query_scalar::<_, i64>(
            "INSERT INTO player (name, country_id) VALUES ('Player 1', ?) RETURNING id",
        )
        .bind(country_id)
        .fetch_one(&pool)
        .await
        .unwrap();

        let player2_id = sqlx::query_scalar::<_, i64>(
            "INSERT INTO player (name, country_id) VALUES ('Player 2', ?) RETURNING id",
        )
        .bind(country_id)
        .fetch_one(&pool)
        .await
        .unwrap();

        // Add player1 to roster
        add_player_to_roster(&pool, participation_id, player1_id)
            .await
            .unwrap();

        // Check
        assert!(is_player_in_roster(&pool, participation_id, player1_id)
            .await
            .unwrap());
        assert!(!is_player_in_roster(&pool, participation_id, player2_id)
            .await
            .unwrap());
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn test_add_player_to_roster(pool: SqlitePool) {
        // Create test data
        let country_id = sqlx::query_scalar::<_, i64>(
            "INSERT INTO country (name, iihf) VALUES ('Canada', 1) RETURNING id",
        )
        .fetch_one(&pool)
        .await
        .unwrap();

        let team_id = sqlx::query_scalar::<_, i64>(
            "INSERT INTO team (name, country_id) VALUES ('Team Canada', ?) RETURNING id",
        )
        .bind(country_id)
        .fetch_one(&pool)
        .await
        .unwrap();

        let event_id = sqlx::query_scalar::<_, i64>(
            "INSERT INTO event (name, country_id) VALUES ('Olympics', ?) RETURNING id",
        )
        .bind(country_id)
        .fetch_one(&pool)
        .await
        .unwrap();

        let season_id = sqlx::query_scalar::<_, i64>(
            "INSERT INTO season (year, event_id) VALUES (2024, ?) RETURNING id",
        )
        .bind(event_id)
        .fetch_one(&pool)
        .await
        .unwrap();

        let participation_id = sqlx::query_scalar::<_, i64>(
            "INSERT INTO team_participation (team_id, season_id) VALUES (?, ?) RETURNING id",
        )
        .bind(team_id)
        .bind(season_id)
        .fetch_one(&pool)
        .await
        .unwrap();

        let player_id = sqlx::query_scalar::<_, i64>(
            "INSERT INTO player (name, country_id) VALUES ('Test Player', ?) RETURNING id",
        )
        .bind(country_id)
        .fetch_one(&pool)
        .await
        .unwrap();

        let contract_id = add_player_to_roster(&pool, participation_id, player_id)
            .await
            .unwrap();

        assert!(contract_id > 0);

        // Verify it was added
        let count =
            sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM player_contract WHERE id = ?")
                .bind(contract_id)
                .fetch_one(&pool)
                .await
                .unwrap();
        assert_eq!(count, 1);
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn test_remove_player_from_roster(pool: SqlitePool) {
        // Create test data
        let country_id = sqlx::query_scalar::<_, i64>(
            "INSERT INTO country (name, iihf) VALUES ('Canada', 1) RETURNING id",
        )
        .fetch_one(&pool)
        .await
        .unwrap();

        let team_id = sqlx::query_scalar::<_, i64>(
            "INSERT INTO team (name, country_id) VALUES ('Team Canada', ?) RETURNING id",
        )
        .bind(country_id)
        .fetch_one(&pool)
        .await
        .unwrap();

        let event_id = sqlx::query_scalar::<_, i64>(
            "INSERT INTO event (name, country_id) VALUES ('Olympics', ?) RETURNING id",
        )
        .bind(country_id)
        .fetch_one(&pool)
        .await
        .unwrap();

        let season_id = sqlx::query_scalar::<_, i64>(
            "INSERT INTO season (year, event_id) VALUES (2024, ?) RETURNING id",
        )
        .bind(event_id)
        .fetch_one(&pool)
        .await
        .unwrap();

        let participation_id = sqlx::query_scalar::<_, i64>(
            "INSERT INTO team_participation (team_id, season_id) VALUES (?, ?) RETURNING id",
        )
        .bind(team_id)
        .bind(season_id)
        .fetch_one(&pool)
        .await
        .unwrap();

        let player_id = sqlx::query_scalar::<_, i64>(
            "INSERT INTO player (name, country_id) VALUES ('Test Player', ?) RETURNING id",
        )
        .bind(country_id)
        .fetch_one(&pool)
        .await
        .unwrap();

        let contract_id = add_player_to_roster(&pool, participation_id, player_id)
            .await
            .unwrap();

        // Remove the player
        let removed = remove_player_from_roster(&pool, contract_id).await.unwrap();
        assert!(removed);

        // Verify it was removed
        let count =
            sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM player_contract WHERE id = ?")
                .bind(contract_id)
                .fetch_one(&pool)
                .await
                .unwrap();
        assert_eq!(count, 0);
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn test_remove_player_from_roster_not_found(pool: SqlitePool) {
        let removed = remove_player_from_roster(&pool, 999).await.unwrap();
        assert!(!removed);
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn test_get_team_participation_id_for_contract(pool: SqlitePool) {
        // Create test data
        let country_id = sqlx::query_scalar::<_, i64>(
            "INSERT INTO country (name, iihf) VALUES ('Canada', 1) RETURNING id",
        )
        .fetch_one(&pool)
        .await
        .unwrap();

        let team_id = sqlx::query_scalar::<_, i64>(
            "INSERT INTO team (name, country_id) VALUES ('Team Canada', ?) RETURNING id",
        )
        .bind(country_id)
        .fetch_one(&pool)
        .await
        .unwrap();

        let event_id = sqlx::query_scalar::<_, i64>(
            "INSERT INTO event (name, country_id) VALUES ('Olympics', ?) RETURNING id",
        )
        .bind(country_id)
        .fetch_one(&pool)
        .await
        .unwrap();

        let season_id = sqlx::query_scalar::<_, i64>(
            "INSERT INTO season (year, event_id) VALUES (2024, ?) RETURNING id",
        )
        .bind(event_id)
        .fetch_one(&pool)
        .await
        .unwrap();

        let participation_id = sqlx::query_scalar::<_, i64>(
            "INSERT INTO team_participation (team_id, season_id) VALUES (?, ?) RETURNING id",
        )
        .bind(team_id)
        .bind(season_id)
        .fetch_one(&pool)
        .await
        .unwrap();

        let player_id = sqlx::query_scalar::<_, i64>(
            "INSERT INTO player (name, country_id) VALUES ('Test Player', ?) RETURNING id",
        )
        .bind(country_id)
        .fetch_one(&pool)
        .await
        .unwrap();

        let contract_id = add_player_to_roster(&pool, participation_id, player_id)
            .await
            .unwrap();

        // Get participation ID from contract
        let result = get_team_participation_id_for_contract(&pool, contract_id)
            .await
            .unwrap();

        assert_eq!(result, Some(participation_id));
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn test_get_team_participation_id_for_contract_not_found(pool: SqlitePool) {
        let result = get_team_participation_id_for_contract(&pool, 999)
            .await
            .unwrap();
        assert!(result.is_none());
    }
}
