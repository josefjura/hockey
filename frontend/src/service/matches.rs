use sqlx::{Row, SqlitePool};

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct MatchEntity {
    pub id: i64,
    pub season_id: i64,
    pub season_name: Option<String>,
    pub event_name: Option<String>,
    pub home_team_id: i64,
    pub home_team_name: String,
    pub home_team_country_iso2: Option<String>,
    pub away_team_id: i64,
    pub away_team_name: String,
    pub away_team_country_iso2: Option<String>,
    pub home_score_unidentified: i32,
    pub away_score_unidentified: i32,
    pub match_date: Option<String>,
    pub status: String,
    pub venue: Option<String>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ScoreEventEntity {
    pub id: i64,
    pub match_id: i64,
    pub team_id: i64,
    pub team_name: String,
    pub scorer_id: Option<i64>,
    pub scorer_name: Option<String>,
    pub assist1_id: Option<i64>,
    pub assist1_name: Option<String>,
    pub assist2_id: Option<i64>,
    pub assist2_name: Option<String>,
    pub period: i32,
    pub time_minutes: Option<i32>,
    pub time_seconds: Option<i32>,
    pub goal_type: Option<String>,
}

#[derive(Debug, Clone)]
pub struct MatchDetailEntity {
    pub match_info: MatchEntity,
    pub score_events: Vec<ScoreEventEntity>,
    pub home_score_identified: i32,
    pub away_score_identified: i32,
    pub home_score_total: i32,
    pub away_score_total: i32,
}

/// Get a single match by ID with all related details
pub async fn get_match_by_id(db: &SqlitePool, id: i64) -> Result<Option<MatchEntity>, sqlx::Error> {
    let row = sqlx::query(
        r#"
        SELECT
            m.id,
            m.season_id,
            s.name as season_name,
            e.name as event_name,
            m.home_team_id,
            ht.name as home_team_name,
            hc.iso2Code as home_team_country_iso2,
            m.away_team_id,
            at.name as away_team_name,
            ac.iso2Code as away_team_country_iso2,
            m.home_score_unidentified,
            m.away_score_unidentified,
            m.match_date,
            m.status,
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
    )
    .bind(id)
    .fetch_optional(db)
    .await?;

    Ok(row.map(|row| MatchEntity {
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
    }))
}

/// Get all score events for a match
pub async fn get_score_events(
    db: &SqlitePool,
    match_id: i64,
) -> Result<Vec<ScoreEventEntity>, sqlx::Error> {
    let rows = sqlx::query(
        r#"
        SELECT
            se.id,
            se.match_id,
            se.team_id,
            t.name as team_name,
            se.scorer_id,
            scorer.name as scorer_name,
            se.assist1_id,
            assist1.name as assist1_name,
            se.assist2_id,
            assist2.name as assist2_name,
            se.period,
            se.time_minutes,
            se.time_seconds,
            se.goal_type
        FROM score_event se
        INNER JOIN team t ON se.team_id = t.id
        LEFT JOIN player scorer ON se.scorer_id = scorer.id
        LEFT JOIN player assist1 ON se.assist1_id = assist1.id
        LEFT JOIN player assist2 ON se.assist2_id = assist2.id
        WHERE se.match_id = ?
        ORDER BY se.period ASC, se.time_minutes ASC, se.time_seconds ASC
        "#,
    )
    .bind(match_id)
    .fetch_all(db)
    .await?;

    Ok(rows
        .into_iter()
        .map(|row| ScoreEventEntity {
            id: row.get("id"),
            match_id: row.get("match_id"),
            team_id: row.get("team_id"),
            team_name: row.get("team_name"),
            scorer_id: row.get("scorer_id"),
            scorer_name: row.get("scorer_name"),
            assist1_id: row.get("assist1_id"),
            assist1_name: row.get("assist1_name"),
            assist2_id: row.get("assist2_id"),
            assist2_name: row.get("assist2_name"),
            period: row.get("period"),
            time_minutes: row.get("time_minutes"),
            time_seconds: row.get("time_seconds"),
            goal_type: row.get("goal_type"),
        })
        .collect())
}

/// Get match detail with calculated scores
pub async fn get_match_detail(
    db: &SqlitePool,
    id: i64,
) -> Result<Option<MatchDetailEntity>, sqlx::Error> {
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

    Ok(Some(MatchDetailEntity {
        match_info,
        score_events,
        home_score_identified,
        away_score_identified,
        home_score_total,
        away_score_total,
    }))
}

/// Delete a match (cascades to score events)
pub async fn delete_match(db: &SqlitePool, id: i64) -> Result<bool, sqlx::Error> {
    let result = sqlx::query("DELETE FROM match WHERE id = ?")
        .bind(id)
        .execute(db)
        .await?;

    Ok(result.rows_affected() > 0)
}
