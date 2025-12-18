use maud::{html, Markup, PreEscaped};

use crate::service::players::{PagedResult, PlayerEntity, PlayerFilters, SortField, SortOrder};
use crate::views::components::table::pagination_pages;

/// Main players page with table and filters
pub fn players_page(
    result: &PagedResult<PlayerEntity>,
    filters: &PlayerFilters,
    sort_field: &SortField,
    sort_order: &SortOrder,
    countries: &[(i64, String)],
) -> Markup {
    html! {
        div class="card" {
            // Header with title and create button
            div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 1.5rem;" {
                div {
                    h1 style="font-size: 2rem; font-weight: 700; margin-bottom: 0.5rem;" {
                        "Players"
                    }
                    p style="color: var(--gray-600);" {
                        "Manage and view all players in the system."
                    }
                }
                button
                    class="btn btn-primary"
                    hx-get="/players/new"
                    hx-target="#modal-container"
                    hx-swap="innerHTML"
                {
                    "+ New Player"
                }
            }

            // Filters
            div style="margin-bottom: 1.5rem; padding: 1rem; background: var(--gray-50); border-radius: 8px;" {
                form hx-get="/players/list" hx-target="#players-table" hx-swap="outerHTML" hx-trigger="submit, change delay:300ms" {
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
                                placeholder="Enter player name..."
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
                                hx-get="/players/list"
                                hx-target="#players-table"
                                hx-swap="outerHTML"
                            {
                                "Clear"
                            }
                        }
                    }
                }
            }

            // Table
            (player_list_content(result, filters, sort_field, sort_order))

            // Modal container
            div id="modal-container" {}
        }
    }
}

/// Players table content (for HTMX updates)
pub fn player_list_content(
    result: &PagedResult<PlayerEntity>,
    filters: &PlayerFilters,
    sort_field: &SortField,
    sort_order: &SortOrder,
) -> Markup {
    html! {
        div id="players-table" {
            @if result.items.is_empty() {
                div style="padding: 3rem; text-align: center; color: var(--gray-500);" {
                    h3 style="font-size: 1.25rem; font-weight: 600; margin-bottom: 0.5rem;" {
                        "No players found"
                    }
                    p {
                        @if filters.name.is_some() || filters.country_id.is_some() {
                            "No players match your search criteria. Try adjusting your filters."
                        } @else {
                            "No players have been added yet."
                        }
                    }
                }
            } @else {
                table class="table" {
                    thead {
                        tr {
                            th { "Photo" }
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
                        @for player in &result.items {
                            tr {
                                td {
                                    @if let Some(photo_path) = &player.photo_path {
                                        img
                                            src=(photo_path)
                                            alt=(format!("{} photo", player.name))
                                            style="width: 40px; height: 40px; object-fit: cover; border-radius: 50%; border: 2px solid var(--gray-300);"
                                            onerror="this.src='data:image/svg+xml,%3Csvg xmlns=%22http://www.w3.org/2000/svg%22 viewBox=%220 0 100 100%22%3E%3Ccircle cx=%2250%22 cy=%2250%22 r=%2250%22 fill=%22%23e5e7eb%22/%3E%3Ctext x=%2250%25%22 y=%2250%25%22 text-anchor=%22middle%22 dy=%22.3em%22 font-size=%2240%22 fill=%22%23666%22%3E%3F%3C/text%3E%3C/svg%3E'";
                                    } @else {
                                        div
                                            style="width: 40px; height: 40px; border-radius: 50%; background: var(--gray-200); display: flex; align-items: center; justify-content: center; font-weight: 600; color: var(--gray-600);"
                                        {
                                            (player.name.chars().next().unwrap_or('?').to_uppercase())
                                        }
                                    }
                                }
                                td { (player.id) }
                                td { (player.name) }
                                td {
                                    span style="display: inline-flex; align-items: center; gap: 0.5rem;" {
                                        img
                                            src=(format!("https://flagcdn.com/w40/{}.png", player.country_iso2_code.to_lowercase()))
                                            alt=(&player.country_name)
                                            style="width: 20px; height: 15px; object-fit: cover; border: 1px solid var(--gray-300);"
                                            onerror="this.style.display='none'";
                                        (&player.country_name)
                                    }
                                }
                                td style="text-align: right;" {
                                    button
                                        class="btn btn-sm"
                                        hx-get=(format!("/players/{}/edit", player.id))
                                        hx-target="#modal-container"
                                        hx-swap="innerHTML"
                                        style="margin-right: 0.5rem;"
                                    {
                                        "Edit"
                                    }
                                    button
                                        class="btn btn-sm btn-danger"
                                        hx-post=(format!("/players/{}/delete", player.id))
                                        hx-target="#players-table"
                                        hx-swap="outerHTML"
                                        hx-confirm="Are you sure you want to delete this player?"
                                    {
                                        "Delete"
                                    }
                                }
                            }
                        }
                    }
                }

                // Pagination
                (pagination(result, filters, sort_field, sort_order))
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
    filters: &PlayerFilters,
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
            hx-target="#players-table"
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
fn build_sort_url(field: &SortField, order: &SortOrder, filters: &PlayerFilters) -> String {
    let mut url = format!("/players/list?sort={}&order={}", field.as_str(), order.as_str());

    if let Some(name) = &filters.name {
        url.push_str(&format!("&name={}", urlencoding::encode(name)));
    }

    if let Some(country_id) = filters.country_id {
        url.push_str(&format!("&country_id={}", country_id));
    }

    url
}

/// Pagination component
fn pagination(
    result: &PagedResult<PlayerEntity>,
    filters: &PlayerFilters,
    sort_field: &SortField,
    sort_order: &SortOrder,
) -> Markup {
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
                " players"
            }

            // Page buttons
            @if result.total_pages > 1 {
                div style="display: flex; gap: 0.5rem;" {
                    // Previous button
                    @if result.has_previous {
                        button
                            class="btn btn-sm"
                            hx-get=(build_pagination_url(result.page - 1, result.page_size, filters, sort_field, sort_order))
                            hx-target="#players-table"
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
                                hx-get=(build_pagination_url(page, result.page_size, filters, sort_field, sort_order))
                                hx-target="#players-table"
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
                            hx-get=(build_pagination_url(result.page + 1, result.page_size, filters, sort_field, sort_order))
                            hx-target="#players-table"
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

/// Helper to build pagination URLs with filters and sorting
fn build_pagination_url(
    page: usize,
    page_size: usize,
    filters: &PlayerFilters,
    sort_field: &SortField,
    sort_order: &SortOrder,
) -> String {
    let mut url = format!(
        "/players/list?page={}&page_size={}&sort={}&order={}",
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

/// Create player modal
pub fn player_create_modal(error: Option<&str>, countries: &[(i64, String)]) -> Markup {
    html! {
        div
            class="modal-backdrop"
            style="position: fixed; top: 0; left: 0; right: 0; bottom: 0; background: rgba(0, 0, 0, 0.5); display: flex; align-items: center; justify-content: center; z-index: 1000;"
            id="player-modal"
        {
            div
                class="modal"
                style="background: white; border-radius: 12px; padding: 2rem; max-width: 500px; width: 90%; max-height: 90vh; overflow-y: auto;"
                onclick="event.stopPropagation()"
            {
                h2 style="margin-bottom: 1.5rem; font-size: 1.5rem; font-weight: 700;" {
                    "Create Player"
                }

                @if let Some(error_msg) = error {
                    div class="error" style="margin-bottom: 1rem; padding: 0.75rem; background: #fee; border: 1px solid #fcc; border-radius: 4px; color: #c00;" {
                        (error_msg)
                    }
                }

                form hx-post="/players" hx-target="#player-modal" hx-swap="outerHTML" {
                    div style="margin-bottom: 1rem;" {
                        label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                            "Player Name"
                            span style="color: red;" { "*" }
                        }
                        input
                            type="text"
                            name="name"
                            required
                            autofocus
                            style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;";
                    }

                    div style="margin-bottom: 1rem;" {
                        label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                            "Country"
                            span style="color: red;" { "*" }
                        }
                        select
                            name="country_id"
                            required
                            style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;"
                        {
                            option value="" { "Select a country" }
                            @for (id, name) in countries {
                                option value=(id) {
                                    (name)
                                }
                            }
                        }
                    }

                    div style="margin-bottom: 1.5rem;" {
                        label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                            "Photo URL"
                        }
                        input
                            type="url"
                            name="photo_path"
                            placeholder="https://example.com/photo.jpg"
                            style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;";
                        p style="font-size: 0.75rem; color: var(--gray-500); margin-top: 0.25rem;" {
                            "Optional: URL to player photo"
                        }
                    }

                    div style="display: flex; gap: 0.5rem; justify-content: flex-end;" {
                        button
                            type="button"
                            class="btn"
                            style="background: white; border: 1px solid var(--gray-300);"
                            onclick="document.getElementById('player-modal').remove()"
                        {
                            "Cancel"
                        }
                        button type="submit" class="btn btn-primary" {
                            "Create Player"
                        }
                    }
                }
            }
        }
        (PreEscaped(r#"
        <script>
            document.getElementById('player-modal').addEventListener('click', function(e) {
                if (e.target === this) {
                    this.remove();
                }
            });
        </script>
        "#))
    }
}

/// Edit player modal
pub fn player_edit_modal(
    player: &PlayerEntity,
    error: Option<&str>,
    countries: &[(i64, String)],
) -> Markup {
    html! {
        div
            class="modal-backdrop"
            style="position: fixed; top: 0; left: 0; right: 0; bottom: 0; background: rgba(0, 0, 0, 0.5); display: flex; align-items: center; justify-content: center; z-index: 1000;"
            id="player-modal"
        {
            div
                class="modal"
                style="background: white; border-radius: 12px; padding: 2rem; max-width: 500px; width: 90%; max-height: 90vh; overflow-y: auto;"
                onclick="event.stopPropagation()"
            {
                h2 style="margin-bottom: 1.5rem; font-size: 1.5rem; font-weight: 700;" {
                    "Edit Player"
                }

                @if let Some(error_msg) = error {
                    div class="error" style="margin-bottom: 1rem; padding: 0.75rem; background: #fee; border: 1px solid #fcc; border-radius: 4px; color: #c00;" {
                        (error_msg)
                    }
                }

                form hx-post=(format!("/players/{}", player.id)) hx-target="#player-modal" hx-swap="outerHTML" {
                    div style="margin-bottom: 1rem;" {
                        label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                            "Player Name"
                            span style="color: red;" { "*" }
                        }
                        input
                            type="text"
                            name="name"
                            value=(player.name)
                            required
                            autofocus
                            style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;";
                    }

                    div style="margin-bottom: 1rem;" {
                        label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                            "Country"
                            span style="color: red;" { "*" }
                        }
                        select
                            name="country_id"
                            required
                            style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;"
                        {
                            option value="" { "Select a country" }
                            @for (id, name) in countries {
                                option value=(id) selected[*id == player.country_id] {
                                    (name)
                                }
                            }
                        }
                    }

                    div style="margin-bottom: 1.5rem;" {
                        label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                            "Photo URL"
                        }
                        input
                            type="url"
                            name="photo_path"
                            value=[player.photo_path.as_ref()]
                            placeholder="https://example.com/photo.jpg"
                            style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;";
                        p style="font-size: 0.75rem; color: var(--gray-500); margin-top: 0.25rem;" {
                            "Optional: URL to player photo"
                        }
                    }

                    div style="display: flex; gap: 0.5rem; justify-content: flex-end;" {
                        button
                            type="button"
                            class="btn"
                            style="background: white; border: 1px solid var(--gray-300);"
                            onclick="document.getElementById('player-modal').remove()"
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
            document.getElementById('player-modal').addEventListener('click', function(e) {
                if (e.target === this) {
                    this.remove();
                }
            });
        </script>
        "#))
    }
}
