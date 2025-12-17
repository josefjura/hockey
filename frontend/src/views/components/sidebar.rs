use maud::{html, Markup};

use crate::auth::Session;
use crate::i18n::{I18n, Locale};

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

pub fn sidebar(session: &Session, current_path: &str, i18n: &I18n, locale: Locale) -> Markup {
    let nav_items = vec![
        NavItem::new("/", i18n.translate(locale, "nav-dashboard"), "📊"),
        NavItem::new("/teams", i18n.translate(locale, "nav-teams"), "🏒"),
        NavItem::new("/players", i18n.translate(locale, "nav-players"), "👤"),
        NavItem::new("/events", i18n.translate(locale, "nav-events"), "🏆"),
        NavItem::new("/seasons", i18n.translate(locale, "nav-seasons"), "📅"),
        NavItem::new("/matches", i18n.translate(locale, "nav-matches"), "🎯"),
        NavItem::new(
            "/management",
            i18n.translate(locale, "nav-management"),
            "⚙️",
        ),
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
                (locale_switcher(i18n, locale))
                (logout_button(i18n, locale))
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

fn locale_switcher(i18n: &I18n, locale: Locale) -> Markup {
    html! {
        div class="locale-switcher" {
            label for="locale-select" class="locale-label" { (i18n.translate(locale, "user-language")) }
            select id="locale-select" class="locale-select" {
                option value="en" selected[locale == Locale::English] { "English" }
                option value="cs" selected[locale == Locale::Czech] { "Čeština" }
            }
        }
    }
}

fn logout_button(i18n: &I18n, locale: Locale) -> Markup {
    html! {
        form method="POST" action="/auth/logout" class="logout-form" {
            button type="submit" class="logout-button" {
                span class="nav-icon" { "🚪" }
                span { (i18n.translate(locale, "user-logout")) }
            }
        }
    }
}
