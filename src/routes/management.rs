use crate::auth::session::Session;
use crate::i18n::TranslationContext;
use crate::views::{layout::admin_layout, pages::management::management_page};
use axum::{response::Html, Extension};

/// GET /management - Management hub page
pub async fn management_get(
    Extension(session): Extension<Session>,
    Extension(t): Extension<TranslationContext>,
) -> Html<String> {
    let content = management_page(&t);
    Html(admin_layout("Management", &session, "/management", &t, content).into_string())
}
