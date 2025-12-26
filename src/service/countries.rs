use sqlx::{Row, SqlitePool};

#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CountryEntity {
    pub id: i64,
    pub name: String,
    pub iihf: bool,
    pub ioc_code: Option<String>,
    pub iso2_code: Option<String>,
    pub is_historical: bool,
    pub years: Option<String>,
    pub enabled: bool,
}

#[derive(Debug, Clone, Default)]
pub struct CountryFilters {
    pub search: Option<String>,
    pub iihf_only: bool,
    pub enabled_only: bool,
}

/// Get all countries with optional filters
pub async fn get_countries(
    db: &SqlitePool,
    filters: &CountryFilters,
) -> Result<Vec<CountryEntity>, sqlx::Error> {
    let mut query = sqlx::QueryBuilder::new(
        "SELECT id, name, iihf, iocCode as ioc_code, iso2Code as iso2_code,
         isHistorical as is_historical, years, enabled
         FROM country WHERE 1=1",
    );

    if let Some(search) = &filters.search {
        query
            .push(" AND name LIKE '%' || ")
            .push_bind(search)
            .push(" || '%'");
    }

    if filters.iihf_only {
        query.push(" AND iihf = 1");
    }

    if filters.enabled_only {
        query.push(" AND enabled = 1");
    }

    query.push(" ORDER BY name");

    let rows = query.build().fetch_all(db).await?;

    let countries = rows
        .into_iter()
        .map(|row| CountryEntity {
            id: row.get("id"),
            name: row.get("name"),
            iihf: row.get("iihf"),
            ioc_code: row.get("ioc_code"),
            iso2_code: row.get("iso2_code"),
            is_historical: row.get("is_historical"),
            years: row.get("years"),
            enabled: row.get("enabled"),
        })
        .collect();

    Ok(countries)
}

/// Get a single country by ID
#[allow(dead_code)]
pub async fn get_country_by_id(
    db: &SqlitePool,
    id: i64,
) -> Result<Option<CountryEntity>, sqlx::Error> {
    let row = sqlx::query(
        "SELECT id, name, iihf, iocCode as ioc_code, iso2Code as iso2_code,
         isHistorical as is_historical, years, enabled
         FROM country WHERE id = ?",
    )
    .bind(id)
    .fetch_optional(db)
    .await?;

    Ok(row.map(|row| CountryEntity {
        id: row.get("id"),
        name: row.get("name"),
        iihf: row.get("iihf"),
        ioc_code: row.get("ioc_code"),
        iso2_code: row.get("iso2_code"),
        is_historical: row.get("is_historical"),
        years: row.get("years"),
        enabled: row.get("enabled"),
    }))
}

/// Get simple list of countries for dropdowns (id, name)
/// Only returns enabled countries
pub async fn get_countries_simple(db: &SqlitePool) -> Result<Vec<(i64, String)>, sqlx::Error> {
    let rows = sqlx::query("SELECT id, name FROM country WHERE enabled = 1 ORDER BY name")
        .fetch_all(db)
        .await?;

    Ok(rows
        .into_iter()
        .map(|row| (row.get("id"), row.get("name")))
        .collect())
}

/// Toggle country enabled status
/// Returns the new enabled status or None if country not found
pub async fn toggle_country_enabled(db: &SqlitePool, id: i64) -> Result<Option<bool>, sqlx::Error> {
    // Toggle the enabled status
    let result = sqlx::query("UPDATE country SET enabled = NOT enabled WHERE id = ?")
        .bind(id)
        .execute(db)
        .await?;

    if result.rows_affected() == 0 {
        return Ok(None);
    }

    // Return the new status
    let row = sqlx::query("SELECT enabled FROM country WHERE id = ?")
        .bind(id)
        .fetch_one(db)
        .await?;

    Ok(Some(row.get("enabled")))
}
