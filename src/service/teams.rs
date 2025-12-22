use sqlx::{Row, SqlitePool};

use crate::common::pagination::PagedResult;

#[derive(Debug, Clone)]
pub struct TeamEntity {
    pub id: i64,
    pub name: String,
    pub country_id: Option<i64>,
    pub country_name: Option<String>,
    pub country_iso2_code: Option<String>,
}

#[derive(Debug, Clone)]
pub struct TeamParticipationWithSeasonEntity {
    pub id: i64,
    pub season_id: i64,
    pub season_year: i64,
    pub season_display_name: Option<String>,
    pub event_id: i64,
    pub event_name: String,
    pub player_count: i64,
}

#[derive(Debug, Clone)]
pub struct TeamDetailEntity {
    pub team_info: TeamEntity,
    pub participations: Vec<TeamParticipationWithSeasonEntity>,
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

/// Sortable fields for teams
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SortField {
    Id,
    Name,
    Country,
}

impl SortField {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "id" => Self::Id,
            "country" => Self::Country,
            _ => Self::Name,
        }
    }

    pub fn to_sql(&self) -> &'static str {
        match self {
            Self::Id => "t.id",
            Self::Name => "t.name",
            Self::Country => "c.name",
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Id => "id",
            Self::Name => "name",
            Self::Country => "country",
        }
    }
}

/// Sort order (ascending/descending)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SortOrder {
    Asc,
    Desc,
}

impl SortOrder {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "desc" => Self::Desc,
            _ => Self::Asc,
        }
    }

    pub fn to_sql(&self) -> &'static str {
        match self {
            Self::Asc => "ASC",
            Self::Desc => "DESC",
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Asc => "asc",
            Self::Desc => "desc",
        }
    }

    pub fn toggle(&self) -> Self {
        match self {
            Self::Asc => Self::Desc,
            Self::Desc => Self::Asc,
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

/// Get teams with filters, sorting, and pagination
pub async fn get_teams(
    db: &SqlitePool,
    filters: &TeamFilters,
    sort_field: &SortField,
    sort_order: &SortOrder,
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

    // Apply sorting
    data_query.push(format!(
        " ORDER BY {} {}",
        sort_field.to_sql(),
        sort_order.to_sql()
    ));

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

/// Get team detail with all participations (seasons/events)
pub async fn get_team_detail(
    db: &SqlitePool,
    id: i64,
) -> Result<Option<TeamDetailEntity>, sqlx::Error> {
    // Get team info
    let team_info = match get_team_by_id(db, id).await? {
        Some(t) => t,
        None => return Ok(None),
    };

    // Get all participations with season and event info
    let rows = sqlx::query(
        "SELECT
            tp.id,
            tp.season_id,
            s.year as season_year,
            s.display_name as season_display_name,
            s.event_id,
            e.name as event_name,
            COALESCE(
                (SELECT COUNT(*) FROM player_contract pc WHERE pc.team_participation_id = tp.id),
                0
            ) as player_count
        FROM team_participation tp
        INNER JOIN season s ON tp.season_id = s.id
        INNER JOIN event e ON s.event_id = e.id
        WHERE tp.team_id = ?
        ORDER BY s.year DESC, e.name ASC",
    )
    .bind(id)
    .fetch_all(db)
    .await?;

    let participations = rows
        .into_iter()
        .map(|row| TeamParticipationWithSeasonEntity {
            id: row.get("id"),
            season_id: row.get("season_id"),
            season_year: row.get("season_year"),
            season_display_name: row.get("season_display_name"),
            event_id: row.get("event_id"),
            event_name: row.get("event_name"),
            player_count: row.get("player_count"),
        })
        .collect();

    Ok(Some(TeamDetailEntity {
        team_info,
        participations,
    }))
}
