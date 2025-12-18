use sqlx::{Row, SqlitePool};

// Re-export common pagination types for convenience
pub use crate::common::pagination::{PagedResult, SortOrder};

#[derive(Debug, Clone)]
pub struct SeasonEntity {
    pub id: i64,
    pub year: i64,
    pub display_name: Option<String>,
    pub event_id: i64,
    pub event_name: String,
}

#[derive(Debug, Clone)]
pub struct CreateSeasonEntity {
    pub year: i64,
    pub display_name: Option<String>,
    pub event_id: i64,
}

#[derive(Debug, Clone)]
pub struct UpdateSeasonEntity {
    pub year: i64,
    pub display_name: Option<String>,
    pub event_id: i64,
}

#[derive(Debug, Clone, Default)]
pub struct SeasonFilters {
    pub event_id: Option<i64>,
    pub year: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SortField {
    Id,
    Year,
    Event,
}

impl SortField {
    pub fn from_str(s: &str) -> Self {
        match s {
            "id" => Self::Id,
            "year" => Self::Year,
            "event" => Self::Event,
            _ => Self::Year, // Default
        }
    }

    pub fn to_sql(&self) -> &'static str {
        match self {
            Self::Id => "s.id",
            Self::Year => "s.year",
            Self::Event => "e.name",
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Id => "id",
            Self::Year => "year",
            Self::Event => "event",
        }
    }
}

/// Create a new season
pub async fn create_season(
    db: &SqlitePool,
    season: CreateSeasonEntity,
) -> Result<i64, sqlx::Error> {
    let result = sqlx::query("INSERT INTO season (year, display_name, event_id) VALUES (?, ?, ?)")
        .bind(season.year)
        .bind(season.display_name)
        .bind(season.event_id)
        .execute(db)
        .await?;

    Ok(result.last_insert_rowid())
}

/// Get seasons with filters and pagination
pub async fn get_seasons(
    db: &SqlitePool,
    filters: &SeasonFilters,
    sort_field: &SortField,
    sort_order: &SortOrder,
    page: usize,
    page_size: usize,
) -> Result<PagedResult<SeasonEntity>, sqlx::Error> {
    // Build count query
    let mut count_query = sqlx::QueryBuilder::new(
        "SELECT COUNT(*) as count FROM season s INNER JOIN event e ON s.event_id = e.id WHERE 1=1",
    );
    apply_filters(&mut count_query, filters);

    let total: i64 = count_query.build().fetch_one(db).await?.get("count");

    // Build data query
    let mut data_query = sqlx::QueryBuilder::new(
        "SELECT s.id, s.year, s.display_name, s.event_id, e.name as event_name
         FROM season s
         INNER JOIN event e ON s.event_id = e.id
         WHERE 1=1",
    );
    apply_filters(&mut data_query, filters);

    // Apply sorting
    data_query.push(" ORDER BY ");
    data_query.push(sort_field.to_sql());
    data_query.push(" ");
    data_query.push(sort_order.to_sql());

    // Apply pagination
    let offset = (page - 1) * page_size;
    data_query.push(" LIMIT ").push_bind(page_size as i64);
    data_query.push(" OFFSET ").push_bind(offset as i64);

    let rows = data_query.build().fetch_all(db).await?;

    let items = rows
        .into_iter()
        .map(|row| SeasonEntity {
            id: row.get("id"),
            year: row.get("year"),
            display_name: row.get("display_name"),
            event_id: row.get("event_id"),
            event_name: row.get("event_name"),
        })
        .collect();

    Ok(PagedResult::new(items, total as usize, page, page_size))
}

/// Get a single season by ID
pub async fn get_season_by_id(db: &SqlitePool, id: i64) -> Result<Option<SeasonEntity>, sqlx::Error> {
    let row = sqlx::query(
        "SELECT s.id, s.year, s.display_name, s.event_id, e.name as event_name
         FROM season s
         INNER JOIN event e ON s.event_id = e.id
         WHERE s.id = ?",
    )
    .bind(id)
    .fetch_optional(db)
    .await?;

    Ok(row.map(|row| SeasonEntity {
        id: row.get("id"),
        year: row.get("year"),
        display_name: row.get("display_name"),
        event_id: row.get("event_id"),
        event_name: row.get("event_name"),
    }))
}

/// Update a season
pub async fn update_season(
    db: &SqlitePool,
    id: i64,
    season: UpdateSeasonEntity,
) -> Result<bool, sqlx::Error> {
    let result = sqlx::query("UPDATE season SET year = ?, display_name = ?, event_id = ? WHERE id = ?")
        .bind(season.year)
        .bind(season.display_name)
        .bind(season.event_id)
        .bind(id)
        .execute(db)
        .await?;

    Ok(result.rows_affected() > 0)
}

/// Delete a season
pub async fn delete_season(db: &SqlitePool, id: i64) -> Result<bool, sqlx::Error> {
    let result = sqlx::query("DELETE FROM season WHERE id = ?")
        .bind(id)
        .execute(db)
        .await?;

    Ok(result.rows_affected() > 0)
}

/// Helper function to apply filters to a query
fn apply_filters<'a>(
    query_builder: &mut sqlx::QueryBuilder<'a, sqlx::Sqlite>,
    filters: &'a SeasonFilters,
) {
    if let Some(event_id) = filters.event_id {
        query_builder
            .push(" AND s.event_id = ")
            .push_bind(event_id);
    }

    if let Some(year) = filters.year {
        query_builder
            .push(" AND s.year = ")
            .push_bind(year);
    }
}

/// Get all events for dropdowns
pub async fn get_events(db: &SqlitePool) -> Result<Vec<(i64, String)>, sqlx::Error> {
    let rows = sqlx::query("SELECT id, name FROM event ORDER BY name")
        .fetch_all(db)
        .await?;

    Ok(rows
        .into_iter()
        .map(|row| (row.get("id"), row.get("name")))
        .collect())
}
