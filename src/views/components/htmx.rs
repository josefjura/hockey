use axum::response::Html;
use maud::html;

use crate::i18n::TranslationContext;
use crate::service::dashboard::DashboardStats;
use crate::views::pages::dashboard::dashboard_stats_partial;

/// Helper function to generate HTMX reload trigger for tables
///
/// Returns an HTML div that triggers an HTMX reload of a target element.
/// This is commonly used after successful form submissions to refresh table data.
///
/// # Arguments
/// * `endpoint` - The URL endpoint to fetch (e.g., "/teams/list")
/// * `target_id` - The DOM element ID to replace (without #, e.g., "teams-table")
///
/// # Example
/// ```rust
/// htmx_reload_table("/teams/list", "teams-table")
/// ```
pub fn htmx_reload_table(endpoint: &str, target_id: &str) -> Html<String> {
    Html(format!(
        "<div hx-get=\"{}\" hx-target=\"#{}\" hx-trigger=\"load\" hx-swap=\"outerHTML\"></div>",
        endpoint, target_id
    ))
}

/// Helper function to generate HTMX reload trigger for tables with dashboard stats update
///
/// Returns an HTML fragment that triggers both:
/// 1. An HTMX reload of a target table element
/// 2. An out-of-band update of dashboard statistics
///
/// This is used after successful create operations from dashboard quick actions
/// to update both the relevant table and the dashboard stats counts.
///
/// # Arguments
/// * `endpoint` - The URL endpoint to fetch (e.g., "/teams/list")
/// * `target_id` - The DOM element ID to replace (without #, e.g., "teams-table")
/// * `t` - Translation context for rendering localized stats
/// * `stats` - Updated dashboard statistics to display
///
/// # Example
/// ```rust
/// let stats = get_dashboard_stats(&db).await.unwrap_or_default();
/// htmx_reload_table_with_stats("/teams/list", "teams-table", &t, &stats)
/// ```
pub fn htmx_reload_table_with_stats(
    endpoint: &str,
    target_id: &str,
    t: &TranslationContext,
    stats: &DashboardStats,
) -> Html<String> {
    Html(
        html! {
            div hx-get=(endpoint) hx-target={"#" (target_id)} hx-trigger="load" hx-swap="outerHTML" {}
            (dashboard_stats_partial(t, stats))
        }
        .into_string(),
    )
}
