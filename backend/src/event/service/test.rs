use crate::common::paging::Paging;
use crate::event::service::{CreateEventEntity, EventFilters, UpdateEventEntity};

#[sqlx::test]
async fn create_event(db: sqlx::SqlitePool) {
    let event = CreateEventEntity {
        name: "Test Event".to_string(),
        country_id: None, // Assuming the event is not associated with a country
    };
    let id = crate::event::service::create_event(&db, event)
        .await
        .unwrap();
    assert!(id > 0);

    // Verify audit columns are set
    let created_event = crate::event::service::get_event_by_id(&db, id)
        .await
        .unwrap()
        .unwrap();
    assert!(!created_event.created_at.is_empty());
    assert!(!created_event.updated_at.is_empty());
    assert_eq!(created_event.created_at, created_event.updated_at); // Should be equal on creation
}

#[sqlx::test(fixtures("get_events"))]
async fn get_events_no_filters(db: sqlx::SqlitePool) {
    let filters = EventFilters::default();
    let result = crate::event::service::get_events(&db, &filters, None)
        .await
        .unwrap();

    assert_eq!(result.items.len(), 2); // Fixture has 2 events
    assert_eq!(result.total, 2);
    assert_eq!(result.page, 1);
    assert_eq!(result.page_size, 20); // Default page size
    assert_eq!(result.total_pages, 1);
    assert!(!result.has_next);
    assert!(!result.has_previous);

    for event in &result.items {
        assert!(event.id > 0);
        assert!(!event.name.is_empty());
        assert!(!event.created_at.is_empty());
        assert!(!event.updated_at.is_empty());
    }
}

#[sqlx::test(fixtures("get_events"))]
async fn get_event_by_id_national(db: sqlx::SqlitePool) {
    let event = crate::event::service::get_event_by_id(&db, 1)
        .await
        .unwrap();
    assert!(event.is_some());
    let event = event.unwrap();
    assert_eq!(event.id, 1);
    assert_eq!(event.name, "Friendly");
    assert_eq!(event.country_id, None);
}

#[sqlx::test(fixtures("get_events"))]
async fn get_event_by_id_international(db: sqlx::SqlitePool) {
    let event = crate::event::service::get_event_by_id(&db, 2)
        .await
        .unwrap();
    assert!(event.is_some());
    let event = event.unwrap();
    assert_eq!(event.id, 2);
    assert_eq!(event.name, "Championship");
    assert_eq!(event.country_id, Some(1));
}

#[sqlx::test(fixtures("get_events"))]
async fn get_event_by_id_empty(db: sqlx::SqlitePool) {
    let event = crate::event::service::get_event_by_id(&db, 999)
        .await
        .unwrap();
    assert!(event.is_none());
}

#[sqlx::test(fixtures("get_events"))]
async fn get_events_with_name_filter(db: sqlx::SqlitePool) {
    let filters = EventFilters::new(Some("Friendly".to_string()), None);
    let result = crate::event::service::get_events(&db, &filters, None)
        .await
        .unwrap();

    assert_eq!(result.items.len(), 1);
    assert_eq!(result.total, 1);
    assert_eq!(result.items[0].name, "Friendly");
}

#[sqlx::test(fixtures("get_events"))]
async fn get_events_with_country_filter(db: sqlx::SqlitePool) {
    let filters = EventFilters::new(None, Some(1));
    let result = crate::event::service::get_events(&db, &filters, None)
        .await
        .unwrap();

    assert_eq!(result.items.len(), 1);
    assert_eq!(result.total, 1);
    assert_eq!(result.items[0].country_id, Some(1));
}

#[sqlx::test(fixtures("get_events"))]
async fn get_events_with_pagination(db: sqlx::SqlitePool) {
    let filters = EventFilters::default();
    let paging = Paging::new(1, 1); // Page 1, 1 item per page
    let result = crate::event::service::get_events(&db, &filters, Some(&paging))
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

#[sqlx::test(fixtures("get_events"))]
async fn get_events_second_page(db: sqlx::SqlitePool) {
    let filters = EventFilters::default();
    let paging = Paging::new(2, 1); // Page 2, 1 item per page
    let result = crate::event::service::get_events(&db, &filters, Some(&paging))
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

#[sqlx::test(fixtures("get_events"))]
async fn get_events_no_results_filter(db: sqlx::SqlitePool) {
    let filters = EventFilters::new(Some("Nonexistent Event".to_string()), None);
    let result = crate::event::service::get_events(&db, &filters, None)
        .await
        .unwrap();

    assert_eq!(result.items.len(), 0);
    assert_eq!(result.total, 0);
    assert_eq!(result.total_pages, 0);
    assert!(!result.has_next);
    assert!(!result.has_previous);
}

#[sqlx::test(fixtures("get_events"))]
async fn update_event_success(db: sqlx::SqlitePool) {
    // Get the original event to compare timestamps
    let original_event = crate::event::service::get_event_by_id(&db, 1)
        .await
        .unwrap()
        .unwrap();
    let original_created_at = original_event.created_at.clone();
    let original_updated_at = original_event.updated_at.clone();

    // Add a small delay to ensure updated_at timestamp is different
    tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;

    let update_data = UpdateEventEntity {
        name: "Updated Event Name".to_string(),
        country_id: Some(2),
    };

    let result = crate::event::service::update_event(&db, 1, update_data)
        .await
        .unwrap();
    assert!(result);

    // Verify the update
    let event = crate::event::service::get_event_by_id(&db, 1)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(event.name, "Updated Event Name");
    assert_eq!(event.country_id, Some(2));

    // Verify audit columns: created_at should remain the same, updated_at should change
    assert_eq!(event.created_at, original_created_at);
    assert_ne!(event.updated_at, original_updated_at);
}

#[sqlx::test(fixtures("get_events"))]
async fn update_event_not_found(db: sqlx::SqlitePool) {
    let update_data = UpdateEventEntity {
        name: "Updated Event Name".to_string(),
        country_id: Some(2),
    };

    let result = crate::event::service::update_event(&db, 999, update_data)
        .await
        .unwrap();
    assert!(!result);
}

#[sqlx::test(fixtures("get_events"))]
async fn delete_event(db: sqlx::SqlitePool) {
    let result = crate::event::service::delete_event(&db, 1).await.unwrap();
    assert!(result);

    // Verify the event is deleted
    let event = crate::event::service::get_event_by_id(&db, 1)
        .await
        .unwrap();
    assert!(event.is_none());

    // Try to delete again, should return false
    let result = crate::event::service::delete_event(&db, 1).await.unwrap();
    assert!(!result);
}
