use maud::{html, Markup, PreEscaped};

use crate::common::pagination::PagedResult;
use crate::views::components::table::pagination_pages;

/// Page header with title, description, and create button
pub fn page_header(
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

/// Generic pagination component
///
/// # Parameters
/// - `result`: The paged result with items and pagination metadata
/// - `entity_name`: Name of the entity for display (e.g., "players", "seasons")
/// - `build_url`: Function that takes a page number and returns the URL for that page
/// - `target_id`: The ID of the element to update (e.g., "players-table")
pub fn pagination<T, F>(
    result: &PagedResult<T>,
    entity_name: &str,
    build_url: F,
    target_id: &str,
) -> Markup
where
    F: Fn(usize) -> String,
{
    html! {
        div style="display: flex; justify-content: space-between; align-items: center; margin-top: 1.5rem; padding-top: 1.5rem; border-top: 1px solid var(--gray-200);" {
            // Stats
            div style="color: var(--gray-600); font-size: 0.875rem;" {
                "Showing "
                strong { (((result.page - 1) * result.page_size + 1)) }
                " to "
                strong { (std::cmp::min(result.page * result.page_size, result.total)) }
                " of "
                strong { (result.total) }
                " " (entity_name)
            }

            // Page buttons
            @if result.total_pages > 1 {
                div style="display: flex; gap: 0.5rem;" {
                    // Previous button
                    @if result.has_previous {
                        button
                            class="btn btn-sm"
                            hx-get=(build_url(result.page - 1))
                            hx-target=(format!("#{}", target_id))
                            hx-swap="outerHTML"
                        {
                            "Previous"
                        }
                    } @else {
                        button class="btn btn-sm" disabled { "Previous" }
                    }

                    // Page numbers
                    @for page in pagination_pages(result.page, result.total_pages) {
                        @if page == 0 {
                            span style="padding: 0.25rem 0.5rem; color: var(--gray-400);" { "..." }
                        } @else if page == result.page {
                            button class="btn btn-sm btn-primary" disabled {
                                (page)
                            }
                        } @else {
                            button
                                class="btn btn-sm"
                                hx-get=(build_url(page))
                                hx-target=(format!("#{}", target_id))
                                hx-swap="outerHTML"
                            {
                                (page)
                            }
                        }
                    }

                    // Next button
                    @if result.has_next {
                        button
                            class="btn btn-sm"
                            hx-get=(build_url(result.page + 1))
                            hx-target=(format!("#{}", target_id))
                            hx-swap="outerHTML"
                        {
                            "Next"
                        }
                    } @else {
                        button class="btn btn-sm" disabled { "Next" }
                    }
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
