use crate::common::paging::{PagedResult, Paging};
use sqlx::Row;
use sqlx::SqlitePool;

#[cfg(test)]
pub mod test;

pub struct CreateSeasonEntity {
    pub year: i64,
    pub display_name: Option<String>,
    pub event_id: i64,
}

#[allow(dead_code)]
pub struct SeasonEntity {
    pub id: i64,
    pub year: i64,
    pub display_name: Option<String>,
    pub event_id: i64,
    pub created_at: String,
    pub updated_at: String,
}

pub struct SeasonWithEventEntity {
    pub id: i64,
    pub year: i64,
    pub display_name: Option<String>,
    pub event_id: i64,
    pub event_name: String,
    pub created_at: String,
    pub updated_at: String,
}

pub struct UpdateSeasonEntity {
    pub year: i64,
    pub display_name: Option<String>,
    pub event_id: i64,
}

#[derive(Debug, Clone)]
pub struct SeasonFilters {
    pub year: Option<i64>,
    pub event_id: Option<i64>,
}

impl Default for SeasonFilters {
    fn default() -> Self {
        Self {
            year: None,
            event_id: None,
        }
    }
}

impl SeasonFilters {
    pub fn new(year: Option<i64>, event_id: Option<i64>) -> Self {
        Self { year, event_id }
    }
}

pub async fn create_season(
    db: &SqlitePool,
    season: CreateSeasonEntity,
) -> Result<i64, sqlx::Error> {
    let result = sqlx::query("INSERT INTO season (year, display_name, event_id, created_at, updated_at) VALUES (?, ?, ?, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)")
        .bind(season.year)
        .bind(season.display_name)
        .bind(season.event_id)
        .execute(db)
        .await?;

    Ok(result.last_insert_rowid())
}

fn apply_season_filters<'a>(
    query_builder: &mut sqlx::QueryBuilder<'a, sqlx::Sqlite>,
    filters: &'a SeasonFilters,
) {
    if let Some(year) = filters.year {
        query_builder.push(" AND s.year = ").push_bind(year);
    }

    if let Some(event_id) = filters.event_id {
        query_builder.push(" AND s.event_id = ").push_bind(event_id);
    }
}

pub async fn get_seasons(
    db: &SqlitePool,
    filters: &SeasonFilters,
    paging: Option<&Paging>,
) -> Result<PagedResult<SeasonWithEventEntity>, sqlx::Error> {
    // Build count query with WHERE 1=1 trick for cleaner filter additions
    let mut count_query_builder = sqlx::QueryBuilder::new(
        "SELECT COUNT(*) as count FROM season s INNER JOIN event e ON s.event_id = e.id WHERE 1=1",
    );
    apply_season_filters(&mut count_query_builder, filters);

    // Execute count query
    let count_query = count_query_builder.build();
    let total: i64 = count_query.fetch_one(db).await?.get("count");

    let total = total as usize;
    let default_paging = Paging::default();
    let paging = paging.unwrap_or(&default_paging);

    // Build main query with JOIN to get event information
    let mut data_query_builder = sqlx::QueryBuilder::new(
        "SELECT s.id, s.year, s.display_name, s.event_id, s.created_at, s.updated_at, e.name as event_name 
         FROM season s 
         INNER JOIN event e ON s.event_id = e.id 
         WHERE 1=1"
    );
    apply_season_filters(&mut data_query_builder, filters);
    data_query_builder.push(" ORDER BY s.year DESC, s.id");

    // Apply paging if provided
    data_query_builder
        .push(" LIMIT ")
        .push_bind(paging.page_size as i64);
    data_query_builder
        .push(" OFFSET ")
        .push_bind(paging.offset() as i64);

    let data_query = data_query_builder.build();
    let rows = data_query.fetch_all(db).await?;

    let items = rows
        .into_iter()
        .map(|row| SeasonWithEventEntity {
            id: row.get("id"),
            year: row.get("year"),
            display_name: row.get("display_name"),
            event_id: row.get("event_id"),
            event_name: row.get("event_name"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
        .collect();

    Ok(PagedResult::new(items, total, paging))
}

pub async fn get_season_by_id(
    db: &SqlitePool,
    id: i64,
) -> Result<Option<SeasonWithEventEntity>, sqlx::Error> {
    let row = sqlx::query(
        "SELECT s.id, s.year, s.display_name, s.event_id, s.created_at, s.updated_at, e.name as event_name 
         FROM season s 
         INNER JOIN event e ON s.event_id = e.id 
         WHERE s.id = ?"
    )
        .bind(id)
        .fetch_optional(db)
        .await?;

    Ok(row.map(|row| SeasonWithEventEntity {
        id: row.get("id"),
        year: row.get("year"),
        display_name: row.get("display_name"),
        event_id: row.get("event_id"),
        event_name: row.get("event_name"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    }))
}

pub async fn update_season(
    db: &SqlitePool,
    id: i64,
    season: UpdateSeasonEntity,
) -> Result<bool, sqlx::Error> {
    let result = sqlx::query("UPDATE season SET year = ?, display_name = ?, event_id = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?")
        .bind(season.year)
        .bind(season.display_name)
        .bind(season.event_id)
        .bind(id)
        .execute(db)
        .await?;
    Ok(result.rows_affected() > 0)
}

pub async fn delete_season(db: &SqlitePool, id: i64) -> Result<bool, sqlx::Error> {
    let result = sqlx::query("DELETE FROM season WHERE id = ?")
        .bind(id)
        .execute(db)
        .await?;
    Ok(result.rows_affected() > 0)
}

pub struct SeasonListItem {
    pub id: i64,
    pub name: Option<String>,
    pub year: i64,
    pub event_name: String,
}

pub async fn get_seasons_list(db: &SqlitePool) -> Result<Vec<SeasonListItem>, sqlx::Error> {
    let rows = sqlx::query(
        "SELECT s.id, s.display_name as name, s.year, e.name as event_name 
         FROM season s 
         INNER JOIN event e ON s.event_id = e.id 
         ORDER BY s.year DESC, e.name",
    )
    .fetch_all(db)
    .await?;

    let items = rows
        .into_iter()
        .map(|row| SeasonListItem {
            id: row.get("id"),
            name: row.get("name"),
            year: row.get("year"),
            event_name: row.get("event_name"),
        })
        .collect();

    Ok(items)
}

pub struct PlayerDropdownItem {
    pub id: i64,
    pub name: String,
    pub nationality: String,
}

pub async fn get_players_for_team_in_season(
    db: &SqlitePool,
    season_id: i64,
    team_id: i64,
) -> Result<Vec<PlayerDropdownItem>, sqlx::Error> {
    let rows = sqlx::query(
        "SELECT p.id, p.name, c.name as nationality
         FROM player p
         INNER JOIN player_contract pc ON p.id = pc.player_id
         INNER JOIN team_participation tp ON pc.team_participation_id = tp.id
         INNER JOIN country c ON p.country_id = c.id
         WHERE tp.season_id = ? AND tp.team_id = ?
         ORDER BY p.name",
    )
    .bind(season_id)
    .bind(team_id)
    .fetch_all(db)
    .await?;

    let items = rows
        .into_iter()
        .map(|row| PlayerDropdownItem {
            id: row.get("id"),
            name: row.get("name"),
            nationality: row.get("nationality"),
        })
        .collect();

    Ok(items)
}
