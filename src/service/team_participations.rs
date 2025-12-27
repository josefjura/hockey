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
}

/// Get all teams participating in a season with team details
pub async fn get_teams_for_season(
    db: &SqlitePool,
    season_id: i64,
) -> Result<Vec<TeamParticipationEntity>, sqlx::Error> {
    let rows = sqlx::query!(
        r#"
        SELECT
            tp.id,
            tp.team_id,
            t.name as "team_name!: String",
            t.country_id,
            c.iso2Code as country_iso2_code,
            tp.season_id
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

    let teams = rows
        .into_iter()
        .map(|row| TeamParticipationEntity {
            id: row.id,
            team_id: row.team_id,
            team_name: row.team_name,
            country_id: row.country_id,
            country_iso2_code: row.country_iso2_code,
            season_id: row.season_id,
        })
        .collect();

    Ok(teams)
}

/// Get teams that are not yet participating in a season (for dropdown)
pub async fn get_available_teams_for_season(
    db: &SqlitePool,
    season_id: i64,
) -> Result<Vec<(i64, String)>, sqlx::Error> {
    let rows = sqlx::query!(
        r#"
        SELECT t.id, t.name as "name!: String"
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
        SELECT s.id, s.display_name, s.year, e.name as event_name
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
        "INSERT INTO team_participation (team_id, season_id) VALUES (?, ?)",
        entity.team_id,
        entity.season_id
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
        SELECT t.id, t.name as "name!: String", c.iso2Code as country_code
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
        SELECT s.id, s.display_name, s.year, e.name as event_name
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
