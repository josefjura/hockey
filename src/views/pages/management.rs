use crate::i18n::TranslationContext;
use maud::{html, Markup};

pub fn management_page(t: &TranslationContext) -> Markup {
    html! {
        div class="card" {
            // Page header
            h1 style="font-size: 2rem; font-weight: 700; margin-bottom: 0.5rem;" {
                (t.messages.management_title())
            }
            p style="color: var(--gray-600); margin-bottom: 2rem;" {
                (t.messages.management_description())
            }

            // Management sections grid
            div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(280px, 1fr)); gap: 1.5rem; margin-top: 2rem;" {
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
            a href=(href) style="
                display: block;
                padding: 1.5rem;
                background: white;
                border: 2px solid var(--gray-200);
                border-radius: 8px;
                text-decoration: none;
                color: inherit;
                transition: all 0.2s;
            " onmouseover="this.style.borderColor='var(--primary)'; this.style.boxShadow='0 4px 6px rgba(0,0,0,0.1)';"
               onmouseout="this.style.borderColor='var(--gray-200)'; this.style.boxShadow='none';" {
                div style="display: flex; align-items: center; margin-bottom: 1rem;" {
                    div style="font-size: 2.5rem; margin-right: 1rem;" { (icon) }
                    h2 style="font-size: 1.25rem; font-weight: 600; color: var(--gray-900);" {
                        (title)
                    }
                }
                p style="color: var(--gray-600); margin-bottom: 1rem; line-height: 1.5;" {
                    (description)
                }
                div style="display: flex; align-items: center; color: var(--primary); font-weight: 500;" {
                    span { "Manage " (title.to_lowercase()) }
                    span style="margin-left: 0.5rem;" { "→" }
                }
            }
        }
    } else {
        html! {
            div style="
                padding: 1.5rem;
                background: var(--gray-50);
                border: 2px solid var(--gray-200);
                border-radius: 8px;
                opacity: 0.6;
                cursor: not-allowed;
            " {
                div style="display: flex; align-items: center; margin-bottom: 1rem;" {
                    div style="font-size: 2.5rem; margin-right: 1rem;" { (icon) }
                    h2 style="font-size: 1.25rem; font-weight: 600; color: var(--gray-600);" {
                        (title)
                    }
                }
                p style="color: var(--gray-500); margin-bottom: 1rem; line-height: 1.5;" {
                    (description)
                }
                span style="
                    display: inline-block;
                    padding: 0.5rem 1rem;
                    background: var(--gray-300);
                    color: var(--gray-600);
                    border-radius: 9999px;
                    font-size: 0.875rem;
                    font-weight: 500;
                " {
                    "Coming soon"
                }
            }
        }
    }
}
