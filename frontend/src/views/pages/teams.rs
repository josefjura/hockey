use maud::{html, Markup};

use crate::common::pagination::PagedResult;
use crate::service::teams::{SortField, SortOrder, TeamEntity, TeamFilters};
use crate::views::components::crud::{empty_state, modal_form, page_header, pagination, table_actions};

/// Main teams page with table and filters
pub fn teams_page(
    result: &PagedResult<TeamEntity>,
    filters: &TeamFilters,
    sort_field: &SortField,
    sort_order: &SortOrder,
    countries: &[(i64, String)],
) -> Markup {
    html! {
        div class="card" {
            // Header with title and create button
            (page_header(
                "Teams",
                "Manage and view all teams in the system.",
                "/teams/new",
                "+ New Team"
            ))

            // Filters
            div style="margin-bottom: 1.5rem; padding: 1rem; background: var(--gray-50); border-radius: 8px;" {
                form hx-get="/teams/list" hx-target="#teams-table" hx-swap="outerHTML" hx-trigger="submit, change delay:300ms" {
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
                                placeholder="Enter team name..."
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
                                hx-get="/teams/list"
                                hx-target="#teams-table"
                                hx-swap="outerHTML"
                            {
                                "Clear"
                            }
                        }
                    }
                }
            }

            // Table
            (team_list_content(result, filters, sort_field, sort_order))

            // Modal container
            div id="modal-container" {}
        }
    }
}

/// Teams table content (for HTMX updates)
pub fn team_list_content(
    result: &PagedResult<TeamEntity>,
    filters: &TeamFilters,
    sort_field: &SortField,
    sort_order: &SortOrder,
) -> Markup {
    html! {
        div id="teams-table" {
            @if result.items.is_empty() {
                (empty_state(
                    "teams",
                    filters.name.is_some() || filters.country_id.is_some()
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
                                    "Name",
                                    &SortField::Name,
                                    sort_field,
                                    sort_order,
                                    filters,
                                ))
                            }
                            th {
                                (sortable_header(
                                    "Country",
                                    &SortField::Country,
                                    sort_field,
                                    sort_order,
                                    filters,
                                ))
                            }
                            th style="text-align: right;" { "Actions" }
                        }
                    }
                    tbody {
                        @for team in &result.items {
                            tr {
                                td { (team.id) }
                                td { (team.name) }
                                td {
                                    @if let Some(country_name) = &team.country_name {
                                        @if let Some(iso2) = &team.country_iso2_code {
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
                                        span style="color: var(--gray-400); font-style: italic;" { "No country" }
                                    }
                                }
                                (table_actions(
                                    &format!("/teams/{}/edit", team.id),
                                    &build_delete_url(team.id, filters, sort_field, sort_order),
                                    "teams-table",
                                    "team"
                                ))
                            }
                        }
                    }
                }

                // Pagination
                (pagination(
                    result,
                    "teams",
                    |page| build_pagination_url(page, result.page_size, filters, sort_field, sort_order),
                    "teams-table"
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
    filters: &TeamFilters,
) -> Markup {
    // Determine if this column is currently sorted
    let is_active = matches!(
        (field, current_sort),
        (SortField::Id, SortField::Id)
            | (SortField::Name, SortField::Name)
            | (SortField::Country, SortField::Country)
    );

    // If this column is active, toggle the order; otherwise default to ASC
    let next_order = if is_active {
        current_order.toggle()
    } else {
        SortOrder::Asc
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
            hx-target="#teams-table"
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
fn build_sort_url(field: &SortField, order: &SortOrder, filters: &TeamFilters) -> String {
    let mut url = format!("/teams/list?sort={}&order={}", field.as_str(), order.as_str());

    if let Some(name) = &filters.name {
        url.push_str(&format!("&name={}", urlencoding::encode(name)));
    }

    if let Some(country_id) = filters.country_id {
        url.push_str(&format!("&country_id={}", country_id));
    }

    url
}

/// Helper to build pagination URLs with filters and sorting
fn build_pagination_url(
    page: usize,
    page_size: usize,
    filters: &TeamFilters,
    sort_field: &SortField,
    sort_order: &SortOrder,
) -> String {
    let mut url = format!(
        "/teams/list?page={}&page_size={}&sort={}&order={}",
        page,
        page_size,
        sort_field.as_str(),
        sort_order.as_str()
    );

    if let Some(name) = &filters.name {
        url.push_str(&format!("&name={}", urlencoding::encode(name)));
    }

    if let Some(country_id) = filters.country_id {
        url.push_str(&format!("&country_id={}", country_id));
    }

    url
}

/// Helper to build delete URL with current filters and sorting
fn build_delete_url(
    team_id: i64,
    filters: &TeamFilters,
    sort_field: &SortField,
    sort_order: &SortOrder,
) -> String {
    let mut url = format!(
        "/teams/{}/delete?sort={}&order={}",
        team_id,
        sort_field.as_str(),
        sort_order.as_str()
    );

    if let Some(name) = &filters.name {
        url.push_str(&format!("&name={}", urlencoding::encode(name)));
    }

    if let Some(country_id) = filters.country_id {
        url.push_str(&format!("&country_id={}", country_id));
    }

    url
}

/// Create team modal
pub fn team_create_modal(error: Option<&str>) -> Markup {
    let form_fields = html! {
        div style="margin-bottom: 1rem;" {
            label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                "Team Name"
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
                "Country"
            }
            country-selector
                name="country_id"
                placeholder="Select a country" {}
        }
    };

    modal_form(
        "team-modal",
        "Create Team",
        error,
        "/teams",
        form_fields,
        "Create Team"
    )
}

/// Edit team modal
pub fn team_edit_modal(team: &TeamEntity, error: Option<&str>) -> Markup {
    let form_fields = html! {
        div style="margin-bottom: 1rem;" {
            label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                "Team Name"
                span style="color: red;" { "*" }
            }
            input
                type="text"
                name="name"
                value=(team.name)
                required
                autofocus
                style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;";
        }

        div style="margin-bottom: 1.5rem;" {
            label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                "Country"
            }
            @if let Some(country_id) = team.country_id {
                country-selector
                    name="country_id"
                    placeholder="Select a country"
                    value=(country_id) {}
            } @else {
                country-selector
                    name="country_id"
                    placeholder="Select a country" {}
            }
        }
    };

    modal_form(
        "team-modal",
        "Edit Team",
        error,
        &format!("/teams/{}", team.id),
        form_fields,
        "Save Changes"
    )
}
