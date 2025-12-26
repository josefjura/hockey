use sqlx::{Row, SqlitePool};

// Re-export common pagination types for convenience
pub use crate::common::pagination::{PagedResult, SortOrder};

#[derive(Debug, Clone)]
pub struct PlayerEntity {
    pub id: i64,
    pub name: String,
    pub country_id: i64,
    pub country_name: String,
    pub country_iso2_code: String,
    pub photo_path: Option<String>,
}

#[derive(Debug, Clone)]
pub struct PlayerContractWithTeamEntity {
    pub team_id: i64,
    pub team_name: String,
    pub team_country_iso2_code: Option<String>,
    pub season_id: i64,
    pub season_year: i64,
    pub season_display_name: Option<String>,
    pub event_name: String,
}

#[derive(Debug, Clone)]
pub struct PlayerDetailEntity {
    pub player_info: PlayerEntity,
    pub contracts: Vec<PlayerContractWithTeamEntity>,
}

#[derive(Debug, Clone)]
pub struct CreatePlayerEntity {
    pub name: String,
    pub country_id: i64,
    pub photo_path: Option<String>,
}

#[derive(Debug, Clone)]
pub struct UpdatePlayerEntity {
    pub name: String,
    pub country_id: i64,
    pub photo_path: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct PlayerFilters {
    pub name: Option<String>,
    pub country_id: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SortField {
    Id,
    Name,
    Country,
}

impl SortField {
    pub fn from_str(s: &str) -> Self {
        match s {
            "id" => Self::Id,
            "name" => Self::Name,
            "country" => Self::Country,
            _ => Self::Name, // Default
        }
    }

    pub fn to_sql(&self) -> &'static str {
        match self {
            Self::Id => "p.id",
            Self::Name => "p.name",
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

/// Create a new player
pub async fn create_player(
    db: &SqlitePool,
    player: CreatePlayerEntity,
) -> Result<i64, sqlx::Error> {
    let result = sqlx::query("INSERT INTO player (name, country_id, photo_path) VALUES (?, ?, ?)")
        .bind(player.name)
        .bind(player.country_id)
        .bind(player.photo_path)
        .execute(db)
        .await?;

    Ok(result.last_insert_rowid())
}

/// Get players with filters and pagination
pub async fn get_players(
    db: &SqlitePool,
    filters: &PlayerFilters,
    sort_field: &SortField,
    sort_order: &SortOrder,
    page: usize,
    page_size: usize,
) -> Result<PagedResult<PlayerEntity>, sqlx::Error> {
    // Build count query
    let mut count_query = sqlx::QueryBuilder::new(
        "SELECT COUNT(*) as count FROM player p INNER JOIN country c ON p.country_id = c.id WHERE 1=1",
    );
    apply_filters(&mut count_query, filters);

    let total: i64 = count_query.build().fetch_one(db).await?.get("count");

    // Build data query
    let mut data_query = sqlx::QueryBuilder::new(
        "SELECT p.id, p.name, p.country_id, p.photo_path, c.name as country_name, c.iso2Code as country_iso2_code
         FROM player p
         INNER JOIN country c ON p.country_id = c.id
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
        .map(|row| PlayerEntity {
            id: row.get("id"),
            name: row.get("name"),
            country_id: row.get("country_id"),
            country_name: row.get("country_name"),
            country_iso2_code: row.get("country_iso2_code"),
            photo_path: row.get("photo_path"),
        })
        .collect();

    Ok(PagedResult::new(items, total as usize, page, page_size))
}

/// Get a single player by ID
pub async fn get_player_by_id(
    db: &SqlitePool,
    id: i64,
) -> Result<Option<PlayerEntity>, sqlx::Error> {
    let row = sqlx::query(
        "SELECT p.id, p.name, p.country_id, p.photo_path, c.name as country_name, c.iso2Code as country_iso2_code
         FROM player p
         INNER JOIN country c ON p.country_id = c.id
         WHERE p.id = ?",
    )
    .bind(id)
    .fetch_optional(db)
    .await?;

    Ok(row.map(|row| PlayerEntity {
        id: row.get("id"),
        name: row.get("name"),
        country_id: row.get("country_id"),
        country_name: row.get("country_name"),
        country_iso2_code: row.get("country_iso2_code"),
        photo_path: row.get("photo_path"),
    }))
}

/// Update a player
pub async fn update_player(
    db: &SqlitePool,
    id: i64,
    player: UpdatePlayerEntity,
) -> Result<bool, sqlx::Error> {
    let result =
        sqlx::query("UPDATE player SET name = ?, country_id = ?, photo_path = ? WHERE id = ?")
            .bind(player.name)
            .bind(player.country_id)
            .bind(player.photo_path)
            .bind(id)
            .execute(db)
            .await?;

    Ok(result.rows_affected() > 0)
}

/// Delete a player
pub async fn delete_player(db: &SqlitePool, id: i64) -> Result<bool, sqlx::Error> {
    let result = sqlx::query("DELETE FROM player WHERE id = ?")
        .bind(id)
        .execute(db)
        .await?;

    Ok(result.rows_affected() > 0)
}

/// Helper function to apply filters to a query
fn apply_filters<'a>(
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

/// Get all countries for dropdowns (only enabled countries)
pub async fn get_countries(db: &SqlitePool) -> Result<Vec<(i64, String)>, sqlx::Error> {
    crate::service::countries::get_countries_simple(db).await
}

/// Get player detail with all contracts (career history)
pub async fn get_player_detail(
    db: &SqlitePool,
    id: i64,
) -> Result<Option<PlayerDetailEntity>, sqlx::Error> {
    // Get player info
    let player_info = match get_player_by_id(db, id).await? {
        Some(p) => p,
        None => return Ok(None),
    };

    // Get all contracts with team, season, and event info
    let rows = sqlx::query(
        "SELECT
            t.id as team_id,
            t.name as team_name,
            tc.iso2Code as team_country_iso2_code,
            s.id as season_id,
            s.year as season_year,
            s.display_name as season_display_name,
            e.name as event_name
        FROM player_contract pc
        INNER JOIN team_participation tp ON pc.team_participation_id = tp.id
        INNER JOIN team t ON tp.team_id = t.id
        LEFT JOIN country tc ON t.country_id = tc.id
        INNER JOIN season s ON tp.season_id = s.id
        INNER JOIN event e ON s.event_id = e.id
        WHERE pc.player_id = ?
        ORDER BY s.year DESC, e.name ASC",
    )
    .bind(id)
    .fetch_all(db)
    .await?;

    let contracts = rows
        .into_iter()
        .map(|row| PlayerContractWithTeamEntity {
            team_id: row.get("team_id"),
            team_name: row.get("team_name"),
            team_country_iso2_code: row.get("team_country_iso2_code"),
            season_id: row.get("season_id"),
            season_year: row.get("season_year"),
            season_display_name: row.get("season_display_name"),
            event_name: row.get("event_name"),
        })
        .collect();

    Ok(Some(PlayerDetailEntity {
        player_info,
        contracts,
    }))
}
