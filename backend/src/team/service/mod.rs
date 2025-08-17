use crate::common::paging::{PagedResult, Paging};
use crate::team::{Team, TeamDetail, TeamRosterPlayer, TeamSeasonParticipation};
use sqlx::Row;
use sqlx::SqlitePool;
use std::collections::HashMap;

#[cfg(test)]
pub mod test;

pub struct CreateTeamEntity {
    pub name: Option<String>,
    pub country_id: i64,
    pub logo_path: Option<String>,
}

#[allow(dead_code)]
pub struct TeamEntity {
    pub id: i64,
    pub name: Option<String>,
    pub country_id: i64,
    pub logo_path: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

pub struct TeamWithCountryEntity {
    pub id: i64,
    pub name: Option<String>,
    pub country_id: i64,
    pub country_name: String,
    pub country_iso2_code: String,
    pub logo_path: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

pub struct UpdateTeamEntity {
    pub name: Option<String>,
    pub country_id: i64,
    pub logo_path: Option<String>,
}

#[derive(Debug, Clone)]
pub struct TeamFilters {
    pub name: Option<String>,
    pub country_id: Option<i64>,
}

impl Default for TeamFilters {
    fn default() -> Self {
        Self {
            name: None,
            country_id: None,
        }
    }
}

impl TeamFilters {
    pub fn new(name: Option<String>, country_id: Option<i64>) -> Self {
        Self { name, country_id }
    }
}

pub async fn create_team(db: &SqlitePool, team: CreateTeamEntity) -> Result<i64, sqlx::Error> {
    let result = sqlx::query("INSERT INTO team (name, country_id, logo_path, created_at, updated_at) VALUES (?, ?, ?, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)")
        .bind(team.name)
        .bind(team.country_id)
        .bind(team.logo_path)
        .execute(db)
        .await?;

    Ok(result.last_insert_rowid())
}

fn apply_team_filters<'a>(
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

pub async fn get_teams(
    db: &SqlitePool,
    filters: &TeamFilters,
    paging: Option<&Paging>,
) -> Result<PagedResult<TeamWithCountryEntity>, sqlx::Error> {
    // Build count query with WHERE 1=1 trick for cleaner filter additions
    let mut count_query_builder = sqlx::QueryBuilder::new(
        "SELECT COUNT(*) as count FROM team t INNER JOIN country c ON t.country_id = c.id WHERE 1=1",
    );
    apply_team_filters(&mut count_query_builder, filters);

    // Execute count query
    let count_query = count_query_builder.build();
    let total: i64 = count_query.fetch_one(db).await?.get("count");

    let total = total as usize;
    let default_paging = Paging::default();
    let paging = paging.unwrap_or(&default_paging);

    // Build main query with JOIN to get country information
    let mut data_query_builder = sqlx::QueryBuilder::new(
        "SELECT t.id, t.name, t.country_id, t.logo_path, t.created_at, t.updated_at, c.name as country_name, c.iso2Code as country_iso2_code 
         FROM team t 
         INNER JOIN country c ON t.country_id = c.id 
         WHERE 1=1"
    );
    apply_team_filters(&mut data_query_builder, filters);
    data_query_builder.push(" ORDER BY t.id");

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
        .map(|row| TeamWithCountryEntity {
            id: row.get("id"),
            name: row.get("name"),
            country_id: row.get("country_id"),
            country_name: row.get("country_name"),
            country_iso2_code: row.get("country_iso2_code"),
            logo_path: row.get("logo_path"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
        .collect();

    Ok(PagedResult::new(items, total, paging))
}

pub async fn get_team_by_id(
    db: &SqlitePool,
    id: i64,
) -> Result<Option<TeamWithCountryEntity>, sqlx::Error> {
    let row = sqlx::query(
        "SELECT t.id, t.name, t.country_id, t.logo_path, t.created_at, t.updated_at, c.name as country_name, c.iso2Code as country_iso2_code 
         FROM team t 
         INNER JOIN country c ON t.country_id = c.id 
         WHERE t.id = ?"
    )
        .bind(id)
        .fetch_optional(db)
        .await?;

    Ok(row.map(|row| TeamWithCountryEntity {
        id: row.get("id"),
        name: row.get("name"),
        country_id: row.get("country_id"),
        country_name: row.get("country_name"),
        country_iso2_code: row.get("country_iso2_code"),
        logo_path: row.get("logo_path"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    }))
}

pub async fn update_team(
    db: &SqlitePool,
    id: i64,
    team: UpdateTeamEntity,
) -> Result<bool, sqlx::Error> {
    let result = sqlx::query(
        "UPDATE team SET name = ?, country_id = ?, logo_path = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?",
    )
    .bind(team.name)
    .bind(team.country_id)
    .bind(team.logo_path)
    .bind(id)
    .execute(db)
    .await?;
    Ok(result.rows_affected() > 0)
}

pub async fn delete_team(db: &SqlitePool, id: i64) -> Result<bool, sqlx::Error> {
    let result = sqlx::query("DELETE FROM team WHERE id = ?")
        .bind(id)
        .execute(db)
        .await?;
    Ok(result.rows_affected() > 0)
}

pub struct TeamListItem {
    pub id: i64,
    pub name: Option<String>,
}

pub async fn get_teams_list(db: &SqlitePool) -> Result<Vec<TeamListItem>, sqlx::Error> {
    let rows = sqlx::query("SELECT id, name FROM team ORDER BY name")
        .fetch_all(db)
        .await?;

    let items = rows
        .into_iter()
        .map(|row| TeamListItem {
            id: row.get("id"),
            name: row.get("name"),
        })
        .collect();

    Ok(items)
}

pub async fn get_team_detail(
    db: &SqlitePool,
    team_id: i64,
) -> Result<Option<TeamDetail>, sqlx::Error> {
    // First get the team basic information
    let team_row = sqlx::query(
        r#"
        SELECT 
            t.id, t.name, t.country_id, t.logo_path, t.created_at, t.updated_at,
            c.name as country_name, c.iso2Code as country_iso2_code
        FROM team t
        INNER JOIN country c ON t.country_id = c.id
        WHERE t.id = ?
        "#,
    )
    .bind(team_id)
    .fetch_optional(db)
    .await?;

    let team_row = match team_row {
        Some(row) => row,
        None => return Ok(None),
    };

    // Create the team object
    let team = Team {
        id: team_row.get("id"),
        name: team_row.get("name"),
        country_id: team_row.get("country_id"),
        country_name: team_row.get("country_name"),
        country_iso2_code: team_row.get("country_iso2_code"),
        logo_path: team_row.get("logo_path"),
        created_at: team_row.get("created_at"),
        updated_at: team_row.get("updated_at"),
    };

    // Get all participations with roster data
    let participation_rows = sqlx::query(
        r#"
        SELECT 
            tp.id as participation_id, 
            tp.season_id,
            s.display_name as season_name,
            p.id as player_id, 
            p.name as player_name, 
            c.name as country_name,
            pc.id as contract_id
        FROM team_participation tp
        INNER JOIN season s ON tp.season_id = s.id
        LEFT JOIN player_contract pc ON tp.id = pc.team_participation_id
        LEFT JOIN player p ON pc.player_id = p.id
        LEFT JOIN country c ON p.country_id = c.id
        WHERE tp.team_id = ?
        ORDER BY s.display_name DESC, p.name ASC
        "#,
    )
    .bind(team_id)
    .fetch_all(db)
    .await?;

    // Group the data by season
    let mut participations_map: HashMap<i64, TeamSeasonParticipation> = HashMap::new();

    for row in participation_rows {
        let season_id: i64 = row.get("season_id");
        let participation_id: i64 = row.get("participation_id");
        let season_name: String = row.get("season_name");

        // Get or create the participation entry
        let participation =
            participations_map
                .entry(season_id)
                .or_insert_with(|| TeamSeasonParticipation {
                    season_id,
                    season_name: season_name.clone(),
                    participation_id,
                    roster: Vec::new(),
                });

        // Add player to roster if present (LEFT JOIN might return NULL)
        if let Some(player_id) = row.try_get::<Option<i64>, _>("player_id")? {
            if let (Some(player_name), Some(country_name), Some(contract_id)) = (
                row.try_get::<Option<String>, _>("player_name")?,
                row.try_get::<Option<String>, _>("country_name")?,
                row.try_get::<Option<i64>, _>("contract_id")?,
            ) {
                participation.roster.push(TeamRosterPlayer {
                    player_id,
                    player_name,
                    country_name,
                    contract_id,
                });
            }
        }
    }

    // Convert to vector and sort by season name (descending - most recent first)
    let mut participations: Vec<TeamSeasonParticipation> =
        participations_map.into_values().collect();
    participations.sort_by(|a, b| b.season_name.cmp(&a.season_name));

    Ok(Some(TeamDetail {
        team,
        participations,
    }))
}
