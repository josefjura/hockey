use maud::{html, Markup};

use crate::service::teams::{TeamEntity, TeamFilters, PagedResult};

/// Main teams page with table and filters
pub fn teams_page(
    result: &PagedResult<TeamEntity>,
    filters: &TeamFilters,
    countries: &[(i64, String)],
) -> Markup {
    html! {
        div class="card" {
            // Header with title
            div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 1.5rem;" {
                div {
                    h1 style="font-size: 2rem; font-weight: 700; margin-bottom: 0.5rem;" {
                        "Teams"
                    }
                    p style="color: var(--gray-600);" {
                        "Manage and view all teams in the system."
                    }
                }
            }

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
            (team_list_content(result, filters))
        }
    }
}

/// Teams table content (for HTMX updates)
pub fn team_list_content(result: &PagedResult<TeamEntity>, filters: &TeamFilters) -> Markup {
    html! {
        div id="teams-table" {
            @if result.items.is_empty() {
                div style="padding: 3rem; text-align: center; color: var(--gray-500);" {
                    h3 style="font-size: 1.25rem; font-weight: 600; margin-bottom: 0.5rem;" {
                        "No teams found"
                    }
                    p {
                        @if filters.name.is_some() || filters.country_id.is_some() {
                            "No teams match your search criteria. Try adjusting your filters."
                        } @else {
                            "No teams have been added yet."
                        }
                    }
                }
            } @else {
                table class="table" {
                    thead {
                        tr {
                            th {
                                button
                                    class="sort-button"
                                    hx-get="/teams/list?sort=id"
                                    hx-target="#teams-table"
                                    hx-swap="outerHTML"
                                    style="background: none; border: none; cursor: pointer; padding: 0; font-weight: 600; display: flex; align-items: center; gap: 0.25rem;"
                                {
                                    "ID"
                                    span style="font-size: 0.75rem;" { "↕" }
                                }
                            }
                            th {
                                button
                                    class="sort-button"
                                    hx-get="/teams/list?sort=name"
                                    hx-target="#teams-table"
                                    hx-swap="outerHTML"
                                    style="background: none; border: none; cursor: pointer; padding: 0; font-weight: 600; display: flex; align-items: center; gap: 0.25rem;"
                                {
                                    "Name"
                                    span style="font-size: 0.75rem;" { "↕" }
                                }
                            }
                            th {
                                button
                                    class="sort-button"
                                    hx-get="/teams/list?sort=country"
                                    hx-target="#teams-table"
                                    hx-swap="outerHTML"
                                    style="background: none; border: none; cursor: pointer; padding: 0; font-weight: 600; display: flex; align-items: center; gap: 0.25rem;"
                                {
                                    "Country"
                                    span style="font-size: 0.75rem;" { "↕" }
                                }
                            }
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
fn pagination(result: &PagedResult<TeamEntity>, filters: &TeamFilters) -> Markup {
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
                " teams"
            }

            // Page buttons
            @if result.total_pages > 1 {
                div style="display: flex; gap: 0.5rem;" {
                    // Previous button
                    @if result.has_previous {
                        button
                            class="btn btn-sm"
                            hx-get=(build_pagination_url(result.page - 1, result.page_size, filters))
                            hx-target="#teams-table"
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
                                hx-get=(build_pagination_url(page, result.page_size, filters))
                                hx-target="#teams-table"
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
                            hx-target="#teams-table"
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
fn build_pagination_url(page: usize, page_size: usize, filters: &TeamFilters) -> String {
    let mut url = format!("/teams/list?page={}&page_size={}", page, page_size);

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
