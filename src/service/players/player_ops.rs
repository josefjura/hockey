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
    pub birth_date: Option<String>,
    pub birth_place: Option<String>,
    pub height_cm: Option<i64>,
    pub weight_kg: Option<i64>,
    pub position: Option<String>,
    pub shoots: Option<String>,
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
    pub birth_date: Option<String>,
    pub birth_place: Option<String>,
    pub height_cm: Option<i64>,
    pub weight_kg: Option<i64>,
    pub position: Option<String>,
    pub shoots: Option<String>,
}

#[derive(Debug, Clone)]
pub struct UpdatePlayerEntity {
    pub name: String,
    pub country_id: i64,
    pub photo_path: Option<String>,
    pub birth_date: Option<String>,
    pub birth_place: Option<String>,
    pub height_cm: Option<i64>,
    pub weight_kg: Option<i64>,
    pub position: Option<String>,
    pub shoots: Option<String>,
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
    let result = sqlx::query!(
        r#"
        INSERT INTO player (name, country_id, photo_path, birth_date, birth_place, height_cm, weight_kg, position, shoots)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#,
        player.name,
        player.country_id,
        player.photo_path,
        player.birth_date,
        player.birth_place,
        player.height_cm,
        player.weight_kg,
        player.position,
        player.shoots
    )
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
        "SELECT p.id, p.name, p.country_id, p.photo_path, p.birth_date, p.birth_place, p.height_cm, p.weight_kg, p.position, p.shoots,
         c.name as country_name, c.iso2Code as country_iso2_code
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
            birth_date: row.get("birth_date"),
            birth_place: row.get("birth_place"),
            height_cm: row.get("height_cm"),
            weight_kg: row.get("weight_kg"),
            position: row.get("position"),
            shoots: row.get("shoots"),
        })
        .collect();

    Ok(PagedResult::new(items, total as usize, page, page_size))
}

/// Get a single player by ID
pub async fn get_player_by_id(
    db: &SqlitePool,
    id: i64,
) -> Result<Option<PlayerEntity>, sqlx::Error> {
    let row = sqlx::query_as!(
        PlayerEntity,
        r#"
        SELECT
            p.id,
            p.name as "name!",
            p.country_id,
            p.photo_path,
            p.birth_date,
            p.birth_place,
            p.height_cm,
            p.weight_kg,
            p.position,
            p.shoots,
            c.name as "country_name!",
            c.iso2Code as "country_iso2_code!"
        FROM player p
        INNER JOIN country c ON p.country_id = c.id
        WHERE p.id = ?
        "#,
        id
    )
    .fetch_optional(db)
    .await?;

    Ok(row)
}

/// Update a player
pub async fn update_player(
    db: &SqlitePool,
    id: i64,
    player: UpdatePlayerEntity,
) -> Result<bool, sqlx::Error> {
    let result = sqlx::query!(
        r#"
        UPDATE player
        SET name = ?, country_id = ?, photo_path = ?, birth_date = ?, birth_place = ?,
            height_cm = ?, weight_kg = ?, position = ?, shoots = ?
        WHERE id = ?
        "#,
        player.name,
        player.country_id,
        player.photo_path,
        player.birth_date,
        player.birth_place,
        player.height_cm,
        player.weight_kg,
        player.position,
        player.shoots,
        id
    )
    .execute(db)
    .await?;

    Ok(result.rows_affected() > 0)
}

/// Delete a player
pub async fn delete_player(db: &SqlitePool, id: i64) -> Result<bool, sqlx::Error> {
    let result = sqlx::query!(
        r#"
        DELETE FROM player
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
    let contracts = sqlx::query_as!(
        PlayerContractWithTeamEntity,
        r#"
        SELECT
            t.id as team_id,
            t.name as "team_name!",
            tc.iso2Code as team_country_iso2_code,
            s.id as season_id,
            s.year as season_year,
            s.display_name as season_display_name,
            e.name as "event_name!"
        FROM player_contract pc
        INNER JOIN team_participation tp ON pc.team_participation_id = tp.id
        INNER JOIN team t ON tp.team_id = t.id
        LEFT JOIN country tc ON t.country_id = tc.id
        INNER JOIN season s ON tp.season_id = s.id
        INNER JOIN event e ON s.event_id = e.id
        WHERE pc.player_id = ?
        ORDER BY s.year DESC, e.name ASC
        "#,
        id
    )
    .fetch_all(db)
    .await?;

    Ok(Some(PlayerDetailEntity {
        player_info,
        contracts,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[sqlx::test(migrations = "./migrations")]
    async fn test_create_player(pool: SqlitePool) {
        // Get Canada's ID from migrations
        let canada_id: i64 = sqlx::query_scalar("SELECT id FROM country WHERE name = 'Canada'")
            .fetch_one(&pool)
            .await
            .unwrap();

        let player = CreatePlayerEntity {
            name: "Wayne Gretzky".to_string(),
            country_id: canada_id,
            photo_path: Some("/photos/gretzky.jpg".to_string()),
            birth_date: Some("1961-01-26".to_string()),
            birth_place: Some("Brantford, ON".to_string()),
            height_cm: Some(183),
            weight_kg: Some(84),
            position: Some("C".to_string()),
            shoots: Some("L".to_string()),
        };

        let id = create_player(&pool, player).await.unwrap();
        assert!(id > 0);

        // Verify player was created
        let result = get_player_by_id(&pool, id).await.unwrap();
        assert!(result.is_some());
        let player = result.unwrap();
        assert_eq!(player.name, "Wayne Gretzky");
        assert_eq!(player.country_id, canada_id);
        assert_eq!(player.position, Some("C".to_string()));
    }

    #[sqlx::test(migrations = "./migrations", fixtures("players"))]
    async fn test_get_players_no_filters(pool: SqlitePool) {
        let filters = PlayerFilters::default();
        let result = get_players(&pool, &filters, &SortField::Name, &SortOrder::Asc, 1, 20)
            .await
            .unwrap();

        assert_eq!(result.items.len(), 10); // All fixture players
        assert_eq!(result.total, 10);

        // Verify sorting by name
        assert!(result.items.iter().any(|p| p.name == "Connor McDavid"));
    }

    #[sqlx::test(migrations = "./migrations", fixtures("players"))]
    async fn test_get_players_with_name_filter(pool: SqlitePool) {
        let filters = PlayerFilters {
            name: Some("McDavid".to_string()),
            ..Default::default()
        };
        let result = get_players(&pool, &filters, &SortField::Name, &SortOrder::Asc, 1, 20)
            .await
            .unwrap();

        assert_eq!(result.items.len(), 1);
        assert_eq!(result.items[0].name, "Connor McDavid");
    }

    #[sqlx::test(migrations = "./migrations", fixtures("players"))]
    async fn test_get_players_with_country_filter(pool: SqlitePool) {
        // Get Canada's ID
        let canada_id: i64 = sqlx::query_scalar("SELECT id FROM country WHERE name = 'Canada'")
            .fetch_one(&pool)
            .await
            .unwrap();

        let filters = PlayerFilters {
            country_id: Some(canada_id),
            ..Default::default()
        };
        let result = get_players(&pool, &filters, &SortField::Name, &SortOrder::Asc, 1, 20)
            .await
            .unwrap();

        assert_eq!(result.items.len(), 3); // 3 Canadian players in fixtures
        assert!(result.items.iter().all(|p| p.country_id == canada_id));
    }

    #[sqlx::test(migrations = "./migrations", fixtures("players"))]
    async fn test_get_players_pagination(pool: SqlitePool) {
        let filters = PlayerFilters::default();
        let result = get_players(&pool, &filters, &SortField::Name, &SortOrder::Asc, 1, 5)
            .await
            .unwrap();

        assert_eq!(result.items.len(), 5); // First page
        assert_eq!(result.total, 10);
        assert_eq!(result.page, 1);
        assert_eq!(result.total_pages, 2);

        // Get second page
        let result_page2 = get_players(&pool, &filters, &SortField::Name, &SortOrder::Asc, 2, 5)
            .await
            .unwrap();

        assert_eq!(result_page2.items.len(), 5); // Second page
        assert_eq!(result_page2.page, 2);
    }

    #[sqlx::test(migrations = "./migrations", fixtures("players"))]
    async fn test_get_players_sorting_by_name(pool: SqlitePool) {
        let filters = PlayerFilters::default();

        // Test ascending order
        let result_asc = get_players(&pool, &filters, &SortField::Name, &SortOrder::Asc, 1, 20)
            .await
            .unwrap();

        for i in 0..result_asc.items.len() - 1 {
            assert!(result_asc.items[i].name <= result_asc.items[i + 1].name);
        }

        // Test descending order
        let result_desc = get_players(&pool, &filters, &SortField::Name, &SortOrder::Desc, 1, 20)
            .await
            .unwrap();

        for i in 0..result_desc.items.len() - 1 {
            assert!(result_desc.items[i].name >= result_desc.items[i + 1].name);
        }
    }

    #[sqlx::test(migrations = "./migrations", fixtures("players"))]
    async fn test_get_players_sorting_by_country(pool: SqlitePool) {
        let filters = PlayerFilters::default();
        let result = get_players(&pool, &filters, &SortField::Country, &SortOrder::Asc, 1, 20)
            .await
            .unwrap();

        // Verify sorting by country name
        for i in 0..result.items.len() - 1 {
            assert!(result.items[i].country_name <= result.items[i + 1].country_name);
        }
    }

    #[sqlx::test(migrations = "./migrations", fixtures("players"))]
    async fn test_get_player_by_id_found(pool: SqlitePool) {
        // Player 1 is Connor McDavid from fixture
        let result = get_player_by_id(&pool, 1).await.unwrap();

        assert!(result.is_some());
        let player = result.unwrap();
        assert_eq!(player.id, 1);
        assert_eq!(player.name, "Connor McDavid");
        assert_eq!(player.position, Some("C".to_string()));
        assert_eq!(player.shoots, Some("L".to_string()));
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn test_get_player_by_id_not_found(pool: SqlitePool) {
        let result = get_player_by_id(&pool, 999).await.unwrap();
        assert!(result.is_none());
    }

    #[sqlx::test(migrations = "./migrations", fixtures("players"))]
    async fn test_update_player(pool: SqlitePool) {
        // Get USA's ID
        let usa_id: i64 = sqlx::query_scalar("SELECT id FROM country WHERE name = 'United States'")
            .fetch_one(&pool)
            .await
            .unwrap();

        // Update player 1 (Connor McDavid)
        let update = UpdatePlayerEntity {
            name: "Connor Andrew McDavid".to_string(),
            country_id: usa_id, // Change nationality for test
            photo_path: Some("/photos/mcdavid_updated.jpg".to_string()),
            birth_date: Some("1997-01-13".to_string()),
            birth_place: Some("Richmond Hill, Ontario".to_string()),
            height_cm: Some(186), // Updated height
            weight_kg: Some(89),  // Updated weight
            position: Some("C".to_string()),
            shoots: Some("L".to_string()),
        };

        let updated = update_player(&pool, 1, update).await.unwrap();
        assert!(updated);

        // Verify changes
        let player = get_player_by_id(&pool, 1).await.unwrap().unwrap();
        assert_eq!(player.name, "Connor Andrew McDavid");
        assert_eq!(player.country_id, usa_id);
        assert_eq!(player.height_cm, Some(186));
        assert_eq!(player.weight_kg, Some(89));
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn test_update_player_not_found(pool: SqlitePool) {
        let canada_id: i64 = sqlx::query_scalar("SELECT id FROM country WHERE name = 'Canada'")
            .fetch_one(&pool)
            .await
            .unwrap();

        let update = UpdatePlayerEntity {
            name: "Ghost Player".to_string(),
            country_id: canada_id,
            photo_path: None,
            birth_date: None,
            birth_place: None,
            height_cm: None,
            weight_kg: None,
            position: None,
            shoots: None,
        };

        let updated = update_player(&pool, 999, update).await.unwrap();
        assert!(!updated); // Should return false for non-existent player
    }

    #[sqlx::test(migrations = "./migrations", fixtures("players"))]
    async fn test_delete_player(pool: SqlitePool) {
        // Delete player 10 (Victor Hedman)
        let deleted = delete_player(&pool, 10).await.unwrap();
        assert!(deleted);

        // Verify player is gone
        let player = get_player_by_id(&pool, 10).await.unwrap();
        assert!(player.is_none());

        // Verify total count decreased
        let result = get_players(
            &pool,
            &PlayerFilters::default(),
            &SortField::Name,
            &SortOrder::Asc,
            1,
            20,
        )
        .await
        .unwrap();
        assert_eq!(result.total, 9); // Was 10, now 9
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn test_delete_player_not_found(pool: SqlitePool) {
        let deleted = delete_player(&pool, 999).await.unwrap();
        assert!(!deleted); // Should return false for non-existent player
    }

    #[sqlx::test(migrations = "./migrations", fixtures("players"))]
    async fn test_get_player_detail_no_contracts(pool: SqlitePool) {
        // Player 1 has no contracts (no team_participations setup)
        let result = get_player_detail(&pool, 1).await.unwrap();

        assert!(result.is_some());
        let detail = result.unwrap();
        assert_eq!(detail.player_info.name, "Connor McDavid");
        assert_eq!(detail.contracts.len(), 0); // No contracts
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn test_get_player_detail_not_found(pool: SqlitePool) {
        let result = get_player_detail(&pool, 999).await.unwrap();
        assert!(result.is_none());
    }

    #[sqlx::test(
        migrations = "./migrations",
        fixtures("events", "seasons", "teams", "team_participations", "players")
    )]
    async fn test_get_player_detail_with_contracts(pool: SqlitePool) {
        // Add a player contract for player 1 in team participation 1
        sqlx::query!(
            "INSERT INTO player_contract (player_id, team_participation_id) VALUES (?, ?)",
            1,
            1
        )
        .execute(&pool)
        .await
        .unwrap();

        let result = get_player_detail(&pool, 1).await.unwrap();

        assert!(result.is_some());
        let detail = result.unwrap();
        assert_eq!(detail.player_info.name, "Connor McDavid");
        assert_eq!(detail.contracts.len(), 1);

        // Verify contract details
        let contract = &detail.contracts[0];
        assert_eq!(contract.team_name, "Team Canada");
        assert_eq!(contract.season_year, 2022);
    }
}
