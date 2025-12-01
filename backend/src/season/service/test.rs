use crate::season::service::{CreateSeasonEntity, SeasonFilters, UpdateSeasonEntity};

#[sqlx::test(fixtures("events"))]
async fn create_season(db: sqlx::SqlitePool) {
    let season = CreateSeasonEntity {
        year: 2024,
        display_name: Some("Test Season".to_string()),
        event_id: 1,
    };
    let id = crate::season::service::create_season(&db, season)
        .await
        .unwrap();
    assert!(id > 0);

    // Verify audit columns are set
    let created_season = crate::season::service::get_season_by_id(&db, id)
        .await
        .unwrap()
        .unwrap();
    assert!(!created_season.created_at.is_empty());
    assert!(!created_season.updated_at.is_empty());
    assert_eq!(created_season.created_at, created_season.updated_at); // Should be equal on creation
}

#[sqlx::test(fixtures("events", "get_seasons"))]
async fn get_seasons(db: sqlx::SqlitePool) {
    let filters = SeasonFilters::default();
    let seasons = crate::season::service::get_seasons(&db, &filters, None)
        .await
        .unwrap();
    assert!(!seasons.items.is_empty());
    assert_eq!(seasons.items.len(), 2); // Assuming the fixture has 2 seasons
    for season in &seasons.items {
        assert!(season.id > 0);
        assert!(season.event_id > 0);
        assert!(season.year > 0);
        assert!(!season.event_name.is_empty());
        assert!(!season.created_at.is_empty());
        assert!(!season.updated_at.is_empty());
    }
}

#[sqlx::test(fixtures("events", "get_seasons"))]
async fn get_season_by_id(db: sqlx::SqlitePool) {
    let season = crate::season::service::get_season_by_id(&db, 1)
        .await
        .unwrap();
    assert!(season.is_some());
    let season = season.unwrap();
    assert_eq!(season.id, 1);
    assert_eq!(season.display_name, Some("2024 Season".to_string()));
    assert_eq!(season.event_id, 1);
    assert_eq!(season.year, 2024);
    assert!(!season.event_name.is_empty());
}

#[sqlx::test(fixtures("events", "get_seasons"))]
async fn get_season_by_id_renamed(db: sqlx::SqlitePool) {
    let season = crate::season::service::get_season_by_id(&db, 2)
        .await
        .unwrap();
    assert!(season.is_some());
    let season = season.unwrap();
    assert_eq!(season.id, 2);
    assert_eq!(season.display_name, None);
    assert_eq!(season.event_id, 2);
    assert_eq!(season.year, 2025);
    assert!(!season.event_name.is_empty());
}

#[sqlx::test(fixtures("events", "get_seasons"))]
async fn get_season_by_id_empty(db: sqlx::SqlitePool) {
    let season = crate::season::service::get_season_by_id(&db, 999)
        .await
        .unwrap();
    assert!(season.is_none());
}

#[sqlx::test(fixtures("events", "get_seasons"))]
async fn update_season(db: sqlx::SqlitePool) {
    // Get the original season to compare timestamps
    let original_season = crate::season::service::get_season_by_id(&db, 1)
        .await
        .unwrap()
        .unwrap();
    let original_created_at = original_season.created_at.clone();
    let original_updated_at = original_season.updated_at.clone();

    // Add a small delay to ensure updated_at timestamp is different
    tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;

    let update_data = UpdateSeasonEntity {
        year: 2026,
        display_name: Some("Updated Season".to_string()),
        event_id: 2,
    };
    let result = crate::season::service::update_season(&db, 1, update_data)
        .await
        .unwrap();
    assert!(result);

    // Verify the season is updated
    let season = crate::season::service::get_season_by_id(&db, 1)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(season.year, 2026);
    assert_eq!(season.display_name, Some("Updated Season".to_string()));
    assert_eq!(season.event_id, 2);

    // Verify audit columns: created_at should remain the same, updated_at should change
    assert_eq!(season.created_at, original_created_at);
    assert_ne!(season.updated_at, original_updated_at);
}

#[sqlx::test(fixtures("events", "get_seasons"))]
async fn get_seasons_with_filters(db: sqlx::SqlitePool) {
    // Test filtering by year
    let filters = SeasonFilters::new(Some(2024), None);
    let seasons = crate::season::service::get_seasons(&db, &filters, None)
        .await
        .unwrap();
    assert_eq!(seasons.items.len(), 1);
    assert_eq!(seasons.items[0].year, 2024);

    // Test filtering by event_id
    let filters = SeasonFilters::new(None, Some(1));
    let seasons = crate::season::service::get_seasons(&db, &filters, None)
        .await
        .unwrap();
    assert_eq!(seasons.items.len(), 1);
    assert_eq!(seasons.items[0].event_id, 1);

    // Test filtering by both
    let filters = SeasonFilters::new(Some(2025), Some(2));
    let seasons = crate::season::service::get_seasons(&db, &filters, None)
        .await
        .unwrap();
    assert_eq!(seasons.items.len(), 1);
    assert_eq!(seasons.items[0].year, 2025);
    assert_eq!(seasons.items[0].event_id, 2);
}

#[sqlx::test(fixtures("events", "get_seasons"))]
async fn delete_season(db: sqlx::SqlitePool) {
    let result = crate::season::service::delete_season(&db, 1).await.unwrap();
    assert!(result);

    // Verify the season is deleted
    let season = crate::season::service::get_season_by_id(&db, 1)
        .await
        .unwrap();
    assert!(season.is_none());
    // Try to delete again, should return false
    let result = crate::season::service::delete_season(&db, 1).await.unwrap();
    assert!(!result);
}

#[sqlx::test(fixtures("events", "get_seasons", "roster"))]
async fn get_players_for_team_in_season(db: sqlx::SqlitePool) {
    // Get players for team 1 in season 1
    let players = crate::season::service::get_players_for_team_in_season(&db, 1, 1)
        .await
        .unwrap();

    assert_eq!(players.len(), 1);
    assert_eq!(players[0].name, "Player A");
    assert_eq!(players[0].nationality, "Country A");

    // Get players for team 2 in season 2
    let players = crate::season::service::get_players_for_team_in_season(&db, 2, 2)
        .await
        .unwrap();

    assert_eq!(players.len(), 1);
    assert_eq!(players[0].name, "Player B");
    assert_eq!(players[0].nationality, "Country B");

    // Test with non-existent combination
    let players = crate::season::service::get_players_for_team_in_season(&db, 1, 2)
        .await
        .unwrap();

    assert_eq!(players.len(), 0);
}
