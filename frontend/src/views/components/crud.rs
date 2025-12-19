use maud::{html, Markup, PreEscaped};

// Re-export table components for convenience
pub use crate::views::components::table::pagination;

/// Page header with title, description, and create button
pub fn page_header(title: &str, description: &str, create_url: &str, create_label: &str) -> Markup {
    html! {
        div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 1.5rem;" {
            div {
                h1 style="font-size: 2rem; font-weight: 700; margin-bottom: 0.5rem;" {
                    (title)
                }
                p style="color: var(--gray-600);" {
                    (description)
                }
            }
            button
                class="btn btn-primary"
                hx-get=(create_url)
                hx-target="#modal-container"
                hx-swap="innerHTML"
            {
                (create_label)
            }
        }
    }
}

/// Empty state when no items are found
pub fn empty_state(entity_name: &str, has_filters: bool) -> Markup {
    html! {
        div style="padding: 3rem; text-align: center; color: var(--gray-500);" {
            h3 style="font-size: 1.25rem; font-weight: 600; margin-bottom: 0.5rem;" {
                "No " (entity_name) " found"
            }
            p {
                @if has_filters {
                    "No " (entity_name) " match your search criteria. Try adjusting your filters."
                } @else {
                    "No " (entity_name) " have been added yet."
                }
            }
        }
    }
}

/// Modal wrapper with backdrop and form structure
///
/// # Parameters
/// - `modal_id`: Unique ID for this modal (e.g., "player-modal", "season-modal")
/// - `title`: Modal title
/// - `error`: Optional error message to display
/// - `form_action`: POST URL for form submission
/// - `form_fields`: The form fields markup
/// - `submit_label`: Label for the submit button
pub fn modal_form(
    modal_id: &str,
    title: &str,
    error: Option<&str>,
    form_action: &str,
    form_fields: Markup,
    submit_label: &str,
) -> Markup {
    html! {
        div
            class="modal-backdrop"
            style="position: fixed; top: 0; left: 0; right: 0; bottom: 0; background: rgba(0, 0, 0, 0.5); display: flex; align-items: center; justify-content: center; z-index: 1000;"
            id=(modal_id)
        {
            div
                class="modal"
                style="background: white; border-radius: 12px; padding: 2rem; max-width: 500px; width: 90%; max-height: 90vh; overflow-y: auto;"
                onclick="event.stopPropagation()"
            {
                h2 style="margin-bottom: 1.5rem; font-size: 1.5rem; font-weight: 700;" {
                    (title)
                }

                @if let Some(error_msg) = error {
                    div class="error" style="margin-bottom: 1rem; padding: 0.75rem; background: #fee; border: 1px solid #fcc; border-radius: 4px; color: #c00;" {
                        (error_msg)
                    }
                }

                form hx-post=(form_action) hx-target=(format!("#{}", modal_id)) hx-swap="outerHTML" {
                    (form_fields)

                    div style="display: flex; gap: 0.5rem; justify-content: flex-end;" {
                        button
                            type="button"
                            class="btn"
                            style="background: white; border: 1px solid var(--gray-300);"
                            onclick=(format!("document.getElementById('{}').remove()", modal_id))
                        {
                            "Cancel"
                        }
                        button type="submit" class="btn btn-primary" {
                            (submit_label)
                        }
                    }
                }
            }
        }
        (PreEscaped(format!(r#"
        <script>
            document.getElementById('{}').addEventListener('click', function(e) {{
                if (e.target === this) {{
                    this.remove();
                }}
            }});
        </script>
        "#, modal_id)))
    }
}

/// Modal wrapper with backdrop and form structure (multipart/form-data encoding)
///
/// Same as `modal_form` but with multipart/form-data encoding for file uploads
///
/// # Parameters
/// - `modal_id`: Unique ID for this modal (e.g., "player-modal", "season-modal")
/// - `title`: Modal title
/// - `error`: Optional error message to display
/// - `form_action`: POST URL for form submission
/// - `form_fields`: The form fields markup
/// - `submit_label`: Label for the submit button
pub fn modal_form_multipart(
    modal_id: &str,
    title: &str,
    error: Option<&str>,
    form_action: &str,
    form_fields: Markup,
    submit_label: &str,
) -> Markup {
    html! {
        div
            class="modal-backdrop"
            style="position: fixed; top: 0; left: 0; right: 0; bottom: 0; background: rgba(0, 0, 0, 0.5); display: flex; align-items: center; justify-content: center; z-index: 1000;"
            id=(modal_id)
        {
            div
                class="modal"
                style="background: white; border-radius: 12px; padding: 2rem; max-width: 500px; width: 90%; max-height: 90vh; overflow-y: auto;"
                onclick="event.stopPropagation()"
            {
                h2 style="margin-bottom: 1.5rem; font-size: 1.5rem; font-weight: 700;" {
                    (title)
                }

                @if let Some(error_msg) = error {
                    div class="error" style="margin-bottom: 1rem; padding: 0.75rem; background: #fee; border: 1px solid #fcc; border-radius: 4px; color: #c00;" {
                        (error_msg)
                    }
                }

                form
                    hx-post=(form_action)
                    hx-target=(format!("#{}", modal_id))
                    hx-swap="outerHTML"
                    hx-encoding="multipart/form-data"
                {
                    (form_fields)

                    div style="display: flex; gap: 0.5rem; justify-content: flex-end;" {
                        button
                            type="button"
                            class="btn"
                            style="background: white; border: 1px solid var(--gray-300);"
                            onclick=(format!("document.getElementById('{}').remove()", modal_id))
                        {
                            "Cancel"
                        }
                        button type="submit" class="btn btn-primary" {
                            (submit_label)
                        }
                    }
                }
            }
        }
        (PreEscaped(format!(r#"
        <script>
            document.getElementById('{}').addEventListener('click', function(e) {{
                if (e.target === this) {{
                    this.remove();
                }}
            }});
        </script>
        "#, modal_id)))
    }
}

/// Table actions (Edit/Delete buttons)
pub fn table_actions(
    edit_url: &str,
    delete_url: &str,
    table_id: &str,
    entity_label: &str,
) -> Markup {
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
                hx-confirm=(format!("Are you sure you want to delete this {}?", entity_label))
            {
                "Delete"
            }
        }
    }
}

// ============================================
// i18n-enabled versions of CRUD components
// ============================================

/// Page header with i18n support
pub fn page_header_i18n(
    title: &str,
    description: &str,
    create_url: &str,
    create_label: &str,
) -> Markup {
    html! {
        div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 1.5rem;" {
            div {
                h1 style="font-size: 2rem; font-weight: 700; margin-bottom: 0.5rem;" {
                    (title)
                }
                p style="color: var(--gray-600);" {
                    (description)
                }
            }
            button
                class="btn btn-primary"
                hx-get=(create_url)
                hx-target="#modal-container"
                hx-swap="innerHTML"
            {
                (create_label)
            }
        }
    }
}

/// Empty state with i18n support
pub fn empty_state_i18n(title: &str, message: &str, _has_filters: bool) -> Markup {
    html! {
        div style="padding: 3rem; text-align: center; color: var(--gray-500);" {
            h3 style="font-size: 1.25rem; font-weight: 600; margin-bottom: 0.5rem;" {
                (title)
            }
            p {
                (message)
            }
        }
    }
}

/// Table actions with i18n support
pub fn table_actions_i18n(
    edit_url: &str,
    delete_url: &str,
    table_id: &str,
    edit_label: &str,
    delete_label: &str,
) -> Markup {
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
            {
                (delete_label)
            }
        }
    }
}

/// Modal form with i18n support
pub fn modal_form_i18n(
    modal_id: &str,
    title: &str,
    error: Option<&str>,
    form_action: &str,
    form_fields: Markup,
    submit_label: &str,
    cancel_label: &str,
) -> Markup {
    html! {
        div
            class="modal-backdrop"
            style="position: fixed; top: 0; left: 0; right: 0; bottom: 0; background: rgba(0, 0, 0, 0.5); display: flex; align-items: center; justify-content: center; z-index: 1000;"
            id=(modal_id)
        {
            div
                class="modal"
                style="background: white; border-radius: 12px; padding: 2rem; max-width: 500px; width: 90%; max-height: 90vh; overflow-y: auto;"
                onclick="event.stopPropagation()"
            {
                h2 style="margin-bottom: 1.5rem; font-size: 1.5rem; font-weight: 700;" {
                    (title)
                }

                @if let Some(error_msg) = error {
                    div class="error" style="margin-bottom: 1rem; padding: 0.75rem; background: #fee; border: 1px solid #fcc; border-radius: 4px; color: #c00;" {
                        (error_msg)
                    }
                }

                form hx-post=(form_action) hx-target=(format!("#{}", modal_id)) hx-swap="outerHTML" {
                    (form_fields)

                    div style="display: flex; gap: 0.5rem; justify-content: flex-end;" {
                        button
                            type="button"
                            class="btn"
                            style="background: white; border: 1px solid var(--gray-300);"
                            onclick=(format!("document.getElementById('{}').remove()", modal_id))
                        {
                            (cancel_label)
                        }
                        button type="submit" class="btn btn-primary" {
                            (submit_label)
                        }
                    }
                }
            }
        }
        (PreEscaped(format!(r#"
        <script>
            document.getElementById('{}').addEventListener('click', function(e) {{
                if (e.target === this) {{
                    this.remove();
                }}
            }});
        </script>
        "#, modal_id)))
    }
}

/// Modal form with i18n support (multipart/form-data encoding for file uploads)
pub fn modal_form_multipart_i18n(
    modal_id: &str,
    title: &str,
    error: Option<&str>,
    form_action: &str,
    form_fields: Markup,
    submit_label: &str,
    cancel_label: &str,
) -> Markup {
    html! {
        div
            class="modal-backdrop"
            style="position: fixed; top: 0; left: 0; right: 0; bottom: 0; background: rgba(0, 0, 0, 0.5); display: flex; align-items: center; justify-content: center; z-index: 1000;"
            id=(modal_id)
        {
            div
                class="modal"
                style="background: white; border-radius: 12px; padding: 2rem; max-width: 500px; width: 90%; max-height: 90vh; overflow-y: auto;"
                onclick="event.stopPropagation()"
            {
                h2 style="margin-bottom: 1.5rem; font-size: 1.5rem; font-weight: 700;" {
                    (title)
                }

                @if let Some(error_msg) = error {
                    div class="error" style="margin-bottom: 1rem; padding: 0.75rem; background: #fee; border: 1px solid #fcc; border-radius: 4px; color: #c00;" {
                        (error_msg)
                    }
                }

                form
                    hx-post=(form_action)
                    hx-target=(format!("#{}", modal_id))
                    hx-swap="outerHTML"
                    hx-encoding="multipart/form-data"
                {
                    (form_fields)

                    div style="display: flex; gap: 0.5rem; justify-content: flex-end;" {
                        button
                            type="button"
                            class="btn"
                            style="background: white; border: 1px solid var(--gray-300);"
                            onclick=(format!("document.getElementById('{}').remove()", modal_id))
                        {
                            (cancel_label)
                        }
                        button type="submit" class="btn btn-primary" {
                            (submit_label)
                        }
                    }
                }
            }
        }
        (PreEscaped(format!(r#"
        <script>
            document.getElementById('{}').addEventListener('click', function(e) {{
                if (e.target === this) {{
                    this.remove();
                }}
            }});
        </script>
        "#, modal_id)))
    }
}
