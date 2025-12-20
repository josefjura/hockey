use maud::{html, Markup};

use crate::i18n::TranslationContext;
use crate::service::seasons::SeasonDetailEntity;
use crate::service::team_participations::TeamParticipationEntity;
use crate::views::components::confirm::{confirm_attrs, ConfirmVariant};
use crate::views::components::crud::modal_form_i18n;

/// Season detail page with team participation management
pub fn season_detail_page(t: &TranslationContext, detail: &SeasonDetailEntity) -> Markup {
    let season = &detail.season_info;

    html! {
        div class="card" {
            // Header with back button and action buttons
            div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 1.5rem;" {
                div style="display: flex; align-items: center; gap: 1rem;" {
                    a
                        href="/seasons"
                        class="btn btn-secondary"
                    {
                        (format!("← {}", t.messages.seasons_back_to_list()))
                    }
                    h1 style="font-size: 2rem; font-weight: 700; margin: 0;" {
                        @if let Some(display_name) = &season.display_name {
                            (display_name)
                        } @else {
                            (format!("{} Season", season.year))
                        }
                    }
                }
                div style="display: flex; gap: 0.5rem;" {
                    button
                        class="btn btn-primary"
                        hx-get=(format!("/seasons/{}/edit", season.id))
                        hx-target="#modal-container"
                        hx-swap="innerHTML"
                    {
                        (t.messages.seasons_edit())
                    }
                    button
                        class="btn btn-danger"
                        hx-post=(format!("/seasons/{}/delete", season.id))
                        hx-confirm-custom=(confirm_attrs(
                            &t.messages.seasons_delete().to_string(),
                            &t.messages.seasons_confirm_delete().to_string(),
                            ConfirmVariant::Danger,
                            Some(&t.messages.common_delete().to_string()),
                            Some(&t.messages.common_cancel().to_string())
                        ))
                    {
                        (t.messages.seasons_delete())
                    }
                }
            }

            // Season Info Card
            (season_info_card(t, season))

            // Participating Teams Section
            div style="margin-top: 2rem;" {
                div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 1.5rem;" {
                    h2 style="font-size: 1.5rem; font-weight: 700; margin: 0;" {
                        (t.messages.seasons_participating_teams())
                    }
                    button
                        class="btn btn-primary"
                        hx-get=(format!("/seasons/{}/teams/add", season.id))
                        hx-target="#modal-container"
                        hx-swap="innerHTML"
                    {
                        (format!("+ {}", t.messages.seasons_add_team()))
                    }
                }

                @if detail.participating_teams.is_empty() {
                    (empty_teams_state(t))
                } @else {
                    (teams_list(t, &detail.participating_teams))
                }
            }

            // Modal container
            div id="modal-container" {}
        }
    }
}

/// Season info card with event, year, and display name
fn season_info_card(
    t: &TranslationContext,
    season: &crate::service::seasons::SeasonEntity,
) -> Markup {
    html! {
        div style="padding: 1.5rem; background: var(--gray-50); border-radius: 8px;" {
            div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 1rem;" {
                div {
                    div style="color: var(--gray-600); font-size: 0.875rem; margin-bottom: 0.25rem;" {
                        (t.messages.seasons_event())
                    }
                    div style="font-weight: 600;" {
                        (season.event_name)
                    }
                }
                div {
                    div style="color: var(--gray-600); font-size: 0.875rem; margin-bottom: 0.25rem;" {
                        (t.messages.seasons_year())
                    }
                    div style="font-weight: 600;" {
                        (season.year)
                    }
                }
                @if let Some(display_name) = &season.display_name {
                    div {
                        div style="color: var(--gray-600); font-size: 0.875rem; margin-bottom: 0.25rem;" {
                            (t.messages.seasons_display_name())
                        }
                        div style="font-weight: 600;" {
                            (display_name)
                        }
                    }
                }
            }
        }
    }
}

/// Teams list in grid layout with flags and action buttons
fn teams_list(t: &TranslationContext, teams: &[TeamParticipationEntity]) -> Markup {
    html! {
        div style="display: grid; grid-template-columns: repeat(auto-fill, minmax(320px, 1fr)); gap: 1rem;" {
            @for team in teams {
                div style="padding: 1rem; border: 1px solid var(--gray-200); border-radius: 8px; display: flex; flex-direction: column; gap: 1rem; transition: box-shadow 0.2s;"
                     onmouseover="this.style.boxShadow='0 2px 8px rgba(0,0,0,0.1)'"
                     onmouseout="this.style.boxShadow='none'"
                {
                    div style="display: flex; align-items: center; gap: 0.75rem; flex: 1;" {
                        @if let Some(iso2) = &team.country_iso2_code {
                            img
                                src=(format!("https://flagcdn.com/w40/{}.png", iso2.to_lowercase()))
                                alt=(team.team_name)
                                style="width: 32px; height: 24px; object-fit: cover; border: 1px solid var(--gray-300); border-radius: 2px;"
                                onerror="this.style.display='none'";
                        }
                        span style="font-weight: 600; font-size: 1rem;" {
                            (team.team_name)
                        }
                    }
                    div style="display: flex; gap: 0.5rem;" {
                        a
                            href=(format!("/team-participations/{}/roster", team.id))
                            class="btn btn-sm btn-primary"
                            style="flex: 1;"
                        {
                            "Manage Roster"
                        }
                        button
                            class="btn btn-sm btn-danger"
                            hx-post=(format!("/team-participations/{}/delete", team.id))
                            hx-confirm-custom=(confirm_attrs(
                                &t.messages.seasons_remove_team().to_string(),
                                &format!("{} {} {}",
                                    t.messages.seasons_confirm_remove_team_1(),
                                    team.team_name,
                                    t.messages.seasons_confirm_remove_team_2()
                                ),
                                ConfirmVariant::Danger,
                                Some(&t.messages.common_remove().to_string()),
                                Some(&t.messages.common_cancel().to_string())
                            ))
                        {
                            (t.messages.common_remove())
                        }
                    }
                }
            }
        }
    }
}

/// Empty state when no teams are participating
fn empty_teams_state(t: &TranslationContext) -> Markup {
    html! {
        div style="padding: 3rem; text-align: center; background: var(--gray-50); border-radius: 8px; border: 2px dashed var(--gray-300);" {
            div style="font-size: 3rem; margin-bottom: 1rem; opacity: 0.3;" {
                "🏒"
            }
            p style="color: var(--gray-600); font-size: 1.125rem; margin-bottom: 0.5rem; font-weight: 500;" {
                (t.messages.seasons_no_teams())
            }
            p style="font-size: 0.875rem; color: var(--gray-500);" {
                (t.messages.seasons_no_teams_hint())
            }
        }
    }
}

/// Modal form to add a team to the season
pub fn add_team_modal(
    t: &TranslationContext,
    season_id: i64,
    error: Option<&str>,
    available_teams: &[(i64, String)],
) -> Markup {
    let form_fields = html! {
        div style="margin-bottom: 1.5rem;" {
            label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                (t.messages.seasons_select_team())
                span style="color: red; margin-left: 0.25rem;" { "*" }
            }

            @if available_teams.is_empty() {
                div style="padding: 1rem; background: var(--gray-100); border-radius: 4px; color: var(--gray-600); text-align: center;" {
                    (t.messages.seasons_no_available_teams())
                }
            } @else {
                select
                    name="team_id"
                    required
                    autofocus
                    style="width: 100%; padding: 0.75rem; border: 1px solid var(--gray-300); border-radius: 4px; font-size: 1rem;"
                {
                    option value="" { (format!("-- {} --", t.messages.seasons_select_team())) }
                    @for (id, name) in available_teams {
                        option value=(id) { (name) }
                    }
                }
            }
        }
    };

    modal_form_i18n(
        "add-team-modal",
        &t.messages.seasons_add_team_modal_title().to_string(),
        error,
        &format!("/seasons/{}/teams", season_id),
        form_fields,
        &t.messages.common_add().to_string(),
        &t.messages.common_cancel().to_string(),
    )
}
