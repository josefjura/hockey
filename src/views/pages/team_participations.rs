use maud::{html, Markup};

use crate::i18n::TranslationContext;
use crate::views::components::crud::modal_form;

/// Create team participation modal
///
/// # Arguments
/// - `t`: Translation context
/// - `error`: Optional error message
/// - `prefill_team_id`: Optional team ID to preselect
/// - `prefill_season_id`: Optional season ID to preselect
/// - `return_to`: Optional return URL after creation
/// - `available_teams`: List of (id, name) for team dropdown
/// - `available_seasons`: List of (id, display_name) for season dropdown
pub fn team_participation_create_modal(
    t: &TranslationContext,
    error: Option<&str>,
    prefill_team_id: Option<i64>,
    prefill_season_id: Option<i64>,
    return_to: Option<&str>,
    available_teams: &[(i64, String)],
    available_seasons: &[(i64, String)],
) -> Markup {
    let form_fields = html! {
        // Team selection
        div style="margin-bottom: 1.5rem;" {
            label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                (t.messages.label_team())
                span style="color: #dc2626;" { " *" }
            }
            select
                name="team_id"
                required
                style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;"
                disabled[prefill_team_id.is_some()]
            {
                @if prefill_team_id.is_none() {
                    option value="" disabled selected {
                        (t.messages.placeholder_select_team())
                    }
                }
                @for (id, name) in available_teams {
                    option
                        value=(id)
                        selected[prefill_team_id == Some(*id)]
                    {
                        (name)
                    }
                }
            }
        }

        // Season selection
        div style="margin-bottom: 1.5rem;" {
            label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                (t.messages.label_season())
                span style="color: #dc2626;" { " *" }
            }
            select
                name="season_id"
                required
                style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;"
                disabled[prefill_season_id.is_some()]
            {
                @if prefill_season_id.is_none() {
                    option value="" disabled selected {
                        (t.messages.placeholder_select_season())
                    }
                }
                @for (id, name) in available_seasons {
                    option
                        value=(id)
                        selected[prefill_season_id == Some(*id)]
                    {
                        (name)
                    }
                }
            }
        }

        // Hidden return_to field
        @if let Some(url) = return_to {
            input type="hidden" name="return_to" value=(url);
        }
    };

    modal_form(
        "team-participation-create-modal",
        &t.messages.title_add_team_to_season().to_string(),
        error,
        "/team-participations",
        form_fields,
        &t.messages.button_add_to_season().to_string(),
    )
}
