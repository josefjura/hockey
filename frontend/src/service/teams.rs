use sqlx::{Row, SqlitePool};

#[derive(Debug, Clone)]
pub struct TeamEntity {
    pub id: i64,
    pub name: String,
    pub country_id: Option<i64>,
    pub country_name: Option<String>,
    pub country_iso2_code: Option<String>,
}

#[derive(Debug, Clone)]
pub struct CreateTeamEntity {
    pub name: String,
    pub country_id: Option<i64>,
}

#[derive(Debug, Clone)]
pub struct UpdateTeamEntity {
    pub name: String,
    pub country_id: Option<i64>,
}

#[derive(Debug, Clone, Default)]
pub struct TeamFilters {
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

/// Create a new team
pub async fn create_team(db: &SqlitePool, team: CreateTeamEntity) -> Result<i64, sqlx::Error> {
    let result = sqlx::query("INSERT INTO team (name, country_id) VALUES (?, ?)")
        .bind(team.name)
        .bind(team.country_id)
        .execute(db)
        .await?;

    Ok(result.last_insert_rowid())
}

/// Get teams with filters and pagination
pub async fn get_teams(
    db: &SqlitePool,
    filters: &TeamFilters,
    page: usize,
    page_size: usize,
) -> Result<PagedResult<TeamEntity>, sqlx::Error> {
    // Build count query
    let mut count_query = sqlx::QueryBuilder::new(
        "SELECT COUNT(*) as count FROM team t LEFT JOIN country c ON t.country_id = c.id WHERE 1=1",
    );
    apply_filters(&mut count_query, filters);

    let total: i64 = count_query.build().fetch_one(db).await?.get("count");

    // Build data query
    let mut data_query = sqlx::QueryBuilder::new(
        "SELECT t.id, t.name, t.country_id, c.name as country_name, c.iso2Code as country_iso2_code
         FROM team t
         LEFT JOIN country c ON t.country_id = c.id
         WHERE 1=1",
    );
    apply_filters(&mut data_query, filters);
    data_query.push(" ORDER BY t.name");

    // Apply pagination
    let offset = (page - 1) * page_size;
    data_query.push(" LIMIT ").push_bind(page_size as i64);
    data_query.push(" OFFSET ").push_bind(offset as i64);

    let rows = data_query.build().fetch_all(db).await?;

    let items = rows
        .into_iter()
        .map(|row| TeamEntity {
            id: row.get("id"),
            name: row.get("name"),
            country_id: row.get("country_id"),
            country_name: row.get("country_name"),
            country_iso2_code: row.get("country_iso2_code"),
        })
        .collect();

    Ok(PagedResult::new(items, total as usize, page, page_size))
}

/// Get a single team by ID
pub async fn get_team_by_id(db: &SqlitePool, id: i64) -> Result<Option<TeamEntity>, sqlx::Error> {
    let row = sqlx::query(
        "SELECT t.id, t.name, t.country_id, c.name as country_name, c.iso2Code as country_iso2_code
         FROM team t
         LEFT JOIN country c ON t.country_id = c.id
         WHERE t.id = ?",
    )
    .bind(id)
    .fetch_optional(db)
    .await?;

    Ok(row.map(|row| TeamEntity {
        id: row.get("id"),
        name: row.get("name"),
        country_id: row.get("country_id"),
        country_name: row.get("country_name"),
        country_iso2_code: row.get("country_iso2_code"),
    }))
}

/// Update a team
pub async fn update_team(
    db: &SqlitePool,
    id: i64,
    team: UpdateTeamEntity,
) -> Result<bool, sqlx::Error> {
    let result = sqlx::query("UPDATE team SET name = ?, country_id = ? WHERE id = ?")
        .bind(team.name)
        .bind(team.country_id)
        .bind(id)
        .execute(db)
        .await?;

    Ok(result.rows_affected() > 0)
}

/// Delete a team
pub async fn delete_team(db: &SqlitePool, id: i64) -> Result<bool, sqlx::Error> {
    let result = sqlx::query("DELETE FROM team WHERE id = ?")
        .bind(id)
        .execute(db)
        .await?;

    Ok(result.rows_affected() > 0)
}

/// Helper function to apply filters to a query
fn apply_filters<'a>(
    query_builder: &mut sqlx::QueryBuilder<'a, sqlx::Sqlite>,
    filters: &'a TeamFilters,
) {
    if let Some(name) = &filters.name {
        query_builder
            .push(" AND t.name LIKE '%' || ")
            .push_bind(name)
            .push(" || '%'");
    }

    if let Some(country_id) = filters.country_id {
        query_builder
            .push(" AND t.country_id = ")
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
