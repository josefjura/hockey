use crate::common::pagination::{PagedResult, SortOrder};
use sqlx::{QueryBuilder, Row, SqlitePool};

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

#[derive(Debug, Clone)]
pub struct MatchFilters {
    pub season_id: Option<i64>,
    pub team_id: Option<i64>, // matches either home or away team
    pub status: Option<String>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
}

#[derive(Debug, Clone)]
pub enum SortField {
    Date,
    Status,
    Event,
}

impl SortField {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "status" => Self::Status,
            "event" => Self::Event,
            _ => Self::Date, // default
        }
    }

    pub fn to_sql(&self) -> &'static str {
        match self {
            Self::Date => "m.match_date",
            Self::Status => "m.status",
            Self::Event => "e.name",
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Date => "date",
            Self::Status => "status",
            Self::Event => "event",
        }
    }
}

/// Get a single match by ID with all related details
pub async fn get_match_by_id(db: &SqlitePool, id: i64) -> Result<Option<MatchEntity>, sqlx::Error> {
    let row = sqlx::query(
        r#"
        SELECT
            m.id,
            m.season_id,
            COALESCE(s.display_name, CAST(s.year AS TEXT)) as season_name,
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

/// Get all seasons for filter dropdown
pub async fn get_seasons(db: &SqlitePool) -> Result<Vec<(i64, String)>, sqlx::Error> {
    let rows = sqlx::query(
        "SELECT s.id, COALESCE(s.display_name, CAST(s.year AS TEXT)) as name \
         FROM season s \
         ORDER BY s.year DESC",
    )
    .fetch_all(db)
    .await?;

    Ok(rows
        .into_iter()
        .map(|row| (row.get("id"), row.get("name")))
        .collect())
}

/// Get all teams for filter dropdown
pub async fn get_teams(db: &SqlitePool) -> Result<Vec<(i64, String)>, sqlx::Error> {
    let rows = sqlx::query(
        "SELECT t.id, t.name \
         FROM team t \
         ORDER BY t.name ASC",
    )
    .fetch_all(db)
    .await?;

    Ok(rows
        .into_iter()
        .map(|row| (row.get("id"), row.get("name")))
        .collect())
}

/// Get teams participating in a specific season (for match creation/editing)
pub async fn get_teams_for_season(
    db: &SqlitePool,
    season_id: i64,
) -> Result<Vec<(i64, String)>, sqlx::Error> {
    let rows = sqlx::query(
        "SELECT DISTINCT t.id, t.name \
         FROM team t \
         INNER JOIN team_participation tp ON t.id = tp.team_id \
         WHERE tp.season_id = ? \
         ORDER BY t.name ASC",
    )
    .bind(season_id)
    .fetch_all(db)
    .await?;

    Ok(rows
        .into_iter()
        .map(|row| (row.get("id"), row.get("name")))
        .collect())
}

/// Check if both teams participate in the given season (for validation)
pub async fn validate_teams_in_season(
    db: &SqlitePool,
    season_id: i64,
    home_team_id: i64,
    away_team_id: i64,
) -> Result<bool, sqlx::Error> {
    let row = sqlx::query(
        "SELECT COUNT(*) as count \
         FROM team_participation \
         WHERE season_id = ? AND (team_id = ? OR team_id = ?)",
    )
    .bind(season_id)
    .bind(home_team_id)
    .bind(away_team_id)
    .fetch_one(db)
    .await?;

    let count: i64 = row.get("count");
    // Should be exactly 2 if both teams participate
    Ok(count == 2)
}

#[derive(Debug, Clone)]
pub struct CreateMatchEntity {
    pub season_id: i64,
    pub home_team_id: i64,
    pub away_team_id: i64,
    pub home_score_unidentified: i32,
    pub away_score_unidentified: i32,
    pub match_date: Option<String>,
    pub status: String,
    pub venue: Option<String>,
}

#[derive(Debug, Clone)]
pub struct UpdateMatchEntity {
    pub season_id: i64,
    pub home_team_id: i64,
    pub away_team_id: i64,
    pub home_score_unidentified: i32,
    pub away_score_unidentified: i32,
    pub match_date: Option<String>,
    pub status: String,
    pub venue: Option<String>,
}

/// Create a new match
pub async fn create_match(db: &SqlitePool, entity: CreateMatchEntity) -> Result<i64, sqlx::Error> {
    let result = sqlx::query(
        "INSERT INTO match (season_id, home_team_id, away_team_id, home_score_unidentified, away_score_unidentified, match_date, status, venue) \
         VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(entity.season_id)
    .bind(entity.home_team_id)
    .bind(entity.away_team_id)
    .bind(entity.home_score_unidentified)
    .bind(entity.away_score_unidentified)
    .bind(entity.match_date)
    .bind(entity.status)
    .bind(entity.venue)
    .execute(db)
    .await?;

    Ok(result.last_insert_rowid())
}

/// Update an existing match
pub async fn update_match(
    db: &SqlitePool,
    id: i64,
    entity: UpdateMatchEntity,
) -> Result<bool, sqlx::Error> {
    let result = sqlx::query(
        "UPDATE match \
         SET season_id = ?, home_team_id = ?, away_team_id = ?, \
             home_score_unidentified = ?, away_score_unidentified = ?, \
             match_date = ?, status = ?, venue = ?, \
             updated_at = CURRENT_TIMESTAMP \
         WHERE id = ?",
    )
    .bind(entity.season_id)
    .bind(entity.home_team_id)
    .bind(entity.away_team_id)
    .bind(entity.home_score_unidentified)
    .bind(entity.away_score_unidentified)
    .bind(entity.match_date)
    .bind(entity.status)
    .bind(entity.venue)
    .bind(id)
    .execute(db)
    .await?;

    Ok(result.rows_affected() > 0)
}

// Score Event CRUD

#[derive(Debug, Clone)]
pub struct CreateScoreEventEntity {
    pub match_id: i64,
    pub team_id: i64,
    pub scorer_id: Option<i64>,
    pub assist1_id: Option<i64>,
    pub assist2_id: Option<i64>,
    pub period: i32,
    pub time_minutes: Option<i32>,
    pub time_seconds: Option<i32>,
    pub goal_type: Option<String>,
}

#[derive(Debug, Clone)]
pub struct UpdateScoreEventEntity {
    pub team_id: i64,
    pub scorer_id: Option<i64>,
    pub assist1_id: Option<i64>,
    pub assist2_id: Option<i64>,
    pub period: i32,
    pub time_minutes: Option<i32>,
    pub time_seconds: Option<i32>,
    pub goal_type: Option<String>,
}

/// Get a single score event by ID
pub async fn get_score_event_by_id(
    db: &SqlitePool,
    id: i64,
) -> Result<Option<ScoreEventEntity>, sqlx::Error> {
    let row = sqlx::query(
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
        WHERE se.id = ?
        "#,
    )
    .bind(id)
    .fetch_optional(db)
    .await?;

    Ok(row.map(|row| ScoreEventEntity {
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
    }))
}

/// Create a new score event and decrement unidentified goal count
pub async fn create_score_event(
    db: &SqlitePool,
    entity: CreateScoreEventEntity,
) -> Result<i64, sqlx::Error> {
    // Start a transaction to ensure both operations succeed or fail together
    let mut tx = db.begin().await?;

    // Insert the score event
    let result = sqlx::query(
        "INSERT INTO score_event (match_id, team_id, scorer_id, assist1_id, assist2_id, period, time_minutes, time_seconds, goal_type) \
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(entity.match_id)
    .bind(entity.team_id)
    .bind(entity.scorer_id)
    .bind(entity.assist1_id)
    .bind(entity.assist2_id)
    .bind(entity.period)
    .bind(entity.time_minutes)
    .bind(entity.time_seconds)
    .bind(entity.goal_type)
    .execute(&mut *tx)
    .await?;

    let score_event_id = result.last_insert_rowid();

    // Get the match to determine home/away team
    let match_row = sqlx::query("SELECT home_team_id, away_team_id, home_score_unidentified, away_score_unidentified FROM match WHERE id = ?")
        .bind(entity.match_id)
        .fetch_one(&mut *tx)
        .await?;

    let home_team_id: i64 = match_row.get("home_team_id");
    let home_score_unidentified: i32 = match_row.get("home_score_unidentified");
    let away_score_unidentified: i32 = match_row.get("away_score_unidentified");

    // Decrement the appropriate unidentified score if it's greater than 0
    if entity.team_id == home_team_id && home_score_unidentified > 0 {
        sqlx::query(
            "UPDATE match SET home_score_unidentified = home_score_unidentified - 1 WHERE id = ?",
        )
        .bind(entity.match_id)
        .execute(&mut *tx)
        .await?;
    } else if entity.team_id != home_team_id && away_score_unidentified > 0 {
        sqlx::query(
            "UPDATE match SET away_score_unidentified = away_score_unidentified - 1 WHERE id = ?",
        )
        .bind(entity.match_id)
        .execute(&mut *tx)
        .await?;
    }

    // Commit the transaction
    tx.commit().await?;

    Ok(score_event_id)
}

/// Update an existing score event
pub async fn update_score_event(
    db: &SqlitePool,
    id: i64,
    entity: UpdateScoreEventEntity,
) -> Result<bool, sqlx::Error> {
    let result = sqlx::query(
        "UPDATE score_event \
         SET team_id = ?, scorer_id = ?, assist1_id = ?, assist2_id = ?, \
             period = ?, time_minutes = ?, time_seconds = ?, goal_type = ? \
         WHERE id = ?",
    )
    .bind(entity.team_id)
    .bind(entity.scorer_id)
    .bind(entity.assist1_id)
    .bind(entity.assist2_id)
    .bind(entity.period)
    .bind(entity.time_minutes)
    .bind(entity.time_seconds)
    .bind(entity.goal_type)
    .bind(id)
    .execute(db)
    .await?;

    Ok(result.rows_affected() > 0)
}

/// Delete a score event and increment unidentified goal count
pub async fn delete_score_event(db: &SqlitePool, id: i64) -> Result<bool, sqlx::Error> {
    // Start a transaction to ensure both operations succeed or fail together
    let mut tx = db.begin().await?;

    // Get the score event to determine match and team
    let score_event_row = sqlx::query("SELECT match_id, team_id FROM score_event WHERE id = ?")
        .bind(id)
        .fetch_optional(&mut *tx)
        .await?;

    let (match_id, team_id) = match score_event_row {
        Some(row) => (row.get::<i64, _>("match_id"), row.get::<i64, _>("team_id")),
        None => return Ok(false), // Score event not found
    };

    // Delete the score event
    let result = sqlx::query("DELETE FROM score_event WHERE id = ?")
        .bind(id)
        .execute(&mut *tx)
        .await?;

    if result.rows_affected() == 0 {
        return Ok(false);
    }

    // Get the match to determine home/away team
    let match_row = sqlx::query("SELECT home_team_id FROM match WHERE id = ?")
        .bind(match_id)
        .fetch_one(&mut *tx)
        .await?;

    let home_team_id: i64 = match_row.get("home_team_id");

    // Increment the appropriate unidentified score
    if team_id == home_team_id {
        sqlx::query(
            "UPDATE match SET home_score_unidentified = home_score_unidentified + 1 WHERE id = ?",
        )
        .bind(match_id)
        .execute(&mut *tx)
        .await?;
    } else {
        sqlx::query(
            "UPDATE match SET away_score_unidentified = away_score_unidentified + 1 WHERE id = ?",
        )
        .bind(match_id)
        .execute(&mut *tx)
        .await?;
    }

    // Commit the transaction
    tx.commit().await?;

    Ok(true)
}

/// Get players for a specific team participation (for dropdowns)
pub async fn get_players_for_team(
    db: &SqlitePool,
    team_id: i64,
) -> Result<Vec<(i64, String)>, sqlx::Error> {
    let rows = sqlx::query(
        "SELECT DISTINCT p.id, p.name \
         FROM player p \
         INNER JOIN player_contract pc ON p.id = pc.player_id \
         INNER JOIN team_participation tp ON pc.team_participation_id = tp.id \
         WHERE tp.team_id = ? \
         ORDER BY p.name ASC",
    )
    .bind(team_id)
    .fetch_all(db)
    .await?;

    Ok(rows
        .into_iter()
        .map(|row| (row.get("id"), row.get("name")))
        .collect())
}

/// Delete a match (cascades to score events)
pub async fn delete_match(db: &SqlitePool, id: i64) -> Result<bool, sqlx::Error> {
    let result = sqlx::query("DELETE FROM match WHERE id = ?")
        .bind(id)
        .execute(db)
        .await?;

    Ok(result.rows_affected() > 0)
}
