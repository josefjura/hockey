use maud::{html, Markup, PreEscaped};

use crate::service::events::{EventEntity, EventFilters, PagedResult};

/// Main events page with table and filters
pub fn events_page(
    result: &PagedResult<EventEntity>,
    filters: &EventFilters,
    countries: &[(i64, String)],
) -> Markup {
    html! {
        div class="card" {
            // Header with title and create button
            div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 1.5rem;" {
                h1 style="font-size: 2rem; font-weight: 700;" {
                    "Events"
                }
                button
                    class="btn btn-primary"
                    hx-get="/events/new"
                    hx-target="#modal-container"
                    hx-swap="innerHTML"
                {
                    "+ New Event"
                }
            }

            // Filters
            div style="margin-bottom: 1.5rem; padding: 1rem; background: var(--gray-50); border-radius: 8px;" {
                form hx-get="/events/list" hx-target="#events-table" hx-swap="outerHTML" hx-trigger="submit, change delay:300ms" {
                    div style="display: grid; grid-template-columns: 1fr 1fr auto; gap: 1rem; align-items: end;" {
                        // Name filter
                        div {
                            label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                                "Search by name"
                            }
                            input
                                type="text"
                                name="name"
                                value=[filters.name.as_ref()]
                                placeholder="Enter event name..."
                                style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;";
                        }

                        // Country filter
                        div {
                            label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                                "Filter by country"
                            }
                            select
                                name="country_id"
                                style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;"
                            {
                                option value="" { "All countries" }
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
                                "Clear"
                            }
                        }
                    }
                }
            }

            // Table
            (event_list_content(result, filters))

            // Modal container
            div id="modal-container" {}
        }
    }
}

/// Events table content (for HTMX updates)
pub fn event_list_content(result: &PagedResult<EventEntity>, filters: &EventFilters) -> Markup {
    html! {
        div id="events-table" {
            @if result.items.is_empty() {
                div style="padding: 3rem; text-align: center; color: var(--gray-500);" {
                    p { "No events found." }
                    @if filters.name.is_some() || filters.country_id.is_some() {
                        p style="margin-top: 0.5rem; font-size: 0.875rem;" {
                            "Try adjusting your filters."
                        }
                    }
                }
            } @else {
                table class="table" {
                    thead {
                        tr {
                            th { "ID" }
                            th { "Name" }
                            th { "Country" }
                            th style="text-align: right;" { "Actions" }
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
                                                    src=(format!("/static/flags/{}.svg", iso2.to_lowercase()))
                                                    alt=(country_name)
                                                    style="width: 20px; height: 15px; object-fit: cover; border: 1px solid var(--gray-300);"
                                                    onerror="this.style.display='none'";
                                                (country_name)
                                            }
                                        } @else {
                                            (country_name)
                                        }
                                    } @else {
                                        span style="color: var(--gray-400); font-style: italic;" { "No country" }
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
                                        "Edit"
                                    }
                                    button
                                        class="btn btn-sm btn-danger"
                                        hx-post=(format!("/events/{}/delete", event.id))
                                        hx-target="#events-table"
                                        hx-swap="outerHTML"
                                        hx-confirm="Are you sure you want to delete this event?"
                                    {
                                        "Delete"
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
pub fn event_create_modal(countries: &[(i64, String)], error: Option<&str>) -> Markup {
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
                    "Create Event"
                }

                @if let Some(error_msg) = error {
                    div class="error" style="margin-bottom: 1rem; padding: 0.75rem; background: #fee; border: 1px solid #fcc; border-radius: 4px; color: #c00;" {
                        (error_msg)
                    }
                }

                form hx-post="/events" hx-target="#event-modal" hx-swap="outerHTML" {
                    div style="margin-bottom: 1rem;" {
                        label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                            "Event Name"
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
                            "Host Country"
                        }
                        select
                            name="country_id"
                            style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;"
                        {
                            option value="" { "No country" }
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
                            "Cancel"
                        }
                        button type="submit" class="btn btn-primary" {
                            "Create Event"
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
                    "Edit Event"
                }

                @if let Some(error_msg) = error {
                    div class="error" style="margin-bottom: 1rem; padding: 0.75rem; background: #fee; border: 1px solid #fcc; border-radius: 4px; color: #c00;" {
                        (error_msg)
                    }
                }

                form hx-post=(format!("/events/{}", event.id)) hx-target="#event-modal" hx-swap="outerHTML" {
                    div style="margin-bottom: 1rem;" {
                        label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                            "Event Name"
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
                            "Host Country"
                        }
                        select
                            name="country_id"
                            style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;"
                        {
                            option value="" selected[event.country_id.is_none()] { "No country" }
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
                            "Cancel"
                        }
                        button type="submit" class="btn btn-primary" {
                            "Save Changes"
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
