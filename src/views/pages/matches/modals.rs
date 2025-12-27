use maud::{html, Markup};

use crate::i18n::TranslationContext;
use crate::service::matches::MatchEntity;
use crate::views::components::crud::modal_form_i18n;

/// Create match modal
pub fn match_create_modal(
    t: &TranslationContext,
    error: Option<&str>,
    seasons: &[(i64, String)],
    teams: &[(i64, String)],
) -> Markup {
    let form_fields = html! {
        div style="display: grid; grid-template-columns: 1fr 1fr; gap: 1rem; margin-bottom: 1rem;" {
            div {
                label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                    (t.messages.matches_season())
                    span style="color: red;" { "*" }
                }
                select
                    name="season_id"
                    required
                    hx-get="/matches/teams-for-season"
                    hx-swap="none"
                    hx-trigger="change"
                    style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;"
                {
                    option value="" { (t.messages.matches_select_season()) }
                    @for (id, name) in seasons {
                        option value=(id) { (name) }
                    }
                }
            }

            div {
                label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                    (t.messages.matches_status())
                    span style="color: red;" { "*" }
                }
                select
                    name="status"
                    required
                    style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;"
                {
                    option value="scheduled" selected { (t.messages.matches_status_scheduled()) }
                    option value="in_progress" { (t.messages.matches_status_in_progress()) }
                    option value="finished" { (t.messages.matches_status_finished()) }
                    option value="cancelled" { (t.messages.matches_status_cancelled()) }
                }
            }
        }

        div style="display: grid; grid-template-columns: 1fr 1fr; gap: 1rem; margin-bottom: 1rem;" {
            div {
                label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                    (t.messages.matches_home_team())
                    span style="color: red;" { "*" }
                }
                select
                    name="home_team_id"
                    id="home_team_id"
                    class="team-select"
                    required
                    style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;"
                {
                    option value="" { (t.messages.matches_select_team()) }
                    @for (id, name) in teams {
                        option value=(id) { (name) }
                    }
                }
            }

            div {
                label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                    (t.messages.matches_away_team())
                    span style="color: red;" { "*" }
                }
                select
                    name="away_team_id"
                    id="away_team_id"
                    class="team-select"
                    required
                    style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;"
                {
                    option value="" { (t.messages.matches_select_team()) }
                    @for (id, name) in teams {
                        option value=(id) { (name) }
                    }
                }
            }
        }

        div style="display: grid; grid-template-columns: 1fr 1fr; gap: 1rem; margin-bottom: 1rem;" {
            div {
                label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                    (t.messages.matches_score()) " (Home)"
                }
                input
                    type="number"
                    name="home_score_unidentified"
                    value="0"
                    min="0"
                    style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;";
            }

            div {
                label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                    (t.messages.matches_score()) " (Away)"
                }
                input
                    type="number"
                    name="away_score_unidentified"
                    value="0"
                    min="0"
                    style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;";
            }
        }

        div style="margin-bottom: 1rem;" {
            label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                (t.messages.matches_date())
            }
            input
                type="datetime-local"
                name="match_date"
                style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;";
        }

        div style="margin-bottom: 1.5rem;" {
            label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                (t.messages.matches_location())
            }
            input
                type="text"
                name="venue"
                placeholder=(t.messages.matches_location_placeholder())
                style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;";
        }
    };

    modal_form_i18n(
        "match-modal",
        &t.messages.matches_create_title().to_string(),
        error,
        "/matches",
        form_fields,
        &t.messages.matches_create_submit().to_string(),
        &t.messages.common_cancel().to_string(),
    )
}

/// Edit match modal
pub fn match_edit_modal(
    t: &TranslationContext,
    match_entity: &MatchEntity,
    error: Option<&str>,
    seasons: &[(i64, String)],
    teams: &[(i64, String)],
) -> Markup {
    let form_fields = html! {
        div style="display: grid; grid-template-columns: 1fr 1fr; gap: 1rem; margin-bottom: 1rem;" {
            div {
                label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                    (t.messages.matches_season())
                    span style="color: red;" { "*" }
                }
                select
                    name="season_id"
                    required
                    hx-get="/matches/teams-for-season"
                    hx-swap="none"
                    hx-trigger="change"
                    style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;"
                {
                    @for (id, name) in seasons {
                        option
                            value=(id)
                            selected[*id == match_entity.season_id]
                        {
                            (name)
                        }
                    }
                }
            }

            div {
                label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                    (t.messages.matches_status())
                    span style="color: red;" { "*" }
                }
                select
                    name="status"
                    required
                    style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;"
                {
                    option value="scheduled" selected[match_entity.status == "scheduled"] { (t.messages.matches_status_scheduled()) }
                    option value="in_progress" selected[match_entity.status == "in_progress"] { (t.messages.matches_status_in_progress()) }
                    option value="finished" selected[match_entity.status == "finished"] { (t.messages.matches_status_finished()) }
                    option value="cancelled" selected[match_entity.status == "cancelled"] { (t.messages.matches_status_cancelled()) }
                }
            }
        }

        div style="display: grid; grid-template-columns: 1fr 1fr; gap: 1rem; margin-bottom: 1rem;" {
            div {
                label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                    (t.messages.matches_home_team())
                    span style="color: red;" { "*" }
                }
                select
                    name="home_team_id"
                    id="edit_home_team_id"
                    class="edit-team-select"
                    required
                    style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;"
                {
                    @for (id, name) in teams {
                        option
                            value=(id)
                            selected[*id == match_entity.home_team_id]
                        {
                            (name)
                        }
                    }
                }
            }

            div {
                label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                    (t.messages.matches_away_team())
                    span style="color: red;" { "*" }
                }
                select
                    name="away_team_id"
                    id="edit_away_team_id"
                    class="edit-team-select"
                    required
                    style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;"
                {
                    @for (id, name) in teams {
                        option
                            value=(id)
                            selected[*id == match_entity.away_team_id]
                        {
                            (name)
                        }
                    }
                }
            }
        }

        div style="display: grid; grid-template-columns: 1fr 1fr; gap: 1rem; margin-bottom: 1rem;" {
            div {
                label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                    (t.messages.matches_score()) " (Home)"
                }
                input
                    type="number"
                    name="home_score_unidentified"
                    value=(match_entity.home_score_unidentified)
                    min="0"
                    style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;";
            }

            div {
                label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                    (t.messages.matches_score()) " (Away)"
                }
                input
                    type="number"
                    name="away_score_unidentified"
                    value=(match_entity.away_score_unidentified)
                    min="0"
                    style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;";
            }
        }

        div style="margin-bottom: 1rem;" {
            label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                (t.messages.matches_date())
            }
            input
                type="datetime-local"
                name="match_date"
                value=[match_entity.match_date.as_ref()]
                style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;";
        }

        div style="margin-bottom: 1.5rem;" {
            label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                (t.messages.matches_location())
            }
            input
                type="text"
                name="venue"
                value=[match_entity.venue.as_ref()]
                placeholder=(t.messages.matches_location_placeholder())
                style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;";
        }
    };

    modal_form_i18n(
        "match-modal",
        &t.messages.matches_edit_title().to_string(),
        error,
        &format!("/matches/{}", match_entity.id),
        form_fields,
        &t.messages.matches_edit_submit().to_string(),
        &t.messages.common_cancel().to_string(),
    )
}
