use maud::{html, Markup};

use crate::views::components::empty_state::{empty_state_enhanced, EmptyStateIcon};

/// Empty state when no items are found (enhanced with icon)
///
/// When `has_filters` is false and both `create_url` and `create_label` are provided,
/// a "Create" call-to-action button is shown so users can immediately add the first item.
pub fn empty_state(
    entity_name: &str,
    has_filters: bool,
    create_url: Option<&str>,
    create_label: Option<&str>,
) -> Markup {
    let (icon, description) = if has_filters {
        (
            EmptyStateIcon::Search,
            format!(
                "No {} match your search criteria. Try adjusting your filters.",
                entity_name.to_lowercase()
            ),
        )
    } else {
        (
            EmptyStateIcon::Box,
            format!("No {} have been added yet.", entity_name.to_lowercase()),
        )
    };

    // Only show the CTA when there are no active filters
    let (action_label, action_url) = if !has_filters {
        (create_label, create_url)
    } else {
        (None, None)
    };

    empty_state_enhanced(
        icon,
        &format!("No {} found", entity_name),
        &description,
        action_label,
        action_url,
        Some("#modal-container"),
    )
}

/// Empty state with i18n support
pub fn empty_state_i18n(title: &str, message: &str, _has_filters: bool) -> Markup {
    html! {
        div style="padding: 3rem; text-align: center; color: var(--gray-500);" {
            h3 style="font-size: 1.25rem; font-weight: 600; margin-bottom: 0.5rem;" {
                (title)
            }
            p {
                (message)
            }
        }
    }
}
