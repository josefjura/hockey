use maud::{html, Markup};

use crate::i18n::TranslationContext;
use crate::service::teams::{TeamDetailEntity, TeamEntity, TeamParticipationWithSeasonEntity};
use crate::views::components::confirm::{confirm_attrs, ConfirmVariant};

/// Team detail page with season participation management
pub fn team_detail_page(t: &TranslationContext, detail: &TeamDetailEntity) -> Markup {
    let team = &detail.team_info;

    html! {
        div class="card" {
            // Header with back button and action buttons
            div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 1.5rem;" {
                div style="display: flex; align-items: center; gap: 1rem;" {
                    a
                        href="/teams"
                        class="btn btn-secondary"
                    {
                        (format!("← {}", t.messages.teams_back_to_list()))
                    }
                    h1 style="font-size: 2rem; font-weight: 700; margin: 0;" {
                        (team.name)
                    }
                }
                div style="display: flex; gap: 0.5rem;" {
                    button
                        class="btn btn-primary"
                        hx-get=(format!("/teams/{}/edit", team.id))
                        hx-target="#modal-container"
                        hx-swap="innerHTML"
                    {
                        (t.messages.teams_edit())
                    }
                    button
                        class="btn btn-danger"
                        hx-post=(format!("/teams/{}/delete", team.id))
                        hx-confirm-custom=(confirm_attrs(
                            &t.messages.teams_delete().to_string(),
                            &t.messages.teams_confirm_delete().to_string(),
                            ConfirmVariant::Danger,
                            Some(&t.messages.common_delete().to_string()),
                            Some(&t.messages.common_cancel().to_string())
                        ))
                    {
                        (t.messages.teams_delete())
                    }
                }
            }

            // Team Info Card
            (team_info_card(t, team))

            // Participations Section
            div style="margin-top: 2rem;" {
                div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 1.5rem;" {
                    h2 style="font-size: 1.5rem; font-weight: 700; margin: 0;" {
                        "Season Participations"
                    }
                    button
                        class="btn btn-primary"
                        hx-get=(format!("/team-participations/new?team_id={}&return_to=/teams/{}", team.id, team.id))
                        hx-target="#modal-container"
                        hx-swap="innerHTML"
                    {
                        (t.messages.teams_add_to_season())
                    }
                }

                @if detail.participations.is_empty() {
                    (empty_participations_state(t))
                } @else {
                    (participations_list(t, &detail.participations))
                }
            }

            // Modal container
            div id="modal-container" {}
        }
    }
}

/// Team info card with country
fn team_info_card(t: &TranslationContext, team: &TeamEntity) -> Markup {
    html! {
        div style="padding: 1.5rem; background: var(--gray-50); border-radius: 8px;" {
            div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 1rem;" {
                div {
                    div style="color: var(--gray-600); font-size: 0.875rem; margin-bottom: 0.25rem;" {
                        (t.messages.teams_name())
                    }
                    div style="font-weight: 600;" {
                        (team.name)
                    }
                }
                @if let Some(country_name) = &team.country_name {
                    div {
                        div style="color: var(--gray-600); font-size: 0.875rem; margin-bottom: 0.25rem;" {
                            (t.messages.teams_country())
                        }
                        div style="display: flex; align-items: center; gap: 0.5rem; font-weight: 600;" {
                            @if let Some(iso2) = &team.country_iso2_code {
                                img
                                    src=(format!("https://flagcdn.com/w40/{}.png", iso2.to_lowercase()))
                                    alt=(country_name)
                                    style="width: 24px; height: 18px; object-fit: cover; border: 1px solid var(--gray-300); border-radius: 2px;"
                                    onerror="this.style.display='none'";
                            }
                            span { (country_name) }
                        }
                    }
                }
            }
        }
    }
}

/// Participations list in grid layout
fn participations_list(
    _t: &TranslationContext,
    participations: &[TeamParticipationWithSeasonEntity],
) -> Markup {
    html! {
        div style="display: grid; grid-template-columns: repeat(auto-fill, minmax(320px, 1fr)); gap: 1rem;" {
            @for participation in participations {
                div style="padding: 1rem; border: 1px solid var(--gray-200); border-radius: 8px; display: flex; flex-direction: column; gap: 1rem; transition: box-shadow 0.2s;"
                     onmouseover="this.style.boxShadow='0 2px 8px rgba(0,0,0,0.1)'"
                     onmouseout="this.style.boxShadow='none'"
                {
                    div style="flex: 1;" {
                        div style="font-weight: 600; font-size: 1rem; margin-bottom: 0.5rem;" {
                            (participation.event_name)
                        }
                        div style="color: var(--gray-600); font-size: 0.875rem; margin-bottom: 0.25rem;" {
                            @if let Some(display_name) = &participation.season_display_name {
                                (display_name)
                                " ("
                                (participation.season_year)
                                ")"
                            } @else {
                                "Season "
                                (participation.season_year)
                            }
                        }
                        div style="color: var(--gray-500); font-size: 0.875rem;" {
                            (participation.player_count)
                            " "
                            @if participation.player_count == 1 {
                                "player"
                            } @else {
                                "players"
                            }
                        }
                    }
                    div style="display: flex; gap: 0.5rem;" {
                        a
                            href=(format!("/team-participations/{}/roster", participation.id))
                            class="btn btn-sm btn-primary"
                            style="flex: 1;"
                        {
                            "Manage Roster"
                        }
                        a
                            href=(format!("/seasons/{}", participation.season_id))
                            class="btn btn-sm btn-secondary"
                            style="flex: 1;"
                        {
                            "View Season"
                        }
                    }
                }
            }
        }
    }
}

/// Empty state when no participations exist
fn empty_participations_state(t: &TranslationContext) -> Markup {
    html! {
        div style="padding: 3rem; text-align: center; background: var(--gray-50); border-radius: 8px; border: 2px dashed var(--gray-300);" {
            div style="font-size: 3rem; margin-bottom: 1rem; opacity: 0.3;" {
                "🏒"
            }
            p style="color: var(--gray-600); font-size: 1.125rem; margin-bottom: 0.5rem; font-weight: 500;" {
                (t.messages.teams_no_participations())
            }
            p style="font-size: 0.875rem; color: var(--gray-500);" {
                (t.messages.teams_no_participations_help())
            }
        }
    }
}