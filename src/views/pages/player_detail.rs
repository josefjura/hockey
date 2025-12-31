use maud::{html, Markup};

use crate::i18n::TranslationContext;
use crate::service::players::{
    PlayerContractWithTeamEntity, PlayerDetailEntity, PlayerEntity, PlayerEventStatsEntity,
    PlayerSeasonStats,
};
use crate::views::components::confirm::{confirm_attrs, ConfirmVariant};

/// Player detail page with career history and scoring
pub fn player_detail_page(
    t: &TranslationContext,
    detail: &PlayerDetailEntity,
    season_stats: &[PlayerSeasonStats],
    event_stats: &[PlayerEventStatsEntity],
) -> Markup {
    let player = &detail.player_info;

    html! {
        div class="card" {
            // Header with back button and action buttons
            div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 1.5rem;" {
                div style="display: flex; align-items: center; gap: 1rem;" {
                    a
                        href="/players"
                        class="btn btn-secondary"
                    {
                        (format!("← {}", t.messages.players_back_to_list()))
                    }
                    h1 style="font-size: 2rem; font-weight: 700; margin: 0;" {
                        (player.name)
                    }
                }
                div style="display: flex; gap: 0.5rem;" {
                    @if !season_stats.is_empty() {
                        a
                            href=(format!("/players/{}/scoring", player.id))
                            class="btn btn-primary"
                        {
                            (t.messages.player_view_scoring())
                        }
                    }
                    button
                        class="btn btn-primary"
                        hx-get=(format!("/players/{}/edit", player.id))
                        hx-target="#modal-container"
                        hx-swap="innerHTML"
                    {
                        (t.messages.players_edit())
                    }
                    button
                        class="btn btn-danger"
                        hx-post=(format!("/players/{}/delete", player.id))
                        hx-confirm-custom=(confirm_attrs(
                            &t.messages.players_delete().to_string(),
                            &t.messages.players_confirm_delete().to_string(),
                            ConfirmVariant::Danger,
                            Some(&t.messages.common_delete().to_string()),
                            Some(&t.messages.common_cancel().to_string())
                        ))
                    {
                        (t.messages.players_delete())
                    }
                }
            }

            // Player Info Card
            (player_info_card(t, player))

            // Career Statistics Summary by Event
            (career_stats_by_event(t, player, event_stats))

            // Career History Section
            div style="margin-top: 2rem;" {
                div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 1.5rem;" {
                    h2 style="font-size: 1.5rem; font-weight: 700; margin: 0;" {
                        (t.messages.players_career_history())
                    }
                }

                @if detail.contracts.is_empty() {
                    (empty_contracts_state(t))
                } @else {
                    (contracts_list(t, &detail.contracts))
                }
            }

            // Modal container
            div id="modal-container" {}
        }
    }
}

/// Player info card with nationality and photo
fn player_info_card(t: &TranslationContext, player: &PlayerEntity) -> Markup {
    html! {
        div style="padding: 1.5rem; background: var(--gray-50); border-radius: 8px;" {
            div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 1rem; align-items: center;" {
                @if let Some(photo_path) = &player.photo_path {
                    div style="grid-column: 1; display: flex; justify-content: center;" {
                        img
                            src=(photo_path)
                            alt=(player.name)
                            style="width: 120px; height: 120px; object-fit: cover; border-radius: 50%; border: 3px solid var(--gray-300);"
                            onerror="this.style.display='none'";
                    }
                }
                div {
                    div style="color: var(--gray-600); font-size: 0.875rem; margin-bottom: 0.25rem;" {
                        (t.messages.players_name())
                    }
                    div style="font-weight: 600;" {
                        (player.name)
                    }
                }
                div {
                    div style="color: var(--gray-600); font-size: 0.875rem; margin-bottom: 0.25rem;" {
                        (t.messages.players_nationality())
                    }
                    div style="display: flex; align-items: center; gap: 0.5rem; font-weight: 600;" {
                        img
                            src=(format!("https://flagcdn.com/w40/{}.png", player.country_iso2_code.to_lowercase()))
                            alt=(player.country_name)
                            style="width: 24px; height: 18px; object-fit: cover; border: 1px solid var(--gray-300); border-radius: 2px;"
                            onerror="this.style.display='none'";
                        span { (player.country_name) }
                    }
                }
                @if let Some(birth_date) = &player.birth_date {
                    div {
                        div style="color: var(--gray-600); font-size: 0.875rem; margin-bottom: 0.25rem;" {
                            (t.messages.players_date_of_birth())
                        }
                        div style="font-weight: 600;" {
                            (birth_date)
                        }
                    }
                }
                @if let Some(birth_place) = &player.birth_place {
                    div {
                        div style="color: var(--gray-600); font-size: 0.875rem; margin-bottom: 0.25rem;" {
                            (t.messages.players_birthplace())
                        }
                        div style="font-weight: 600;" {
                            (birth_place)
                        }
                    }
                }
                @if let Some(height_cm) = player.height_cm {
                    div {
                        div style="color: var(--gray-600); font-size: 0.875rem; margin-bottom: 0.25rem;" {
                            (t.messages.players_height())
                        }
                        div style="font-weight: 600;" {
                            (format!("{} cm", height_cm))
                        }
                    }
                }
                @if let Some(weight_kg) = player.weight_kg {
                    div {
                        div style="color: var(--gray-600); font-size: 0.875rem; margin-bottom: 0.25rem;" {
                            (t.messages.players_weight())
                        }
                        div style="font-weight: 600;" {
                            (format!("{} kg", weight_kg))
                        }
                    }
                }
                @if let Some(position) = &player.position {
                    div {
                        div style="color: var(--gray-600); font-size: 0.875rem; margin-bottom: 0.25rem;" {
                            (t.messages.players_position())
                        }
                        div style="font-weight: 600;" {
                            (position)
                        }
                    }
                }
                @if let Some(shoots) = &player.shoots {
                    div {
                        div style="color: var(--gray-600); font-size: 0.875rem; margin-bottom: 0.25rem;" {
                            (t.messages.players_shoots_catches())
                        }
                        div style="font-weight: 600;" {
                            (shoots)
                        }
                    }
                }
            }
        }
    }
}

/// Contracts list in grid layout showing career history
fn contracts_list(t: &TranslationContext, contracts: &[PlayerContractWithTeamEntity]) -> Markup {
    html! {
        div style="display: grid; grid-template-columns: repeat(auto-fill, minmax(320px, 1fr)); gap: 1rem;" {
            @for contract in contracts {
                div style="padding: 1rem; border: 1px solid var(--gray-200); border-radius: 8px; display: flex; flex-direction: column; gap: 1rem; transition: box-shadow 0.2s;"
                     onmouseover="this.style.boxShadow='0 2px 8px rgba(0,0,0,0.1)'"
                     onmouseout="this.style.boxShadow='none'"
                {
                    div style="flex: 1;" {
                        div style="display: flex; align-items: center; gap: 0.5rem; margin-bottom: 0.5rem;" {
                            @if let Some(iso2) = &contract.team_country_iso2_code {
                                img
                                    src=(format!("https://flagcdn.com/w40/{}.png", iso2.to_lowercase()))
                                    alt=(contract.team_name)
                                    style="width: 24px; height: 18px; object-fit: cover; border: 1px solid var(--gray-300); border-radius: 2px;"
                                    onerror="this.style.display='none'";
                            }
                            span style="font-weight: 600; font-size: 1rem;" {
                                (contract.team_name)
                            }
                        }
                        div style="color: var(--gray-600); font-size: 0.875rem; margin-bottom: 0.25rem;" {
                            (contract.event_name)
                        }
                        div style="color: var(--gray-500); font-size: 0.875rem;" {
                            @if let Some(display_name) = &contract.season_display_name {
                                (display_name)
                                " ("
                                (contract.season_year)
                                ")"
                            } @else {
                                (t.messages.players_season())
                                " "
                                (contract.season_year)
                            }
                        }
                    }
                    div style="display: flex; gap: 0.5rem;" {
                        a
                            href=(format!("/teams/{}", contract.team_id))
                            class="btn btn-sm btn-secondary"
                            style="flex: 1;"
                        {
                            (t.messages.players_view_team())
                        }
                        a
                            href=(format!("/seasons/{}", contract.season_id))
                            class="btn btn-sm btn-secondary"
                            style="flex: 1;"
                        {
                            (t.messages.players_view_season())
                        }
                    }
                }
            }
        }
    }
}

/// Empty state when no contracts exist
fn empty_contracts_state(t: &TranslationContext) -> Markup {
    html! {
        div style="padding: 3rem; text-align: center; background: var(--gray-50); border-radius: 8px; border: 2px dashed var(--gray-300);" {
            div style="font-size: 3rem; margin-bottom: 1rem; opacity: 0.3;" {
                "🏒"
            }
            p style="color: var(--gray-600); font-size: 1.125rem; margin-bottom: 0.5rem; font-weight: 500;" {
                (t.messages.players_no_career_history())
            }
            p style="font-size: 0.875rem; color: var(--gray-500);" {
                (t.messages.players_no_career_history_help())
            }
        }
    }
}

/// Career statistics by event (competition-specific totals)
fn career_stats_by_event(
    t: &TranslationContext,
    player: &PlayerEntity,
    event_stats: &[PlayerEventStatsEntity],
) -> Markup {
    html! {
        @if !event_stats.is_empty() {
            div style="margin-top: 1.5rem;" {
                div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 1rem;" {
                    h3 style="font-size: 1.125rem; font-weight: 700;" {
                        (t.messages.players_career_stats_by_competition())
                    }
                    button
                        class="btn btn-sm btn-secondary"
                        hx-get=(format!("/players/{}/event-stats/new", player.id))
                        hx-target="#modal-container"
                        hx-swap="innerHTML"
                    {
                        (t.messages.players_add_event_stats())
                    }
                }

                div style="display: grid; gap: 1rem;" {
                    @for stats in event_stats {
                        div class="event-stats-card" {
                            div class="event-stats-header" {
                                h4 class="event-stats-title" {
                                    (&stats.event_name)
                                }
                                div class="event-stats-actions" {
                                    button
                                        class="btn btn-sm event-stats-btn"
                                        hx-get=(format!("/players/{}/event-stats/{}/edit", player.id, stats.id))
                                        hx-target="#modal-container"
                                        hx-swap="innerHTML"
                                    {
                                        (t.messages.players_event_stats_edit())
                                    }
                                }
                            }

                            div class="event-stats-grid" {
                                div class="event-stats-item" {
                                    div class="event-stats-label" {
                                        (t.messages.player_scoring_total_goals())
                                    }
                                    div class="event-stats-value" {
                                        (stats.goals_total)
                                    }
                                    @if stats.goals_total > 0 {
                                        div class="event-stats-detail" {
                                            "(" (stats.goals_identified) " " (t.messages.players_event_stats_identified()) ")"
                                        }
                                    }
                                }
                                div class="event-stats-item" {
                                    div class="event-stats-label" {
                                        (t.messages.player_scoring_total_assists())
                                    }
                                    div class="event-stats-value" {
                                        (stats.assists_total)
                                    }
                                    @if stats.assists_total > 0 {
                                        div class="event-stats-detail" {
                                            "(" (stats.assists_identified) " " (t.messages.players_event_stats_identified()) ")"
                                        }
                                    }
                                }
                                div class="event-stats-item" {
                                    div class="event-stats-label" {
                                        (t.messages.player_scoring_total_points())
                                    }
                                    div class="event-stats-value" {
                                        (stats.points_total)
                                    }
                                    @if stats.points_total > 0 {
                                        div class="event-stats-detail" {
                                            "(" (stats.points_identified) " " (t.messages.players_event_stats_identified()) ")"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } @else {
            div style="margin-top: 1.5rem; text-align: center;" {
                button
                    class="btn btn-secondary"
                    hx-get=(format!("/players/{}/event-stats/new", player.id))
                    hx-target="#modal-container"
                    hx-swap="innerHTML"
                {
                    (t.messages.players_add_career_stats())
                }
            }
        }
    }
}
