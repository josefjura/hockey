use sqlx::{Row, SqlitePool};

// Re-export common pagination types for convenience
pub use crate::common::pagination::{PagedResult, SortOrder};

// Import team participation types for detail view
use super::team_participations::TeamParticipationEntity;

#[derive(Debug, Clone)]
pub struct SeasonEntity {
    pub id: i64,
    pub year: i64,
    pub display_name: Option<String>,
    pub event_id: i64,
    pub event_name: String,
    pub country_id: Option<i64>, // Season's host country (e.g., WC 2024 in Sweden)
    #[allow(dead_code)]
    pub event_country_id: Option<i64>, // Event's default country (e.g., Czech Premier League)
    pub country_name: Option<String>, // Season's host country name
    pub event_country_name: Option<String>, // Event's default country name
}

impl SeasonEntity {
    /// Get the effective country ID for this season
    /// Falls back to event's country if season doesn't have one
    #[allow(dead_code)]
    pub fn effective_country_id(&self) -> Option<i64> {
        self.country_id.or(self.event_country_id)
    }

    /// Get the effective country name for this season
    /// Falls back to event's country name if season doesn't have one
    pub fn effective_country_name(&self) -> Option<&String> {
        self.country_name
            .as_ref()
            .or(self.event_country_name.as_ref())
    }
}

#[derive(Debug, Clone)]
pub struct CreateSeasonEntity {
    pub year: i64,
    pub display_name: Option<String>,
    pub event_id: i64,
    pub country_id: Option<i64>, // Host country for this season
}

#[derive(Debug, Clone)]
pub struct UpdateSeasonEntity {
    pub year: i64,
    pub display_name: Option<String>,
    pub event_id: i64,
    pub country_id: Option<i64>, // Host country for this season
}

#[derive(Debug, Clone)]
pub struct SeasonDetailEntity {
    pub season_info: SeasonEntity,
    pub participating_teams: Vec<TeamParticipationEntity>,
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
    let result = sqlx::query!(
        "INSERT INTO season (year, display_name, event_id, country_id) VALUES (?, ?, ?, ?)",
        season.year,
        season.display_name,
        season.event_id,
        season.country_id
    )
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
        "SELECT s.id, s.year, s.display_name, s.event_id, e.name as event_name,
                s.country_id, e.country_id as event_country_id,
                c1.name as country_name, c2.name as event_country_name
         FROM season s
         INNER JOIN event e ON s.event_id = e.id
         LEFT JOIN country c1 ON s.country_id = c1.id
         LEFT JOIN country c2 ON e.country_id = c2.id
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
            country_id: row.get("country_id"),
            event_country_id: row.get("event_country_id"),
            country_name: row.get("country_name"),
            event_country_name: row.get("event_country_name"),
        })
        .collect();

    Ok(PagedResult::new(items, total as usize, page, page_size))
}

/// Get a single season by ID
pub async fn get_season_by_id(
    db: &SqlitePool,
    id: i64,
) -> Result<Option<SeasonEntity>, sqlx::Error> {
    let row = sqlx::query_as!(
        SeasonEntity,
        r#"
        SELECT
            s.id,
            s.year,
            s.display_name,
            s.event_id,
            e.name as "event_name!: String",
            s.country_id as "country_id?: i64",
            e.country_id as "event_country_id?: i64",
            c1.name as "country_name?: String",
            c2.name as "event_country_name?: String"
        FROM season s
        INNER JOIN event e ON s.event_id = e.id
        LEFT JOIN country c1 ON s.country_id = c1.id
        LEFT JOIN country c2 ON e.country_id = c2.id
        WHERE s.id = ?
        "#,
        id
    )
    .fetch_optional(db)
    .await?;

    Ok(row)
}

/// Get season detail with all participating teams
pub async fn get_season_detail(
    db: &SqlitePool,
    id: i64,
) -> Result<Option<SeasonDetailEntity>, sqlx::Error> {
    // Get season info
    let season_info = match get_season_by_id(db, id).await? {
        Some(s) => s,
        None => return Ok(None),
    };

    // Get participating teams
    let participating_teams = super::team_participations::get_teams_for_season(db, id).await?;

    Ok(Some(SeasonDetailEntity {
        season_info,
        participating_teams,
    }))
}

/// Update a season
pub async fn update_season(
    db: &SqlitePool,
    id: i64,
    season: UpdateSeasonEntity,
) -> Result<bool, sqlx::Error> {
    let result = sqlx::query!(
        "UPDATE season SET year = ?, display_name = ?, event_id = ?, country_id = ? WHERE id = ?",
        season.year,
        season.display_name,
        season.event_id,
        season.country_id,
        id
    )
    .execute(db)
    .await?;

    Ok(result.rows_affected() > 0)
}

/// Delete a season
pub async fn delete_season(db: &SqlitePool, id: i64) -> Result<bool, sqlx::Error> {
    let result = sqlx::query!("DELETE FROM season WHERE id = ?", id)
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
        query_builder.push(" AND s.event_id = ").push_bind(event_id);
    }

    if let Some(year) = filters.year {
        query_builder.push(" AND s.year = ").push_bind(year);
    }
}

/// Get all events for dropdowns
pub async fn get_events(db: &SqlitePool) -> Result<Vec<(i64, String)>, sqlx::Error> {
    let rows = sqlx::query!(
        r#"
        SELECT id, name
        FROM event
        ORDER BY name
        "#
    )
    .fetch_all(db)
    .await?;

    Ok(rows.into_iter().map(|row| (row.id, row.name)).collect())
}

/// Get all countries for dropdowns (only enabled countries)
pub async fn get_countries(db: &SqlitePool) -> Result<Vec<(i64, String)>, sqlx::Error> {
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
