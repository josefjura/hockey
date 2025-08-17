use crate::common::paging::Paging;
use crate::team::service::{CreateTeamEntity, TeamFilters, UpdateTeamEntity};

#[sqlx::test]
async fn create_team(db: sqlx::SqlitePool) {
    let team = CreateTeamEntity {
        name: Some("Test Team".to_string()),
        country_id: 1, // Assuming country with ID 1 exists
        logo_path: Some("http://localhost:9000/hockey-uploads/test-logo.jpg".to_string()),
    };
    let id = crate::team::service::create_team(&db, team).await.unwrap();
    assert!(id > 0);

    // Verify audit columns are set
    let created_team = crate::team::service::get_team_by_id(&db, id)
        .await
        .unwrap()
        .unwrap();
    assert!(!created_team.created_at.is_empty());
    assert!(!created_team.updated_at.is_empty());
    assert_eq!(created_team.created_at, created_team.updated_at); // Should be equal on creation
}

#[sqlx::test(fixtures("get_teams"))]
async fn get_teams_no_filters(db: sqlx::SqlitePool) {
    let filters = TeamFilters::default();
    let result = crate::team::service::get_teams(&db, &filters, None)
        .await
        .unwrap();

    assert_eq!(result.items.len(), 2); // Fixture has 2 teams
    assert_eq!(result.total, 2);
    assert_eq!(result.page, 1);
    assert_eq!(result.page_size, 20); // Default page size
    assert_eq!(result.total_pages, 1);
    assert!(!result.has_next);
    assert!(!result.has_previous);

    for team in &result.items {
        assert!(team.id > 0);
        assert!(team.country_id > 0);
        assert!(!team.created_at.is_empty());
        assert!(!team.updated_at.is_empty());
    }
}

#[sqlx::test(fixtures("get_teams"))]
async fn get_team_by_id_league(db: sqlx::SqlitePool) {
    let team = crate::team::service::get_team_by_id(&db, 1).await.unwrap();
    assert!(team.is_some());
    let team = team.unwrap();
    assert_eq!(team.id, 1);
    assert_eq!(team.name, Some("Team A".to_string()));
}

#[sqlx::test(fixtures("get_teams"))]
async fn get_team_by_id_national(db: sqlx::SqlitePool) {
    let team = crate::team::service::get_team_by_id(&db, 2).await.unwrap();
    assert!(team.is_some());
    let team = team.unwrap();
    assert_eq!(team.id, 2);
    assert_eq!(team.name, None);
}

#[sqlx::test(fixtures("get_teams"))]
async fn get_team_by_id_empty(db: sqlx::SqlitePool) {
    let team = crate::team::service::get_team_by_id(&db, 999)
        .await
        .unwrap();
    assert!(team.is_none());
}

#[sqlx::test(fixtures("get_teams"))]
async fn get_teams_with_name_filter(db: sqlx::SqlitePool) {
    let filters = TeamFilters::new(Some("Team A".to_string()), None);
    let result = crate::team::service::get_teams(&db, &filters, None)
        .await
        .unwrap();

    assert_eq!(result.items.len(), 1);
    assert_eq!(result.total, 1);
    assert_eq!(result.items[0].name, Some("Team A".to_string()));
}

#[sqlx::test(fixtures("get_teams"))]
async fn get_teams_with_country_filter(db: sqlx::SqlitePool) {
    let filters = TeamFilters::new(None, Some(1));
    let result = crate::team::service::get_teams(&db, &filters, None)
        .await
        .unwrap();

    assert_eq!(result.items.len(), 1);
    assert_eq!(result.total, 1);
    assert_eq!(result.items[0].country_id, 1);
}

#[sqlx::test(fixtures("get_teams"))]
async fn get_teams_with_pagination(db: sqlx::SqlitePool) {
    let filters = TeamFilters::default();
    let paging = Paging::new(1, 1); // Page 1, 1 item per page
    let result = crate::team::service::get_teams(&db, &filters, Some(&paging))
        .await
        .unwrap();

    assert_eq!(result.items.len(), 1);
    assert_eq!(result.total, 2);
    assert_eq!(result.page, 1);
    assert_eq!(result.page_size, 1);
    assert_eq!(result.total_pages, 2);
    assert!(result.has_next);
    assert!(!result.has_previous);
}

#[sqlx::test(fixtures("get_teams"))]
async fn get_teams_second_page(db: sqlx::SqlitePool) {
    let filters = TeamFilters::default();
    let paging = Paging::new(2, 1); // Page 2, 1 item per page
    let result = crate::team::service::get_teams(&db, &filters, Some(&paging))
        .await
        .unwrap();

    assert_eq!(result.items.len(), 1);
    assert_eq!(result.total, 2);
    assert_eq!(result.page, 2);
    assert_eq!(result.page_size, 1);
    assert_eq!(result.total_pages, 2);
    assert!(!result.has_next);
    assert!(result.has_previous);
}

#[sqlx::test(fixtures("get_teams"))]
async fn get_teams_no_results_filter(db: sqlx::SqlitePool) {
    let filters = TeamFilters::new(Some("Nonexistent Team".to_string()), None);
    let result = crate::team::service::get_teams(&db, &filters, None)
        .await
        .unwrap();

    assert_eq!(result.items.len(), 0);
    assert_eq!(result.total, 0);
    assert_eq!(result.total_pages, 0);
    assert!(!result.has_next);
    assert!(!result.has_previous);
}

#[sqlx::test(fixtures("get_teams"))]
async fn update_team_success(db: sqlx::SqlitePool) {
    // Get the original team to compare timestamps
    let original_team = crate::team::service::get_team_by_id(&db, 1)
        .await
        .unwrap()
        .unwrap();
    let original_created_at = original_team.created_at.clone();
    let original_updated_at = original_team.updated_at.clone();

    // Add a small delay to ensure updated_at timestamp is different
    tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;

    let update_data = UpdateTeamEntity {
        name: Some("Updated Team Name".to_string()),
        country_id: 2,
        logo_path: Some("http://localhost:9000/hockey-uploads/updated-logo.jpg".to_string()),
    };

    let result = crate::team::service::update_team(&db, 1, update_data)
        .await
        .unwrap();
    assert!(result);

    // Verify the update
    let team = crate::team::service::get_team_by_id(&db, 1)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(team.name, Some("Updated Team Name".to_string()));
    assert_eq!(team.country_id, 2);

    // Verify audit columns: created_at should remain the same, updated_at should change
    assert_eq!(team.created_at, original_created_at);
    assert_ne!(team.updated_at, original_updated_at);
}

#[sqlx::test(fixtures("get_teams"))]
async fn update_team_not_found(db: sqlx::SqlitePool) {
    let update_data = UpdateTeamEntity {
        name: Some("Updated Team Name".to_string()),
        country_id: 2,
        logo_path: None,
    };

    let result = crate::team::service::update_team(&db, 999, update_data)
        .await
        .unwrap();
    assert!(!result);
}

#[sqlx::test(fixtures("get_teams"))]
async fn delete_team(db: sqlx::SqlitePool) {
    let result = crate::team::service::delete_team(&db, 1).await.unwrap();
    assert!(result);

    // Verify the team is deleted
    let team = crate::team::service::get_team_by_id(&db, 1).await.unwrap();
    assert!(team.is_none());

    // Try to delete again, should return false
    let result = crate::team::service::delete_team(&db, 1).await.unwrap();
    assert!(!result);
}

#[sqlx::test(fixtures("team_detail"))]
async fn get_team_detail_success(db: sqlx::SqlitePool) {
    let team_detail = crate::team::service::get_team_detail(&db, 1)
        .await
        .unwrap()
        .expect("Team detail should exist");

    // Verify team information
    assert_eq!(team_detail.team.id, 1);
    assert_eq!(team_detail.team.name, Some("Test Team A".to_string()));
    assert_eq!(team_detail.team.country_id, 34); // Canada
    assert_eq!(team_detail.team.country_name, "Canada");
    assert_eq!(team_detail.team.country_iso2_code, "CA");
    assert_eq!(
        team_detail.team.logo_path,
        Some("test-logo-a.jpg".to_string())
    );

    // Verify participations are returned (should be 2: 2023 and 2024 seasons)
    assert_eq!(team_detail.participations.len(), 2);

    // Verify participations are sorted by season name (descending)
    assert_eq!(team_detail.participations[0].season_name, "2024 Season");
    assert_eq!(team_detail.participations[1].season_name, "2023 Season");

    // Verify 2024 season participation
    let season_2024 = &team_detail.participations[0];
    assert_eq!(season_2024.season_id, 2);
    assert_eq!(season_2024.participation_id, 2);
    assert_eq!(season_2024.roster.len(), 2); // John Doe and Mike Johnson

    // Verify roster players in 2024
    let roster_2024 = &season_2024.roster;
    assert!(
        roster_2024
            .iter()
            .any(|p| p.player_name == "John Doe" && p.country_name == "Canada")
    );
    assert!(
        roster_2024
            .iter()
            .any(|p| p.player_name == "Mike Johnson" && p.country_name == "Finland")
    );

    // Verify 2023 season participation
    let season_2023 = &team_detail.participations[1];
    assert_eq!(season_2023.season_id, 1);
    assert_eq!(season_2023.participation_id, 1);
    assert_eq!(season_2023.roster.len(), 2); // John Doe and Jane Smith

    // Verify roster players in 2023
    let roster_2023 = &season_2023.roster;
    assert!(
        roster_2023
            .iter()
            .any(|p| p.player_name == "John Doe" && p.country_name == "Canada")
    );
    assert!(
        roster_2023
            .iter()
            .any(|p| p.player_name == "Jane Smith" && p.country_name == "Sweden")
    );
}

#[sqlx::test(fixtures("team_detail"))]
async fn get_team_detail_with_no_roster(db: sqlx::SqlitePool) {
    let team_detail = crate::team::service::get_team_detail(&db, 2)
        .await
        .unwrap()
        .expect("Team detail should exist");

    // Verify team information
    assert_eq!(team_detail.team.id, 2);
    assert_eq!(team_detail.team.name, Some("Test Team B".to_string()));

    // Verify participations (should be 1: 2023 season)
    assert_eq!(team_detail.participations.len(), 1);

    let participation = &team_detail.participations[0];
    assert_eq!(participation.season_id, 1);
    assert_eq!(participation.season_name, "2023 Season");
    assert_eq!(participation.roster.len(), 1); // Only Erik Karlsson

    let player = &participation.roster[0];
    assert_eq!(player.player_name, "Erik Karlsson");
    assert_eq!(player.country_name, "Sweden");
}

#[sqlx::test(fixtures("team_detail"))]
async fn get_team_detail_nonexistent_team(db: sqlx::SqlitePool) {
    let result = crate::team::service::get_team_detail(&db, 999)
        .await
        .unwrap();
    assert!(result.is_none());
}

#[sqlx::test]
async fn get_team_detail_empty_database(db: sqlx::SqlitePool) {
    let result = crate::team::service::get_team_detail(&db, 1).await.unwrap();
    assert!(result.is_none());
}
