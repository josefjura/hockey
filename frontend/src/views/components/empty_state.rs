use maud::{html, Markup, PreEscaped};

/// Empty state icons for different contexts
pub enum EmptyStateIcon {
    /// Generic empty box
    Box,
    /// Search/filter with no results
    Search,
    /// Document/file list
    Document,
    /// User/person list
    Users,
    /// Calendar/events
    Calendar,
    /// Settings/config
    Settings,
    /// Custom SVG icon
    Custom(&'static str),
}

impl EmptyStateIcon {
    fn svg(&self) -> &'static str {
        match self {
            EmptyStateIcon::Box => {
                r#"<svg width="64" height="64" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
                    <path d="M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z"/>
                    <polyline points="3.27 6.96 12 12.01 20.73 6.96"/>
                    <line x1="12" y1="22.08" x2="12" y2="12"/>
                </svg>"#
            }
            EmptyStateIcon::Search => {
                r#"<svg width="64" height="64" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
                    <circle cx="11" cy="11" r="8"/>
                    <line x1="21" y1="21" x2="16.65" y2="16.65"/>
                    <line x1="8" y1="11" x2="14" y2="11"/>
                </svg>"#
            }
            EmptyStateIcon::Document => {
                r#"<svg width="64" height="64" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
                    <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/>
                    <polyline points="14 2 14 8 20 8"/>
                    <line x1="12" y1="18" x2="12" y2="12"/>
                    <line x1="9" y1="15" x2="15" y2="15"/>
                </svg>"#
            }
            EmptyStateIcon::Users => {
                r#"<svg width="64" height="64" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
                    <path d="M17 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2"/>
                    <circle cx="9" cy="7" r="4"/>
                    <path d="M23 21v-2a4 4 0 0 0-3-3.87"/>
                    <path d="M16 3.13a4 4 0 0 1 0 7.75"/>
                </svg>"#
            }
            EmptyStateIcon::Calendar => {
                r#"<svg width="64" height="64" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
                    <rect x="3" y="4" width="18" height="18" rx="2" ry="2"/>
                    <line x1="16" y1="2" x2="16" y2="6"/>
                    <line x1="8" y1="2" x2="8" y2="6"/>
                    <line x1="3" y1="10" x2="21" y2="10"/>
                </svg>"#
            }
            EmptyStateIcon::Settings => {
                r#"<svg width="64" height="64" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
                    <circle cx="12" cy="12" r="3"/>
                    <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"/>
                </svg>"#
            }
            EmptyStateIcon::Custom(svg) => svg,
        }
    }
}

/// Enhanced empty state with icon, title, description, and optional action button
///
/// # Arguments
/// - `icon`: Icon to display
/// - `title`: Title text
/// - `description`: Description text
/// - `action_label`: Optional button label
/// - `action_url`: Optional URL for the action (HTMX GET)
/// - `action_target`: Optional target for HTMX
pub fn empty_state_enhanced(
    icon: EmptyStateIcon,
    title: &str,
    description: &str,
    action_label: Option<&str>,
    action_url: Option<&str>,
    action_target: Option<&str>,
) -> Markup {
    html! {
        div
            class="empty-state"
            style="
                display: flex;
                flex-direction: column;
                align-items: center;
                justify-content: center;
                padding: 3rem 2rem;
                text-align: center;
                min-height: 300px;
            "
        {
            // Icon
            div style="color: var(--gray-300); margin-bottom: 1.5rem;" {
                (PreEscaped(icon.svg()))
            }

            // Title
            h3 style="
                font-size: 1.25rem;
                font-weight: 600;
                color: var(--gray-700);
                margin-bottom: 0.5rem;
            " {
                (title)
            }

            // Description
            p style="
                color: var(--gray-500);
                max-width: 360px;
                line-height: 1.5;
                margin-bottom: 1.5rem;
            " {
                (description)
            }

            // Action button
            @if let (Some(label), Some(url)) = (action_label, action_url) {
                button
                    class="btn btn-primary"
                    hx-get=(url)
                    hx-target=(action_target.unwrap_or("#modal-container"))
                    hx-swap="innerHTML"
                {
                    (label)
                }
            }
        }
    }
}

/// Simple empty state for tables and lists with search/filter context
///
/// # Arguments
/// - `entity_name`: Name of the entity (e.g., "events", "teams")
/// - `has_filters`: Whether filters are currently applied
/// - `create_url`: Optional URL for creating new item
/// - `create_label`: Optional label for create button
pub fn empty_state_table(
    entity_name: &str,
    has_filters: bool,
    create_url: Option<&str>,
    create_label: Option<&str>,
) -> Markup {
    let icon = if has_filters {
        EmptyStateIcon::Search
    } else {
        EmptyStateIcon::Box
    };

    let title = if has_filters {
        format!("No {} match your search", entity_name)
    } else {
        format!("No {} yet", entity_name)
    };

    let description = if has_filters {
        "Try adjusting your filters or search criteria to find what you're looking for.".to_string()
    } else {
        format!(
            "Get started by creating your first {}.",
            entity_name.trim_end_matches('s')
        )
    };

    let (action_label, action_url) = if !has_filters {
        (create_label, create_url)
    } else {
        (None, None)
    };

    empty_state_enhanced(
        icon,
        &title,
        &description,
        action_label,
        action_url,
        Some("#modal-container"),
    )
}

/// Compact empty state for small areas
pub fn empty_state_compact(message: &str) -> Markup {
    html! {
        div
            class="empty-state-compact"
            style="
                padding: 1.5rem;
                text-align: center;
                color: var(--gray-500);
                font-size: 0.875rem;
            "
        {
            // Small icon
            div style="color: var(--gray-300); margin-bottom: 0.5rem;" {
                (PreEscaped(r#"<svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
                    <path d="M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z"/>
                    <polyline points="3.27 6.96 12 12.01 20.73 6.96"/>
                    <line x1="12" y1="22.08" x2="12" y2="12"/>
                </svg>"#))
            }
            (message)
        }
    }
}
