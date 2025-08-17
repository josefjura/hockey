use crate::common::paging::{PagedResult, Paging};
use sqlx::Row;
use sqlx::SqlitePool;

#[cfg(test)]
pub mod test;

pub struct CreateMatchEntity {
    pub season_id: i64,
    pub home_team_id: i64,
    pub away_team_id: i64,
    pub home_score_unidentified: i32,
    pub away_score_unidentified: i32,
    pub match_date: Option<String>,
    pub status: Option<String>,
    pub venue: Option<String>,
}

#[allow(dead_code)]
pub struct MatchEntity {
    pub id: i64,
    pub season_id: i64,
    pub home_team_id: i64,
    pub away_team_id: i64,
    pub home_score_unidentified: i32,
    pub away_score_unidentified: i32,
    pub match_date: Option<String>,
    pub status: String,
    pub venue: Option<String>,
}

pub struct MatchWithNamesEntity {
    pub id: i64,
    pub season_id: i64,
    pub home_team_id: i64,
    pub away_team_id: i64,
    pub home_score_unidentified: i32,
    pub away_score_unidentified: i32,
    pub home_score_total: i32,
    pub away_score_total: i32,
    pub match_date: Option<String>,
    pub status: String,
    pub venue: Option<String>,
    pub season_name: String,
    pub home_team_name: String,
    pub away_team_name: String,
}

pub struct UpdateMatchEntity {
    pub season_id: Option<i64>,
    pub home_team_id: Option<i64>,
    pub away_team_id: Option<i64>,
    pub home_score_unidentified: Option<i32>,
    pub away_score_unidentified: Option<i32>,
    pub match_date: Option<String>,
    pub status: Option<String>,
    pub venue: Option<String>,
}

pub struct CreateScoreEventEntity {
    pub match_id: i64,
    pub team_id: i64,
    pub scorer_id: Option<i64>,
    pub assist1_id: Option<i64>,
    pub assist2_id: Option<i64>,
    pub period: Option<i32>,
    pub time_minutes: Option<i32>,
    pub time_seconds: Option<i32>,
    pub goal_type: Option<String>,
}

#[allow(dead_code)]
pub struct ScoreEventEntity {
    pub id: i64,
    pub match_id: i64,
    pub team_id: i64,
    pub scorer_id: Option<i64>,
    pub assist1_id: Option<i64>,
    pub assist2_id: Option<i64>,
    pub period: Option<i32>,
    pub time_minutes: Option<i32>,
    pub time_seconds: Option<i32>,
    pub goal_type: Option<String>,
}

pub struct ScoreEventWithNamesEntity {
    pub id: i64,
    pub match_id: i64,
    pub team_id: i64,
    pub scorer_id: Option<i64>,
    pub assist1_id: Option<i64>,
    pub assist2_id: Option<i64>,
    pub period: Option<i32>,
    pub time_minutes: Option<i32>,
    pub time_seconds: Option<i32>,
    pub goal_type: Option<String>,
    pub scorer_name: Option<String>,
    pub assist1_name: Option<String>,
    pub assist2_name: Option<String>,
}

#[derive(Debug, Clone)]
pub struct MatchFilters {
    pub season_id: Option<i64>,
    pub team_id: Option<i64>, // Either home or away team
    pub status: Option<String>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
}

impl Default for MatchFilters {
    fn default() -> Self {
        Self {
            season_id: None,
            team_id: None,
            status: None,
            date_from: None,
            date_to: None,
        }
    }
}

impl MatchFilters {
    pub fn new(
        season_id: Option<i64>,
        team_id: Option<i64>,
        status: Option<String>,
        date_from: Option<String>,
        date_to: Option<String>,
    ) -> Self {
        Self {
            season_id,
            team_id,
            status,
            date_from,
            date_to,
        }
    }
}

pub async fn create_match(
    db: &SqlitePool,
    match_data: CreateMatchEntity,
) -> Result<i64, sqlx::Error> {
    let result = sqlx::query(
        "INSERT INTO match (season_id, home_team_id, away_team_id, home_score_unidentified, away_score_unidentified, match_date, status, venue) 
         VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
    )
        .bind(match_data.season_id)
        .bind(match_data.home_team_id)
        .bind(match_data.away_team_id)
        .bind(match_data.home_score_unidentified)
        .bind(match_data.away_score_unidentified)
        .bind(match_data.match_date)
        .bind(match_data.status.unwrap_or_else(|| "scheduled".to_string()))
        .bind(match_data.venue)
        .execute(db)
        .await?;

    Ok(result.last_insert_rowid())
}

fn apply_match_filters<'a>(
    query_builder: &mut sqlx::QueryBuilder<'a, sqlx::Sqlite>,
    filters: &'a MatchFilters,
) {
    if let Some(season_id) = filters.season_id {
        query_builder
            .push(" AND m.season_id = ")
            .push_bind(season_id);
    }

    if let Some(team_id) = filters.team_id {
        query_builder
            .push(" AND (m.home_team_id = ")
            .push_bind(team_id)
            .push(" OR m.away_team_id = ")
            .push_bind(team_id)
            .push(")");
    }

    if let Some(status) = &filters.status {
        query_builder.push(" AND m.status = ").push_bind(status);
    }

    if let Some(date_from) = &filters.date_from {
        query_builder
            .push(" AND m.match_date >= ")
            .push_bind(date_from);
    }

    if let Some(date_to) = &filters.date_to {
        query_builder
            .push(" AND m.match_date <= ")
            .push_bind(date_to);
    }
}

pub async fn get_matches(
    db: &SqlitePool,
    filters: &MatchFilters,
    paging: Option<&Paging>,
) -> Result<PagedResult<MatchWithNamesEntity>, sqlx::Error> {
    // Build count query
    let mut count_query_builder = sqlx::QueryBuilder::new(
        "SELECT COUNT(*) as count FROM match m 
         INNER JOIN season s ON m.season_id = s.id
         INNER JOIN team ht ON m.home_team_id = ht.id
         INNER JOIN team at ON m.away_team_id = at.id
         WHERE 1=1",
    );
    apply_match_filters(&mut count_query_builder, filters);

    let count_query = count_query_builder.build();
    let total: i64 = count_query.fetch_one(db).await?.get("count");

    let total = total as usize;
    let default_paging = Paging::default();
    let paging = paging.unwrap_or(&default_paging);

    // Build main query
    let mut data_query_builder = sqlx::QueryBuilder::new(
        "SELECT m.id, m.season_id, m.home_team_id, m.away_team_id, 
                m.home_score_unidentified, m.away_score_unidentified,
                m.home_score_unidentified + COALESCE((SELECT COUNT(*) FROM score_event se WHERE se.match_id = m.id AND se.team_id = m.home_team_id), 0) as home_score_total,
                m.away_score_unidentified + COALESCE((SELECT COUNT(*) FROM score_event se WHERE se.match_id = m.id AND se.team_id = m.away_team_id), 0) as away_score_total,
                m.match_date, m.status, m.venue,
                COALESCE(s.display_name, CAST(s.year AS TEXT)) as season_name,
                COALESCE(ht.name, cht.name) as home_team_name,
                COALESCE(at.name, cat.name) as away_team_name
         FROM match m 
         INNER JOIN season s ON m.season_id = s.id
         INNER JOIN team ht ON m.home_team_id = ht.id
         INNER JOIN country cht ON ht.country_id = cht.id
         INNER JOIN team at ON m.away_team_id = at.id
         INNER JOIN country cat ON at.country_id = cat.id
         WHERE 1=1",
    );
    apply_match_filters(&mut data_query_builder, filters);

    data_query_builder.push(" ORDER BY m.match_date DESC, m.id DESC");
    data_query_builder
        .push(" LIMIT ")
        .push_bind(paging.page_size as i32);
    data_query_builder
        .push(" OFFSET ")
        .push_bind(paging.offset() as i32);

    let data_query = data_query_builder.build();
    let rows = data_query.fetch_all(db).await?;

    let items: Vec<MatchWithNamesEntity> = rows
        .into_iter()
        .map(|row| MatchWithNamesEntity {
            id: row.get("id"),
            season_id: row.get("season_id"),
            home_team_id: row.get("home_team_id"),
            away_team_id: row.get("away_team_id"),
            home_score_unidentified: row.get("home_score_unidentified"),
            away_score_unidentified: row.get("away_score_unidentified"),
            home_score_total: row.get("home_score_total"),
            away_score_total: row.get("away_score_total"),
            match_date: row.get("match_date"),
            status: row.get("status"),
            venue: row.get("venue"),
            season_name: row.get("season_name"),
            home_team_name: row.get("home_team_name"),
            away_team_name: row.get("away_team_name"),
        })
        .collect();

    Ok(PagedResult::new(items, total, paging))
}

pub async fn get_match_by_id(
    db: &SqlitePool,
    id: i64,
) -> Result<Option<MatchWithNamesEntity>, sqlx::Error> {
    let row = sqlx::query(
        "SELECT m.id, m.season_id, m.home_team_id, m.away_team_id, 
                m.home_score_unidentified, m.away_score_unidentified,
                m.home_score_unidentified + COALESCE((SELECT COUNT(*) FROM score_event se WHERE se.match_id = m.id AND se.team_id = m.home_team_id), 0) as home_score_total,
                m.away_score_unidentified + COALESCE((SELECT COUNT(*) FROM score_event se WHERE se.match_id = m.id AND se.team_id = m.away_team_id), 0) as away_score_total,
                m.match_date, m.status, m.venue,
                COALESCE(s.display_name, CAST(s.year AS TEXT)) as season_name,
                COALESCE(ht.name, cht.name) as home_team_name,
                COALESCE(at.name, cat.name) as away_team_name
         FROM match m 
         INNER JOIN season s ON m.season_id = s.id
         INNER JOIN team ht ON m.home_team_id = ht.id
         INNER JOIN country cht ON ht.country_id = cht.id
         INNER JOIN team at ON m.away_team_id = at.id
         INNER JOIN country cat ON at.country_id = cat.id
         WHERE m.id = ?",
    )
    .bind(id)
    .fetch_optional(db)
    .await?;

    Ok(row.map(|row| MatchWithNamesEntity {
        id: row.get("id"),
        season_id: row.get("season_id"),
        home_team_id: row.get("home_team_id"),
        away_team_id: row.get("away_team_id"),
        home_score_unidentified: row.get("home_score_unidentified"),
        away_score_unidentified: row.get("away_score_unidentified"),
        home_score_total: row.get("home_score_total"),
        away_score_total: row.get("away_score_total"),
        match_date: row.get("match_date"),
        status: row.get("status"),
        venue: row.get("venue"),
        season_name: row.get("season_name"),
        home_team_name: row.get("home_team_name"),
        away_team_name: row.get("away_team_name"),
    }))
}

pub async fn update_match(
    db: &SqlitePool,
    id: i64,
    match_data: UpdateMatchEntity,
) -> Result<bool, sqlx::Error> {
    let mut query_builder =
        sqlx::QueryBuilder::new("UPDATE match SET updated_at = CURRENT_TIMESTAMP");
    let mut has_updates = false;

    if let Some(season_id) = match_data.season_id {
        query_builder.push(", season_id = ").push_bind(season_id);
        has_updates = true;
    }

    if let Some(home_team_id) = match_data.home_team_id {
        query_builder
            .push(", home_team_id = ")
            .push_bind(home_team_id);
        has_updates = true;
    }

    if let Some(away_team_id) = match_data.away_team_id {
        query_builder
            .push(", away_team_id = ")
            .push_bind(away_team_id);
        has_updates = true;
    }

    if let Some(home_score_unidentified) = match_data.home_score_unidentified {
        query_builder
            .push(", home_score_unidentified = ")
            .push_bind(home_score_unidentified);
        has_updates = true;
    }

    if let Some(away_score_unidentified) = match_data.away_score_unidentified {
        query_builder
            .push(", away_score_unidentified = ")
            .push_bind(away_score_unidentified);
        has_updates = true;
    }

    if let Some(match_date) = match_data.match_date {
        query_builder.push(", match_date = ").push_bind(match_date);
        has_updates = true;
    }

    if let Some(status) = match_data.status {
        query_builder.push(", status = ").push_bind(status);
        has_updates = true;
    }

    if let Some(venue) = match_data.venue {
        query_builder.push(", venue = ").push_bind(venue);
        has_updates = true;
    }

    if !has_updates {
        return Ok(false);
    }

    query_builder.push(" WHERE id = ").push_bind(id);

    let result = query_builder.build().execute(db).await?;
    Ok(result.rows_affected() > 0)
}

pub async fn delete_match(db: &SqlitePool, id: i64) -> Result<bool, sqlx::Error> {
    let result = sqlx::query("DELETE FROM match WHERE id = ?")
        .bind(id)
        .execute(db)
        .await?;
    Ok(result.rows_affected() > 0)
}

// Score Event functions

pub async fn create_score_event(
    db: &SqlitePool,
    event: CreateScoreEventEntity,
) -> Result<i64, sqlx::Error> {
    let result = sqlx::query(
        "INSERT INTO score_event (match_id, team_id, scorer_id, assist1_id, assist2_id, period, time_minutes, time_seconds, goal_type) 
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"
    )
        .bind(event.match_id)
        .bind(event.team_id)
        .bind(event.scorer_id)
        .bind(event.assist1_id)
        .bind(event.assist2_id)
        .bind(event.period)
        .bind(event.time_minutes)
        .bind(event.time_seconds)
        .bind(event.goal_type)
        .execute(db)
        .await?;

    Ok(result.last_insert_rowid())
}

pub async fn get_score_events_for_match(
    db: &SqlitePool,
    match_id: i64,
) -> Result<Vec<ScoreEventWithNamesEntity>, sqlx::Error> {
    let rows = sqlx::query(
        "SELECT se.id, se.match_id, se.team_id, se.scorer_id, se.assist1_id, se.assist2_id,
                se.period, se.time_minutes, se.time_seconds, se.goal_type,
                s.name as scorer_name,
                a1.name as assist1_name,
                a2.name as assist2_name
         FROM score_event se
         LEFT JOIN player s ON se.scorer_id = s.id
         LEFT JOIN player a1 ON se.assist1_id = a1.id
         LEFT JOIN player a2 ON se.assist2_id = a2.id
         WHERE se.match_id = ?
         ORDER BY se.period ASC, se.time_minutes ASC, se.time_seconds ASC",
    )
    .bind(match_id)
    .fetch_all(db)
    .await?;

    Ok(rows
        .into_iter()
        .map(|row| ScoreEventWithNamesEntity {
            id: row.get("id"),
            match_id: row.get("match_id"),
            team_id: row.get("team_id"),
            scorer_id: row.get("scorer_id"),
            assist1_id: row.get("assist1_id"),
            assist2_id: row.get("assist2_id"),
            period: row.get("period"),
            time_minutes: row.get("time_minutes"),
            time_seconds: row.get("time_seconds"),
            goal_type: row.get("goal_type"),
            scorer_name: row.get("scorer_name"),
            assist1_name: row.get("assist1_name"),
            assist2_name: row.get("assist2_name"),
        })
        .collect())
}

pub async fn get_score_event_by_id(
    db: &SqlitePool,
    id: i64,
) -> Result<Option<ScoreEventWithNamesEntity>, sqlx::Error> {
    let row = sqlx::query(
        "SELECT se.id, se.match_id, se.team_id, se.scorer_id, se.assist1_id, se.assist2_id,
                se.period, se.time_minutes, se.time_seconds, se.goal_type,
                s.name as scorer_name,
                a1.name as assist1_name,
                a2.name as assist2_name
         FROM score_event se
         LEFT JOIN player s ON se.scorer_id = s.id
         LEFT JOIN player a1 ON se.assist1_id = a1.id
         LEFT JOIN player a2 ON se.assist2_id = a2.id
         WHERE se.id = ?",
    )
    .bind(id)
    .fetch_optional(db)
    .await?;

    Ok(row.map(|row| ScoreEventWithNamesEntity {
        id: row.get("id"),
        match_id: row.get("match_id"),
        team_id: row.get("team_id"),
        scorer_id: row.get("scorer_id"),
        assist1_id: row.get("assist1_id"),
        assist2_id: row.get("assist2_id"),
        period: row.get("period"),
        time_minutes: row.get("time_minutes"),
        time_seconds: row.get("time_seconds"),
        goal_type: row.get("goal_type"),
        scorer_name: row.get("scorer_name"),
        assist1_name: row.get("assist1_name"),
        assist2_name: row.get("assist2_name"),
    }))
}

pub async fn delete_score_event(db: &SqlitePool, id: i64) -> Result<bool, sqlx::Error> {
    let result = sqlx::query("DELETE FROM score_event WHERE id = ?")
        .bind(id)
        .execute(db)
        .await?;
    Ok(result.rows_affected() > 0)
}

/// Business logic: Calculate total scores for a match
pub async fn get_match_with_stats(
    db: &SqlitePool,
    match_id: i64,
) -> Result<Option<(MatchWithNamesEntity, i32, i32, i32, i32)>, sqlx::Error> {
    let match_data = match match_id {
        id => get_match_by_id(db, id).await?,
    };

    let Some(match_data) = match_data else {
        return Ok(None);
    };

    // Count detailed goals for each team
    let home_detailed_count = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM score_event WHERE match_id = ? AND team_id = ?",
    )
    .bind(match_id)
    .bind(match_data.home_team_id)
    .fetch_one(db)
    .await? as i32;

    let away_detailed_count = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM score_event WHERE match_id = ? AND team_id = ?",
    )
    .bind(match_id)
    .bind(match_data.away_team_id)
    .fetch_one(db)
    .await? as i32;

    let home_total = home_detailed_count + match_data.home_score_unidentified;
    let away_total = away_detailed_count + match_data.away_score_unidentified;

    Ok(Some((
        match_data,
        home_total,
        away_total,
        home_detailed_count,
        away_detailed_count,
    )))
}

/// Convert an unidentified goal to a detailed score event
pub async fn identify_goal(
    db: &SqlitePool,
    match_id: i64,
    team_id: i64,
    event: CreateScoreEventEntity,
) -> Result<i64, sqlx::Error> {
    let mut tx = db.begin().await?;

    // First, check if the match has unidentified goals for this team
    let match_data = sqlx::query(
        "SELECT home_team_id, away_team_id, home_score_unidentified, away_score_unidentified FROM match WHERE id = ?"
    )
        .bind(match_id)
        .fetch_optional(&mut *tx)
        .await?;

    let Some(match_row) = match_data else {
        return Err(sqlx::Error::RowNotFound);
    };

    let home_team_id: i64 = match_row.get("home_team_id");
    let away_team_id: i64 = match_row.get("away_team_id");
    let home_score_unidentified: i32 = match_row.get("home_score_unidentified");
    let away_score_unidentified: i32 = match_row.get("away_score_unidentified");

    // Check if the team has unidentified goals
    if team_id == home_team_id && home_score_unidentified > 0 {
        // Reduce home unidentified goals
        sqlx::query(
            "UPDATE match SET home_score_unidentified = home_score_unidentified - 1 WHERE id = ?",
        )
        .bind(match_id)
        .execute(&mut *tx)
        .await?;
    } else if team_id == away_team_id && away_score_unidentified > 0 {
        // Reduce away unidentified goals
        sqlx::query(
            "UPDATE match SET away_score_unidentified = away_score_unidentified - 1 WHERE id = ?",
        )
        .bind(match_id)
        .execute(&mut *tx)
        .await?;
    } else {
        return Err(sqlx::Error::ColumnNotFound(
            "No unidentified goals available for this team".into(),
        ));
    }

    // Create the detailed score event
    let result = sqlx::query(
        "INSERT INTO score_event (match_id, team_id, scorer_id, assist1_id, assist2_id, period, time_minutes, time_seconds, goal_type) 
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"
    )
        .bind(event.match_id)
        .bind(event.team_id)
        .bind(event.scorer_id)
        .bind(event.assist1_id)
        .bind(event.assist2_id)
        .bind(event.period)
        .bind(event.time_minutes)
        .bind(event.time_seconds)
        .bind(event.goal_type)
        .execute(&mut *tx)
        .await?;

    tx.commit().await?;
    Ok(result.last_insert_rowid())
}
