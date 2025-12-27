use axum::{
    extract::{Path, State},
    response::{Html, IntoResponse},
    Extension,
};

use crate::app_state::AppState;
use crate::auth::Session;
use crate::i18n::TranslationContext;
use crate::service::matches;
use crate::views::{layout::admin_layout, pages::matches::match_detail_page};

/// GET /matches/{id} - Match detail page
pub async fn match_detail(
    Extension(session): Extension<Session>,
    Extension(t): Extension<TranslationContext>,
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    // Get match detail
    let match_detail = match matches::get_match_detail(&state.db, id).await {
        Ok(Some(detail)) => detail,
        Ok(None) => {
            return Html(
                admin_layout(
                    "Match Not Found",
                    &session,
                    "/matches",
                    &t,
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
                    &t,
                    crate::views::components::error::error_message("Failed to load match detail"),
                )
                .into_string(),
            );
        }
    };

    let content = match_detail_page(&t, &match_detail);
    Html(admin_layout("Match Detail", &session, "/matches", &t, content).into_string())
}
