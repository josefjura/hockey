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
    let result = sqlx::query!(
        r#"
        INSERT INTO event (name, country_id)
        VALUES (?, ?)
        "#,
        event.name,
        event.country_id
    )
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
    let row = sqlx::query_as!(
        EventEntity,
        r#"
        SELECT
            e.id,
            e.name as "name!",
            e.country_id,
            c.name as country_name,
            c.iso2Code as country_iso2_code
        FROM event e
        LEFT JOIN country c ON e.country_id = c.id
        WHERE e.id = ?
        "#,
        id
    )
    .fetch_optional(db)
    .await?;

    Ok(row)
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
    let seasons = sqlx::query_as!(
        SeasonEntity,
        r#"
        SELECT id as "id!", year, display_name
        FROM season
        WHERE event_id = ?
        ORDER BY year DESC
        "#,
        event_id
    )
    .fetch_all(db)
    .await?;

    // let seasons = rows
    //     .into_iter()
    //     .map(|row| SeasonEntity {
    //         id: row.id,
    //         year: row.year,
    //         display_name: row.display_name,
    //     })
    //     .collect();

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
    let result = sqlx::query!(
        r#"
        UPDATE event
        SET name = ?, country_id = ?
        WHERE id = ?
        "#,
        event.name,
        event.country_id,
        id
    )
    .execute(db)
    .await?;

    Ok(result.rows_affected() > 0)
}

/// Delete an event
pub async fn delete_event(db: &SqlitePool, id: i64) -> Result<bool, sqlx::Error> {
    let result = sqlx::query!(
        r#"
        DELETE FROM event
        WHERE id = ?
        "#,
        id
    )
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

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::SqlitePool;

    #[sqlx::test(migrations = "./migrations")]
    async fn test_create_event(pool: SqlitePool) {
        let event = CreateEventEntity {
            name: "Test Tournament".to_string(),
            country_id: Some(1), // Canada from migrations
        };

        let id = create_event(&pool, event).await.unwrap();
        assert!(id > 0);

        let result = get_event_by_id(&pool, id).await.unwrap();
        assert!(result.is_some());
        let event = result.unwrap();
        assert_eq!(event.name, "Test Tournament");
    }

    #[sqlx::test(migrations = "./migrations", fixtures("events"))]
    async fn test_get_events_no_filters(pool: SqlitePool) {
        let filters = EventFilters::default();
        let result = get_events(&pool, &filters, 1, 20).await.unwrap();

        assert!(result.items.len() >= 3);
        assert!(result.total >= 3);
    }

    #[sqlx::test(migrations = "./migrations", fixtures("events"))]
    async fn test_get_events_with_name_filter(pool: SqlitePool) {
        let filters = EventFilters {
            name: Some("Olympics".to_string()),
            country_id: None,
        };
        let result = get_events(&pool, &filters, 1, 20).await.unwrap();

        assert!(!result.items.is_empty());
        assert!(result.items.iter().all(|e| e.name.contains("Olympics")));
    }

    #[sqlx::test(migrations = "./migrations", fixtures("events"))]
    async fn test_get_event_by_id_found(pool: SqlitePool) {
        let result = get_event_by_id(&pool, 1).await.unwrap();
        assert!(result.is_some());
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn test_get_event_by_id_not_found(pool: SqlitePool) {
        let result = get_event_by_id(&pool, 999).await.unwrap();
        assert!(result.is_none());
    }

    #[sqlx::test(migrations = "./migrations", fixtures("events"))]
    async fn test_get_event_detail(pool: SqlitePool) {
        let detail = get_event_detail(&pool, 1).await.unwrap();
        assert!(detail.is_some());
        let detail = detail.unwrap();
        assert_eq!(detail.event_info.id, 1);
    }

    #[sqlx::test(migrations = "./migrations", fixtures("events"))]
    async fn test_update_event(pool: SqlitePool) {
        let update = UpdateEventEntity {
            name: "Updated Olympics".to_string(),
            country_id: Some(1),
        };

        let success = update_event(&pool, 1, update).await.unwrap();
        assert!(success);

        let event = get_event_by_id(&pool, 1).await.unwrap().unwrap();
        assert_eq!(event.name, "Updated Olympics");
    }

    #[sqlx::test(migrations = "./migrations", fixtures("events"))]
    async fn test_delete_event(pool: SqlitePool) {
        let success = delete_event(&pool, 1).await.unwrap();
        assert!(success);

        let result = get_event_by_id(&pool, 1).await.unwrap();
        assert!(result.is_none());
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn test_get_countries_for_event_creation(pool: SqlitePool) {
        let countries = crate::service::countries::get_countries_simple(&pool)
            .await
            .unwrap();
        assert!(!countries.is_empty());
        assert!(countries.iter().any(|(_, name)| name == "Canada"));
    }
}
