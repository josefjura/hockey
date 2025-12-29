use crate::common::pagination::{PagedResult, SortOrder};
use sqlx::{QueryBuilder, Row, SqlitePool};

use super::entities::{MatchEntity, MatchFilters, ScoreEventEntity, SortField};

/// Get a single match by ID with all related details
pub async fn get_match_by_id(db: &SqlitePool, id: i64) -> Result<Option<MatchEntity>, sqlx::Error> {
    let row = sqlx::query!(
        r#"
        SELECT
            m.id as "id!: i64",
            m.season_id as "season_id!: i64",
            COALESCE(s.display_name, CAST(s.year AS TEXT)) as season_name,
            e.name as event_name,
            m.home_team_id as "home_team_id!: i64",
            ht.name as "home_team_name!: String",
            hc.iso2Code as home_team_country_iso2,
            m.away_team_id as "away_team_id!: i64",
            at.name as "away_team_name!: String",
            ac.iso2Code as away_team_country_iso2,
            m.home_score_unidentified as "home_score_unidentified!: i32",
            m.away_score_unidentified as "away_score_unidentified!: i32",
            m.match_date,
            m.status as "status!: String",
            m.venue
        FROM match m
        INNER JOIN team ht ON m.home_team_id = ht.id
        INNER JOIN team at ON m.away_team_id = at.id
        LEFT JOIN country hc ON ht.country_id = hc.id
        LEFT JOIN country ac ON at.country_id = ac.id
        LEFT JOIN season s ON m.season_id = s.id
        LEFT JOIN event e ON s.event_id = e.id
        WHERE m.id = ?
        "#,
        id
    )
    .fetch_optional(db)
    .await?;

    Ok(row.map(|row| MatchEntity {
        id: row.id,
        season_id: row.season_id,
        season_name: row.season_name,
        event_name: row.event_name,
        home_team_id: row.home_team_id,
        home_team_name: row.home_team_name,
        home_team_country_iso2: row.home_team_country_iso2,
        away_team_id: row.away_team_id,
        away_team_name: row.away_team_name,
        away_team_country_iso2: row.away_team_country_iso2,
        home_score_unidentified: row.home_score_unidentified,
        away_score_unidentified: row.away_score_unidentified,
        match_date: row.match_date,
        status: row.status,
        venue: row.venue,
    }))
}

/// Get all score events for a match
pub async fn get_score_events(
    db: &SqlitePool,
    match_id: i64,
) -> Result<Vec<ScoreEventEntity>, sqlx::Error> {
    let rows = sqlx::query!(
        r#"
        SELECT
            se.id as "id!: i64",
            se.match_id as "match_id!: i64",
            se.team_id as "team_id!: i64",
            t.name as "team_name!: String",
            se.scorer_id,
            scorer.name as scorer_name,
            se.assist1_id,
            assist1.name as assist1_name,
            se.assist2_id,
            assist2.name as assist2_name,
            se.period as "period!: i32",
            se.time_minutes as "time_minutes: i32",
            se.time_seconds as "time_seconds: i32",
            se.goal_type
        FROM score_event se
        INNER JOIN team t ON se.team_id = t.id
        LEFT JOIN player scorer ON se.scorer_id = scorer.id
        LEFT JOIN player assist1 ON se.assist1_id = assist1.id
        LEFT JOIN player assist2 ON se.assist2_id = assist2.id
        WHERE se.match_id = ?
        ORDER BY se.period ASC, se.time_minutes ASC, se.time_seconds ASC
        "#,
        match_id
    )
    .fetch_all(db)
    .await?;

    Ok(rows
        .into_iter()
        .map(|row| ScoreEventEntity {
            id: row.id,
            match_id: row.match_id,
            team_id: row.team_id,
            team_name: row.team_name,
            scorer_id: row.scorer_id,
            scorer_name: Some(row.scorer_name),
            assist1_id: row.assist1_id,
            assist1_name: Some(row.assist1_name),
            assist2_id: row.assist2_id,
            assist2_name: Some(row.assist2_name),
            period: row.period,
            time_minutes: row.time_minutes,
            time_seconds: row.time_seconds,
            goal_type: row.goal_type,
        })
        .collect())
}

/// Get match detail with calculated scores
pub async fn get_match_detail(
    db: &SqlitePool,
    id: i64,
) -> Result<Option<super::entities::MatchDetailEntity>, sqlx::Error> {
    let match_info = match get_match_by_id(db, id).await? {
        Some(m) => m,
        None => return Ok(None),
    };

    let score_events = get_score_events(db, id).await?;

    // Calculate identified scores from score events
    let home_score_identified = score_events
        .iter()
        .filter(|se| se.team_id == match_info.home_team_id)
        .count() as i32;

    let away_score_identified = score_events
        .iter()
        .filter(|se| se.team_id == match_info.away_team_id)
        .count() as i32;

    let home_score_total = home_score_identified + match_info.home_score_unidentified;
    let away_score_total = away_score_identified + match_info.away_score_unidentified;

    Ok(Some(super::entities::MatchDetailEntity {
        match_info,
        score_events,
        home_score_identified,
        away_score_identified,
        home_score_total,
        away_score_total,
    }))
}

/// Get matches with filtering, sorting, and pagination
pub async fn get_matches(
    db: &SqlitePool,
    filters: &MatchFilters,
    sort_field: &SortField,
    sort_order: &SortOrder,
    page: usize,
    page_size: usize,
) -> Result<PagedResult<MatchEntity>, sqlx::Error> {
    // Build the base query
    let mut count_query = QueryBuilder::new(
        "SELECT COUNT(*) as total FROM match m \
         LEFT JOIN season s ON m.season_id = s.id \
         LEFT JOIN event e ON s.event_id = e.id WHERE 1=1",
    );

    let mut data_query = QueryBuilder::new(
        "SELECT \
            m.id, m.season_id, COALESCE(s.display_name, CAST(s.year AS TEXT)) as season_name, e.name as event_name, \
            m.home_team_id, ht.name as home_team_name, hc.iso2Code as home_team_country_iso2, \
            m.away_team_id, at.name as away_team_name, ac.iso2Code as away_team_country_iso2, \
            m.home_score_unidentified, m.away_score_unidentified, \
            m.match_date, m.status, m.venue \
         FROM match m \
         INNER JOIN team ht ON m.home_team_id = ht.id \
         INNER JOIN team at ON m.away_team_id = at.id \
         LEFT JOIN country hc ON ht.country_id = hc.id \
         LEFT JOIN country ac ON at.country_id = ac.id \
         LEFT JOIN season s ON m.season_id = s.id \
         LEFT JOIN event e ON s.event_id = e.id \
         WHERE 1=1"
    );

    // Apply filters
    if let Some(season_id) = filters.season_id {
        count_query.push(" AND m.season_id = ").push_bind(season_id);
        data_query.push(" AND m.season_id = ").push_bind(season_id);
    }

    if let Some(team_id) = filters.team_id {
        count_query
            .push(" AND (m.home_team_id = ")
            .push_bind(team_id)
            .push(" OR m.away_team_id = ")
            .push_bind(team_id)
            .push(")");
        data_query
            .push(" AND (m.home_team_id = ")
            .push_bind(team_id)
            .push(" OR m.away_team_id = ")
            .push_bind(team_id)
            .push(")");
    }

    if let Some(status) = &filters.status {
        count_query.push(" AND m.status = ").push_bind(status);
        data_query.push(" AND m.status = ").push_bind(status);
    }

    if let Some(date_from) = &filters.date_from {
        count_query
            .push(" AND m.match_date >= ")
            .push_bind(date_from);
        data_query
            .push(" AND m.match_date >= ")
            .push_bind(date_from);
    }

    if let Some(date_to) = &filters.date_to {
        count_query.push(" AND m.match_date <= ").push_bind(date_to);
        data_query.push(" AND m.match_date <= ").push_bind(date_to);
    }

    // Get total count
    let count_row = count_query.build().fetch_one(db).await?;
    let total: i64 = count_row.get("total");

    // Add sorting
    data_query
        .push(" ORDER BY ")
        .push(sort_field.to_sql())
        .push(" ")
        .push(sort_order.to_sql());

    // Add pagination
    let offset = (page - 1) * page_size;
    data_query
        .push(" LIMIT ")
        .push_bind(page_size as i64)
        .push(" OFFSET ")
        .push_bind(offset as i64);

    // Execute data query
    let rows = data_query.build().fetch_all(db).await?;

    let items: Vec<MatchEntity> = rows
        .into_iter()
        .map(|row| MatchEntity {
            id: row.get("id"),
            season_id: row.get("season_id"),
            season_name: row.get("season_name"),
            event_name: row.get("event_name"),
            home_team_id: row.get("home_team_id"),
            home_team_name: row.get("home_team_name"),
            home_team_country_iso2: row.get("home_team_country_iso2"),
            away_team_id: row.get("away_team_id"),
            away_team_name: row.get("away_team_name"),
            away_team_country_iso2: row.get("away_team_country_iso2"),
            home_score_unidentified: row.get("home_score_unidentified"),
            away_score_unidentified: row.get("away_score_unidentified"),
            match_date: row.get("match_date"),
            status: row.get("status"),
            venue: row.get("venue"),
        })
        .collect();

    Ok(PagedResult::new(items, total as usize, page, page_size))
}

/// Check if both teams participate in the given season (for validation)
pub async fn validate_teams_in_season(
    db: &SqlitePool,
    season_id: i64,
    home_team_id: i64,
    away_team_id: i64,
) -> Result<bool, sqlx::Error> {
    let row = sqlx::query!(
        r#"
        SELECT COUNT(*) as count
        FROM team_participation
        WHERE season_id = ? AND (team_id = ? OR team_id = ?)
        "#,
        season_id,
        home_team_id,
        away_team_id
    )
    .fetch_one(db)
    .await?;

    // Should be exactly 2 if both teams participate
    Ok(row.count == 2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::service::matches::CreateMatchEntity;
    use sqlx::SqlitePool;

    #[sqlx::test(
        migrations = "./migrations",
        fixtures("events", "seasons", "teams", "team_participations")
    )]
    async fn test_get_match_by_id_found(pool: SqlitePool) {
        // Create a match first
        let create_match = CreateMatchEntity {
            season_id: 1,
            home_team_id: 1,
            away_team_id: 2,
            home_score_unidentified: 0,
            away_score_unidentified: 0,
            match_date: Some("2024-01-15".to_string()),
            status: "scheduled".to_string(),
            venue: Some("Test Arena".to_string()),
        };
        let id = crate::service::matches::create_match(&pool, create_match)
            .await
            .unwrap();

        let result = get_match_by_id(&pool, id).await.unwrap();
        assert!(result.is_some());
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn test_get_match_by_id_not_found(pool: SqlitePool) {
        let result = get_match_by_id(&pool, 999).await.unwrap();
        assert!(result.is_none());
    }

    #[sqlx::test(
        migrations = "./migrations",
        fixtures("events", "seasons", "teams", "team_participations")
    )]
    async fn test_get_matches(pool: SqlitePool) {
        let filters = MatchFilters {
            season_id: None,
            team_id: None,
            status: None,
            date_from: None,
            date_to: None,
        };
        let result = get_matches(&pool, &filters, &SortField::Date, &SortOrder::Desc, 1, 20)
            .await
            .unwrap();

        // Just verify the query executes successfully - total is usize so always >= 0
        assert!(result.items.len() <= result.total);
    }

    #[sqlx::test(
        migrations = "./migrations",
        fixtures("events", "seasons", "teams", "team_participations")
    )]
    async fn test_validate_teams_in_season_both_present(pool: SqlitePool) {
        // Both teams participate in season 1 (from fixtures)
        let result = validate_teams_in_season(&pool, 1, 1, 2).await.unwrap();
        assert!(result);
    }

    #[sqlx::test(
        migrations = "./migrations",
        fixtures("events", "seasons", "teams", "team_participations")
    )]
    async fn test_validate_teams_in_season_not_both_present(pool: SqlitePool) {
        // Team 999 doesn't exist
        let result = validate_teams_in_season(&pool, 1, 1, 999).await.unwrap();
        assert!(!result);
    }
}
