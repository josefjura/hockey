use axum::{
    extract::{Path, State},
    response::{Html, IntoResponse},
    Extension,
};

use crate::app_state::AppState;
use crate::auth::Session;
use crate::i18n::Locale;
use crate::service::matches;
use crate::views::{layout::admin_layout, pages::matches::match_detail_page};

/// GET /matches/{id} - Match detail page
pub async fn match_detail_get(
    Extension(session): Extension<Session>,
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    let locale = Locale::English;

    // Get match detail
    let match_detail = match matches::get_match_detail(&state.db, id).await {
        Ok(Some(detail)) => detail,
        Ok(None) => {
            return Html(
                admin_layout(
                    "Match Not Found",
                    &session,
                    "/matches",
                    locale,
                    crate::views::components::error::error_message("Match not found"),
                )
                .into_string(),
            );
        }
        Err(e) => {
            tracing::error!("Failed to fetch match detail: {}", e);
            return Html(
                admin_layout(
                    "Error",
                    &session,
                    "/matches",
                    locale,
                    crate::views::components::error::error_message("Failed to load match detail"),
                )
                .into_string(),
            );
        }
    };

    let content = match_detail_page(&match_detail);
    Html(admin_layout("Match Detail", &session, "/matches", locale, content).into_string())
}

/// POST /matches/{id}/delete - Delete match
pub async fn match_delete_post(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    match matches::delete_match(&state.db, id).await {
        Ok(true) => {
            // Redirect to matches list using HTMX redirect header
            Html(r#"<div hx-redirect="/matches"></div>"#.to_string())
        }
        Ok(false) => {
            Html(crate::views::components::error::error_message("Match not found").into_string())
        }
        Err(e) => {
            tracing::error!("Failed to delete match: {}", e);
            Html(
                crate::views::components::error::error_message("Failed to delete match")
                    .into_string(),
            )
        }
    }
}
