use axum::response::Html;

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

/// Helper function to reload the entire page
///
/// Returns an HTML div that triggers a full page reload.
/// This is commonly used after successful form submissions that should refresh the entire page.
///
/// # Example
/// ```rust
/// htmx_reload_page()
/// ```
pub fn htmx_reload_page() -> Html<String> {
    Html("<div hx-get=\"\" hx-target=\"body\" hx-trigger=\"load\" hx-swap=\"outerHTML\" hx-push-url=\"true\"></div>".to_string())
}
