use maud::{html, Markup};

use crate::i18n::TranslationContext;
use crate::service::matches::MatchEntity;
use crate::views::components::crud::modal_form_i18n;
use crate::views::components::loading::htmx_loading_indicator;

/// Create match modal
pub fn match_create_modal(
    t: &TranslationContext,
    error: Option<&str>,
    seasons: &[(i64, String)],
    teams: &[(i64, String)],
) -> Markup {
    let form_fields = html! {
        div class="form-row" style="margin-bottom: 1rem;" {
            div class="form-group" {
                label class="form-label" {
                    (t.messages.matches_season())
                    span class="required-indicator" { " *" }
                }
                select
                    name="season_id"
                    required
                    hx-get="/matches/teams-for-season"
                    hx-swap="none"
                    hx-trigger="change"
                    hx-indicator="#season-loading-create"
                {
                    option value="" { (t.messages.matches_select_season()) }
                    @for (id, name) in seasons {
                        option value=(id) { (name) }
                    }
                }
                (htmx_loading_indicator("season-loading-create", Some(&t.messages.matches_loading_teams().to_string())))
            }

            div class="form-group" {
                label class="form-label" {
                    (t.messages.matches_status())
                    span class="required-indicator" { " *" }
                }
                select name="status" required {
                    option value="scheduled" selected { (t.messages.matches_status_scheduled()) }
                    option value="in_progress" { (t.messages.matches_status_in_progress()) }
                    option value="finished" { (t.messages.matches_status_finished()) }
                    option value="cancelled" { (t.messages.matches_status_cancelled()) }
                }
            }
        }

        div class="form-row" style="margin-bottom: 1rem;" {
            div class="form-group" {
                label class="form-label" {
                    (t.messages.matches_home_team())
                    span class="required-indicator" { " *" }
                }
                select
                    name="home_team_id"
                    id="home_team_id"
                    class="team-select"
                    required
                {
                    option value="" { (t.messages.matches_select_team()) }
                    @for (id, name) in teams {
                        option value=(id) { (name) }
                    }
                }
            }

            div class="form-group" {
                label class="form-label" {
                    (t.messages.matches_away_team())
                    span class="required-indicator" { " *" }
                }
                select
                    name="away_team_id"
                    id="away_team_id"
                    class="team-select"
                    required
                {
                    option value="" { (t.messages.matches_select_team()) }
                    @for (id, name) in teams {
                        option value=(id) { (name) }
                    }
                }
            }
        }

        div class="form-row" style="margin-bottom: 1rem;" {
            div class="form-group" {
                label class="form-label" {
                    (t.messages.matches_score()) " (Home)"
                }
                input
                    type="number"
                    name="home_score_unidentified"
                    value="0"
                    min="0";
            }

            div class="form-group" {
                label class="form-label" {
                    (t.messages.matches_score()) " (Away)"
                }
                input
                    type="number"
                    name="away_score_unidentified"
                    value="0"
                    min="0";
            }
        }

        div class="form-group" {
            label class="form-label" {
                (t.messages.matches_date())
            }
            input
                type="datetime-local"
                name="match_date";
        }

        div class="form-group" {
            label class="form-label" {
                (t.messages.matches_location())
            }
            input
                type="text"
                name="venue"
                placeholder=(t.messages.matches_location_placeholder());
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
        div class="form-row" style="margin-bottom: 1rem;" {
            div class="form-group" {
                label class="form-label" {
                    (t.messages.matches_season())
                    span class="required-indicator" { " *" }
                }
                select
                    name="season_id"
                    required
                    hx-get="/matches/teams-for-season"
                    hx-swap="none"
                    hx-trigger="change"
                    hx-indicator="#season-loading-edit"
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
                (htmx_loading_indicator("season-loading-edit", Some(&t.messages.matches_loading_teams().to_string())))
            }

            div class="form-group" {
                label class="form-label" {
                    (t.messages.matches_status())
                    span class="required-indicator" { " *" }
                }
                select name="status" required {
                    option value="scheduled" selected[match_entity.status == "scheduled"] { (t.messages.matches_status_scheduled()) }
                    option value="in_progress" selected[match_entity.status == "in_progress"] { (t.messages.matches_status_in_progress()) }
                    option value="finished" selected[match_entity.status == "finished"] { (t.messages.matches_status_finished()) }
                    option value="cancelled" selected[match_entity.status == "cancelled"] { (t.messages.matches_status_cancelled()) }
                }
            }
        }

        div class="form-row" style="margin-bottom: 1rem;" {
            div class="form-group" {
                label class="form-label" {
                    (t.messages.matches_home_team())
                    span class="required-indicator" { " *" }
                }
                select
                    name="home_team_id"
                    id="edit_home_team_id"
                    class="edit-team-select"
                    required
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

            div class="form-group" {
                label class="form-label" {
                    (t.messages.matches_away_team())
                    span class="required-indicator" { " *" }
                }
                select
                    name="away_team_id"
                    id="edit_away_team_id"
                    class="edit-team-select"
                    required
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

        div class="form-row" style="margin-bottom: 1rem;" {
            div class="form-group" {
                label class="form-label" {
                    (t.messages.matches_score()) " (Home)"
                }
                input
                    type="number"
                    name="home_score_unidentified"
                    value=(match_entity.home_score_unidentified)
                    min="0";
            }

            div class="form-group" {
                label class="form-label" {
                    (t.messages.matches_score()) " (Away)"
                }
                input
                    type="number"
                    name="away_score_unidentified"
                    value=(match_entity.away_score_unidentified)
                    min="0";
            }
        }

        div class="form-group" {
            label class="form-label" {
                (t.messages.matches_date())
            }
            input
                type="datetime-local"
                name="match_date"
                value=[match_entity.match_date.as_ref()];
        }

        div class="form-group" {
            label class="form-label" {
                (t.messages.matches_location())
            }
            input
                type="text"
                name="venue"
                value=[match_entity.venue.as_ref()]
                placeholder=(t.messages.matches_location_placeholder());
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
