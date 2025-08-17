use crate::player::service::{CreatePlayerEntity, PlayerFilters, UpdatePlayerEntity};

#[sqlx::test]
async fn create_player(db: sqlx::SqlitePool) {
    let player = CreatePlayerEntity {
        name: "Test Player".to_string(),
        country_id: 1, // Assuming the player is not associated with a country
        photo_path: Some("http://localhost:9000/hockey-uploads/test-player-photo.jpg".to_string()),
    };
    let id = crate::player::service::create_player(&db, player)
        .await
        .unwrap();
    assert!(id > 0);

    // Verify audit columns are set
    let created_player = crate::player::service::get_player_by_id(&db, id)
        .await
        .unwrap()
        .unwrap();
    assert!(!created_player.created_at.is_empty());
    assert!(!created_player.updated_at.is_empty());
    assert_eq!(created_player.created_at, created_player.updated_at); // Should be equal on creation
}

#[sqlx::test(fixtures("get_players"))]
async fn get_players(db: sqlx::SqlitePool) {
    let filters = PlayerFilters::default();
    let players = crate::player::service::get_players(&db, &filters, None)
        .await
        .unwrap();
    assert!(!players.items.is_empty());
    assert_eq!(players.items.len(), 2); // Assuming the fixture has 2 players
    for player in &players.items {
        assert!(player.id > 0);
        assert!(!player.name.is_empty());
        assert!(!player.country_name.is_empty());
        assert!(!player.country_iso2_code.is_empty());
        assert!(!player.created_at.is_empty());
        assert!(!player.updated_at.is_empty());
    }
}

#[sqlx::test(fixtures("get_players"))]
async fn get_player_by_id(db: sqlx::SqlitePool) {
    let player = crate::player::service::get_player_by_id(&db, 1)
        .await
        .unwrap();
    assert!(player.is_some());
    let player = player.unwrap();
    assert_eq!(player.id, 1);
    assert_eq!(player.name, "Player A".to_string());
    assert_eq!(player.country_id, 1);
    assert!(!player.country_name.is_empty());
    assert!(!player.country_iso2_code.is_empty());
}

#[sqlx::test(fixtures("get_players"))]
async fn get_player_by_id_empty(db: sqlx::SqlitePool) {
    let player = crate::player::service::get_player_by_id(&db, 999)
        .await
        .unwrap();
    assert!(player.is_none());
}

#[sqlx::test(fixtures("get_players"))]
async fn delete_player(db: sqlx::SqlitePool) {
    let result = crate::player::service::delete_player(&db, 1).await.unwrap();
    assert!(result);

    // Verify the player is deleted
    let player = crate::player::service::get_player_by_id(&db, 1)
        .await
        .unwrap();
    assert!(player.is_none());
    // Try to delete again, should return false
    let result = crate::player::service::delete_player(&db, 1).await.unwrap();
    assert!(!result);
}

#[sqlx::test(fixtures("get_players"))]
async fn update_player_success(db: sqlx::SqlitePool) {
    // Get the original player to compare timestamps
    let original_player = crate::player::service::get_player_by_id(&db, 1)
        .await
        .unwrap()
        .unwrap();
    let original_created_at = original_player.created_at.clone();
    let original_updated_at = original_player.updated_at.clone();

    // Add a small delay to ensure updated_at timestamp is different
    tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;

    let update_data = UpdatePlayerEntity {
        name: "Updated Player Name".to_string(),
        country_id: 2,
        photo_path: Some(
            "http://localhost:9000/hockey-uploads/updated-player-photo.jpg".to_string(),
        ),
    };

    let result = crate::player::service::update_player(&db, 1, update_data)
        .await
        .unwrap();
    assert!(result);

    // Verify the update
    let player = crate::player::service::get_player_by_id(&db, 1)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(player.name, "Updated Player Name");
    assert_eq!(player.country_id, 2);

    // Verify audit columns: created_at should remain the same, updated_at should change
    assert_eq!(player.created_at, original_created_at);
    assert_ne!(player.updated_at, original_updated_at);
}

#[sqlx::test(fixtures("get_players"))]
async fn update_player_not_found(db: sqlx::SqlitePool) {
    let update_data = UpdatePlayerEntity {
        name: "Updated Player Name".to_string(),
        country_id: 2,
        photo_path: None,
    };

    let result = crate::player::service::update_player(&db, 999, update_data)
        .await
        .unwrap();
    assert!(!result);
}
