use maud::{html, Markup};

use crate::i18n::TranslationContext;
use crate::service::players::{PlayerEntity, PropertyChangeEntity};
use crate::views::components::crud::modal_form;

/// Create modal for adding property change
pub fn property_change_create_modal(
    t: &TranslationContext,
    player: &PlayerEntity,
    seasons: &[(i64, String)],
    error: Option<&str>,
) -> Markup {
    let title = format!(
        "{} - {}",
        t.messages.player_property_change_add(),
        player.name
    );

    let form_fields = html! {
        div class="form-group" {
            label {
                (t.messages.player_property_change_date())
                span style="color: red;" { " *" }
            }
            input
                type="date"
                name="change_date"
                required;
            p style="font-size: 0.875rem; color: var(--gray-600); margin-top: 0.25rem;" {
                (t.messages.player_property_change_date_help())
            }
        }

        div class="form-group" {
            label {
                (t.messages.player_property_change_type())
                span style="color: red;" { " *" }
            }
            select name="property_type" required {
                option value="" { (t.messages.player_property_change_select_type()) }
                option value="Position" { (t.messages.player_property_change_type_position()) }
                option value="Trade" { (t.messages.player_property_change_type_trade()) }
                option value="Role" { (t.messages.player_property_change_type_role()) }
                option value="JerseyNumber" { (t.messages.player_property_change_type_jersey()) }
                option value="Status" { (t.messages.player_property_change_type_status()) }
                option value="Retirement" { (t.messages.player_property_change_type_retirement()) }
                option value="Other" { (t.messages.player_property_change_type_other()) }
            }
        }

        div style="display: grid; grid-template-columns: 1fr 1fr; gap: 1rem; margin-bottom: 1rem;" {
            div class="form-group" {
                label { (t.messages.player_property_change_old_value()) }
                input
                    type="text"
                    name="old_value"
                    placeholder=(t.messages.player_property_change_old_value_placeholder());
            }
            div class="form-group" {
                label { (t.messages.player_property_change_new_value()) }
                input
                    type="text"
                    name="new_value"
                    placeholder=(t.messages.player_property_change_new_value_placeholder());
            }
        }

        div class="form-group" {
            label {
                (t.messages.player_property_change_description())
                span style="color: red;" { " *" }
            }
            textarea
                name="description"
                rows="3"
                required
                placeholder=(t.messages.player_property_change_description_placeholder());
        }

        div class="form-group" {
            label { (t.messages.player_property_change_season()) }
            select name="season_id" {
                option value="" { (t.messages.player_property_change_no_season()) }
                @for (id, name) in seasons {
                    option value=(id) { (name) }
                }
            }
            p style="font-size: 0.875rem; color: var(--gray-600); margin-top: 0.25rem;" {
                (t.messages.player_property_change_season_help())
            }
        }
    };

    modal_form(
        "property-change-modal",
        &title,
        error,
        &format!("/players/{}/property-changes", player.id),
        form_fields,
        &t.messages.common_create(),
    )
}

/// Edit modal for updating property change
pub fn property_change_edit_modal(
    t: &TranslationContext,
    player: &PlayerEntity,
    change: &PropertyChangeEntity,
    seasons: &[(i64, String)],
    error: Option<&str>,
) -> Markup {
    let title = format!(
        "{} - {}",
        t.messages.player_property_change_edit(),
        player.name
    );

    let form_fields = html! {
        div class="form-group" {
            label {
                (t.messages.player_property_change_date())
                span style="color: red;" { " *" }
            }
            input
                type="date"
                name="change_date"
                value=(change.change_date)
                required;
        }

        div class="form-group" {
            label {
                (t.messages.player_property_change_type())
                span style="color: red;" { " *" }
            }
            select name="property_type" required {
                option value="Position" selected[change.property_type == "Position"] {
                    (t.messages.player_property_change_type_position())
                }
                option value="Trade" selected[change.property_type == "Trade"] {
                    (t.messages.player_property_change_type_trade())
                }
                option value="Role" selected[change.property_type == "Role"] {
                    (t.messages.player_property_change_type_role())
                }
                option value="JerseyNumber" selected[change.property_type == "JerseyNumber"] {
                    (t.messages.player_property_change_type_jersey())
                }
                option value="Status" selected[change.property_type == "Status"] {
                    (t.messages.player_property_change_type_status())
                }
                option value="Retirement" selected[change.property_type == "Retirement"] {
                    (t.messages.player_property_change_type_retirement())
                }
                option value="Other" selected[change.property_type == "Other"] {
                    (t.messages.player_property_change_type_other())
                }
            }
        }

        div style="display: grid; grid-template-columns: 1fr 1fr; gap: 1rem; margin-bottom: 1rem;" {
            div class="form-group" {
                label { (t.messages.player_property_change_old_value()) }
                input
                    type="text"
                    name="old_value"
                    value=[change.old_value.as_ref()]
                    placeholder=(t.messages.player_property_change_old_value_placeholder());
            }
            div class="form-group" {
                label { (t.messages.player_property_change_new_value()) }
                input
                    type="text"
                    name="new_value"
                    value=[change.new_value.as_ref()]
                    placeholder=(t.messages.player_property_change_new_value_placeholder());
            }
        }

        div class="form-group" {
            label {
                (t.messages.player_property_change_description())
                span style="color: red;" { " *" }
            }
            textarea
                name="description"
                rows="3"
                required
                placeholder=(t.messages.player_property_change_description_placeholder())
            {
                (change.description)
            }
        }

        div class="form-group" {
            label { (t.messages.player_property_change_season()) }
            select name="season_id" {
                option value="" selected[change.season_id.is_none()] {
                    (t.messages.player_property_change_no_season())
                }
                @for (id, name) in seasons {
                    option value=(id) selected[change.season_id == Some(*id)] { (name) }
                }
            }
        }

        div style="margin-top: 1.5rem; padding-top: 1rem; border-top: 1px solid var(--gray-200);" {
            button
                type="button"
                class="btn btn-danger"
                style="width: 100%;"
                hx-post=(format!("/players/{}/property-changes/{}/delete", player.id, change.id))
                hx-target="#modal-container"
                hx-swap="innerHTML"
                hx-confirm=(t.messages.player_property_change_confirm_delete())
            {
                (t.messages.player_property_change_delete())
            }
        }
    };

    modal_form(
        "property-change-edit-modal",
        &title,
        error,
        &format!("/players/{}/property-changes/{}", player.id, change.id),
        form_fields,
        &t.messages.common_save(),
    )
}
