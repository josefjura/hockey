use maud::{html, Markup};

use crate::service::seasons::{PagedResult, SeasonEntity, SeasonFilters, SortField, SortOrder};
use crate::views::components::crud::{empty_state, modal_form, page_header, pagination, table_actions};

/// Main seasons page with table and filters
pub fn seasons_page(
    result: &PagedResult<SeasonEntity>,
    filters: &SeasonFilters,
    sort_field: &SortField,
    sort_order: &SortOrder,
    events: &[(i64, String)],
) -> Markup {
    html! {
        div class="card" {
            // Header with title and create button
            (page_header(
                "Seasons",
                "Manage seasons for events.",
                "/seasons/new",
                "+ New Season"
            ))

            // Filters
            div style="margin-bottom: 1.5rem; padding: 1rem; background: var(--gray-50); border-radius: 8px;" {
                form hx-get="/seasons/list" hx-target="#seasons-table" hx-swap="outerHTML" hx-trigger="submit, change delay:300ms" {
                    div style="display: grid; grid-template-columns: 1fr 1fr auto; gap: 1rem; align-items: end;" {
                        // Event filter
                        div {
                            label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                                "Filter by event"
                            }
                            select
                                name="event_id"
                                style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;"
                            {
                                option value="" { "All events" }
                                @for (id, name) in events {
                                    option
                                        value=(id)
                                        selected[filters.event_id == Some(*id)]
                                    {
                                        (name)
                                    }
                                }
                            }
                        }

                        // Year filter
                        div {
                            label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                                "Filter by year"
                            }
                            input
                                type="number"
                                name="year"
                                value=[filters.year.as_ref()]
                                placeholder="Enter year..."
                                min="1900"
                                max="2100"
                                style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;";
                        }

                        // Clear button
                        div {
                            button
                                type="button"
                                class="btn"
                                style="background: white; border: 1px solid var(--gray-300);"
                                hx-get="/seasons/list"
                                hx-target="#seasons-table"
                                hx-swap="outerHTML"
                            {
                                "Clear"
                            }
                        }
                    }
                }
            }

            // Table
            (season_list_content(result, filters, sort_field, sort_order))

            // Modal container
            div id="modal-container" {}
        }
    }
}

/// Seasons table content (for HTMX updates)
pub fn season_list_content(
    result: &PagedResult<SeasonEntity>,
    filters: &SeasonFilters,
    sort_field: &SortField,
    sort_order: &SortOrder,
) -> Markup {
    html! {
        div id="seasons-table" {
            @if result.items.is_empty() {
                (empty_state(
                    "seasons",
                    filters.event_id.is_some() || filters.year.is_some()
                ))
            } @else {
                table class="table" {
                    thead {
                        tr {
                            th {
                                (sortable_header(
                                    "ID",
                                    &SortField::Id,
                                    sort_field,
                                    sort_order,
                                    filters,
                                ))
                            }
                            th {
                                (sortable_header(
                                    "Year",
                                    &SortField::Year,
                                    sort_field,
                                    sort_order,
                                    filters,
                                ))
                            }
                            th { "Display Name" }
                            th {
                                (sortable_header(
                                    "Event",
                                    &SortField::Event,
                                    sort_field,
                                    sort_order,
                                    filters,
                                ))
                            }
                            th style="text-align: right;" { "Actions" }
                        }
                    }
                    tbody {
                        @for season in &result.items {
                            tr {
                                td { (season.id) }
                                td { (season.year) }
                                td {
                                    @if let Some(display_name) = &season.display_name {
                                        (display_name)
                                    } @else {
                                        span style="color: var(--gray-400); font-style: italic;" {
                                            (format!("{} Season", season.year))
                                        }
                                    }
                                }
                                td { (&season.event_name) }
                                (table_actions(
                                    &format!("/seasons/{}/edit", season.id),
                                    &build_delete_url(season.id, filters, sort_field, sort_order),
                                    "seasons-table",
                                    "season"
                                ))
                            }
                        }
                    }
                }

                // Pagination
                (pagination(
                    result,
                    "seasons",
                    |page| build_pagination_url(page, result.page_size, filters, sort_field, sort_order),
                    "seasons-table"
                ))
            }
        }
    }
}

/// Sortable table header
fn sortable_header(
    label: &str,
    field: &SortField,
    current_sort: &SortField,
    current_order: &SortOrder,
    filters: &SeasonFilters,
) -> Markup {
    // Determine if this column is currently sorted
    let is_active = matches!(
        (field, current_sort),
        (SortField::Id, SortField::Id)
            | (SortField::Year, SortField::Year)
            | (SortField::Event, SortField::Event)
    );

    // If this column is active, toggle the order; otherwise default to DESC for year, ASC for others
    let next_order = if is_active {
        current_order.toggle()
    } else {
        match field {
            SortField::Year => SortOrder::Desc,
            _ => SortOrder::Asc,
        }
    };

    // Build the URL
    let url = build_sort_url(field, &next_order, filters);

    // Choose the indicator
    let indicator = if is_active {
        match current_order {
            SortOrder::Asc => "↑",
            SortOrder::Desc => "↓",
        }
    } else {
        "↕"
    };

    html! {
        button
            class="sort-button"
            hx-get=(url)
            hx-target="#seasons-table"
            hx-swap="outerHTML"
            style="background: none; border: none; cursor: pointer; padding: 0; font-weight: 600; display: flex; align-items: center; gap: 0.25rem;"
        {
            (label)
            span style=(if is_active { "font-size: 0.75rem; color: var(--primary-color);" } else { "font-size: 0.75rem;" }) {
                (indicator)
            }
        }
    }
}

/// Helper to build sort URLs
fn build_sort_url(field: &SortField, order: &SortOrder, filters: &SeasonFilters) -> String {
    let mut url = format!("/seasons/list?sort={}&order={}", field.as_str(), order.as_str());

    if let Some(event_id) = filters.event_id {
        url.push_str(&format!("&event_id={}", event_id));
    }

    if let Some(year) = filters.year {
        url.push_str(&format!("&year={}", year));
    }

    url
}

/// Helper to build pagination URLs with filters and sorting
fn build_pagination_url(
    page: usize,
    page_size: usize,
    filters: &SeasonFilters,
    sort_field: &SortField,
    sort_order: &SortOrder,
) -> String {
    let mut url = format!(
        "/seasons/list?page={}&page_size={}&sort={}&order={}",
        page,
        page_size,
        sort_field.as_str(),
        sort_order.as_str()
    );

    if let Some(event_id) = filters.event_id {
        url.push_str(&format!("&event_id={}", event_id));
    }

    if let Some(year) = filters.year {
        url.push_str(&format!("&year={}", year));
    }

    url
}

/// Helper to build delete URL with current filters and sorting
fn build_delete_url(
    season_id: i64,
    filters: &SeasonFilters,
    sort_field: &SortField,
    sort_order: &SortOrder,
) -> String {
    let mut url = format!(
        "/seasons/{}/delete?sort={}&order={}",
        season_id,
        sort_field.as_str(),
        sort_order.as_str()
    );

    if let Some(event_id) = filters.event_id {
        url.push_str(&format!("&event_id={}", event_id));
    }

    if let Some(year) = filters.year {
        url.push_str(&format!("&year={}", year));
    }

    url
}

/// Create season modal
pub fn season_create_modal(error: Option<&str>, events: &[(i64, String)]) -> Markup {
    let form_fields = html! {
        div style="margin-bottom: 1rem;" {
            label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                "Event"
                span style="color: red;" { "*" }
            }
            select
                name="event_id"
                required
                autofocus
                style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;"
            {
                option value="" { "Select an event" }
                @for (id, name) in events {
                    option value=(id) {
                        (name)
                    }
                }
            }
        }

        div style="margin-bottom: 1rem;" {
            label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                "Year"
                span style="color: red;" { "*" }
            }
            input
                type="number"
                name="year"
                required
                min="1900"
                max="2100"
                placeholder="2024"
                style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;";
        }

        div style="margin-bottom: 1.5rem;" {
            label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                "Display Name"
            }
            input
                type="text"
                name="display_name"
                placeholder="e.g., 2024 Championship"
                style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;";
            p style="font-size: 0.75rem; color: var(--gray-500); margin-top: 0.25rem;" {
                "Optional: Custom display name (defaults to year)"
            }
        }
    };

    modal_form(
        "season-modal",
        "Create Season",
        error,
        "/seasons",
        form_fields,
        "Create Season"
    )
}

/// Edit season modal
pub fn season_edit_modal(
    season: &SeasonEntity,
    error: Option<&str>,
    events: &[(i64, String)],
) -> Markup {
    let form_fields = html! {
        div style="margin-bottom: 1rem;" {
            label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                "Event"
                span style="color: red;" { "*" }
            }
            select
                name="event_id"
                required
                autofocus
                style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;"
            {
                option value="" { "Select an event" }
                @for (id, name) in events {
                    option value=(id) selected[*id == season.event_id] {
                        (name)
                    }
                }
            }
        }

        div style="margin-bottom: 1rem;" {
            label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                "Year"
                span style="color: red;" { "*" }
            }
            input
                type="number"
                name="year"
                value=(season.year)
                required
                min="1900"
                max="2100"
                style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;";
        }

        div style="margin-bottom: 1.5rem;" {
            label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                "Display Name"
            }
            input
                type="text"
                name="display_name"
                value=[season.display_name.as_ref()]
                placeholder="e.g., 2024 Championship"
                style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;";
            p style="font-size: 0.75rem; color: var(--gray-500); margin-top: 0.25rem;" {
                "Optional: Custom display name (defaults to year)"
            }
        }
    };

    modal_form(
        "season-modal",
        "Edit Season",
        error,
        &format!("/seasons/{}", season.id),
        form_fields,
        "Save Changes"
    )
}
