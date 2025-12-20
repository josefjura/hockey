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
    let teams = sqlx::query_as!(
        TeamParticipationEntity,
        r#"
        SELECT
            tp.id,
            tp.team_id,
            t.name as "team_name!",
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

    Ok(teams)
}

/// Get teams that are not yet participating in a season (for dropdown)
pub async fn get_available_teams_for_season(
    db: &SqlitePool,
    season_id: i64,
) -> Result<Vec<(i64, String)>, sqlx::Error> {
    let teams = sqlx::query_as::<_, (i64, String)>(
        r#"
        SELECT t.id, t.name
        FROM team t
        WHERE t.id NOT IN (
            SELECT team_id FROM team_participation WHERE season_id = ?
        )
        ORDER BY t.name ASC
        "#,
    )
    .bind(season_id)
    .fetch_all(db)
    .await?;

    Ok(teams)
}

/// Check if a team is already participating in a season
pub async fn is_team_in_season(
    db: &SqlitePool,
    season_id: i64,
    team_id: i64,
) -> Result<bool, sqlx::Error> {
    let result = sqlx::query_scalar::<_, i64>(
        r#"
        SELECT COUNT(*)
        FROM team_participation
        WHERE season_id = ? AND team_id = ?
        "#,
    )
    .bind(season_id)
    .bind(team_id)
    .fetch_one(db)
    .await?;

    Ok(result > 0)
}

/// Add a team to a season (create team participation)
pub async fn add_team_to_season(
    db: &SqlitePool,
    entity: CreateTeamParticipationEntity,
) -> Result<i64, sqlx::Error> {
    let result = sqlx::query!(
        r#"
        INSERT INTO team_participation (team_id, season_id)
        VALUES (?, ?)
        "#,
        entity.team_id,
        entity.season_id
    )
    .execute(db)
    .await?;

    Ok(result.last_insert_rowid())
}

/// Remove a team from a season (delete team participation)
pub async fn remove_team_from_season(db: &SqlitePool, id: i64) -> Result<bool, sqlx::Error> {
    let result = sqlx::query!(
        r#"
        DELETE FROM team_participation
        WHERE id = ?
        "#,
        id
    )
    .execute(db)
    .await?;

    Ok(result.rows_affected() > 0)
}

/// Get the season_id for a team participation (useful for redirects after delete)
pub async fn get_season_id_for_participation(
    db: &SqlitePool,
    id: i64,
) -> Result<Option<i64>, sqlx::Error> {
    let result = sqlx::query_scalar::<_, i64>(
        r#"
        SELECT season_id
        FROM team_participation
        WHERE id = ?
        "#,
    )
    .bind(id)
    .fetch_optional(db)
    .await?;

    Ok(result)
}
