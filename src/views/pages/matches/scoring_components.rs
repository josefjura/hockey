use maud::{html, Markup};

use crate::i18n::TranslationContext;
use crate::service::matches::{MatchEntity, ScoreEventEntity};
use crate::views::components::crud::modal_form_i18n;

/// Create score event modal
pub fn score_event_create_modal(
    t: &TranslationContext,
    error: Option<&str>,
    match_info: &MatchEntity,
    home_players: &[(i64, String)],
    away_players: &[(i64, String)],
) -> Markup {
    let form_fields = html! {
        div style="margin-bottom: 1rem;" {
            label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                (t.messages.matches_filter_team())
                span style="color: red;" { "*" }
            }
            select
                name="team_id"
                required
                style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;"
            {
                option value=(match_info.home_team_id) { (match_info.home_team_name) " (Home)" }
                option value=(match_info.away_team_id) { (match_info.away_team_name) " (Away)" }
            }
        }

        div style="display: grid; grid-template-columns: 1fr 1fr 1fr; gap: 1rem; margin-bottom: 1rem;" {
            div {
                label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                    (t.messages.matches_period())
                    span style="color: red;" { "*" }
                }
                select
                    name="period"
                    required
                    style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;"
                {
                    option value="1" selected { "1st" }
                    option value="2" { "2nd" }
                    option value="3" { "3rd" }
                    option value="4" { (t.messages.matches_overtime()) }
                    option value="5" { (t.messages.matches_shootout()) }
                }
            }

            div {
                label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                    (t.messages.matches_minutes())
                }
                input
                    type="number"
                    name="time_minutes"
                    min="0"
                    max="60"
                    placeholder="0"
                    style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;";
            }

            div {
                label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                    (t.messages.matches_seconds())
                }
                input
                    type="number"
                    name="time_seconds"
                    min="0"
                    max="59"
                    placeholder="0"
                    style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;";
            }
        }

        div style="margin-bottom: 1rem;" {
            label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                (t.messages.matches_goal_type())
            }
            select
                name="goal_type"
                style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;"
            {
                option value="" { "---" }
                option value="even_strength" { (t.messages.matches_regular()) }
                option value="power_play" { (t.messages.matches_power_play()) }
                option value="short_handed" { (t.messages.matches_short_handed()) }
                option value="penalty_shot" { (t.messages.matches_penalty_shot()) }
                option value="empty_net" { (t.messages.matches_empty_net()) }
            }
        }

        div style="margin-bottom: 1rem;" {
            label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                (t.messages.matches_goal_scorer())
            }
            select
                name="scorer_id"
                style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;"
            {
                option value="" { "---" }
                optgroup label=(match_info.home_team_name) {
                    @for (id, name) in home_players {
                        option value=(id) { (name) }
                    }
                }
                optgroup label=(match_info.away_team_name) {
                    @for (id, name) in away_players {
                        option value=(id) { (name) }
                    }
                }
            }
        }

        div style="margin-bottom: 1rem;" {
            label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                (t.messages.matches_goal_assist_1())
            }
            select
                name="assist1_id"
                style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;"
            {
                option value="" { "---" }
                optgroup label=(match_info.home_team_name) {
                    @for (id, name) in home_players {
                        option value=(id) { (name) }
                    }
                }
                optgroup label=(match_info.away_team_name) {
                    @for (id, name) in away_players {
                        option value=(id) { (name) }
                    }
                }
            }
        }

        div style="margin-bottom: 1.5rem;" {
            label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                (t.messages.matches_goal_assist_2())
            }
            select
                name="assist2_id"
                style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;"
            {
                option value="" { "---" }
                optgroup label=(match_info.home_team_name) {
                    @for (id, name) in home_players {
                        option value=(id) { (name) }
                    }
                }
                optgroup label=(match_info.away_team_name) {
                    @for (id, name) in away_players {
                        option value=(id) { (name) }
                    }
                }
            }
        }
    };

    modal_form_i18n(
        "score-event-modal",
        &t.messages.matches_add_score_event().to_string(),
        error,
        &format!("/matches/{}/score-events", match_info.id),
        form_fields,
        &t.messages.common_save().to_string(),
        &t.messages.common_cancel().to_string(),
    )
}

/// Edit score event modal
pub fn score_event_edit_modal(
    t: &TranslationContext,
    error: Option<&str>,
    score_event: &ScoreEventEntity,
    match_info: &MatchEntity,
    home_players: &[(i64, String)],
    away_players: &[(i64, String)],
) -> Markup {
    let form_fields = html! {
        div style="margin-bottom: 1rem;" {
            label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                (t.messages.matches_filter_team())
                span style="color: red;" { "*" }
            }
            select
                name="team_id"
                required
                style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;"
            {
                option
                    value=(match_info.home_team_id)
                    selected[score_event.team_id == match_info.home_team_id]
                {
                    (match_info.home_team_name) " (Home)"
                }
                option
                    value=(match_info.away_team_id)
                    selected[score_event.team_id == match_info.away_team_id]
                {
                    (match_info.away_team_name) " (Away)"
                }
            }
        }

        div style="display: grid; grid-template-columns: 1fr 1fr 1fr; gap: 1rem; margin-bottom: 1rem;" {
            div {
                label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                    (t.messages.matches_period())
                    span style="color: red;" { "*" }
                }
                select
                    name="period"
                    required
                    style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;"
                {
                    option value="1" selected[score_event.period == 1] { "1st" }
                    option value="2" selected[score_event.period == 2] { "2nd" }
                    option value="3" selected[score_event.period == 3] { "3rd" }
                    option value="4" selected[score_event.period == 4] { (t.messages.matches_overtime()) }
                    option value="5" selected[score_event.period == 5] { (t.messages.matches_shootout()) }
                }
            }

            div {
                label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                    (t.messages.matches_minutes())
                }
                input
                    type="number"
                    name="time_minutes"
                    value=[score_event.time_minutes.as_ref()]
                    min="0"
                    max="60"
                    placeholder="0"
                    style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;";
            }

            div {
                label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                    (t.messages.matches_seconds())
                }
                input
                    type="number"
                    name="time_seconds"
                    value=[score_event.time_seconds.as_ref()]
                    min="0"
                    max="59"
                    placeholder="0"
                    style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;";
            }
        }

        div style="margin-bottom: 1rem;" {
            label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                (t.messages.matches_goal_type())
            }
            select
                name="goal_type"
                style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;"
            {
                option value="" selected[score_event.goal_type.is_none()] { "---" }
                option value="even_strength" selected[score_event.goal_type.as_deref() == Some("even_strength")] { (t.messages.matches_regular()) }
                option value="power_play" selected[score_event.goal_type.as_deref() == Some("power_play")] { (t.messages.matches_power_play()) }
                option value="short_handed" selected[score_event.goal_type.as_deref() == Some("short_handed")] { (t.messages.matches_short_handed()) }
                option value="penalty_shot" selected[score_event.goal_type.as_deref() == Some("penalty_shot")] { (t.messages.matches_penalty_shot()) }
                option value="empty_net" selected[score_event.goal_type.as_deref() == Some("empty_net")] { (t.messages.matches_empty_net()) }
            }
        }

        div style="margin-bottom: 1rem;" {
            label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                (t.messages.matches_goal_scorer())
            }
            select
                name="scorer_id"
                style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;"
            {
                option value="" selected[score_event.scorer_id.is_none()] { "---" }
                optgroup label=(match_info.home_team_name) {
                    @for (id, name) in home_players {
                        option value=(id) selected[score_event.scorer_id == Some(*id)] { (name) }
                    }
                }
                optgroup label=(match_info.away_team_name) {
                    @for (id, name) in away_players {
                        option value=(id) selected[score_event.scorer_id == Some(*id)] { (name) }
                    }
                }
            }
        }

        div style="margin-bottom: 1rem;" {
            label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                (t.messages.matches_goal_assist_1())
            }
            select
                name="assist1_id"
                style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;"
            {
                option value="" selected[score_event.assist1_id.is_none()] { "---" }
                optgroup label=(match_info.home_team_name) {
                    @for (id, name) in home_players {
                        option value=(id) selected[score_event.assist1_id == Some(*id)] { (name) }
                    }
                }
                optgroup label=(match_info.away_team_name) {
                    @for (id, name) in away_players {
                        option value=(id) selected[score_event.assist1_id == Some(*id)] { (name) }
                    }
                }
            }
        }

        div style="margin-bottom: 1.5rem;" {
            label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                (t.messages.matches_goal_assist_2())
            }
            select
                name="assist2_id"
                style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;"
            {
                option value="" selected[score_event.assist2_id.is_none()] { "---" }
                optgroup label=(match_info.home_team_name) {
                    @for (id, name) in home_players {
                        option value=(id) selected[score_event.assist2_id == Some(*id)] { (name) }
                    }
                }
                optgroup label=(match_info.away_team_name) {
                    @for (id, name) in away_players {
                        option value=(id) selected[score_event.assist2_id == Some(*id)] { (name) }
                    }
                }
            }
        }
    };

    modal_form_i18n(
        "score-event-modal",
        &t.messages.matches_edit_score_event().to_string(),
        error,
        &format!("/matches/score-events/{}", score_event.id),
        form_fields,
        &t.messages.common_save().to_string(),
        &t.messages.common_cancel().to_string(),
    )
}
