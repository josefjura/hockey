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
    let rows = sqlx::query!(
        r#"
        SELECT
            pc.id as player_contract_id,
            p.id as player_id,
            p.name as "player_name!: String",
            p.photo_path,
            c.id as country_id,
            c.name as "country_name!: String",
            c.iso2Code as "country_iso2_code!: String"
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

    let players = rows
        .into_iter()
        .map(|row| PlayerInRoster {
            player_contract_id: row.player_contract_id,
            player_id: row.player_id,
            player_name: row.player_name,
            country_id: row.country_id,
            country_name: row.country_name,
            country_iso2_code: row.country_iso2_code,
            photo_path: row.photo_path,
        })
        .collect();

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
            t.name as "team_name!: String",
            c.iso2Code as country_iso2_code,
            s.id as season_id,
            s.year as season_year,
            s.display_name as season_display_name,
            e.id as event_id,
            e.name as "event_name!: String"
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
