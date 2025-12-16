use maud::{html, Markup};

use crate::auth::Session;
use crate::i18n::Translations;

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

pub fn sidebar(session: &Session, current_path: &str, t: &Translations) -> Markup {
    let nav_items = vec![
        NavItem::new("/", t.nav_dashboard(), "📊"),
        NavItem::new("/teams", t.nav_teams(), "🏒"),
        NavItem::new("/players", t.nav_players(), "👤"),
        NavItem::new("/events", t.nav_events(), "🏆"),
        NavItem::new("/seasons", t.nav_seasons(), "📅"),
        NavItem::new("/matches", t.nav_matches(), "🎯"),
        NavItem::new("/management", t.nav_management(), "⚙️"),
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

fn locale_switcher(t: &Translations) -> Markup {
    html! {
        div class="locale-switcher" {
            label for="locale-select" class="locale-label" { (t.language()) }
            select id="locale-select" class="locale-select" {
                option value="en" selected[t.locale == crate::i18n::Locale::English] { "English" }
                option value="cs" selected[t.locale == crate::i18n::Locale::Czech] { "Čeština" }
            }
        }
    }
}

fn logout_button(t: &Translations) -> Markup {
    html! {
        form method="POST" action="/auth/logout" class="logout-form" {
            button type="submit" class="logout-button" {
                span class="nav-icon" { "🚪" }
                span { (t.logout()) }
            }
        }
    }
}
