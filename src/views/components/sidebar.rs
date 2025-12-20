use maud::{html, Markup};

use crate::auth::Session;
use crate::i18n::{Locale, TranslationContext};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NavItem {
    pub path: String,
    pub label: String,
    pub icon: &'static str,
}

impl NavItem {
    pub fn new(path: impl Into<String>, label: impl Into<String>, icon: &'static str) -> Self {
        Self {
            path: path.into(),
            label: label.into(),
            icon,
        }
    }

    pub fn is_active(&self, current_path: &str) -> bool {
        if self.path == "/" {
            current_path == "/"
        } else {
            current_path.starts_with(&self.path)
        }
    }
}

pub fn sidebar(session: &Session, current_path: &str, t: &TranslationContext) -> Markup {
    let nav_items = vec![
        NavItem::new("/", t.messages.nav_dashboard().to_string(), "📊"),
        NavItem::new("/teams", t.messages.nav_teams().to_string(), "🏒"),
        NavItem::new("/players", t.messages.nav_players().to_string(), "👤"),
        NavItem::new("/events", t.messages.nav_events().to_string(), "🏆"),
        NavItem::new("/seasons", t.messages.nav_seasons().to_string(), "📅"),
        NavItem::new("/matches", t.messages.nav_matches().to_string(), "🎯"),
        NavItem::new("/management", t.messages.nav_management().to_string(), "⚙️"),
    ];

    html! {
        aside class="sidebar" {
            // Logo/Brand
            div class="sidebar-brand" {
                h1 { "🏒 Hockey Manager" }
            }

            // Navigation
            nav class="sidebar-nav" {
                @for item in &nav_items {
                    (nav_link(&item, current_path))
                }
            }

            // User info and controls at bottom
            div class="sidebar-footer" {
                (user_info(session))
                (locale_switcher(t))
                (logout_button(t))
                (version_info())
            }
        }
    }
}

fn nav_link(item: &NavItem, current_path: &str) -> Markup {
    let active_class = if item.is_active(current_path) {
        "nav-link active"
    } else {
        "nav-link"
    };

    html! {
        a href=(item.path) class=(active_class) {
            span class="nav-icon" { (item.icon) }
            span class="nav-label" { (item.label) }
        }
    }
}

fn user_info(session: &Session) -> Markup {
    html! {
        div class="user-info" {
            div class="user-avatar" {
                (session.user_name.chars().next().unwrap_or('U').to_uppercase())
            }
            div class="user-details" {
                div class="user-name" { (session.user_name) }
                div class="user-email" { (session.user_email) }
            }
        }
    }
}

fn locale_switcher(t: &TranslationContext) -> Markup {
    html! {
        div class="locale-switcher" {
            label for="locale-select" class="locale-label" { (t.messages.user_language()) }
            select id="locale-select" class="locale-select"
                   onchange="window.location.href = '/locale/' + this.value" {
                option value="en" selected[t.locale == Locale::English] { "English" }
                option value="cs" selected[t.locale == Locale::Czech] { "Čeština" }
            }
        }
    }
}

fn logout_button(t: &TranslationContext) -> Markup {
    html! {
        form method="POST" action="/auth/logout" class="logout-form" {
            button type="submit" class="logout-button" {
                span class="nav-icon" { "🚪" }
                span { (t.messages.user_logout()) }
            }
        }
    }
}

fn version_info() -> Markup {
    let version = env!("CARGO_PKG_VERSION");
    html! {
        div class="version-info" {
            span class="version-text" { "v" (version) }
        }
    }
}
