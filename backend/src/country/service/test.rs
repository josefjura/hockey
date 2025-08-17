use crate::common::paging::Paging;
use crate::country::service::CountryFilters;

#[sqlx::test]
async fn get_countries(db: sqlx::SqlitePool) {
    let result = crate::country::service::get_countries(&db, &CountryFilters::default(), None)
        .await
        .unwrap();

    assert!(!result.items.is_empty());
    assert!(result.items.len() > 0);
    assert!(result.total > 0);

    for country in result.items {
        assert!(country.id > 0);
        assert!(!country.name.is_empty());
    }
}

#[sqlx::test]
async fn get_countries_with_paging(db: sqlx::SqlitePool) {
    let paging = Paging::new(1, 5);
    let result =
        crate::country::service::get_countries(&db, &CountryFilters::default(), Some(&paging))
            .await
            .unwrap();

    assert!(result.items.len() <= 5);
    assert_eq!(result.page, 1);
    assert_eq!(result.page_size, 5);
    assert!(result.total > 0);
}

#[sqlx::test]
async fn get_enabled_countries(db: sqlx::SqlitePool) {
    // First, check how many countries we have in total
    let all_result = crate::country::service::get_countries(&db, &CountryFilters::default(), None)
        .await
        .unwrap();
    println!("Total countries: {}", all_result.total);

    // First, enable some countries for testing
    let _ = crate::country::service::update_country_status(&db, 1, true).await;
    let _ = crate::country::service::update_country_status(&db, 2, true).await;

    let result = crate::country::service::get_countries(
        &db,
        &CountryFilters {
            enabled: Some(true),
            ..Default::default()
        },
        None,
    )
    .await
    .unwrap();

    println!("Enabled countries: {}", result.items.len());

    // Should have at least the 2 we enabled
    assert!(result.items.len() >= 2);

    for country in result.items {
        assert!(country.id > 0);
        assert!(!country.name.is_empty());
        assert!(country.enabled);
    }
}

#[sqlx::test]
async fn get_countries_with_filters(db: sqlx::SqlitePool) {
    // First, check how many countries we have in total
    let all_result = crate::country::service::get_countries(&db, &CountryFilters::default(), None)
        .await
        .unwrap();
    println!("Total countries: {}", all_result.total);

    // Test filtering by name
    let result = crate::country::service::get_countries(
        &db,
        &CountryFilters {
            name: Some("Afghan".to_string()),
            ..Default::default()
        },
        None,
    )
    .await
    .unwrap();

    println!("Countries with 'Afghan': {}", result.items.len());
    for country in &result.items {
        println!("Country: {}", country.name);
        assert!(country.name.contains("Afghan"));
    }

    // Test filtering by IIHF
    let result = crate::country::service::get_countries(
        &db,
        &CountryFilters {
            iihf: Some(true),
            ..Default::default()
        },
        None,
    )
    .await
    .unwrap();

    println!("IIHF countries: {}", result.items.len());
    for country in &result.items {
        assert!(country.iihf);
    }

    // Test filtering by disabled countries
    let result = crate::country::service::get_countries(
        &db,
        &CountryFilters {
            enabled: Some(false),
            ..Default::default()
        },
        None,
    )
    .await
    .unwrap();

    println!("Disabled countries: {}", result.items.len());
    // Should have many disabled countries (since most are disabled by default)
    assert!(result.items.len() > 10); // Lower threshold for test
    for country in &result.items {
        assert!(!country.enabled);
    }
}

#[sqlx::test]
async fn get_country_by_id(db: sqlx::SqlitePool) {
    let country = crate::country::service::get_country_by_id(&db, 1).await;
    assert!(country.is_ok());
    let country = country.unwrap();
    assert_eq!(country.id, 1);
    assert_eq!(country.name, "Afghanistan");
    assert!(!country.iihf);
    assert!(!country.is_historical);
    assert_eq!(country.iso2_code, "AF");
    assert_eq!(country.ioc_code, "AFG");
    assert!(!country.enabled);
}

#[sqlx::test]
async fn get_country_by_id_empty(db: sqlx::SqlitePool) {
    let country = crate::country::service::get_country_by_id(&db, 999).await;
    assert!(country.is_err());
}

#[sqlx::test]
async fn update_country_status(db: sqlx::SqlitePool) {
    let result = crate::country::service::update_country_status(&db, 1, true).await;
    assert!(result.is_ok());

    // Verify the country status is updated
    let country = crate::country::service::get_country_by_id(&db, 1)
        .await
        .unwrap();
    assert!(country.enabled);

    let result = crate::country::service::update_country_status(&db, 1, false).await;
    assert!(result.is_ok());

    // Verify the country status is updated back to false
    let country = crate::country::service::get_country_by_id(&db, 1)
        .await
        .unwrap();
    assert!(!country.enabled);
}

#[sqlx::test]
async fn update_country_status_empty(db: sqlx::SqlitePool) {
    let result = crate::country::service::update_country_status(&db, 999, true).await;
    assert!(result.is_err()); // Should return an error for non-existing country
}
