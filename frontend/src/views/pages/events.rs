use maud::{html, Markup, PreEscaped};

use crate::i18n::TranslationContext;
use crate::service::events::{EventEntity, EventFilters, PagedResult};
use crate::views::components::confirm::{confirm_attrs, ConfirmVariant};
use crate::views::components::empty_state::{empty_state_enhanced, EmptyStateIcon};

/// Main events page with table and filters
pub fn events_page(
    t: &TranslationContext,
    result: &PagedResult<EventEntity>,
    filters: &EventFilters,
    countries: &[(i64, String)],
) -> Markup {
    html! {
        div class="card" {
            // Header with title and create button
            div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 1.5rem;" {
                h1 style="font-size: 2rem; font-weight: 700;" {
                    (t.messages.events_title())
                }
                button
                    class="btn btn-primary"
                    hx-get="/events/new"
                    hx-target="#modal-container"
                    hx-swap="innerHTML"
                {
                    (t.messages.events_create())
                }
            }

            // Filters
            div style="margin-bottom: 1.5rem; padding: 1rem; background: var(--gray-50); border-radius: 8px;" {
                form hx-get="/events/list" hx-target="#events-table" hx-swap="outerHTML" hx-trigger="submit, change delay:300ms" {
                    div style="display: grid; grid-template-columns: 1fr 1fr auto; gap: 1rem; align-items: end;" {
                        // Name filter
                        div {
                            label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                                (t.messages.common_search_by_name())
                            }
                            input
                                type="text"
                                name="name"
                                value=[filters.name.as_ref()]
                                placeholder=(t.messages.events_name_placeholder())
                                style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;";
                        }

                        // Country filter
                        div {
                            label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                                (t.messages.common_filter_by_country())
                            }
                            select
                                name="country_id"
                                style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;"
                            {
                                option value="" { (t.messages.common_all_countries()) }
                                @for (id, name) in countries {
                                    option
                                        value=(id)
                                        selected[filters.country_id == Some(*id)]
                                    {
                                        (name)
                                    }
                                }
                            }
                        }

                        // Clear button
                        div {
                            button
                                type="button"
                                class="btn"
                                style="background: white; border: 1px solid var(--gray-300);"
                                hx-get="/events/list"
                                hx-target="#events-table"
                                hx-swap="outerHTML"
                            {
                                (t.messages.common_clear())
                            }
                        }
                    }
                }
            }

            // Table
            (event_list_content(t, result, filters))

            // Modal container
            div id="modal-container" {}
        }
    }
}

/// Events table content (for HTMX updates)
pub fn event_list_content(
    t: &TranslationContext,
    result: &PagedResult<EventEntity>,
    filters: &EventFilters,
) -> Markup {
    let has_filters = filters.name.is_some() || filters.country_id.is_some();
    let empty_title = t.messages.events_empty_title().to_string();
    let empty_message = if has_filters {
        t.messages.events_empty_message().to_string()
    } else {
        "Create your first event to get started.".to_string()
    };
    let create_label = t.messages.events_create().to_string();
    
    html! {
        div id="events-table" class="loading-overlay" {
            // Loading spinner overlay
            div class="loading-spinner-overlay" {
                hockey-loading-spinner size="lg" {}
            }
            
            @if result.items.is_empty() {
                (empty_state_enhanced(
                    if has_filters { EmptyStateIcon::Search } else { EmptyStateIcon::Calendar },
                    &empty_title,
                    &empty_message,
                    if !has_filters { Some(create_label.as_str()) } else { None },
                    if !has_filters { Some("/events/new") } else { None },
                    if !has_filters { Some("#modal-container") } else { None }
                ))
            } @else {
                table class="table" {
                    thead {
                        tr {
                            th { (t.messages.common_id()) }
                            th { (t.messages.form_name()) }
                            th { (t.messages.form_country()) }
                            th style="text-align: right;" { (t.messages.common_actions()) }
                        }
                    }
                    tbody {
                        @for event in &result.items {
                            tr {
                                td { (event.id) }
                                td { (event.name) }
                                td {
                                    @if let Some(country_name) = &event.country_name {
                                        @if let Some(iso2) = &event.country_iso2_code {
                                            span style="display: inline-flex; align-items: center; gap: 0.5rem;" {
                                                img
                                                    src=(format!("https://flagcdn.com/w40/{}.png", iso2.to_lowercase()))
                                                    alt=(country_name)
                                                    style="width: 20px; height: 15px; object-fit: cover; border: 1px solid var(--gray-300);"
                                                    onerror="this.style.display='none'";
                                                (country_name)
                                            }
                                        } @else {
                                            (country_name)
                                        }
                                    } @else {
                                        span style="color: var(--gray-400); font-style: italic;" { (t.messages.common_no_country()) }
                                    }
                                }
                                td style="text-align: right;" {
                                    button
                                        class="btn btn-sm"
                                        hx-get=(format!("/events/{}/edit", event.id))
                                        hx-target="#modal-container"
                                        hx-swap="innerHTML"
                                        style="margin-right: 0.5rem;"
                                    {
                                        (t.messages.common_edit())
                                    }
                                    button
                                        class="btn btn-sm btn-danger"
                                        hx-post=(format!("/events/{}/delete", event.id))
                                        hx-target="#events-table"
                                        hx-swap="outerHTML"
                                        hx-confirm-custom=(confirm_attrs(
                                            &t.messages.events_delete().to_string(),
                                            &t.messages.events_confirm_delete().to_string(),
                                            ConfirmVariant::Danger,
                                            Some(&t.messages.common_delete().to_string()),
                                            Some(&t.messages.common_cancel().to_string())
                                        ))
                                    {
                                        (t.messages.common_delete())
                                    }
                                }
                            }
                        }
                    }
                }

                // Pagination
                (pagination(result, filters))
            }
        }
    }
}

/// Pagination component
fn pagination(result: &PagedResult<EventEntity>, filters: &EventFilters) -> Markup {
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
                " events"
            }

            // Page buttons
            @if result.total_pages > 1 {
                div style="display: flex; gap: 0.5rem;" {
                    // Previous button
                    @if result.has_previous {
                        button
                            class="btn btn-sm"
                            hx-get=(build_pagination_url(result.page - 1, result.page_size, filters))
                            hx-target="#events-table"
                            hx-swap="outerHTML"
                        {
                            "Previous"
                        }
                    } @else {
                        button class="btn btn-sm" disabled { "Previous" }
                    }

                    // Page numbers
                    @for page in pagination_pages(result.page, result.total_pages) {
                        @if page == result.page {
                            button class="btn btn-sm btn-primary" disabled {
                                (page)
                            }
                        } @else {
                            button
                                class="btn btn-sm"
                                hx-get=(build_pagination_url(page, result.page_size, filters))
                                hx-target="#events-table"
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
                            hx-get=(build_pagination_url(result.page + 1, result.page_size, filters))
                            hx-target="#events-table"
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

/// Helper to build pagination URLs with filters
fn build_pagination_url(page: usize, page_size: usize, filters: &EventFilters) -> String {
    let mut url = format!("/events/list?page={}&page_size={}", page, page_size);

    if let Some(name) = &filters.name {
        url.push_str(&format!("&name={}", urlencoding::encode(name)));
    }

    if let Some(country_id) = filters.country_id {
        url.push_str(&format!("&country_id={}", country_id));
    }

    url
}

/// Helper to generate page numbers for pagination
fn pagination_pages(current_page: usize, total_pages: usize) -> Vec<usize> {
    let mut pages = Vec::new();

    if total_pages <= 7 {
        // Show all pages if there are 7 or fewer
        for page in 1..=total_pages {
            pages.push(page);
        }
    } else {
        // Show first page
        pages.push(1);

        // Show pages around current page
        let start = std::cmp::max(2, current_page.saturating_sub(1));
        let end = std::cmp::min(total_pages - 1, current_page + 1);

        if start > 2 {
            pages.push(0); // Placeholder for "..."
        }

        for page in start..=end {
            pages.push(page);
        }

        if end < total_pages - 1 {
            pages.push(0); // Placeholder for "..."
        }

        // Show last page
        pages.push(total_pages);
    }

    pages
}

/// Create event modal
pub fn event_create_modal(
    t: &TranslationContext,
    countries: &[(i64, String)],
    error: Option<&str>,
) -> Markup {
    html! {
        div
            class="modal-backdrop"
            style="position: fixed; top: 0; left: 0; right: 0; bottom: 0; background: rgba(0, 0, 0, 0.5); display: flex; align-items: center; justify-content: center; z-index: 1000;"
            id="event-modal"
        {
            div
                class="modal"
                style="background: white; border-radius: 12px; padding: 2rem; max-width: 500px; width: 90%; max-height: 90vh; overflow-y: auto;"
                onclick="event.stopPropagation()"
            {
                h2 style="margin-bottom: 1.5rem; font-size: 1.5rem; font-weight: 700;" {
                    (t.messages.events_create_title())
                }

                @if let Some(error_msg) = error {
                    div class="error" style="margin-bottom: 1rem; padding: 0.75rem; background: #fee; border: 1px solid #fcc; border-radius: 4px; color: #c00;" {
                        (error_msg)
                    }
                }

                form hx-post="/events" hx-target="#event-modal" hx-swap="outerHTML" {
                    div style="margin-bottom: 1rem;" {
                        label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                            (t.messages.events_name_label())
                            span style="color: red;" { "*" }
                        }
                        input
                            type="text"
                            name="name"
                            required
                            autofocus
                            style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;";
                    }

                    div style="margin-bottom: 1.5rem;" {
                        label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                            (t.messages.events_host_country())
                        }
                        select
                            name="country_id"
                            style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;"
                        {
                            option value="" { (t.messages.common_no_country()) }
                            @for (id, name) in countries {
                                option value=(id) { (name) }
                            }
                        }
                    }

                    div style="display: flex; gap: 0.5rem; justify-content: flex-end;" {
                        button
                            type="button"
                            class="btn"
                            style="background: white; border: 1px solid var(--gray-300);"
                            onclick="document.getElementById('event-modal').remove()"
                        {
                            (t.messages.common_cancel())
                        }
                        button type="submit" class="btn btn-primary" {
                            (t.messages.events_create_submit())
                        }
                    }
                }
            }
        }
        (PreEscaped(r#"
        <script>
            document.getElementById('event-modal').addEventListener('click', function(e) {
                if (e.target === this) {
                    this.remove();
                }
            });
        </script>
        "#))
    }
}

/// Edit event modal
pub fn event_edit_modal(
    t: &TranslationContext,
    event: &EventEntity,
    countries: &[(i64, String)],
    error: Option<&str>,
) -> Markup {
    html! {
        div
            class="modal-backdrop"
            style="position: fixed; top: 0; left: 0; right: 0; bottom: 0; background: rgba(0, 0, 0, 0.5); display: flex; align-items: center; justify-content: center; z-index: 1000;"
            id="event-modal"
        {
            div
                class="modal"
                style="background: white; border-radius: 12px; padding: 2rem; max-width: 500px; width: 90%; max-height: 90vh; overflow-y: auto;"
                onclick="event.stopPropagation()"
            {
                h2 style="margin-bottom: 1.5rem; font-size: 1.5rem; font-weight: 700;" {
                    (t.messages.events_edit_title())
                }

                @if let Some(error_msg) = error {
                    div class="error" style="margin-bottom: 1rem; padding: 0.75rem; background: #fee; border: 1px solid #fcc; border-radius: 4px; color: #c00;" {
                        (error_msg)
                    }
                }

                form hx-post=(format!("/events/{}", event.id)) hx-target="#event-modal" hx-swap="outerHTML" {
                    div style="margin-bottom: 1rem;" {
                        label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                            (t.messages.events_name_label())
                            span style="color: red;" { "*" }
                        }
                        input
                            type="text"
                            name="name"
                            value=(event.name)
                            required
                            autofocus
                            style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;";
                    }

                    div style="margin-bottom: 1.5rem;" {
                        label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                            (t.messages.events_host_country())
                        }
                        select
                            name="country_id"
                            style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;"
                        {
                            option value="" selected[event.country_id.is_none()] { (t.messages.common_no_country()) }
                            @for (id, name) in countries {
                                option
                                    value=(id)
                                    selected[event.country_id == Some(*id)]
                                {
                                    (name)
                                }
                            }
                        }
                    }

                    div style="display: flex; gap: 0.5rem; justify-content: flex-end;" {
                        button
                            type="button"
                            class="btn"
                            style="background: white; border: 1px solid var(--gray-300);"
                            onclick="document.getElementById('event-modal').remove()"
                        {
                            (t.messages.common_cancel())
                        }
                        button type="submit" class="btn btn-primary" {
                            (t.messages.common_save())
                        }
                    }
                }
            }
        }
        (PreEscaped(r#"
        <script>
            document.getElementById('event-modal').addEventListener('click', function(e) {
                if (e.target === this) {
                    this.remove();
                }
            });
        </script>
        "#))
    }
}
