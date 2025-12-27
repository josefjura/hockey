use maud::{html, Markup};

use crate::i18n::TranslationContext;
use crate::service::matches::{MatchDetailEntity, ScoreEventEntity};
use crate::views::components::confirm::{confirm_attrs, ConfirmVariant};

/// Match detail page with score tracking
pub fn match_detail_page(t: &TranslationContext, detail: &MatchDetailEntity) -> Markup {
    let match_info = &detail.match_info;

    html! {
        div class="card" {
            // Header with back button and action buttons
            div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 1.5rem;" {
                div style="display: flex; align-items: center; gap: 1rem;" {
                    a
                        href="/matches"
                        class="btn btn-secondary"
                    {
                        (format!("← {}", t.messages.matches_back_to_list()))
                    }
                    h1 style="font-size: 2rem; font-weight: 700; margin: 0;" {
                        (t.messages.matches_match_info())
                    }
                }
                div style="display: flex; gap: 0.5rem;" {
                    button
                        class="btn btn-primary"
                        hx-get=(format!("/matches/{}/edit", match_info.id))
                        hx-target="#modal-container"
                        hx-swap="innerHTML"
                    {
                        (t.messages.matches_edit())
                    }
                    button
                        class="btn btn-danger"
                        hx-post=(format!("/matches/{}/delete", match_info.id))
                        hx-confirm-custom=(confirm_attrs(
                            &t.messages.matches_delete().to_string(),
                            &t.messages.matches_confirm_delete().to_string(),
                            ConfirmVariant::Danger,
                            Some(&t.messages.common_delete().to_string()),
                            Some(&t.messages.common_cancel().to_string())
                        ))
                    {
                        (t.messages.matches_delete())
                    }
                }
            }

            // Match Info Card
            div style="margin-bottom: 2rem; padding: 1.5rem; background: var(--gray-50); border-radius: 8px;" {
                // Match Score
                div style="text-align: center; margin-bottom: 1.5rem;" {
                    div style="display: flex; justify-content: center; align-items: center; gap: 2rem; margin-bottom: 1rem;" {
                        // Home Team
                        div style="flex: 1; text-align: right;" {
                            div style="display: flex; justify-content: flex-end; align-items: center; gap: 0.5rem; margin-bottom: 0.5rem;" {
                                @if let Some(iso2) = &match_info.home_team_country_iso2 {
                                    flag-icon
                                        country-code=(iso2.to_lowercase())
                                        country-name=(match_info.home_team_name)
                                        size="sm";
                                }
                                span style="font-size: 1.5rem; font-weight: 600;" {
                                    (match_info.home_team_name)
                                }
                            }
                        }

                        // Score
                        div style="font-size: 3rem; font-weight: 700; padding: 0 2rem;" {
                            (detail.home_score_total)
                            " : "
                            (detail.away_score_total)
                        }

                        // Away Team
                        div style="flex: 1; text-align: left;" {
                            div style="display: flex; justify-content: flex-start; align-items: center; gap: 0.5rem; margin-bottom: 0.5rem;" {
                                span style="font-size: 1.5rem; font-weight: 600;" {
                                    (match_info.away_team_name)
                                }
                                @if let Some(iso2) = &match_info.away_team_country_iso2 {
                                    flag-icon
                                        country-code=(iso2.to_lowercase())
                                        country-name=(match_info.away_team_name)
                                        size="sm";
                                }
                            }
                        }
                    }

                    // Status Badge
                    div {
                        (status_badge(&match_info.status))
                    }
                }

                // Match Information Grid
                div style="display: grid; grid-template-columns: repeat(2, 1fr); gap: 1rem; margin-top: 1.5rem; padding-top: 1.5rem; border-top: 1px solid var(--gray-200);" {
                    div {
                        div style="color: var(--gray-600); font-size: 0.875rem; margin-bottom: 0.25rem;" {
                            "Event"
                        }
                        div style="font-weight: 500;" {
                            @if let Some(event_name) = &match_info.event_name {
                                (event_name)
                            } @else {
                                span style="color: var(--gray-400); font-style: italic;" { "Unknown" }
                            }
                        }
                    }
                    div {
                        div style="color: var(--gray-600); font-size: 0.875rem; margin-bottom: 0.25rem;" {
                            "Season"
                        }
                        div style="font-weight: 500;" {
                            @if let Some(season_name) = &match_info.season_name {
                                (season_name)
                            } @else {
                                span style="color: var(--gray-400); font-style: italic;" { "Unknown" }
                            }
                        }
                    }
                    div {
                        div style="color: var(--gray-600); font-size: 0.875rem; margin-bottom: 0.25rem;" {
                            "Date"
                        }
                        div style="font-weight: 500;" {
                            @if let Some(date) = &match_info.match_date {
                                (format_date(date))
                            } @else {
                                span style="color: var(--gray-400); font-style: italic;" { "Not scheduled" }
                            }
                        }
                    }
                    div {
                        div style="color: var(--gray-600); font-size: 0.875rem; margin-bottom: 0.25rem;" {
                            "Venue"
                        }
                        div style="font-weight: 500;" {
                            @if let Some(venue) = &match_info.venue {
                                (venue)
                            } @else {
                                span style="color: var(--gray-400); font-style: italic;" { "TBD" }
                            }
                        }
                    }
                }
            }

            // Score Breakdown
            div style="margin-bottom: 2rem;" {
                h2 style="font-size: 1.5rem; font-weight: 700; margin-bottom: 1rem;" {
                    "Score Breakdown"
                }
                div style="display: grid; grid-template-columns: repeat(2, 1fr); gap: 1rem;" {
                    // Home Team Breakdown
                    div style="padding: 1rem; background: var(--gray-50); border-radius: 8px;" {
                        div style="font-weight: 600; margin-bottom: 0.5rem;" {
                            (match_info.home_team_name)
                        }
                        div style="display: flex; justify-content: space-between; padding: 0.5rem 0; border-bottom: 1px solid var(--gray-200);" {
                            span { "Identified goals" }
                            span style="font-weight: 600;" { (detail.home_score_identified) }
                        }
                        div style="display: flex; justify-content: space-between; padding: 0.5rem 0; border-bottom: 1px solid var(--gray-200);" {
                            span { "Unidentified goals" }
                            span style="font-weight: 600;" { (match_info.home_score_unidentified) }
                        }
                        div style="display: flex; justify-content: space-between; padding: 0.5rem 0; font-size: 1.125rem;" {
                            span style="font-weight: 600;" { "Total" }
                            span style="font-weight: 700;" { (detail.home_score_total) }
                        }
                    }

                    // Away Team Breakdown
                    div style="padding: 1rem; background: var(--gray-50); border-radius: 8px;" {
                        div style="font-weight: 600; margin-bottom: 0.5rem;" {
                            (match_info.away_team_name)
                        }
                        div style="display: flex; justify-content: space-between; padding: 0.5rem 0; border-bottom: 1px solid var(--gray-200);" {
                            span { "Identified goals" }
                            span style="font-weight: 600;" { (detail.away_score_identified) }
                        }
                        div style="display: flex; justify-content: space-between; padding: 0.5rem 0; border-bottom: 1px solid var(--gray-200);" {
                            span { "Unidentified goals" }
                            span style="font-weight: 600;" { (match_info.away_score_unidentified) }
                        }
                        div style="display: flex; justify-content: space-between; padding: 0.5rem 0; font-size: 1.125rem;" {
                            span style="font-weight: 600;" { "Total" }
                            span style="font-weight: 700;" { (detail.away_score_total) }
                        }
                    }
                }
            }

            // Score Events (Goals)
            div {
                div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 1rem;" {
                    h2 style="font-size: 1.5rem; font-weight: 700; margin: 0;" {
                        "Goals"
                    }
                    button
                        class="btn btn-primary"
                        hx-get=(format!("/matches/{}/score-events/new", match_info.id))
                        hx-target="#modal-container"
                        hx-swap="innerHTML"
                    {
                        "+ Identify Goal"
                    }
                }

                @if detail.score_events.is_empty() {
                    div style="padding: 3rem; text-align: center; color: var(--gray-500); background: var(--gray-50); border-radius: 8px;" {
                        p { "No goals identified yet." }
                        p style="margin-top: 0.5rem; font-size: 0.875rem;" {
                            "Click 'Identify Goal' to assign goals to players."
                        }
                    }
                } @else {
                    (score_events_list(&detail.score_events, match_info.home_team_id, match_info.away_team_id))
                }
            }

            // Modal container
            div id="modal-container" {}
        }
    }
}

/// Render score events list
pub fn score_events_list(
    events: &[ScoreEventEntity],
    home_team_id: i64,
    _away_team_id: i64,
) -> Markup {
    html! {
        div style="border: 1px solid var(--gray-200); border-radius: 8px; overflow: hidden;" {
            @for event in events {
                div style=(format!(
                    "display: flex; justify-content: space-between; align-items: center; padding: 1rem; border-bottom: 1px solid var(--gray-200); {}",
                    if event.team_id == home_team_id {
                        "background: linear-gradient(90deg, rgba(59, 130, 246, 0.05) 0%, transparent 100%);"
                    } else {
                        "background: linear-gradient(270deg, rgba(239, 68, 68, 0.05) 0%, transparent 100%);"
                    }
                )) {
                    // Time and Period
                    div style="min-width: 100px;" {
                        div style="font-weight: 600; color: var(--gray-900);" {
                            (period_name(event.period))
                        }
                        @if let (Some(min), Some(sec)) = (event.time_minutes, event.time_seconds) {
                            div style="font-size: 0.875rem; color: var(--gray-600);" {
                                (format!("{}:{:02}", min, sec))
                            }
                        }
                    }

                    // Team
                    div style="flex: 1;" {
                        div style="font-weight: 600; color: var(--gray-900);" {
                            (event.team_name)
                        }
                        @if let Some(goal_type) = &event.goal_type {
                            div style="font-size: 0.875rem; color: var(--gray-600);" {
                                (format_goal_type(goal_type))
                            }
                        }
                    }

                    // Scorer and Assists
                    div style="flex: 2;" {
                        div {
                            span style="font-weight: 600;" { "Goal: " }
                            @if let Some(scorer_name) = &event.scorer_name {
                                span { (scorer_name) }
                            } @else {
                                span style="color: var(--gray-400); font-style: italic;" { "Unknown" }
                            }
                        }
                        @if event.assist1_id.is_some() || event.assist2_id.is_some() {
                            div style="font-size: 0.875rem; color: var(--gray-600); margin-top: 0.25rem;" {
                                span { "Assists: " }
                                @if let Some(assist1_name) = &event.assist1_name {
                                    span { (assist1_name) }
                                    @if event.assist2_name.is_some() {
                                        span { ", " }
                                    }
                                }
                                @if let Some(assist2_name) = &event.assist2_name {
                                    span { (assist2_name) }
                                }
                            }
                        }
                    }

                    // Actions
                    div {
                        button
                            class="btn btn-sm"
                            hx-get=(format!("/matches/score-events/{}/edit", event.id))
                            hx-target="#modal-container"
                            hx-swap="innerHTML"
                            style="margin-right: 0.5rem;"
                        {
                            "Edit"
                        }
                        button
                            class="btn btn-sm btn-danger"
                            hx-post=(format!("/matches/score-events/{}/delete", event.id))
                            hx-confirm-custom=(confirm_attrs(
                                "Delete Goal",
                                "Are you sure you want to delete this goal? This action cannot be undone.",
                                ConfirmVariant::Danger,
                                Some("Delete"),
                                Some("Cancel")
                            ))
                        {
                            "Delete"
                        }
                    }
                }
            }
        }
    }
}

/// Format match status as a badge
pub fn status_badge(status: &str) -> Markup {
    let text = match status {
        "scheduled" => "Scheduled",
        "in_progress" => "In Progress",
        "finished" => "Finished",
        "cancelled" => "Cancelled",
        _ => status,
    };

    html! {
        span
            style=(format!(
                "display: inline-block; padding: 0.25rem 0.75rem; border-radius: 9999px; font-size: 0.875rem; font-weight: 500; {}",
                match status {
                    "scheduled" => "color: #1e40af; background: #dbeafe;",
                    "in_progress" => "color: #15803d; background: #dcfce7;",
                    "finished" => "color: #374151; background: #f3f4f6;",
                    "cancelled" => "color: #b91c1c; background: #fee2e2;",
                    _ => "color: #374151; background: #f3f4f6;",
                }
            ))
        {
            (text)
        }
    }
}

/// Format period number to readable name
pub fn period_name(period: i32) -> &'static str {
    match period {
        1 => "1st Period",
        2 => "2nd Period",
        3 => "3rd Period",
        4 => "Overtime",
        5 => "Shootout",
        _ => "Unknown",
    }
}

/// Format goal type to readable text
pub fn format_goal_type(goal_type: &str) -> String {
    match goal_type {
        "even_strength" => "Even Strength".to_string(),
        "power_play" => "Power Play".to_string(),
        "short_handed" => "Short Handed".to_string(),
        "penalty_shot" => "Penalty Shot".to_string(),
        "empty_net" => "Empty Net".to_string(),
        _ => goal_type.to_string(),
    }
}

/// Format ISO date to readable format
pub fn format_date(date: &str) -> String {
    // Simple formatting - just display the date part
    if let Some(date_part) = date.split('T').next() {
        date_part.to_string()
    } else {
        date.to_string()
    }
}
