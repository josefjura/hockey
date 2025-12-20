use maud::{html, Markup};

use crate::i18n::TranslationContext;
use crate::service::player_contracts::{PlayerInRoster, TeamParticipationContext};
use crate::views::components::confirm::{confirm_attrs, ConfirmVariant};
use crate::views::components::crud::modal_form_i18n;

/// Main roster management page
pub fn roster_page(
    _t: &TranslationContext,
    context: &TeamParticipationContext,
    roster: &[PlayerInRoster],
) -> Markup {
    html! {
        div class="card" {
            // Header with back button and context
            div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 1.5rem;" {
                div style="display: flex; align-items: center; gap: 1rem;" {
                    a
                        href=(format!("/seasons/{}", context.season_id))
                        class="btn btn-secondary"
                    {
                        (format!("← {}", "Back to Season"))
                    }
                    h1 style="font-size: 2rem; font-weight: 700; margin: 0;" {
                        "Roster Management"
                    }
                }
            }

            // Context info card
            (context_card(context))

            // Roster section
            div style="margin-top: 2rem;" {
                div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 1.5rem;" {
                    h2 style="font-size: 1.5rem; font-weight: 700; margin: 0;" {
                        (format!("Players ({} total)", roster.len()))
                    }
                    button
                        class="btn btn-primary"
                        hx-get=(format!("/team-participations/{}/roster/add-player", context.team_participation_id))
                        hx-target="#modal-container"
                        hx-swap="innerHTML"
                    {
                        "+ Add Player"
                    }
                }

                @if roster.is_empty() {
                    (empty_roster_state())
                } @else {
                    (roster_table(roster))
                }
            }

            // Modal container
            div id="modal-container" {}
        }
    }
}

/// Context card showing team, event, and season info
fn context_card(context: &TeamParticipationContext) -> Markup {
    html! {
        div style="padding: 1.5rem; background: var(--gray-50); border-radius: 8px;" {
            div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 1rem;" {
                div {
                    div style="color: var(--gray-600); font-size: 0.875rem; margin-bottom: 0.25rem;" {
                        "Team"
                    }
                    div style="font-weight: 600; display: flex; align-items: center; gap: 0.5rem;" {
                        @if let Some(iso2) = &context.country_iso2_code {
                            img
                                src=(format!("https://flagcdn.com/w40/{}.png", iso2.to_lowercase()))
                                alt=(context.team_name)
                                style="width: 24px; height: 18px; object-fit: cover; border: 1px solid var(--gray-300); border-radius: 2px;"
                                onerror="this.style.display='none'";
                        }
                        (context.team_name)
                    }
                }
                div {
                    div style="color: var(--gray-600); font-size: 0.875rem; margin-bottom: 0.25rem;" {
                        "Event"
                    }
                    div style="font-weight: 600;" {
                        (context.event_name)
                    }
                }
                div {
                    div style="color: var(--gray-600); font-size: 0.875rem; margin-bottom: 0.25rem;" {
                        "Season"
                    }
                    div style="font-weight: 600;" {
                        @if let Some(display_name) = &context.season_display_name {
                            (display_name)
                        } @else {
                            (format!("{} Season", context.season_year))
                        }
                    }
                }
            }
        }
    }
}

/// Roster table showing all players
fn roster_table(roster: &[PlayerInRoster]) -> Markup {
    html! {
        table class="table" {
            thead {
                tr {
                    th { "Player" }
                    th { "Nationality" }
                    th style="text-align: right;" { "Actions" }
                }
            }
            tbody {
                @for player in roster {
                    tr {
                        td {
                            div style="display: flex; align-items: center; gap: 0.5rem;" {
                                @if let Some(_photo) = &player.photo_path {
                                    img
                                        src="https://via.placeholder.com/40"
                                        alt=(player.player_name)
                                        style="width: 40px; height: 40px; border-radius: 50%; object-fit: cover;"
                                        onerror="this.style.display='none'";
                                }
                                span style="font-weight: 500;" {
                                    (player.player_name)
                                }
                            }
                        }
                        td {
                            div style="display: flex; align-items: center; gap: 0.5rem;" {
                                img
                                    src=(format!("https://flagcdn.com/w40/{}.png", player.country_iso2_code.to_lowercase()))
                                    alt=(player.country_name)
                                    style="width: 24px; height: 18px; object-fit: cover; border: 1px solid var(--gray-300); border-radius: 2px;"
                                    onerror="this.style.display='none'";
                                (player.country_name)
                            }
                        }
                        td style="text-align: right;" {
                            button
                                class="btn btn-sm btn-danger"
                                hx-post=(format!("/player-contracts/{}/delete", player.player_contract_id))
                                hx-confirm-custom=(confirm_attrs(
                                    "Remove Player",
                                    &format!("Are you sure you want to remove {} from the roster?", player.player_name),
                                    ConfirmVariant::Danger,
                                    Some("Remove"),
                                    Some("Cancel")
                                ))
                            {
                                "Remove"
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Empty state when no players in roster
fn empty_roster_state() -> Markup {
    html! {
        div style="padding: 3rem; text-align: center; background: var(--gray-50); border-radius: 8px; border: 2px dashed var(--gray-300);" {
            div style="font-size: 3rem; margin-bottom: 1rem; opacity: 0.3;" {
                "👤"
            }
            p style="color: var(--gray-600); font-size: 1.125rem; margin-bottom: 0.5rem; font-weight: 500;" {
                "No players in roster"
            }
            p style="font-size: 0.875rem; color: var(--gray-500);" {
                "Click the 'Add Player' button to build your team roster"
            }
        }
    }
}

/// Modal form to add a player to the roster
pub fn add_player_modal(
    t: &TranslationContext,
    team_participation_id: i64,
    error: Option<&str>,
    available_players: &[(i64, String, String)],
) -> Markup {
    let form_fields = html! {
        div style="margin-bottom: 1.5rem;" {
            label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                "Select Player"
                span style="color: red; margin-left: 0.25rem;" { "*" }
            }

            @if available_players.is_empty() {
                div style="padding: 1rem; background: var(--gray-100); border-radius: 4px; color: var(--gray-600); text-align: center;" {
                    "No available players to add. All players are already in the roster."
                }
            } @else {
                select
                    name="player_id"
                    required
                    autofocus
                    style="width: 100%; padding: 0.75rem; border: 1px solid var(--gray-300); border-radius: 4px; font-size: 1rem;"
                {
                    option value="" { "-- Select a player --" }
                    @for (id, name, country) in available_players {
                        option value=(id) {
                            (format!("{} ({})", name, country))
                        }
                    }
                }
            }
        }
    };

    modal_form_i18n(
        "add-player-modal",
        "Add Player to Roster",
        error,
        &format!("/team-participations/{}/roster", team_participation_id),
        form_fields,
        "Add Player",
        &t.messages.common_cancel().to_string(),
    )
}
