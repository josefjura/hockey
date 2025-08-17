use crate::common::paging::{PagedResult, Paging};
use sqlx::Row;
use sqlx::SqlitePool;

#[cfg(test)]
pub mod test;

pub struct CountryEntity {
    pub id: i64,
    pub name: String,
    pub iihf: bool,
    pub is_historical: bool,
    pub iso2_code: String,
    pub ioc_code: String,
    pub enabled: bool,
}

#[derive(Debug, Clone)]
pub struct CountryFilters {
    pub enabled: Option<bool>,
    pub iihf: Option<bool>,
    pub is_historical: Option<bool>,
    pub name: Option<String>,
    pub iso2_code: Option<String>,
    pub ioc_code: Option<String>,
}

impl Default for CountryFilters {
    fn default() -> Self {
        Self {
            enabled: None,
            iihf: None,
            is_historical: None,
            name: None,
            iso2_code: None,
            ioc_code: None,
        }
    }
}

impl CountryFilters {
    pub fn new(
        enabled: Option<bool>,
        iihf: Option<bool>,
        is_historical: Option<bool>,
        name: Option<String>,
        iso2_code: Option<String>,
        ioc_code: Option<String>,
    ) -> Self {
        Self {
            enabled,
            iihf,
            is_historical,
            name,
            iso2_code,
            ioc_code,
        }
    }
}

// Helper function to apply filters to any query builder
fn apply_country_filters<'a>(
    query_builder: &mut sqlx::QueryBuilder<'a, sqlx::Sqlite>,
    filters: &'a CountryFilters,
) {
    if let Some(enabled) = filters.enabled {
        query_builder.push(" AND enabled = ").push_bind(enabled);
    }

    if let Some(iihf) = filters.iihf {
        query_builder.push(" AND iihf = ").push_bind(iihf);
    }

    if let Some(is_historical) = filters.is_historical {
        query_builder
            .push(" AND isHistorical = ")
            .push_bind(is_historical);
    }

    if let Some(name) = &filters.name {
        query_builder
            .push(" AND name LIKE ")
            .push_bind(format!("%{}%", name));
    }

    if let Some(iso2_code) = &filters.iso2_code {
        query_builder.push(" AND iso2Code = ").push_bind(iso2_code);
    }

    if let Some(ioc_code) = &filters.ioc_code {
        query_builder.push(" AND iocCode = ").push_bind(ioc_code);
    }
}

pub async fn get_countries(
    db: &SqlitePool,
    filters: &CountryFilters,
    paging: Option<&Paging>,
) -> Result<PagedResult<CountryEntity>, sqlx::Error> {
    // Build count query with WHERE 1=1 trick for cleaner filter additions
    let mut count_query_builder =
        sqlx::QueryBuilder::new("SELECT COUNT(*) as count FROM country WHERE 1=1");

    apply_country_filters(&mut count_query_builder, filters);

    // Execute count query
    let count_query = count_query_builder.build();
    let count_row = count_query.fetch_one(db).await?;
    let total: i64 = count_row.get("count");

    // Build main query with WHERE 1=1 trick
    let mut query_builder = sqlx::QueryBuilder::new(
        "SELECT id, name, iihf, isHistorical, iso2Code, iocCode, enabled FROM country WHERE 1=1",
    );

    apply_country_filters(&mut query_builder, filters);

    // Apply ordering
    query_builder.push(" ORDER BY name");

    // Apply paging if provided
    if let Some(paging) = paging {
        query_builder
            .push(" LIMIT ")
            .push_bind(paging.limit() as i64);
        query_builder
            .push(" OFFSET ")
            .push_bind(paging.offset() as i64);
    }

    // Execute main query
    let query = query_builder.build();
    let rows = query.fetch_all(db).await?;

    let entities: Vec<CountryEntity> = rows
        .into_iter()
        .map(|row| CountryEntity {
            id: row.get("id"),
            name: row.get("name"),
            iihf: row.get("iihf"),
            is_historical: row.get("isHistorical"),
            iso2_code: row.get("iso2Code"),
            ioc_code: row.get("iocCode"),
            enabled: row.get("enabled"),
        })
        .collect();

    let default_paging = Paging::default();
    let paging = paging.unwrap_or(&default_paging);
    Ok(PagedResult::new(entities, total as usize, paging))
}

pub async fn get_country_by_id(db: &SqlitePool, id: i64) -> Result<CountryEntity, sqlx::Error> {
    let row = sqlx::query(
        "SELECT id, name, iihf, isHistorical, iso2Code, iocCode, enabled FROM country WHERE id = ?",
    )
    .bind(id)
    .fetch_optional(db)
    .await?;

    if row.is_none() {
        return Err(sqlx::Error::RowNotFound);
    }

    match row {
        Some(row) => Ok(CountryEntity {
            id: row.get("id"),
            name: row.get("name"),
            iihf: row.get("iihf"),
            is_historical: row.get("isHistorical"),
            iso2_code: row.get("iso2Code"),
            ioc_code: row.get("iocCode"),
            enabled: row.get("enabled"),
        }),
        None => Err(sqlx::Error::RowNotFound),
    }
}

pub async fn update_country_status(
    db: &SqlitePool,
    id: i64,
    enabled: bool,
) -> Result<(), sqlx::Error> {
    let result = sqlx::query("UPDATE country SET enabled = ? WHERE id = ?")
        .bind(enabled)
        .bind(id)
        .execute(db)
        .await?;

    if result.rows_affected() == 0 {
        return Err(sqlx::Error::RowNotFound);
    }

    Ok(())
}
