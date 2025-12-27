use maud::{html, Markup};

use crate::i18n::TranslationContext;
use crate::service::dashboard::{DashboardStats, RecentActivity};

pub fn dashboard_page(
    t: &TranslationContext,
    stats: &DashboardStats,
    recent_activity: &[RecentActivity],
) -> Markup {
    html! {
        div class="card" {
            h1 class="dashboard-title" {
                (t.messages.dashboard_title())
            }
            p class="dashboard-subtitle" {
                (t.messages.dashboard_subtitle())
            }

            // Stats cards
            div id="dashboard-stats" class="stats-grid" {
                (stat_card(&t.messages.nav_teams().to_string(), &stats.teams_count.to_string(), "/teams"))
                (stat_card(&t.messages.nav_players().to_string(), &stats.players_count.to_string(), "/players"))
                (stat_card(&t.messages.nav_events().to_string(), &stats.events_count.to_string(), "/events"))
                (stat_card(&t.messages.nav_seasons().to_string(), &stats.seasons_count.to_string(), "/seasons"))
                (stat_card(&t.messages.nav_matches().to_string(), &stats.matches_count.to_string(), "/matches"))
            }

            // Quick actions section
            div class="dashboard-section" {
                h2 class="section-heading" {
                    (t.messages.dashboard_quick_actions())
                }
                div class="quick-actions" {
                    (quick_action_button(&t.messages.dashboard_add_team().to_string(), "/teams/new"))
                    (quick_action_button(&t.messages.dashboard_add_player().to_string(), "/players/new"))
                    (quick_action_button(&t.messages.dashboard_add_event().to_string(), "/events/new"))
                    (quick_action_button(&t.messages.dashboard_add_season().to_string(), "/seasons/new"))
                    (quick_action_button(&t.messages.dashboard_add_match().to_string(), "/matches/new"))
                }
            }

            // Recent activity section
            div class="dashboard-section" {
                h2 class="section-heading" {
                    (t.messages.dashboard_recent_activity())
                }
                @if recent_activity.is_empty() {
                    div class="info" style="padding: 1rem;" {
                        (t.messages.dashboard_no_activity())
                    }
                } @else {
                    div class="activity-feed" {
                        @for activity in recent_activity.iter() {
                            div class="activity-item" {
                                // Entity type icon
                                span class="activity-icon" {
                                    (get_entity_icon(&activity.entity_type))
                                }
                                // Activity details
                                div class="activity-details" {
                                    span class="activity-name" {
                                        (activity.entity_name)
                                    }
                                    span class="activity-action" {
                                        (activity.action)
                                    }
                                }
                                // Timestamp
                                span class="activity-timestamp" {
                                    (format_timestamp(&activity.timestamp))
                                }
                            }
                        }
                    }
                }
            }

            // Getting started info
            div class="dashboard-section" {
                div class="info" {
                    strong { (t.messages.dashboard_getting_started()) }
                    " "
                    (t.messages.dashboard_getting_started_text())
                }
            }
        }
    }
}

/// Partial template for dashboard stats (for out-of-band updates)
pub fn dashboard_stats_partial(t: &TranslationContext, stats: &DashboardStats) -> Markup {
    html! {
        div id="dashboard-stats" class="stats-grid" hx-swap-oob="true" {
            (stat_card(&t.messages.nav_teams().to_string(), &stats.teams_count.to_string(), "/teams"))
            (stat_card(&t.messages.nav_players().to_string(), &stats.players_count.to_string(), "/players"))
            (stat_card(&t.messages.nav_events().to_string(), &stats.events_count.to_string(), "/events"))
            (stat_card(&t.messages.nav_seasons().to_string(), &stats.seasons_count.to_string(), "/seasons"))
            (stat_card(&t.messages.nav_matches().to_string(), &stats.matches_count.to_string(), "/matches"))
        }
    }
}

fn stat_card(title: &str, value: &str, link: &str) -> Markup {
    html! {
        a href=(link) class="stat-card" {
            div class="stat-card-header" {
                div class="stat-card-value" { (value) }
            }
            div class="stat-card-title" { (title) }
        }
    }
}

fn quick_action_button(label: &str, href: &str) -> Markup {
    html! {
        a
            href=(href)
            hx-get=(href)
            hx-target="#modal-container"
            hx-swap="innerHTML"
            class="quick-action-btn"
        {
            span { (label) }
        }
    }
}

fn get_entity_icon(entity_type: &str) -> &'static str {
    match entity_type {
        "Team" => "🏒",
        "Player" => "👤",
        "Event" => "🏆",
        "Season" => "📅",
        "Match" => "⚔️",
        _ => "📋",
    }
}

fn format_timestamp(timestamp: &str) -> String {
    // Parse and format the timestamp for display
    // For now, just return the date portion if available
    if timestamp.len() >= 10 {
        timestamp[..10].to_string()
    } else {
        timestamp.to_string()
    }
}
