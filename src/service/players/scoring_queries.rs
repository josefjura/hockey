use crate::common::pagination::{PagedResult, SortOrder};
use sqlx::{QueryBuilder, Row, SqlitePool};

use super::scoring_entities::{
    PlayerScoringEventEntity, PlayerScoringFilters, PlayerSeasonStats, ScoringEventSortField,
};

/// Get player statistics grouped by season
pub async fn get_player_season_stats(
    db: &SqlitePool,
    player_id: i64,
) -> Result<Vec<PlayerSeasonStats>, sqlx::Error> {
    let rows = sqlx::query!(
        r#"
        SELECT
            s.id as "season_id!: i64",
            s.year as "season_year!: i64",
            s.display_name as season_display_name,
            e.id as "event_id!: i64",
            e.name as "event_name!: String",
            COALESCE(SUM(CASE WHEN se.scorer_id = ? THEN 1 ELSE 0 END), 0) as "goals!: i32",
            COALESCE(SUM(CASE WHEN se.assist1_id = ? OR se.assist2_id = ? THEN 1 ELSE 0 END), 0) as "assists!: i32"
        FROM season s
        INNER JOIN event e ON s.event_id = e.id
        INNER JOIN match m ON m.season_id = s.id
        INNER JOIN score_event se ON se.match_id = m.id
        WHERE se.scorer_id = ? OR se.assist1_id = ? OR se.assist2_id = ?
        GROUP BY s.id, s.year, s.display_name, e.id, e.name
        ORDER BY s.year DESC
        "#,
        player_id, player_id, player_id,
        player_id, player_id, player_id
    )
    .fetch_all(db)
    .await?;

    Ok(rows
        .into_iter()
        .map(|row| PlayerSeasonStats {
            season_id: row.season_id,
            season_year: row.season_year,
            season_display_name: row.season_display_name,
            event_id: row.event_id,
            event_name: row.event_name,
            goals: row.goals,
            assists: row.assists,
            points: row.goals + row.assists,
        })
        .collect())
}

/// Get player scoring events with pagination and filters
pub async fn get_player_scoring_events(
    db: &SqlitePool,
    player_id: i64,
    filters: &PlayerScoringFilters,
    sort_field: &ScoringEventSortField,
    sort_order: &SortOrder,
    page: usize,
    page_size: usize,
) -> Result<PagedResult<PlayerScoringEventEntity>, sqlx::Error> {
    // Build the count query with CTE
    let mut count_query = QueryBuilder::new(
        "WITH player_events AS (
            SELECT
                se.id as score_event_id,
                se.match_id,
                se.team_id,
                se.period,
                se.time_minutes,
                se.time_seconds,
                se.goal_type,
                se.scorer_id,
                se.assist1_id,
                se.assist2_id,
                CASE
                    WHEN se.scorer_id = ",
    );
    count_query.push_bind(player_id);
    count_query.push(" THEN 'goal' WHEN se.assist1_id = ");
    count_query.push_bind(player_id);
    count_query.push(" THEN 'assist_primary' WHEN se.assist2_id = ");
    count_query.push_bind(player_id);
    count_query.push(
        " THEN 'assist_secondary' END as event_type FROM score_event se WHERE se.scorer_id = ",
    );
    count_query.push_bind(player_id);
    count_query.push(" OR se.assist1_id = ");
    count_query.push_bind(player_id);
    count_query.push(" OR se.assist2_id = ");
    count_query.push_bind(player_id);
    count_query.push(
        " )
         SELECT COUNT(*) as total FROM player_events pe
         INNER JOIN match m ON pe.match_id = m.id
         INNER JOIN season s ON m.season_id = s.id
         INNER JOIN event e ON s.event_id = e.id
         WHERE 1=1",
    );

    // Apply filters to count query
    apply_filters(&mut count_query, filters);

    let count_row = count_query.build().fetch_one(db).await?;
    let total: i64 = count_row.get("total");

    // Data query with full joins
    let mut data_query = QueryBuilder::new(
        "WITH player_events AS (
            SELECT
                se.id as score_event_id,
                se.match_id,
                se.team_id,
                se.period,
                se.time_minutes,
                se.time_seconds,
                se.goal_type,
                se.scorer_id,
                se.assist1_id,
                se.assist2_id,
                CASE
                    WHEN se.scorer_id = ",
    );
    data_query.push_bind(player_id);
    data_query.push(" THEN 'goal' WHEN se.assist1_id = ");
    data_query.push_bind(player_id);
    data_query.push(" THEN 'assist_primary' WHEN se.assist2_id = ");
    data_query.push_bind(player_id);
    data_query.push(
        " THEN 'assist_secondary' END as event_type FROM score_event se WHERE se.scorer_id = ",
    );
    data_query.push_bind(player_id);
    data_query.push(" OR se.assist1_id = ");
    data_query.push_bind(player_id);
    data_query.push(" OR se.assist2_id = ");
    data_query.push_bind(player_id);
    data_query.push(
        " )
        SELECT
            pe.score_event_id,
            pe.match_id,
            m.match_date,
            s.id as season_id,
            s.year as season_year,
            s.display_name as season_display_name,
            e.name as event_name,
            m.home_team_id,
            ht.name as home_team_name,
            hc.iso2Code as home_team_iso2,
            m.away_team_id,
            at.name as away_team_name,
            ac.iso2Code as away_team_iso2,
            pe.team_id,
            t.name as team_name,
            tc.iso2Code as team_iso2,
            pe.event_type,
            pe.period,
            pe.time_minutes,
            pe.time_seconds,
            pe.goal_type,
            scorer.id as scorer_id,
            scorer.name as scorer_name,
            assist1.id as assist1_id,
            assist1.name as assist1_name,
            assist2.id as assist2_id,
            assist2.name as assist2_name
        FROM player_events pe
        INNER JOIN match m ON pe.match_id = m.id
        INNER JOIN team ht ON m.home_team_id = ht.id
        INNER JOIN team at ON m.away_team_id = at.id
        LEFT JOIN country hc ON ht.country_id = hc.id
        LEFT JOIN country ac ON at.country_id = ac.id
        INNER JOIN team t ON pe.team_id = t.id
        LEFT JOIN country tc ON t.country_id = tc.id
        INNER JOIN season s ON m.season_id = s.id
        INNER JOIN event e ON s.event_id = e.id
        LEFT JOIN player scorer ON pe.scorer_id = scorer.id
        LEFT JOIN player assist1 ON pe.assist1_id = assist1.id
        LEFT JOIN player assist2 ON pe.assist2_id = assist2.id
        WHERE 1=1",
    );

    // Apply filters to data query
    apply_filters(&mut data_query, filters);

    // Apply sorting
    data_query
        .push(" ORDER BY ")
        .push(sort_field.to_sql())
        .push(" ")
        .push(sort_order.to_sql());

    // Secondary sort by match date DESC if not primary sort
    if !matches!(sort_field, ScoringEventSortField::Date) {
        data_query.push(", m.match_date DESC");
    }

    // Apply pagination
    let offset = (page - 1) * page_size;
    data_query
        .push(" LIMIT ")
        .push_bind(page_size as i64)
        .push(" OFFSET ")
        .push_bind(offset as i64);

    // Execute data query
    let rows = data_query.build().fetch_all(db).await?;

    let items: Vec<PlayerScoringEventEntity> = rows
        .into_iter()
        .map(|row| PlayerScoringEventEntity {
            score_event_id: row.get("score_event_id"),
            match_id: row.get("match_id"),
            match_date: row.get("match_date"),
            season_id: row.get("season_id"),
            season_year: row.get("season_year"),
            season_display_name: row.get("season_display_name"),
            event_name: row.get("event_name"),
            home_team_id: row.get("home_team_id"),
            home_team_name: row.get("home_team_name"),
            home_team_iso2: row.get("home_team_iso2"),
            away_team_id: row.get("away_team_id"),
            away_team_name: row.get("away_team_name"),
            away_team_iso2: row.get("away_team_iso2"),
            team_id: row.get("team_id"),
            team_name: row.get("team_name"),
            team_iso2: row.get("team_iso2"),
            event_type: row.get("event_type"),
            period: row.get("period"),
            time_minutes: row.get("time_minutes"),
            time_seconds: row.get("time_seconds"),
            goal_type: row.get("goal_type"),
            scorer_id: row.get("scorer_id"),
            scorer_name: row.get("scorer_name"),
            assist1_id: row.get("assist1_id"),
            assist1_name: row.get("assist1_name"),
            assist2_id: row.get("assist2_id"),
            assist2_name: row.get("assist2_name"),
        })
        .collect();

    Ok(PagedResult::new(items, total as usize, page, page_size))
}

/// Helper to apply filters to query builder
fn apply_filters<'a>(
    query: &mut QueryBuilder<'a, sqlx::Sqlite>,
    filters: &'a PlayerScoringFilters,
) {
    // Event type filter
    if let Some(event_type) = &filters.event_type {
        match event_type.as_str() {
            "goals" => {
                query.push(" AND pe.event_type = 'goal'");
            }
            "assists" => {
                query.push(
                    " AND (pe.event_type = 'assist_primary' OR pe.event_type = 'assist_secondary')",
                );
            }
            _ => {} // "all" - no filter
        }
    }

    // Season filter
    if let Some(season_id) = filters.season_id {
        query.push(" AND m.season_id = ").push_bind(season_id);
    }

    // Team filter
    if let Some(team_id) = filters.team_id {
        query.push(" AND pe.team_id = ").push_bind(team_id);
    }

    // Date range filters
    if let Some(date_from) = &filters.date_from {
        query.push(" AND m.match_date >= ").push_bind(date_from);
    }

    if let Some(date_to) = &filters.date_to {
        query.push(" AND m.match_date <= ").push_bind(date_to);
    }
}

/// Get seasons for filter dropdown (only seasons where player participated)
pub async fn get_player_seasons(
    db: &SqlitePool,
    player_id: i64,
) -> Result<Vec<(i64, String)>, sqlx::Error> {
    let rows = sqlx::query!(
        r#"
        SELECT DISTINCT
            s.id,
            COALESCE(s.display_name, CAST(s.year AS TEXT)) as "display_name!: String"
        FROM season s
        INNER JOIN team_participation tp ON s.id = tp.season_id
        INNER JOIN player_contract pc ON tp.id = pc.team_participation_id
        WHERE pc.player_id = ?
        ORDER BY s.year DESC
        "#,
        player_id
    )
    .fetch_all(db)
    .await?;

    Ok(rows
        .into_iter()
        .map(|row| (row.id, row.display_name))
        .collect())
}

/// Get teams for filter dropdown (only teams where player participated)
pub async fn get_player_teams(
    db: &SqlitePool,
    player_id: i64,
) -> Result<Vec<(i64, String)>, sqlx::Error> {
    let rows = sqlx::query!(
        r#"
        SELECT DISTINCT
            t.id,
            t.name as "name!: String"
        FROM team t
        INNER JOIN team_participation tp ON t.id = tp.team_id
        INNER JOIN player_contract pc ON tp.id = pc.team_participation_id
        WHERE pc.player_id = ?
        ORDER BY t.name ASC
        "#,
        player_id
    )
    .fetch_all(db)
    .await?;

    Ok(rows.into_iter().map(|row| (row.id, row.name)).collect())
}
