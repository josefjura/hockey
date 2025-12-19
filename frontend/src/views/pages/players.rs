use maud::{html, Markup};

use crate::service::players::{PagedResult, PlayerEntity, PlayerFilters, SortField, SortOrder};
use crate::views::components::crud::{empty_state, modal_form_multipart, page_header, pagination, table_actions};

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
            (page_header(
                "Players",
                "Manage and view all players in the system.",
                "/players/new",
                "+ New Player"
            ))

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
                (empty_state(
                    "players",
                    filters.name.is_some() || filters.country_id.is_some()
                ))
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
                                (table_actions(
                                    &format!("/players/{}/edit", player.id),
                                    &build_delete_url(player.id, filters, sort_field, sort_order),
                                    "players-table",
                                    "player"
                                ))
                            }
                        }
                    }
                }

                // Pagination
                (pagination(
                    result,
                    "players",
                    |page| build_pagination_url(page, result.page_size, filters, sort_field, sort_order),
                    "players-table"
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

/// Helper to build delete URL with current filters and sorting
fn build_delete_url(
    player_id: i64,
    filters: &PlayerFilters,
    sort_field: &SortField,
    sort_order: &SortOrder,
) -> String {
    let mut url = format!(
        "/players/{}/delete?sort={}&order={}",
        player_id,
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
    let form_fields = html! {
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

        div style="margin-bottom: 1rem;" {
            label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                "Upload Photo"
            }
            input
                type="file"
                name="photo_file"
                accept="image/jpeg,image/png,image/gif,image/webp"
                style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;";
            p style="font-size: 0.75rem; color: var(--gray-500); margin-top: 0.25rem;" {
                "Optional: Upload player photo (JPG, PNG, GIF, or WebP)"
            }
        }

        div style="margin-bottom: 1.5rem;" {
            label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                "Or Photo URL"
            }
            input
                type="url"
                name="photo_url"
                placeholder="https://example.com/photo.jpg"
                style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;";
            p style="font-size: 0.75rem; color: var(--gray-500); margin-top: 0.25rem;" {
                "Optional: URL to player photo (used if no file uploaded)"
            }
        }
    };

    modal_form_multipart(
        "player-modal",
        "Create Player",
        error,
        "/players",
        form_fields,
        "Create Player"
    )
}

/// Edit player modal
pub fn player_edit_modal(
    player: &PlayerEntity,
    error: Option<&str>,
    countries: &[(i64, String)],
) -> Markup {
    let form_fields = html! {
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

        @if let Some(current_photo) = &player.photo_path {
            div style="margin-bottom: 1rem;" {
                label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                    "Current Photo"
                }
                img
                    src=(current_photo)
                    alt="Current player photo"
                    style="max-width: 200px; max-height: 200px; border-radius: 8px; border: 1px solid var(--gray-300);"
                    onerror="this.style.display='none'";
            }
        }

        div style="margin-bottom: 1rem;" {
            label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                "Upload New Photo"
            }
            input
                type="file"
                name="photo_file"
                accept="image/jpeg,image/png,image/gif,image/webp"
                style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;";
            p style="font-size: 0.75rem; color: var(--gray-500); margin-top: 0.25rem;" {
                "Optional: Upload new player photo (JPG, PNG, GIF, or WebP)"
            }
        }

        div style="margin-bottom: 1.5rem;" {
            label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                "Or Photo URL"
            }
            input
                type="url"
                name="photo_url"
                value=[player.photo_path.as_ref()]
                placeholder="https://example.com/photo.jpg"
                style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;";
            p style="font-size: 0.75rem; color: var(--gray-500); margin-top: 0.25rem;" {
                "Optional: URL to player photo (used if no file uploaded)"
            }
        }
    };

    modal_form_multipart(
        "player-modal",
        "Edit Player",
        error,
        &format!("/players/{}", player.id),
        form_fields,
        "Save Changes"
    )
}
