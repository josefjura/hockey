use maud::{html, Markup};

use crate::views::components::empty_state::{empty_state_enhanced, EmptyStateIcon};

/// Empty state when no items are found (enhanced with icon)
pub fn empty_state(entity_name: &str, has_filters: bool) -> Markup {
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

    empty_state_enhanced(
        icon,
        &format!("No {} found", entity_name),
        &description,
        None,
        None,
        None,
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
