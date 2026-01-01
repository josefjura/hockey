use axum::{
    extract::{Path, Query, State},
    response::{Html, IntoResponse},
    Extension, Form,
};
use serde::Deserialize;

use crate::app_state::AppState;
use crate::auth::Session;
use crate::i18n::TranslationContext;
use crate::service::teams::{
    self, CreateTeamEntity, SortField, SortOrder, TeamFilters, UpdateTeamEntity,
};
use crate::validation::validate_name;
use crate::views::{
    components::{error::error_message, htmx::htmx_reload_table},
    layout::admin_layout,
    pages::team_detail::team_detail_page,
    pages::teams::{team_create_modal, team_edit_modal, team_list_content, teams_page},
};

#[derive(Debug, Deserialize)]
pub struct TeamsQuery {
    #[serde(default = "default_page")]
    page: usize,
    #[serde(default = "default_page_size")]
    page_size: usize,
    #[serde(default, deserialize_with = "crate::utils::empty_string_as_none")]
    name: Option<String>,
    #[serde(default, deserialize_with = "crate::utils::empty_string_as_none_i64")]
    country_id: Option<i64>,
    #[serde(default = "default_sort")]
    sort: String,
    #[serde(default = "default_order")]
    order: String,
}

fn default_page() -> usize {
    1
}

fn default_page_size() -> usize {
    20
}

fn default_sort() -> String {
    "name".to_string()
}

fn default_order() -> String {
    "asc".to_string()
}

#[derive(Debug, Deserialize)]
pub struct CreateTeamForm {
    name: String,
    country_id: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTeamForm {
    name: String,
    country_id: Option<i64>,
}

/// GET /teams - Teams list page
pub async fn teams_get(
    Extension(session): Extension<Session>,
    Extension(t): Extension<TranslationContext>,
    State(state): State<AppState>,
    Query(query): Query<TeamsQuery>,
) -> impl IntoResponse {
    // Build filters
    let filters = TeamFilters {
        name: query.name.clone(),
        country_id: query.country_id,
    };

    // Parse sort parameters
    let sort_field = SortField::from_str(&query.sort);
    let sort_order = SortOrder::from_str(&query.order);

    // Get teams
    let result = match teams::get_teams(
        &state.db,
        &filters,
        &sort_field,
        &sort_order,
        query.page,
        query.page_size,
    )
    .await
    {
        Ok(result) => result,
        Err(e) => {
            tracing::error!("Failed to fetch teams: {}", e);
            return Html(
                admin_layout(
                    "Teams",
                    &session,
                    "/teams",
                    &t,
                    crate::views::components::error::error_message("Failed to load teams"),
                )
                .into_string(),
            );
        }
    };

    // Get countries for filter
    let countries = teams::get_countries(&state.db).await.unwrap_or_default();

    let content = teams_page(&t, &result, &filters, &sort_field, &sort_order, &countries);
    Html(admin_layout("Teams", &session, "/teams", &t, content).into_string())
}

/// GET /teams/list - HTMX endpoint for table updates
pub async fn teams_list_partial(
    Extension(t): Extension<TranslationContext>,
    State(state): State<AppState>,
    Query(query): Query<TeamsQuery>,
) -> impl IntoResponse {
    let filters = TeamFilters {
        name: query.name.clone(),
        country_id: query.country_id,
    };

    // Parse sort parameters
    let sort_field = SortField::from_str(&query.sort);
    let sort_order = SortOrder::from_str(&query.order);

    let result = match teams::get_teams(
        &state.db,
        &filters,
        &sort_field,
        &sort_order,
        query.page,
        query.page_size,
    )
    .await
    {
        Ok(result) => result,
        Err(e) => {
            tracing::error!("Failed to fetch teams: {}", e);
            return Html(
                crate::views::components::error::error_message("Failed to load teams")
                    .into_string(),
            );
        }
    };

    Html(team_list_content(&t, &result, &filters, &sort_field, &sort_order).into_string())
}

/// GET /teams/new - Show create modal
pub async fn team_create_form(Extension(t): Extension<TranslationContext>) -> impl IntoResponse {
    Html(team_create_modal(&t, None).into_string())
}

/// POST /teams - Create new team
pub async fn team_create(
    Extension(t): Extension<TranslationContext>,
    State(state): State<AppState>,
    Form(form): Form<CreateTeamForm>,
) -> impl IntoResponse {
    // Validation
    let name = match validate_name(&form.name) {
        Ok(n) => n,
        Err(error) => {
            return Html(team_create_modal(&t, Some(error)).into_string()).into_response()
        }
    };

    // Create team
    match teams::create_team(
        &state.db,
        CreateTeamEntity {
            name: name.to_string(),
            country_id: form.country_id,
        },
    )
    .await
    {
        Ok(_) => {
            use axum::http::header::{HeaderMap, HeaderName};

            // Return HTMX response to close modal and reload table
            // Trigger entity-created event for dashboard stats update
            let mut headers = HeaderMap::new();
            headers.insert(
                HeaderName::from_static("hx-trigger"),
                "entity-created".parse().unwrap(),
            );
            (headers, htmx_reload_table("/teams/list", "teams-table")).into_response()
        }
        Err(e) => {
            tracing::error!("Failed to create team: {}", e);
            Html(team_create_modal(&t, Some("Failed to create team")).into_string()).into_response()
        }
    }
}

/// GET /teams/{id}/edit - Show edit modal
pub async fn team_edit_form(
    Extension(t): Extension<TranslationContext>,
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    let team = match teams::get_team_by_id(&state.db, id).await {
        Ok(Some(team)) => team,
        Ok(None) => {
            return Html(
                crate::views::components::error::error_message("Team not found").into_string(),
            );
        }
        Err(e) => {
            tracing::error!("Failed to fetch team: {}", e);
            return Html(
                crate::views::components::error::error_message("Failed to load team").into_string(),
            );
        }
    };

    Html(team_edit_modal(&t, &team, None).into_string())
}

/// POST /teams/{id} - Update team
pub async fn team_update(
    Extension(t): Extension<TranslationContext>,
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Form(form): Form<UpdateTeamForm>,
) -> impl IntoResponse {
    // Validation
    let name = match validate_name(&form.name) {
        Ok(n) => n,
        Err(error) => {
            let team = teams::get_team_by_id(&state.db, id).await.ok().flatten();
            let Some(team) = team else {
                return Html(error_message("Team not found").into_string());
            };
            return Html(team_edit_modal(&t, &team, Some(error)).into_string());
        }
    };

    // Update team
    match teams::update_team(
        &state.db,
        id,
        UpdateTeamEntity {
            name: name.to_string(),
            country_id: form.country_id,
        },
    )
    .await
    {
        Ok(true) => {
            // Return HTMX response to close modal and reload table
            htmx_reload_table("/teams/list", "teams-table")
        }
        Ok(false) => Html(error_message("Team not found").into_string()),
        Err(e) => {
            tracing::error!("Failed to update team: {}", e);
            let team = teams::get_team_by_id(&state.db, id).await.ok().flatten();
            let Some(team) = team else {
                return Html(error_message("Team not found").into_string());
            };
            Html(team_edit_modal(&t, &team, Some("Failed to update team")).into_string())
        }
    }
}

/// POST /teams/{id}/delete - Delete team
pub async fn team_delete(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Query(query): Query<TeamsQuery>,
) -> impl IntoResponse {
    match teams::delete_team(&state.db, id).await {
        Ok(true) => {
            // Build URL to reload table with current filters and sorting
            let mut reload_url = format!(
                "/teams/list?page={}&page_size={}&sort={}&order={}",
                query.page, query.page_size, query.sort, query.order
            );

            if let Some(name) = &query.name {
                reload_url.push_str(&format!("&name={}", urlencoding::encode(name)));
            }

            if let Some(country_id) = query.country_id {
                reload_url.push_str(&format!("&country_id={}", country_id));
            }

            // Return HTMX response to reload table with filters
            Html(format!(
                "<div hx-get=\"{}\" hx-target=\"#teams-table\" hx-trigger=\"load\" hx-swap=\"outerHTML\"></div>",
                reload_url
            ))
        }
        Ok(false) => {
            Html(crate::views::components::error::error_message("Team not found").into_string())
        }
        Err(e) => {
            tracing::error!("Failed to delete team: {}", e);
            Html(
                crate::views::components::error::error_message("Failed to delete team")
                    .into_string(),
            )
        }
    }
}

/// GET /teams/{id} - Team detail page
pub async fn team_detail(
    Extension(session): Extension<Session>,
    Extension(t): Extension<TranslationContext>,
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    let detail = match teams::get_team_detail(&state.db, id).await {
        Ok(Some(detail)) => detail,
        Ok(None) => {
            return Html(
                admin_layout(
                    "Team Not Found",
                    &session,
                    "/teams",
                    &t,
                    crate::views::components::error::error_message("Team not found"),
                )
                .into_string(),
            );
        }
        Err(e) => {
            tracing::error!("Failed to fetch team detail: {}", e);
            return Html(
                admin_layout(
                    "Error",
                    &session,
                    "/teams",
                    &t,
                    crate::views::components::error::error_message("Failed to load team"),
                )
                .into_string(),
            );
        }
    };

    let content = team_detail_page(&t, &detail);
    Html(admin_layout("Team Detail", &session, "/teams", &t, content).into_string())
}

#[cfg(test)]
mod tests {
    use crate::test_utils::{create_test_app, create_test_session, session_cookie};
    use axum_test::TestServer;
    use sqlx::SqlitePool;

    #[sqlx::test(migrations = "./migrations", fixtures("users", "teams"))]
    async fn test_teams_get_full_page(pool: SqlitePool) {
        let app = create_test_app(pool.clone());
        let server = TestServer::new(app).unwrap();
        let session = create_test_session(&pool).await;

        let response = server
            .get("/teams")
            .add_cookie(session_cookie(&session))
            .await;

        response.assert_status_ok();
        let body = response.text();
        assert!(body.contains("<html"));
        assert!(body.contains("Teams"));
    }

    #[sqlx::test(migrations = "./migrations", fixtures("users", "teams"))]
    async fn test_teams_get_requires_auth(pool: SqlitePool) {
        let app = create_test_app(pool);
        let server = TestServer::new(app).unwrap();

        let response = server.get("/teams").await;

        // Should redirect to login
        response.assert_status(axum::http::StatusCode::SEE_OTHER);
    }

    #[sqlx::test(migrations = "./migrations", fixtures("users", "teams"))]
    async fn test_teams_list_partial(pool: SqlitePool) {
        let app = create_test_app(pool.clone());
        let server = TestServer::new(app).unwrap();
        let session = create_test_session(&pool).await;

        let response = server
            .get("/teams/list")
            .add_cookie(session_cookie(&session))
            .add_header("HX-Request", "true")
            .await;

        response.assert_status_ok();
        let body = response.text();
        // HTMX partials should NOT include full HTML layout
        assert!(!body.contains("<html"));
        assert!(!body.contains("<!DOCTYPE"));
        // Should contain table rows
        assert!(body.contains("Team Canada") || body.contains("<tr"));
    }

    #[sqlx::test(migrations = "./migrations", fixtures("users", "teams"))]
    async fn test_teams_list_partial_with_filters(pool: SqlitePool) {
        let app = create_test_app(pool.clone());
        let server = TestServer::new(app).unwrap();
        let session = create_test_session(&pool).await;

        let response = server
            .get("/teams/list?name=Canada")
            .add_cookie(session_cookie(&session))
            .add_header("HX-Request", "true")
            .await;

        response.assert_status_ok();
        let body = response.text();
        assert!(body.contains("Canada"));
    }

    #[sqlx::test(migrations = "./migrations", fixtures("users"))]
    async fn test_team_create_form(pool: SqlitePool) {
        let app = create_test_app(pool.clone());
        let server = TestServer::new(app).unwrap();
        let session = create_test_session(&pool).await;

        let response = server
            .get("/teams/new")
            .add_cookie(session_cookie(&session))
            .await;

        response.assert_status_ok();
        let body = response.text();
        assert!(body.contains("form"));
    }

    #[sqlx::test(migrations = "./migrations", fixtures("users"))]
    async fn test_team_create_success(pool: SqlitePool) {
        let app = create_test_app(pool.clone());
        let server = TestServer::new(app).unwrap();
        let session = create_test_session(&pool).await;

        let response = server
            .post("/teams")
            .add_cookie(session_cookie(&session))
            .form(&[("name", "New Team"), ("country_id", "1")])
            .await;

        response.assert_status_ok();
        let body = response.text();
        // Should return HTMX response to reload table
        assert!(body.contains("HX-Trigger") || body.contains("hx-trigger"));

        // Verify team was created
        let count =
            sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM team WHERE name = 'New Team'")
                .fetch_one(&pool)
                .await
                .unwrap();
        assert_eq!(count, 1);
    }

    #[sqlx::test(migrations = "./migrations", fixtures("users"))]
    async fn test_team_create_validation_fails(pool: SqlitePool) {
        let app = create_test_app(pool.clone());
        let server = TestServer::new(app).unwrap();
        let session = create_test_session(&pool).await;

        let response = server
            .post("/teams")
            .add_cookie(session_cookie(&session))
            .form(&[("name", ""), ("country_id", "1")]) // Empty name should fail
            .await;

        response.assert_status_ok();
        let body = response.text();
        // Should return error message
        assert!(body.contains("error") || body.contains("required"));

        // Verify no team was created
        let count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM team")
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(count, 0);
    }

    #[sqlx::test(migrations = "./migrations", fixtures("users", "teams"))]
    async fn test_team_edit_form(pool: SqlitePool) {
        let app = create_test_app(pool.clone());
        let server = TestServer::new(app).unwrap();
        let session = create_test_session(&pool).await;

        let response = server
            .get("/teams/1/edit")
            .add_cookie(session_cookie(&session))
            .await;

        response.assert_status_ok();
        let body = response.text();
        assert!(body.contains("form"));
        assert!(body.contains("Team Canada"));
    }

    #[sqlx::test(migrations = "./migrations", fixtures("users", "teams"))]
    async fn test_team_update_success(pool: SqlitePool) {
        let app = create_test_app(pool.clone());
        let server = TestServer::new(app).unwrap();
        let session = create_test_session(&pool).await;

        let response = server
            .post("/teams/1")
            .add_cookie(session_cookie(&session))
            .form(&[("name", "Updated Team Canada"), ("country_id", "1")])
            .await;

        response.assert_status_ok();

        // Verify team was updated
        let name = sqlx::query_scalar::<_, String>("SELECT name FROM team WHERE id = 1")
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(name, "Updated Team Canada");
    }

    #[sqlx::test(migrations = "./migrations", fixtures("users", "teams"))]
    async fn test_team_delete_success(pool: SqlitePool) {
        let app = create_test_app(pool.clone());
        let server = TestServer::new(app).unwrap();
        let session = create_test_session(&pool).await;

        let response = server
            .post("/teams/1/delete")
            .add_cookie(session_cookie(&session))
            .await;

        response.assert_status_ok();

        // Verify team was deleted
        let count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM team WHERE id = 1")
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(count, 0);
    }

    #[sqlx::test(migrations = "./migrations", fixtures("users", "teams"))]
    async fn test_team_detail_page(pool: SqlitePool) {
        let app = create_test_app(pool.clone());
        let server = TestServer::new(app).unwrap();
        let session = create_test_session(&pool).await;

        let response = server
            .get("/teams/1")
            .add_cookie(session_cookie(&session))
            .await;

        response.assert_status_ok();
        let body = response.text();
        assert!(body.contains("Team Canada"));
        assert!(body.contains("<html"));
    }

    #[sqlx::test(migrations = "./migrations", fixtures("users", "teams"))]
    async fn test_team_detail_not_found(pool: SqlitePool) {
        let app = create_test_app(pool.clone());
        let server = TestServer::new(app).unwrap();
        let session = create_test_session(&pool).await;

        let response = server
            .get("/teams/999")
            .add_cookie(session_cookie(&session))
            .await;

        response.assert_status_ok();
        let body = response.text();
        assert!(body.contains("not found") || body.contains("Not found"));
    }
}
