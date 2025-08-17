use crate::team_participation::service::CreateTeamParticipationEntity;

#[sqlx::test(fixtures("events", "seasons", "teams"))]
async fn create_team_participation(db: sqlx::SqlitePool) {
    let team_participation = CreateTeamParticipationEntity {
        team_id: 1,
        season_id: 1,
    };
    let id = crate::team_participation::service::create_team_participation(&db, team_participation)
        .await
        .unwrap();
    assert!(id > 0);
}

#[sqlx::test(fixtures("events", "seasons", "teams", "get_team_participation"))]
async fn get_team_participation(db: sqlx::SqlitePool) {
    let team_participations = crate::team_participation::service::get_team_participation(&db)
        .await
        .unwrap();
    assert!(!team_participations.is_empty());
    assert_eq!(team_participations.len(), 2); // Assuming the fixture has 2 team participations
    for team_participation in team_participations {
        assert!(team_participation.id > 0);
        assert!(team_participation.season_id > 0);
        assert!(team_participation.team_id > 0);
    }
}

#[sqlx::test(fixtures("events", "seasons", "teams", "get_team_participation"))]
async fn get_team_participation_by_id(db: sqlx::SqlitePool) {
    let team_participation =
        crate::team_participation::service::get_team_participation_by_id(&db, 1)
            .await
            .unwrap();
    assert!(team_participation.is_some());
    let team_participation = team_participation.unwrap();
    assert_eq!(team_participation.id, 1);
    assert_eq!(team_participation.team_id, 1);
    assert_eq!(team_participation.season_id, 1);
}

#[sqlx::test(fixtures("events", "seasons", "teams", "get_team_participation"))]
async fn get_team_participation_by_id_empty(db: sqlx::SqlitePool) {
    let team_participation =
        crate::team_participation::service::get_team_participation_by_id(&db, 999)
            .await
            .unwrap();
    assert!(team_participation.is_none());
}

#[sqlx::test(fixtures("events", "seasons", "teams", "get_team_participation"))]
async fn delete_team_participation(db: sqlx::SqlitePool) {
    let result = crate::team_participation::service::delete_team_participation(&db, 1)
        .await
        .unwrap();
    assert!(result);

    // Verify the team participation is deleted
    let team_participation =
        crate::team_participation::service::get_team_participation_by_id(&db, 1)
            .await
            .unwrap();
    assert!(team_participation.is_none());
    // Try to delete again, should return false
    let result = crate::team_participation::service::delete_team_participation(&db, 1)
        .await
        .unwrap();
    assert!(!result);
}
