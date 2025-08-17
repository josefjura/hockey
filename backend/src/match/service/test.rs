use crate::common::paging::Paging;
use crate::r#match::service::{
    CreateMatchEntity, CreateScoreEventEntity, MatchFilters, UpdateMatchEntity,
};

#[sqlx::test(fixtures("events", "seasons", "teams", "get_teams"))]
async fn create_match(db: sqlx::SqlitePool) {
    let match_data = CreateMatchEntity {
        season_id: 1,
        home_team_id: 1,
        away_team_id: 2,
        home_score_unidentified: 3,
        away_score_unidentified: 2,
        match_date: Some("2024-03-15T19:00:00Z".to_string()),
        status: Some("finished".to_string()),
        venue: Some("Hockey Arena".to_string()),
    };
    let id = crate::r#match::service::create_match(&db, match_data)
        .await
        .unwrap();
    assert!(id > 0);
}

#[sqlx::test(fixtures("events", "seasons", "teams", "get_teams", "matches"))]
async fn get_matches_no_filters(db: sqlx::SqlitePool) {
    let filters = MatchFilters::default();
    let result = crate::r#match::service::get_matches(&db, &filters, None)
        .await
        .unwrap();

    assert_eq!(result.items.len(), 2); // Fixture has 2 matches
    assert_eq!(result.total, 2);
    assert_eq!(result.page, 1);
    assert_eq!(result.page_size, 20); // Default page size
    assert_eq!(result.total_pages, 1);
    assert!(!result.has_next);
    assert!(!result.has_previous);

    for match_item in &result.items {
        assert!(match_item.id > 0);
        assert!(match_item.season_id > 0);
        assert!(match_item.home_team_id > 0);
        assert!(match_item.away_team_id > 0);
    }
}

#[sqlx::test(fixtures("events", "seasons", "teams", "get_teams", "matches"))]
async fn get_match_by_id(db: sqlx::SqlitePool) {
    let match_item = crate::r#match::service::get_match_by_id(&db, 1)
        .await
        .unwrap();
    assert!(match_item.is_some());
    let match_item = match_item.unwrap();
    assert_eq!(match_item.id, 1);
    assert_eq!(match_item.season_id, 1);
    assert_eq!(match_item.home_team_id, 1);
    assert_eq!(match_item.away_team_id, 2);
}

#[sqlx::test(fixtures("events", "seasons", "teams", "get_teams", "matches"))]
async fn get_match_by_id_empty(db: sqlx::SqlitePool) {
    let match_item = crate::r#match::service::get_match_by_id(&db, 999)
        .await
        .unwrap();
    assert!(match_item.is_none());
}

#[sqlx::test(fixtures("events", "seasons", "teams", "get_teams", "matches"))]
async fn get_matches_with_season_filter(db: sqlx::SqlitePool) {
    let filters = MatchFilters::new(Some(1), None, None, None, None);
    let result = crate::r#match::service::get_matches(&db, &filters, None)
        .await
        .unwrap();

    assert_eq!(result.items.len(), 2);
    for match_item in &result.items {
        assert_eq!(match_item.season_id, 1);
    }
}

#[sqlx::test(fixtures("events", "seasons", "teams", "get_teams", "matches"))]
async fn get_matches_with_team_filter(db: sqlx::SqlitePool) {
    let filters = MatchFilters::new(None, Some(1), None, None, None);
    let result = crate::r#match::service::get_matches(&db, &filters, None)
        .await
        .unwrap();

    assert_eq!(result.items.len(), 2); // Team 1 plays in both matches
    for match_item in &result.items {
        assert!(match_item.home_team_id == 1 || match_item.away_team_id == 1);
    }
}

#[sqlx::test(fixtures("events", "seasons", "teams", "get_teams", "matches"))]
async fn get_matches_with_status_filter(db: sqlx::SqlitePool) {
    let filters = MatchFilters::new(None, None, Some("finished".to_string()), None, None);
    let result = crate::r#match::service::get_matches(&db, &filters, None)
        .await
        .unwrap();

    assert_eq!(result.items.len(), 1);
    assert_eq!(result.items[0].status, "finished");
}

#[sqlx::test(fixtures("events", "seasons", "teams", "get_teams", "matches"))]
async fn get_matches_with_pagination(db: sqlx::SqlitePool) {
    let filters = MatchFilters::default();
    let paging = Paging::new(1, 1);
    let result = crate::r#match::service::get_matches(&db, &filters, Some(&paging))
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

#[sqlx::test(fixtures("events", "seasons", "teams", "get_teams", "matches"))]
async fn update_match_success(db: sqlx::SqlitePool) {
    let update_data = UpdateMatchEntity {
        season_id: None,
        home_team_id: None,
        away_team_id: None,
        home_score_unidentified: Some(5),
        away_score_unidentified: Some(3),
        match_date: Some("2024-03-16T19:00:00Z".to_string()),
        status: Some("finished".to_string()),
        venue: Some("Updated Arena".to_string()),
    };

    let result = crate::r#match::service::update_match(&db, 1, update_data)
        .await
        .unwrap();
    assert!(result);

    // Verify the update
    let match_item = crate::r#match::service::get_match_by_id(&db, 1)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(match_item.home_score_unidentified, 5);
    assert_eq!(match_item.away_score_unidentified, 3);
    assert_eq!(
        match_item.match_date,
        Some("2024-03-16T19:00:00Z".to_string())
    );
    assert_eq!(match_item.status, "finished");
    assert_eq!(match_item.venue, Some("Updated Arena".to_string()));
}

#[sqlx::test(fixtures("events", "seasons", "teams", "get_teams", "matches"))]
async fn update_match_not_found(db: sqlx::SqlitePool) {
    let update_data = UpdateMatchEntity {
        season_id: None,
        home_team_id: None,
        away_team_id: None,
        home_score_unidentified: Some(5),
        away_score_unidentified: Some(3),
        match_date: None,
        status: None,
        venue: None,
    };

    let result = crate::r#match::service::update_match(&db, 999, update_data)
        .await
        .unwrap();
    assert!(!result);
}

#[sqlx::test(fixtures("events", "seasons", "teams", "get_teams", "matches"))]
async fn delete_match(db: sqlx::SqlitePool) {
    let result = crate::r#match::service::delete_match(&db, 1).await.unwrap();
    assert!(result);

    // Verify the match is deleted
    let match_item = crate::r#match::service::get_match_by_id(&db, 1)
        .await
        .unwrap();
    assert!(match_item.is_none());

    // Try to delete again, should return false
    let result = crate::r#match::service::delete_match(&db, 1).await.unwrap();
    assert!(!result);
}

#[sqlx::test(fixtures("events", "seasons", "teams", "get_teams", "players", "matches"))]
async fn create_score_event(db: sqlx::SqlitePool) {
    let event = CreateScoreEventEntity {
        match_id: 1,
        team_id: 1,
        scorer_id: Some(1),
        assist1_id: Some(2),
        assist2_id: None,
        period: Some(1),
        time_minutes: Some(15),
        time_seconds: Some(30),
        goal_type: Some("even_strength".to_string()),
    };
    let id = crate::r#match::service::create_score_event(&db, event)
        .await
        .unwrap();
    assert!(id > 0);
}

#[sqlx::test(fixtures(
    "events",
    "seasons",
    "teams",
    "get_teams",
    "players",
    "matches",
    "score_events"
))]
async fn get_score_events_for_match(db: sqlx::SqlitePool) {
    let events = crate::r#match::service::get_score_events_for_match(&db, 1)
        .await
        .unwrap();
    assert!(!events.is_empty());
    assert_eq!(events.len(), 2); // Fixture has 2 score events for match 1

    for event in &events {
        assert_eq!(event.match_id, 1);
        assert!(event.id > 0);
        assert!(event.team_id > 0);
    }
}

#[sqlx::test(fixtures(
    "events",
    "seasons",
    "teams",
    "get_teams",
    "players",
    "matches",
    "score_events"
))]
async fn get_score_event_by_id(db: sqlx::SqlitePool) {
    let event = crate::r#match::service::get_score_event_by_id(&db, 1)
        .await
        .unwrap();
    assert!(event.is_some());
    let event = event.unwrap();
    assert_eq!(event.id, 1);
    assert_eq!(event.match_id, 1);
}

#[sqlx::test(fixtures(
    "events",
    "seasons",
    "teams",
    "get_teams",
    "players",
    "matches",
    "score_events"
))]
async fn delete_score_event(db: sqlx::SqlitePool) {
    let result = crate::r#match::service::delete_score_event(&db, 1)
        .await
        .unwrap();
    assert!(result);

    // Verify the event is deleted
    let event = crate::r#match::service::get_score_event_by_id(&db, 1)
        .await
        .unwrap();
    assert!(event.is_none());

    // Try to delete again, should return false
    let result = crate::r#match::service::delete_score_event(&db, 1)
        .await
        .unwrap();
    assert!(!result);
}

#[sqlx::test(fixtures(
    "events",
    "seasons",
    "teams",
    "get_teams",
    "players",
    "matches",
    "score_events"
))]
async fn get_match_with_stats(db: sqlx::SqlitePool) {
    let result = crate::r#match::service::get_match_with_stats(&db, 1)
        .await
        .unwrap();
    assert!(result.is_some());
    let (match_data, home_total, away_total, home_detailed, away_detailed) = result.unwrap();

    assert_eq!(match_data.id, 1);
    // Based on fixture: match 1 has 2 unidentified goals for home, 1 for away, plus 2 detailed events
    assert_eq!(home_total, 3); // 2 unidentified + 1 detailed
    assert_eq!(away_total, 2); // 1 unidentified + 1 detailed
    assert_eq!(home_detailed, 1); // 1 detailed event for home team
    assert_eq!(away_detailed, 1); // 1 detailed event for away team
}

#[sqlx::test(fixtures("events", "seasons", "teams", "get_teams", "players", "matches"))]
async fn identify_goal_success(db: sqlx::SqlitePool) {
    let event = CreateScoreEventEntity {
        match_id: 1,
        team_id: 1, // Home team
        scorer_id: Some(1),
        assist1_id: None,
        assist2_id: None,
        period: Some(2),
        time_minutes: Some(10),
        time_seconds: Some(15),
        goal_type: Some("power_play".to_string()),
    };

    // Get initial unidentified count
    let initial_match = crate::r#match::service::get_match_by_id(&db, 1)
        .await
        .unwrap()
        .unwrap();
    let initial_home_unidentified = initial_match.home_score_unidentified;

    let id = crate::r#match::service::identify_goal(&db, 1, 1, event)
        .await
        .unwrap();
    assert!(id > 0);

    // Verify unidentified count decreased
    let updated_match = crate::r#match::service::get_match_by_id(&db, 1)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(
        updated_match.home_score_unidentified,
        initial_home_unidentified - 1
    );

    // Verify the score event was created
    let created_event = crate::r#match::service::get_score_event_by_id(&db, id)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(created_event.match_id, 1);
    assert_eq!(created_event.team_id, 1);
    assert_eq!(created_event.period, Some(2));
}

#[sqlx::test(fixtures("events", "seasons", "teams", "get_teams", "players", "matches"))]
async fn identify_goal_no_unidentified_goals(db: sqlx::SqlitePool) {
    // First, set unidentified goals to 0
    let update_data = UpdateMatchEntity {
        season_id: None,
        home_team_id: None,
        away_team_id: None,
        home_score_unidentified: Some(0),
        away_score_unidentified: Some(0),
        match_date: None,
        status: None,
        venue: None,
    };
    crate::r#match::service::update_match(&db, 1, update_data)
        .await
        .unwrap();

    let event = CreateScoreEventEntity {
        match_id: 1,
        team_id: 1,
        scorer_id: Some(1),
        assist1_id: None,
        assist2_id: None,
        period: Some(2),
        time_minutes: Some(10),
        time_seconds: Some(15),
        goal_type: Some("power_play".to_string()),
    };

    // This should fail because there are no unidentified goals
    let result = crate::r#match::service::identify_goal(&db, 1, 1, event).await;
    assert!(result.is_err());
}
