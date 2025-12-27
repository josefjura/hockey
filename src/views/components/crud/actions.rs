use maud::{html, Markup};

use crate::views::components::confirm::{confirm_attrs, ConfirmVariant};

/// Table actions (Edit/Delete buttons) with custom confirmation dialog
pub fn table_actions(
    edit_url: &str,
    delete_url: &str,
    table_id: &str,
    entity_label: &str,
) -> Markup {
    let confirm = confirm_attrs(
        &format!("Delete {}", entity_label),
        &format!(
            "Are you sure you want to delete this {}? This action cannot be undone.",
            entity_label.to_lowercase()
        ),
        ConfirmVariant::Danger,
        Some("Delete"),
        Some("Cancel"),
    );

    html! {
        td style="text-align: right;" {
            button
                class="btn btn-sm"
                hx-get=(edit_url)
                hx-target="#modal-container"
                hx-swap="innerHTML"
                style="margin-right: 0.5rem;"
            {
                "Edit"
            }
            button
                class="btn btn-sm btn-danger"
                hx-post=(delete_url)
                hx-target=(format!("#{}", table_id))
                hx-swap="outerHTML"
                hx-confirm-custom=(confirm)
            {
                "Delete"
            }
        }
    }
}

/// Table actions with i18n support and custom confirmation dialog
#[allow(dead_code)]
pub fn table_actions_i18n(
    edit_url: &str,
    delete_url: &str,
    table_id: &str,
    edit_label: &str,
    delete_label: &str,
    confirm_title: &str,
    confirm_message: &str,
) -> Markup {
    let confirm = confirm_attrs(
        confirm_title,
        confirm_message,
        ConfirmVariant::Danger,
        Some(delete_label),
        None,
    );

    html! {
        td style="text-align: right;" {
            button
                class="btn btn-sm"
                hx-get=(edit_url)
                hx-target="#modal-container"
                hx-swap="innerHTML"
                style="margin-right: 0.5rem;"
            {
                (edit_label)
            }
            button
                class="btn btn-sm btn-danger"
                hx-post=(delete_url)
                hx-target=(format!("#{}", table_id))
                hx-swap="outerHTML"
                hx-confirm-custom=(confirm)
            {
                (delete_label)
            }
        }
    }
}
