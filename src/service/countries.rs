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
    let row = sqlx::query_as!(
        CountryEntity,
        r#"
        SELECT
            id,
            name,
            iihf as "iihf!: bool",
            iocCode as ioc_code,
            iso2Code as iso2_code,
            isHistorical as "is_historical!: bool",
            years,
            enabled as "enabled!: bool"
        FROM country
        WHERE id = ?
        "#,
        id
    )
    .fetch_optional(db)
    .await?;

    Ok(row)
}

/// Get simple list of countries for dropdowns (id, name)
/// Only returns enabled countries
pub async fn get_countries_simple(db: &SqlitePool) -> Result<Vec<(i64, String)>, sqlx::Error> {
    let rows = sqlx::query!(
        r#"
        SELECT id, name
        FROM country
        WHERE enabled = 1
        ORDER BY name
        "#
    )
    .fetch_all(db)
    .await?;

    Ok(rows.into_iter().map(|row| (row.id, row.name)).collect())
}

/// Toggle country enabled status
/// Returns the new enabled status or None if country not found
pub async fn toggle_country_enabled(db: &SqlitePool, id: i64) -> Result<Option<bool>, sqlx::Error> {
    // Toggle the enabled status
    let result = sqlx::query!(
        r#"
        UPDATE country
        SET enabled = NOT enabled
        WHERE id = ?
        "#,
        id
    )
    .execute(db)
    .await?;

    if result.rows_affected() == 0 {
        return Ok(None);
    }

    // Return the new status
    let row = sqlx::query!(
        r#"
        SELECT enabled as "enabled!: bool"
        FROM country
        WHERE id = ?
        "#,
        id
    )
    .fetch_one(db)
    .await?;

    Ok(Some(row.enabled))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[sqlx::test(migrations = "./migrations")]
    async fn test_get_countries_no_filters(pool: SqlitePool) {
        let filters = CountryFilters::default();
        let countries = get_countries(&pool, &filters).await.unwrap();

        // Migration seeds ~230 countries
        assert!(
            countries.len() > 200,
            "Should return all countries from migration"
        );
        assert!(countries.iter().any(|c| c.name == "Canada"));
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn test_get_countries_iihf_filter(pool: SqlitePool) {
        let filters = CountryFilters {
            iihf_only: true,
            ..Default::default()
        };
        let countries = get_countries(&pool, &filters).await.unwrap();

        assert!(
            countries.iter().all(|c| c.iihf),
            "All countries should be IIHF members"
        );
        assert!(countries.len() > 50, "Should have many IIHF members");
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn test_get_countries_enabled_filter(pool: SqlitePool) {
        let filters = CountryFilters {
            enabled_only: true,
            ..Default::default()
        };
        let countries = get_countries(&pool, &filters).await.unwrap();

        assert!(
            countries.iter().all(|c| c.enabled),
            "All countries should be enabled"
        );
        // Migration enables IIHF members and historical countries
        assert!(countries.len() > 50);
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn test_get_countries_search_filter(pool: SqlitePool) {
        let filters = CountryFilters {
            search: Some("Canada".to_string()),
            ..Default::default()
        };
        let countries = get_countries(&pool, &filters).await.unwrap();

        assert_eq!(countries.len(), 1, "Should find only Canada");
        assert_eq!(countries[0].name, "Canada");
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn test_get_countries_combined_filters(pool: SqlitePool) {
        let filters = CountryFilters {
            iihf_only: true,
            enabled_only: true,
            ..Default::default()
        };
        let countries = get_countries(&pool, &filters).await.unwrap();

        assert!(countries.iter().all(|c| c.iihf && c.enabled));
        assert!(countries.len() > 50);
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn test_get_country_by_id_found(pool: SqlitePool) {
        // Find Canada's ID first
        let canada_list = get_countries(
            &pool,
            &CountryFilters {
                search: Some("Canada".to_string()),
                ..Default::default()
            },
        )
        .await
        .unwrap();

        assert!(!canada_list.is_empty());
        let canada_id = canada_list[0].id;

        let country = get_country_by_id(&pool, canada_id).await.unwrap();
        assert!(country.is_some());
        let country = country.unwrap();
        assert_eq!(country.name, "Canada");
        assert_eq!(country.iso2_code, Some("CA".to_string()));
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn test_get_country_by_id_not_found(pool: SqlitePool) {
        let country = get_country_by_id(&pool, 99999).await.unwrap();
        assert!(country.is_none());
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn test_get_countries_simple(pool: SqlitePool) {
        let countries = get_countries_simple(&pool).await.unwrap();

        // Should only return enabled countries
        assert!(countries.len() > 50);
        assert!(countries.iter().all(|(id, _)| *id > 0));
        assert!(countries.iter().any(|(_, name)| name == "Canada"));
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn test_toggle_country_enabled(pool: SqlitePool) {
        // Find Canada
        let canada_list = get_countries(
            &pool,
            &CountryFilters {
                search: Some("Canada".to_string()),
                ..Default::default()
            },
        )
        .await
        .unwrap();
        let canada_id = canada_list[0].id;

        // Canada should start as enabled (IIHF member)
        let country = get_country_by_id(&pool, canada_id).await.unwrap().unwrap();
        let initial_status = country.enabled;

        // Toggle
        let new_status = toggle_country_enabled(&pool, canada_id).await.unwrap();
        assert_eq!(new_status, Some(!initial_status));

        // Verify it was toggled
        let country = get_country_by_id(&pool, canada_id).await.unwrap().unwrap();
        assert_eq!(country.enabled, !initial_status);

        // Toggle back
        let new_status = toggle_country_enabled(&pool, canada_id).await.unwrap();
        assert_eq!(new_status, Some(initial_status));
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn test_toggle_country_enabled_not_found(pool: SqlitePool) {
        let result = toggle_country_enabled(&pool, 9999).await.unwrap();
        assert!(result.is_none());
    }
}
