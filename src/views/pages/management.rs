use crate::i18n::TranslationContext;
use maud::{html, Markup};

pub fn management_page(t: &TranslationContext) -> Markup {
    html! {
        div class="card" {
            // Page header
            h1 class="page-title" {
                (t.messages.management_title())
            }
            p class="page-description" {
                (t.messages.management_description())
            }

            // Management sections grid
            div class="management-grid" {
                // Countries card - active
                (management_card(
                    "🌍",
                    &t.messages.management_countries_title().to_string(),
                    "Manage country data, IIHF membership, and availability",
                    "/countries",
                    true
                ))

                // Future: Users card (placeholder)
                (management_card(
                    "👥",
                    "Users",
                    "Manage user accounts and permissions",
                    "#",
                    false
                ))

                // Future: System settings (placeholder)
                (management_card(
                    "⚙️",
                    "Settings",
                    "Configure system preferences and options",
                    "#",
                    false
                ))
            }
        }
    }
}

/// Render a management card
fn management_card(icon: &str, title: &str, description: &str, href: &str, active: bool) -> Markup {
    if active {
        html! {
            a href=(href) class="management-card" {
                div class="management-card-header" {
                    div class="management-card-icon" { (icon) }
                    h2 class="management-card-title" {
                        (title)
                    }
                }
                p class="management-card-description" {
                    (description)
                }
                div class="management-card-link" {
                    span { "Manage " (title.to_lowercase()) }
                    span class="management-card-link-arrow" { "→" }
                }
            }
        }
    } else {
        html! {
            div class="management-card-disabled" {
                div class="management-card-header" {
                    div class="management-card-icon" { (icon) }
                    h2 class="management-card-title-disabled" {
                        (title)
                    }
                }
                p class="management-card-description-disabled" {
                    (description)
                }
                span class="coming-soon-badge" {
                    "Coming soon"
                }
            }
        }
    }
}
