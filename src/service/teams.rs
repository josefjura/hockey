use sqlx::{Row, SqlitePool};

use crate::common::pagination::PagedResult;

// Re-export SortOrder for backwards compatibility
pub use crate::common::pagination::SortOrder;

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

// SortOrder is now imported from crate::common::pagination

/// Create a new team
pub async fn create_team(db: &SqlitePool, team: CreateTeamEntity) -> Result<i64, sqlx::Error> {
    let result = sqlx::query!(
        r#"
        INSERT INTO team (name, country_id)
        VALUES (?, ?)
        "#,
        team.name,
        team.country_id
    )
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
    // SECURITY: Using .push() method to safely append enum values.
    // This prevents SQL injection as values come from trusted enum matches.
    data_query
        .push(" ORDER BY ")
        .push(sort_field.to_sql())
        .push(" ")
        .push(sort_order.to_sql());

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
    let row = sqlx::query_as!(
        TeamEntity,
        r#"
        SELECT
            t.id,
            t.name as name,
            t.country_id,
            c.name as country_name,
            c.iso2Code as country_iso2_code
        FROM team t
        LEFT JOIN country c ON t.country_id = c.id
        WHERE t.id = ?
        "#,
        id
    )
    .fetch_optional(db)
    .await?;

    Ok(row)
}

/// Update a team
pub async fn update_team(
    db: &SqlitePool,
    id: i64,
    team: UpdateTeamEntity,
) -> Result<bool, sqlx::Error> {
    let result = sqlx::query!(
        r#"
        UPDATE team
        SET name = ?, country_id = ?
        WHERE id = ?
        "#,
        team.name,
        team.country_id,
        id
    )
    .execute(db)
    .await?;

    Ok(result.rows_affected() > 0)
}

/// Delete a team
pub async fn delete_team(db: &SqlitePool, id: i64) -> Result<bool, sqlx::Error> {
    let result = sqlx::query!(
        r#"
        DELETE FROM team
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

/// Get all countries for dropdowns (only enabled countries)
pub async fn get_countries(db: &SqlitePool) -> Result<Vec<(i64, String)>, sqlx::Error> {
    crate::service::countries::get_countries_simple(db).await
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
    let rows = sqlx::query_as!(
        TeamParticipationWithSeasonEntity,
        r#"
        SELECT
            tp.id as "id!",
            tp.season_id as "season_id!",
            s.year as "season_year!",
            s.display_name as season_display_name,
            e.name as "event_name!",
            COALESCE(
                (SELECT COUNT(*) FROM player_contract pc WHERE pc.team_participation_id = tp.id),
                0
            ) as "player_count!: i64"
        FROM team_participation tp
        INNER JOIN season s ON tp.season_id = s.id
        INNER JOIN event e ON s.event_id = e.id
        WHERE tp.team_id = ?
        ORDER BY s.year DESC, e.name ASC
        "#,
        id
    )
    .fetch_all(db)
    .await?;

    let participations = rows;

    Ok(Some(TeamDetailEntity {
        team_info,
        participations,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::SqlitePool;

    #[sqlx::test(migrations = "./migrations")]
    async fn test_create_team(pool: SqlitePool) {
        let team = CreateTeamEntity {
            name: "Test Team".to_string(),
            country_id: Some(1), // Canada from migrations
        };

        let id = create_team(&pool, team).await.unwrap();
        assert!(id > 0);

        // Verify team was created
        let result = get_team_by_id(&pool, id).await.unwrap();
        assert!(result.is_some());
        let team = result.unwrap();
        assert_eq!(team.name, "Test Team");
        assert_eq!(team.country_id, Some(1));
    }

    #[sqlx::test(migrations = "./migrations", fixtures("teams"))]
    async fn test_get_teams_no_filters(pool: SqlitePool) {
        let filters = TeamFilters::default();
        let result = get_teams(&pool, &filters, &SortField::Name, &SortOrder::Asc, 1, 20)
            .await
            .unwrap();

        assert!(result.items.len() >= 5); // At least the fixture teams
        assert!(result.total >= 5);
        assert!(result.items.iter().any(|t| t.name == "Team Canada"));
    }

    #[sqlx::test(migrations = "./migrations", fixtures("teams"))]
    async fn test_get_teams_with_name_filter(pool: SqlitePool) {
        let filters = TeamFilters {
            name: Some("Canada".to_string()),
            ..Default::default()
        };
        let result = get_teams(&pool, &filters, &SortField::Name, &SortOrder::Asc, 1, 20)
            .await
            .unwrap();

        assert!(!result.items.is_empty());
        assert!(result.items.iter().all(|t| t.name.contains("Canada")));
    }

    #[sqlx::test(migrations = "./migrations", fixtures("teams"))]
    async fn test_get_teams_with_country_filter(pool: SqlitePool) {
        let canada_id: i64 = sqlx::query_scalar("SELECT id FROM country WHERE name = 'Canada'")
            .fetch_one(&pool)
            .await
            .unwrap();

        let filters = TeamFilters {
            country_id: Some(canada_id),
            ..Default::default()
        };
        let result = get_teams(&pool, &filters, &SortField::Name, &SortOrder::Asc, 1, 20)
            .await
            .unwrap();

        assert!(!result.items.is_empty());
        assert!(result.items.iter().all(|t| t.country_id == Some(canada_id)));
    }

    #[sqlx::test(migrations = "./migrations", fixtures("teams"))]
    async fn test_get_teams_pagination(pool: SqlitePool) {
        let filters = TeamFilters::default();
        let result = get_teams(&pool, &filters, &SortField::Name, &SortOrder::Asc, 1, 2)
            .await
            .unwrap();

        assert!(result.items.len() <= 2);
        assert!(result.total >= 5);
    }

    #[sqlx::test(migrations = "./migrations", fixtures("teams"))]
    async fn test_get_teams_sorting(pool: SqlitePool) {
        let filters = TeamFilters::default();
        let result = get_teams(&pool, &filters, &SortField::Name, &SortOrder::Desc, 1, 20)
            .await
            .unwrap();

        // Verify descending order
        for i in 0..result.items.len() - 1 {
            assert!(result.items[i].name >= result.items[i + 1].name);
        }
    }

    #[sqlx::test(migrations = "./migrations", fixtures("teams"))]
    async fn test_get_team_by_id_found(pool: SqlitePool) {
        let result = get_team_by_id(&pool, 1).await.unwrap();

        assert!(result.is_some());
        let team = result.unwrap();
        assert_eq!(team.id, 1);
        assert_eq!(team.name, "Team Canada");
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn test_get_team_by_id_not_found(pool: SqlitePool) {
        let result = get_team_by_id(&pool, 999).await.unwrap();
        assert!(result.is_none());
    }

    #[sqlx::test(migrations = "./migrations", fixtures("teams"))]
    async fn test_update_team(pool: SqlitePool) {
        let update = UpdateTeamEntity {
            name: "Updated Team Canada".to_string(),
            country_id: Some(1),
        };

        let success = update_team(&pool, 1, update).await.unwrap();
        assert!(success);

        // Verify update
        let team = get_team_by_id(&pool, 1).await.unwrap().unwrap();
        assert_eq!(team.name, "Updated Team Canada");
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn test_update_team_not_found(pool: SqlitePool) {
        let update = UpdateTeamEntity {
            name: "Non-existent Team".to_string(),
            country_id: Some(1),
        };

        let success = update_team(&pool, 999, update).await.unwrap();
        assert!(!success);
    }

    #[sqlx::test(migrations = "./migrations", fixtures("teams"))]
    async fn test_delete_team(pool: SqlitePool) {
        let success = delete_team(&pool, 1).await.unwrap();
        assert!(success);

        // Verify deletion
        let result = get_team_by_id(&pool, 1).await.unwrap();
        assert!(result.is_none());
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn test_delete_team_not_found(pool: SqlitePool) {
        let success = delete_team(&pool, 999).await.unwrap();
        assert!(!success);
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn test_get_countries_for_team_creation(pool: SqlitePool) {
        let countries = get_countries(&pool).await.unwrap();

        assert!(!countries.is_empty());
        assert!(countries.len() > 50); // Many countries in migrations
                                       // Verify format is (id, name)
        assert!(countries.iter().any(|(_, name)| name == "Canada"));
    }

    #[sqlx::test(migrations = "./migrations", fixtures("teams"))]
    async fn test_get_team_detail_basic(pool: SqlitePool) {
        // Team 1 is "Team Canada" from fixtures
        let detail = get_team_detail(&pool, 1).await.unwrap();

        assert!(detail.is_some());
        let detail = detail.unwrap();
        // Just verify we got team details, don't assume specific country
        assert_eq!(detail.team_info.id, 1);
        assert!(!detail.team_info.name.is_empty());
        // Participations list can be empty for basic team
        assert_eq!(detail.participations.len(), 0);
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn test_get_team_detail_not_found(pool: SqlitePool) {
        let detail = get_team_detail(&pool, 999).await.unwrap();
        assert!(detail.is_none());
    }
}
