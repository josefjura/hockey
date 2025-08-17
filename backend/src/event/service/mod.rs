use crate::common::paging::{PagedResult, Paging};
use sqlx::Row;
use sqlx::SqlitePool;

#[cfg(test)]
pub mod test;

pub struct CreateEventEntity {
    pub name: String,
    pub country_id: Option<i64>,
}

#[allow(dead_code)]
pub struct EventEntity {
    pub id: i64,
    pub name: String,
    pub country_id: Option<i64>,
    pub created_at: String,
    pub updated_at: String,
}

pub struct EventWithCountryEntity {
    pub id: i64,
    pub name: String,
    pub country_id: Option<i64>,
    pub country_name: Option<String>,
    pub country_iso2_code: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

pub struct UpdateEventEntity {
    pub name: String,
    pub country_id: Option<i64>,
}

#[derive(Debug, Clone)]
pub struct EventFilters {
    pub name: Option<String>,
    pub country_id: Option<i64>,
}

impl Default for EventFilters {
    fn default() -> Self {
        Self {
            name: None,
            country_id: None,
        }
    }
}

impl EventFilters {
    pub fn new(name: Option<String>, country_id: Option<i64>) -> Self {
        Self { name, country_id }
    }
}

pub async fn create_event(db: &SqlitePool, event: CreateEventEntity) -> Result<i64, sqlx::Error> {
    let result = sqlx::query("INSERT INTO event (name, country_id, created_at, updated_at) VALUES (?, ?, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)")
        .bind(event.name)
        .bind(event.country_id)
        .execute(db)
        .await?;

    Ok(result.last_insert_rowid())
}

fn apply_event_filters<'a>(
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

pub async fn get_events(
    db: &SqlitePool,
    filters: &EventFilters,
    paging: Option<&Paging>,
) -> Result<PagedResult<EventWithCountryEntity>, sqlx::Error> {
    // Build count query with LEFT JOIN for events that may not have a country
    let mut count_query_builder = sqlx::QueryBuilder::new(
        "SELECT COUNT(*) as count FROM event e LEFT JOIN country c ON e.country_id = c.id WHERE 1=1",
    );
    apply_event_filters(&mut count_query_builder, filters);

    // Execute count query
    let count_query = count_query_builder.build();
    let total: i64 = count_query.fetch_one(db).await?.get("count");

    let total = total as usize;
    let default_paging = Paging::default();
    let paging = paging.unwrap_or(&default_paging);

    // Build main query with LEFT JOIN to get country information (events may not have a country)
    let mut data_query_builder = sqlx::QueryBuilder::new(
        "SELECT e.id, e.name, e.country_id, e.created_at, e.updated_at, c.name as country_name, c.iso2Code as country_iso2_code 
         FROM event e 
         LEFT JOIN country c ON e.country_id = c.id 
         WHERE 1=1"
    );
    apply_event_filters(&mut data_query_builder, filters);
    data_query_builder.push(" ORDER BY e.id");

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
        .map(|row| EventWithCountryEntity {
            id: row.get("id"),
            name: row.get("name"),
            country_id: row.get("country_id"),
            country_name: row.get("country_name"),
            country_iso2_code: row.get("country_iso2_code"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
        .collect();

    Ok(PagedResult::new(items, total, paging))
}

pub async fn get_event_by_id(
    db: &SqlitePool,
    id: i64,
) -> Result<Option<EventWithCountryEntity>, sqlx::Error> {
    let row = sqlx::query(
        "SELECT e.id, e.name, e.country_id, e.created_at, e.updated_at, c.name as country_name, c.iso2Code as country_iso2_code 
         FROM event e 
         LEFT JOIN country c ON e.country_id = c.id 
         WHERE e.id = ?"
    )
        .bind(id)
        .fetch_optional(db)
        .await?;

    Ok(row.map(|row| EventWithCountryEntity {
        id: row.get("id"),
        name: row.get("name"),
        country_id: row.get("country_id"),
        country_name: row.get("country_name"),
        country_iso2_code: row.get("country_iso2_code"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    }))
}

pub async fn update_event(
    db: &SqlitePool,
    id: i64,
    event: UpdateEventEntity,
) -> Result<bool, sqlx::Error> {
    let result = sqlx::query(
        "UPDATE event SET name = ?, country_id = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?",
    )
    .bind(event.name)
    .bind(event.country_id)
    .bind(id)
    .execute(db)
    .await?;
    Ok(result.rows_affected() > 0)
}

pub async fn delete_event(db: &SqlitePool, id: i64) -> Result<bool, sqlx::Error> {
    let result = sqlx::query("DELETE FROM event WHERE id = ?")
        .bind(id)
        .execute(db)
        .await?;
    Ok(result.rows_affected() > 0)
}
