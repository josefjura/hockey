use maud::{html, Markup};

use crate::i18n::TranslationContext;
use crate::service::players::{PlayerEntity, PlayerEventStatsEntity};
use crate::views::components::crud::modal_form;

/// Create modal for adding event-specific career stats
pub fn event_stats_create_modal(
    t: &TranslationContext,
    player: &PlayerEntity,
    events: &[(i64, String)],
    error: Option<&str>,
) -> Markup {
    let title = format!("Add Career Statistics - {}", player.name);

    let form_fields = html! {
        div style="margin-bottom: 1rem;" {
            label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                (t.messages.players_event_stats_competition())
                span style="color: red;" { " *" }
            }
            select
                name="event_id"
                required
                style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;"
            {
                option value="" { (t.messages.players_event_stats_select_competition()) }
                @for (id, name) in events {
                    option value=(id) { (name) }
                }
            }
            p style="font-size: 0.875rem; color: var(--gray-600); margin-top: 0.25rem;" {
                (t.messages.players_event_stats_select_help())
            }
        }

        div style="display: grid; grid-template-columns: 1fr 1fr; gap: 1rem; margin-bottom: 1rem;" {
            div {
                label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                    (t.messages.player_scoring_total_goals())
                }
                input
                    type="number"
                    name="goals_total"
                    value="0"
                    min="0"
                    required
                    style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;";
            }
            div {
                label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                    (t.messages.player_scoring_total_assists())
                }
                input
                    type="number"
                    name="assists_total"
                    value="0"
                    min="0"
                    required
                    style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;";
            }
        }

        p style="font-size: 0.875rem; color: var(--gray-600); margin-bottom: 1.5rem;" {
            (t.messages.players_event_stats_form_help())
        }
    };

    modal_form(
        "event-stats-modal",
        &title,
        error,
        &format!("/players/{}/event-stats", player.id),
        form_fields,
        &t.messages.players_event_stats_add_statistics(),
    )
}

/// Edit modal for updating event-specific career stats
pub fn event_stats_edit_modal(
    t: &TranslationContext,
    player: &PlayerEntity,
    stats: &PlayerEventStatsEntity,
    error: Option<&str>,
) -> Markup {
    let title = format!("Edit {} Career - {}", stats.event_name, player.name);

    let form_fields = html! {
        div style="margin-bottom: 1rem; padding: 1rem; background: var(--gray-50); border-radius: 4px;" {
            div style="font-weight: 600; margin-bottom: 0.5rem;" {
                (stats.event_name)
            }
            div style="font-size: 0.875rem; color: var(--gray-600);" {
                (t.messages.players_event_stats_currently())
                " "
                (stats.goals_identified) " " (t.messages.players_event_stats_goals())
                " "
                (stats.assists_identified) " " (t.messages.players_event_stats_assists())
            }
        }

        div style="display: grid; grid-template-columns: 1fr 1fr; gap: 1rem; margin-bottom: 1rem;" {
            div {
                label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                    (t.messages.player_scoring_total_goals())
                }
                input
                    type="number"
                    name="goals_total"
                    value=(stats.goals_total)
                    min="0"
                    required
                    style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;";
            }
            div {
                label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                    (t.messages.player_scoring_total_assists())
                }
                input
                    type="number"
                    name="assists_total"
                    value=(stats.assists_total)
                    min="0"
                    required
                    style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;";
            }
        }

        div style="margin-bottom: 1rem;" {
            button
                type="button"
                class="btn btn-danger"
                style="width: 100%;"
                hx-post=(format!("/players/{}/event-stats/{}/delete", player.id, stats.id))
                hx-target="#modal-container"
                hx-swap="innerHTML"
                hx-confirm=(t.messages.players_event_stats_confirm_delete())
            {
                (t.messages.players_event_stats_delete_statistics())
            }
        }
    };

    modal_form(
        "event-stats-edit-modal",
        &title,
        error,
        &format!("/players/{}/event-stats/{}", player.id, stats.id),
        form_fields,
        &t.messages.players_event_stats_save_changes(),
    )
}
