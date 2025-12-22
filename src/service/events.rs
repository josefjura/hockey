use sqlx::{Row, SqlitePool};

#[derive(Debug, Clone)]
pub struct EventEntity {
    pub id: i64,
    pub name: String,
    pub country_id: Option<i64>,
    pub country_name: Option<String>,
    pub country_iso2_code: Option<String>,
}

#[derive(Debug, Clone)]
pub struct CreateEventEntity {
    pub name: String,
    pub country_id: Option<i64>,
}

#[derive(Debug, Clone)]
pub struct UpdateEventEntity {
    pub name: String,
    pub country_id: Option<i64>,
}

#[derive(Debug, Clone, Default)]
pub struct EventFilters {
    pub name: Option<String>,
    pub country_id: Option<i64>,
}

#[derive(Debug, Clone)]
pub struct PagedResult<T> {
    pub items: Vec<T>,
    pub total: usize,
    pub page: usize,
    pub page_size: usize,
    pub total_pages: usize,
    pub has_next: bool,
    pub has_previous: bool,
}

impl<T> PagedResult<T> {
    pub fn new(items: Vec<T>, total: usize, page: usize, page_size: usize) -> Self {
        let total_pages = total.div_ceil(page_size);
        let has_next = page < total_pages;
        let has_previous = page > 1;

        Self {
            items,
            total,
            page,
            page_size,
            total_pages,
            has_next,
            has_previous,
        }
    }
}

/// Create a new event
pub async fn create_event(db: &SqlitePool, event: CreateEventEntity) -> Result<i64, sqlx::Error> {
    let result = sqlx::query("INSERT INTO event (name, country_id) VALUES (?, ?)")
        .bind(event.name)
        .bind(event.country_id)
        .execute(db)
        .await?;

    Ok(result.last_insert_rowid())
}

/// Get events with filters and pagination
pub async fn get_events(
    db: &SqlitePool,
    filters: &EventFilters,
    page: usize,
    page_size: usize,
) -> Result<PagedResult<EventEntity>, sqlx::Error> {
    // Build count query
    let mut count_query = sqlx::QueryBuilder::new(
        "SELECT COUNT(*) as count FROM event e LEFT JOIN country c ON e.country_id = c.id WHERE 1=1",
    );
    apply_filters(&mut count_query, filters);

    let total: i64 = count_query.build().fetch_one(db).await?.get("count");

    // Build data query
    let mut data_query = sqlx::QueryBuilder::new(
        "SELECT e.id, e.name, e.country_id, c.name as country_name, c.iso2Code as country_iso2_code
         FROM event e
         LEFT JOIN country c ON e.country_id = c.id
         WHERE 1=1",
    );
    apply_filters(&mut data_query, filters);
    data_query.push(" ORDER BY e.id");

    // Apply pagination
    let offset = (page - 1) * page_size;
    data_query.push(" LIMIT ").push_bind(page_size as i64);
    data_query.push(" OFFSET ").push_bind(offset as i64);

    let rows = data_query.build().fetch_all(db).await?;

    let items = rows
        .into_iter()
        .map(|row| EventEntity {
            id: row.get("id"),
            name: row.get("name"),
            country_id: row.get("country_id"),
            country_name: row.get("country_name"),
            country_iso2_code: row.get("country_iso2_code"),
        })
        .collect();

    Ok(PagedResult::new(items, total as usize, page, page_size))
}

/// Get a single event by ID
pub async fn get_event_by_id(db: &SqlitePool, id: i64) -> Result<Option<EventEntity>, sqlx::Error> {
    let row = sqlx::query(
        "SELECT e.id, e.name, e.country_id, c.name as country_name, c.iso2Code as country_iso2_code
         FROM event e
         LEFT JOIN country c ON e.country_id = c.id
         WHERE e.id = ?",
    )
    .bind(id)
    .fetch_optional(db)
    .await?;

    Ok(row.map(|row| EventEntity {
        id: row.get("id"),
        name: row.get("name"),
        country_id: row.get("country_id"),
        country_name: row.get("country_name"),
        country_iso2_code: row.get("country_iso2_code"),
    }))
}

#[derive(Debug, Clone)]
pub struct SeasonEntity {
    pub id: i64,
    pub year: i64,
    pub display_name: Option<String>,
}

#[derive(Debug, Clone)]
pub struct EventDetailEntity {
    pub event_info: EventEntity,
    pub seasons: Vec<SeasonEntity>,
}

/// Get event detail with all seasons
pub async fn get_event_detail(
    db: &SqlitePool,
    event_id: i64,
) -> Result<Option<EventDetailEntity>, sqlx::Error> {
    // Get event info
    let event_info = match get_event_by_id(db, event_id).await? {
        Some(event) => event,
        None => return Ok(None),
    };

    // Get seasons for this event
    let rows = sqlx::query(
        "SELECT id, year, display_name
         FROM season
         WHERE event_id = ?
         ORDER BY year DESC",
    )
    .bind(event_id)
    .fetch_all(db)
    .await?;

    let seasons = rows
        .into_iter()
        .map(|row| SeasonEntity {
            id: row.get("id"),
            year: row.get("year"),
            display_name: row.get("display_name"),
        })
        .collect();

    Ok(Some(EventDetailEntity {
        event_info,
        seasons,
    }))
}

/// Update an event
pub async fn update_event(
    db: &SqlitePool,
    id: i64,
    event: UpdateEventEntity,
) -> Result<bool, sqlx::Error> {
    let result = sqlx::query("UPDATE event SET name = ?, country_id = ? WHERE id = ?")
        .bind(event.name)
        .bind(event.country_id)
        .bind(id)
        .execute(db)
        .await?;

    Ok(result.rows_affected() > 0)
}

/// Delete an event
pub async fn delete_event(db: &SqlitePool, id: i64) -> Result<bool, sqlx::Error> {
    let result = sqlx::query("DELETE FROM event WHERE id = ?")
        .bind(id)
        .execute(db)
        .await?;

    Ok(result.rows_affected() > 0)
}

/// Helper function to apply filters to a query
fn apply_filters<'a>(
    query_builder: &mut sqlx::QueryBuilder<'a, sqlx::Sqlite>,
    filters: &'a EventFilters,
) {
    if let Some(name) = &filters.name {
        query_builder
            .push(" AND e.name LIKE '%' || ")
            .push_bind(name)
            .push(" || '%'");
    }

    if let Some(country_id) = filters.country_id {
        query_builder
            .push(" AND e.country_id = ")
            .push_bind(country_id);
    }
}

/// Get all countries for dropdowns
pub async fn get_countries(db: &SqlitePool) -> Result<Vec<(i64, String)>, sqlx::Error> {
    let rows = sqlx::query("SELECT id, name FROM country ORDER BY name")
        .fetch_all(db)
        .await?;

    Ok(rows
        .into_iter()
        .map(|row| (row.get("id"), row.get("name")))
        .collect())
}
