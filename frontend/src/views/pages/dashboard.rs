use maud::{html, Markup};

use crate::i18n::{I18n, Locale};
use crate::service::dashboard::{DashboardStats, RecentActivity};

pub fn dashboard_page(
    i18n: &I18n,
    locale: Locale,
    stats: &DashboardStats,
    recent_activity: &[RecentActivity],
) -> Markup {
    let t = |key: &str| i18n.translate(locale, key);
    html! {
        div class="card" {
            h1 style="font-size: 2rem; font-weight: 700; margin-bottom: 1rem;" {
                (t("dashboard-title"))
            }
            p style="color: var(--gray-600); margin-bottom: 2rem;" {
                (t("dashboard-subtitle"))
            }

            // Stats cards
            div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 1.5rem; margin-top: 2rem;" {
                (stat_card(&t("nav-teams"), &stats.teams_count.to_string(), "/teams"))
                (stat_card(&t("nav-players"), &stats.players_count.to_string(), "/players"))
                (stat_card(&t("nav-events"), &stats.events_count.to_string(), "/events"))
                (stat_card(&t("nav-seasons"), &stats.seasons_count.to_string(), "/seasons"))
                (stat_card(&t("nav-matches"), &stats.matches_count.to_string(), "/matches"))
            }

            // Quick actions section
            div style="margin-top: 3rem;" {
                h2 style="font-size: 1.25rem; font-weight: 600; margin-bottom: 1rem;" {
                    (t("dashboard-quick-actions"))
                }
                div style="display: flex; flex-wrap: wrap; gap: 0.75rem;" {
                    (quick_action_button(&t("dashboard-add-team"), "/teams/new"))
                    (quick_action_button(&t("dashboard-add-player"), "/players/new"))
                    (quick_action_button(&t("dashboard-add-event"), "/events/new"))
                    (quick_action_button(&t("dashboard-add-season"), "/seasons/new"))
                    (quick_action_button(&t("dashboard-add-match"), "/matches/new"))
                }
            }

            // Recent activity section
            div style="margin-top: 3rem;" {
                h2 style="font-size: 1.25rem; font-weight: 600; margin-bottom: 1rem;" {
                    (t("dashboard-recent-activity"))
                }
                @if recent_activity.is_empty() {
                    div class="info" style="padding: 1rem;" {
                        (t("dashboard-no-activity"))
                    }
                } @else {
                    div style="background: var(--gray-50); border-radius: 8px; overflow: hidden;" {
                        @for (i, activity) in recent_activity.iter().enumerate() {
                            div style={
                                @if i > 0 { "border-top: 1px solid var(--gray-200);" }
                                "padding: 0.75rem 1rem; display: flex; align-items: center; gap: 0.75rem;"
                            } {
                                // Entity type icon
                                span style="font-size: 1.25rem;" {
                                    (get_entity_icon(&activity.entity_type))
                                }
                                // Activity details
                                div style="flex: 1;" {
                                    span style="font-weight: 500;" {
                                        (activity.entity_name)
                                    }
                                    span style="color: var(--gray-500); margin-left: 0.5rem;" {
                                        (activity.action)
                                    }
                                }
                                // Timestamp
                                span style="color: var(--gray-400); font-size: 0.875rem;" {
                                    (format_timestamp(&activity.timestamp))
                                }
                            }
                        }
                    }
                }
            }

            // Getting started info
            div style="margin-top: 3rem;" {
                div class="info" {
                    strong { (t("dashboard-getting-started")) }
                    " "
                    (t("dashboard-getting-started-text"))
                }
            }
        }
    }
}

fn stat_card(title: &str, value: &str, link: &str) -> Markup {
    html! {
        a href=(link) style="text-decoration: none;" {
            div style="
                background: linear-gradient(135deg, var(--primary-color) 0%, var(--primary-dark) 100%);
                padding: 1.5rem;
                border-radius: 12px;
                color: white;
                box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
                transition: transform 0.2s, box-shadow 0.2s;
                cursor: pointer;
            " 
            onmouseover="this.style.transform='translateY(-2px)'; this.style.boxShadow='0 6px 12px rgba(0, 0, 0, 0.15)';"
            onmouseout="this.style.transform='translateY(0)'; this.style.boxShadow='0 4px 6px rgba(0, 0, 0, 0.1)';"
            {
                div style="display: flex; justify-content: space-between; align-items: start; margin-bottom: 1rem;" {
                    div style="font-size: 2.5rem; font-weight: 700; line-height: 1;" { (value) }
                }
                div style="font-size: 0.875rem; opacity: 0.9; font-weight: 500;" { (title) }
            }
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
            style="
                display: inline-flex;
                align-items: center;
                gap: 0.5rem;
                padding: 0.75rem 1.25rem;
                background: white;
                border: 1px solid var(--gray-300);
                border-radius: 8px;
                color: var(--gray-700);
                font-weight: 500;
                text-decoration: none;
                transition: all 0.2s;
            "
            onmouseover="this.style.borderColor='var(--primary-color)'; this.style.color='var(--primary-color)';"
            onmouseout="this.style.borderColor='var(--gray-300)'; this.style.color='var(--gray-700)';"
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
