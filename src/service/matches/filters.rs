use sqlx::SqlitePool;

/// Get all seasons for filter dropdown
pub async fn get_seasons(db: &SqlitePool) -> Result<Vec<(i64, String)>, sqlx::Error> {
    let rows = sqlx::query!(
        r#"
        SELECT s.id as "id!: i64", COALESCE(s.display_name, CAST(s.year AS TEXT)) as name
        FROM season s
        ORDER BY s.year DESC
        "#
    )
    .fetch_all(db)
    .await?;

    Ok(rows.into_iter().map(|row| (row.id, row.name)).collect())
}

/// Get all teams for filter dropdown
pub async fn get_teams(db: &SqlitePool) -> Result<Vec<(i64, String)>, sqlx::Error> {
    let rows = sqlx::query!(
        r#"
        SELECT t.id, t.name
        FROM team t
        ORDER BY t.name ASC
        "#
    )
    .fetch_all(db)
    .await?;

    Ok(rows.into_iter().map(|row| (row.id, row.name)).collect())
}

/// Get teams participating in a specific season (for match creation/editing)
pub async fn get_teams_for_season(
    db: &SqlitePool,
    season_id: i64,
) -> Result<Vec<(i64, String)>, sqlx::Error> {
    let rows = sqlx::query!(
        r#"
        SELECT DISTINCT t.id as "id!", t.name
        FROM team t
        INNER JOIN team_participation tp ON t.id = tp.team_id
        WHERE tp.season_id = ?
        ORDER BY t.name ASC
        "#,
        season_id
    )
    .fetch_all(db)
    .await?;

    Ok(rows.into_iter().map(|row| (row.id, row.name)).collect())
}

/// Get players for a specific team participation (for dropdowns)
pub async fn get_players_for_team(
    db: &SqlitePool,
    team_id: i64,
) -> Result<Vec<(i64, String)>, sqlx::Error> {
    let rows = sqlx::query!(
        r#"
        SELECT DISTINCT p.id as "id!", p.name
        FROM player p
        INNER JOIN player_contract pc ON p.id = pc.player_id
        INNER JOIN team_participation tp ON pc.team_participation_id = tp.id
        WHERE tp.team_id = ?
        ORDER BY p.name ASC
        "#,
        team_id
    )
    .fetch_all(db)
    .await?;

    Ok(rows.into_iter().map(|row| (row.id, row.name)).collect())
}
