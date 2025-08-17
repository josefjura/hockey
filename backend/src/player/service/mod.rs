use crate::common::paging::{PagedResult, Paging};
use sqlx::Row;
use sqlx::SqlitePool;

#[cfg(test)]
pub mod test;

pub struct CreatePlayerEntity {
    pub name: String,
    pub country_id: i64,
    pub photo_path: Option<String>,
}

#[allow(dead_code)]
pub struct PlayerEntity {
    pub id: i64,
    pub name: String,
    pub country_id: i64,
    pub photo_path: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

pub struct PlayerWithCountryEntity {
    pub id: i64,
    pub name: String,
    pub country_id: i64,
    pub country_name: String,
    pub country_iso2_code: String,
    pub photo_path: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

pub struct UpdatePlayerEntity {
    pub name: String,
    pub country_id: i64,
    pub photo_path: Option<String>,
}

#[derive(Debug, Clone)]
pub struct PlayerFilters {
    pub name: Option<String>,
    pub country_id: Option<i64>,
}

impl Default for PlayerFilters {
    fn default() -> Self {
        Self {
            name: None,
            country_id: None,
        }
    }
}

impl PlayerFilters {
    pub fn new(name: Option<String>, country_id: Option<i64>) -> Self {
        Self { name, country_id }
    }
}

pub async fn create_player(
    db: &SqlitePool,
    player: CreatePlayerEntity,
) -> Result<i64, sqlx::Error> {
    let result = sqlx::query("INSERT INTO player (name, country_id, photo_path, created_at, updated_at) VALUES (?, ?, ?, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)")
        .bind(player.name)
        .bind(player.country_id)
        .bind(player.photo_path)
        .execute(db)
        .await?;

    Ok(result.last_insert_rowid())
}

fn apply_player_filters<'a>(
    query_builder: &mut sqlx::QueryBuilder<'a, sqlx::Sqlite>,
    filters: &'a PlayerFilters,
) {
    if let Some(name) = &filters.name {
        query_builder
            .push(" AND p.name LIKE '%' || ")
            .push_bind(name)
            .push(" || '%'");
    }

    if let Some(country_id) = filters.country_id {
        query_builder
            .push(" AND p.country_id = ")
            .push_bind(country_id);
    }
}

pub async fn get_players(
    db: &SqlitePool,
    filters: &PlayerFilters,
    paging: Option<&Paging>,
) -> Result<PagedResult<PlayerWithCountryEntity>, sqlx::Error> {
    // Build count query with WHERE 1=1 trick for cleaner filter additions
    let mut count_query_builder = sqlx::QueryBuilder::new(
        "SELECT COUNT(*) as count FROM player p INNER JOIN country c ON p.country_id = c.id WHERE 1=1",
    );
    apply_player_filters(&mut count_query_builder, filters);

    // Execute count query
    let count_query = count_query_builder.build();
    let total: i64 = count_query.fetch_one(db).await?.get("count");

    let total = total as usize;
    let default_paging = Paging::default();
    let paging = paging.unwrap_or(&default_paging);

    // Build main query with JOIN to get country information
    let mut data_query_builder = sqlx::QueryBuilder::new(
        "SELECT p.id, p.name, p.country_id, p.photo_path, p.created_at, p.updated_at, c.name as country_name, c.iso2Code as country_iso2_code 
         FROM player p 
         INNER JOIN country c ON p.country_id = c.id 
         WHERE 1=1"
    );
    apply_player_filters(&mut data_query_builder, filters);
    data_query_builder.push(" ORDER BY p.id");

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
        .map(|row| PlayerWithCountryEntity {
            id: row.get("id"),
            name: row.get("name"),
            country_id: row.get("country_id"),
            country_name: row.get("country_name"),
            country_iso2_code: row.get("country_iso2_code"),
            photo_path: row.get("photo_path"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
        .collect();

    Ok(PagedResult::new(items, total, paging))
}

pub async fn get_player_by_id(
    db: &SqlitePool,
    id: i64,
) -> Result<Option<PlayerWithCountryEntity>, sqlx::Error> {
    let row = sqlx::query(
        "SELECT p.id, p.name, p.country_id, p.photo_path, p.created_at, p.updated_at, c.name as country_name, c.iso2Code as country_iso2_code 
         FROM player p 
         INNER JOIN country c ON p.country_id = c.id 
         WHERE p.id = ?"
    )
        .bind(id)
        .fetch_optional(db)
        .await?;

    Ok(row.map(|row| PlayerWithCountryEntity {
        id: row.get("id"),
        name: row.get("name"),
        country_id: row.get("country_id"),
        country_name: row.get("country_name"),
        country_iso2_code: row.get("country_iso2_code"),
        photo_path: row.get("photo_path"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    }))
}

pub async fn update_player(
    db: &SqlitePool,
    id: i64,
    player: UpdatePlayerEntity,
) -> Result<bool, sqlx::Error> {
    let result = sqlx::query(
        "UPDATE player SET name = ?, country_id = ?, photo_path = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?",
    )
    .bind(player.name)
    .bind(player.country_id)
    .bind(player.photo_path)
    .bind(id)
    .execute(db)
    .await?;
    Ok(result.rows_affected() > 0)
}

pub async fn delete_player(db: &SqlitePool, id: i64) -> Result<bool, sqlx::Error> {
    let result = sqlx::query("DELETE FROM player WHERE id = ?")
        .bind(id)
        .execute(db)
        .await?;
    Ok(result.rows_affected() > 0)
}
