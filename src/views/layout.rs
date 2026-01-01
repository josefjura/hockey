use maud::{html, Markup, DOCTYPE};

use super::components::sidebar;
use super::components::toast::htmx_toast_event_handler;
use crate::auth::Session;
use crate::i18n::TranslationContext;

pub fn base_layout(title: &str, content: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                title { (title) " - Hockey Management" }
                // CSS Files
                link rel="stylesheet" href="/static/css/theme.css";
                link rel="stylesheet" href="/static/css/reset.css";
                link rel="stylesheet" href="/static/css/layout.css";
                link rel="stylesheet" href="/static/css/components.css";
                link rel="stylesheet" href="/static/css/forms.css";
                link rel="stylesheet" href="/static/css/tables.css";
                link rel="stylesheet" href="/static/css/modals.css";
                link rel="stylesheet" href="/static/css/utils.css";
                link rel="stylesheet" href="/static/css/pages.css";
                // HTMX library for dynamic HTML updates
                script src="https://unpkg.com/htmx.org@2.0.4" {}
                // Import map for Lit web components
                script type="importmap" {
                    (maud::PreEscaped(r#"
{
    "imports": {
        "lit": "https://cdn.jsdelivr.net/npm/lit@3/index.js",
        "lit/": "https://cdn.jsdelivr.net/npm/lit@3/",
        "lit/decorators.js": "https://cdn.jsdelivr.net/npm/lit@3/decorators.js",
        "@lit/reactive-element": "https://cdn.jsdelivr.net/npm/@lit/reactive-element@2/reactive-element.js",
        "@lit/reactive-element/": "https://cdn.jsdelivr.net/npm/@lit/reactive-element@2/",
        "lit-html": "https://cdn.jsdelivr.net/npm/lit-html@3/lit-html.js",
        "lit-html/": "https://cdn.jsdelivr.net/npm/lit-html@3/",
        "lit-element/": "https://cdn.jsdelivr.net/npm/lit-element@4/"
    }
}
                    "#))
                }
                // Web Components
                script type="module" src="/static/js/components/country-selector.js" {}
                script type="module" src="/static/js/components/badge.js" {}
                script type="module" src="/static/js/components/flag-icon.js" {}
                script type="module" src="/static/js/components/toggle-switch.js" {}
                script type="module" src="/static/js/components/client-data-table.js" {}
                script type="module" src="/static/js/components/countries-table.js" {}
                script type="module" src="/static/js/components/loading-spinner.js" {}
                script type="module" src="/static/js/components/loading-state.js" {}
                script type="module" src="/static/js/components/toast.js" {}
                script type="module" src="/static/js/components/confirm-dialog.js" {}
                script type="module" src="/static/js/components/modal.js" {}
            }
            body {
                (content)
                // Toast notification container
                hockey-toast-container position="top-right" {}
                // Confirmation dialog
                hockey-confirm-dialog {}
                // HTMX toast event handler
                (htmx_toast_event_handler())
            }
        }
    }
}

/// Admin layout with sidebar navigation
pub fn admin_layout(
    title: &str,
    session: &Session,
    current_path: &str,
    t: &TranslationContext,
    content: Markup,
) -> Markup {
    base_layout(
        title,
        html! {
            div class="app-layout" {
                // Mobile menu toggle button
                button
                    class="mobile-menu-toggle"
                    onclick="document.querySelector('.sidebar').classList.toggle('active'); document.querySelector('.sidebar-overlay').classList.toggle('active');"
                    aria-label="Toggle navigation menu"
                {
                    "☰"
                }
                // Sidebar overlay for mobile
                div
                    class="sidebar-overlay"
                    onclick="document.querySelector('.sidebar').classList.remove('active'); this.classList.remove('active');"
                {}
                (sidebar(session, current_path, t))
                main class="main-content" {
                    div class="content-wrapper" {
                        (content)
                    }
                }
            }
            // Modal container for HTMX modal loading
            div id="modal-container" {}
        },
    )
}

pub fn auth_layout(title: &str, content: Markup) -> Markup {
    base_layout(
        title,
        html! {
            div.container style="max-width: 480px; margin-top: 4rem;" {
                (content)
            }
        },
    )
}
