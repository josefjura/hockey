use maud::{html, Markup};

use crate::i18n::TranslationContext;
use crate::service::players::{PagedResult, PlayerEntity, PlayerFilters, SortField, SortOrder};
use crate::views::components::crud::{
    empty_state, modal_form_multipart, page_header, pagination, table_actions,
};

/// Main players page with table and filters
pub fn players_page(
    t: &TranslationContext,
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
                &t.messages.players_title().to_string(),
                &t.messages.players_description().to_string(),
                "/players/new",
                &t.messages.players_create().to_string()
            ))

            // Filters
            div style="margin-bottom: 1.5rem; padding: 1rem; background: var(--gray-50); border-radius: 8px;" {
                form hx-get="/players/list" hx-target="#players-table" hx-swap="outerHTML" hx-trigger="submit, change delay:300ms" {
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
                                placeholder=(t.messages.players_name_placeholder())
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
                                class="btn btn-secondary"
                                hx-get="/players/list"
                                hx-target="#players-table"
                                hx-swap="outerHTML"
                            {
                                (t.messages.common_clear())
                            }
                        }
                    }
                }
            }

            // Table
            (player_list_content(t, result, filters, sort_field, sort_order))

            // Modal container
            div id="modal-container" {}
        }
    }
}

/// Players table content (for HTMX updates)
pub fn player_list_content(
    t: &TranslationContext,
    result: &PagedResult<PlayerEntity>,
    filters: &PlayerFilters,
    sort_field: &SortField,
    sort_order: &SortOrder,
) -> Markup {
    html! {
        div id="players-table" class="loading-overlay" {
            // Loading spinner overlay
            div class="loading-spinner-overlay" {
                hockey-loading-spinner size="lg" {}
            }

            @if result.items.is_empty() {
                (empty_state(
                    &t.messages.players_entity().to_string(),
                    filters.name.is_some() || filters.country_id.is_some()
                ))
            } @else {
                table class="table" {
                    thead {
                        tr {
                            th { (t.messages.form_photo()) }
                            th {
                                (sortable_header(
                                    &t.messages.common_id().to_string(),
                                    &SortField::Id,
                                    sort_field,
                                    sort_order,
                                    filters,
                                ))
                            }
                            th {
                                (sortable_header(
                                    &t.messages.form_name().to_string(),
                                    &SortField::Name,
                                    sort_field,
                                    sort_order,
                                    filters,
                                ))
                            }
                            th {
                                (sortable_header(
                                    &t.messages.form_country().to_string(),
                                    &SortField::Country,
                                    sort_field,
                                    sort_order,
                                    filters,
                                ))
                            }
                            th style="text-align: right;" { (t.messages.common_actions()) }
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
                                td {
                                    a
                                        href=(format!("/players/{}", player.id))
                                        style="color: var(--primary-color); text-decoration: none; font-weight: 500;"
                                    {
                                        (player.name)
                                    }
                                }
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
                                    &t.messages.players_entity().to_string()
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
    let mut url = format!(
        "/players/list?sort={}&order={}",
        field.as_str(),
        order.as_str()
    );

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
pub fn player_create_modal(
    t: &TranslationContext,
    error: Option<&str>,
    _countries: &[(i64, String)],
) -> Markup {
    let form_fields = html! {
        div style="margin-bottom: 1rem;" {
            label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                (t.messages.players_name_label())
                span style="color: red;" { "*" }
            }
            input
                type="text"
                name="name"
                required
                style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;";
        }

        div style="margin-bottom: 1rem;" {
            label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                (t.messages.form_country())
                span style="color: red;" { "*" }
            }
            country-selector
                name="country_id"
                placeholder=(t.messages.players_select_country())
                enabled-only
                required;
        }

        div style="margin-bottom: 1rem;" {
            label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                (t.messages.players_upload_photo())
            }
            input
                type="file"
                name="photo_file"
                accept="image/jpeg,image/png,image/gif,image/webp"
                style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;";
            p style="font-size: 0.75rem; color: var(--gray-500); margin-top: 0.25rem;" {
                (t.messages.players_photo_hint())
            }
        }

        div style="margin-bottom: 1.5rem;" {
            label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                (t.messages.players_photo_url())
            }
            input
                type="url"
                name="photo_url"
                placeholder="https://example.com/photo.jpg"
                style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;";
            p style="font-size: 0.75rem; color: var(--gray-500); margin-top: 0.25rem;" {
                (t.messages.players_photo_url_hint())
            }
        }

        // Biographical Information Section
        h3 style="margin: 1.5rem 0 1rem; font-size: 1rem; font-weight: 600; color: var(--gray-700);" {
            "Biographical Information"
        }

        div style="display: grid; grid-template-columns: 1fr 1fr; gap: 1rem; margin-bottom: 1rem;" {
            div {
                label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                    "Date of Birth"
                }
                input
                    type="date"
                    name="birth_date"
                    style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;";
            }
            div {
                label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                    "Birthplace"
                }
                input
                    type="text"
                    name="birth_place"
                    placeholder="City, Country"
                    style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;";
            }
        }

        div style="display: grid; grid-template-columns: 1fr 1fr; gap: 1rem; margin-bottom: 1rem;" {
            div {
                label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                    "Height (cm)"
                }
                input
                    type="number"
                    name="height_cm"
                    placeholder="180"
                    min="0"
                    style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;";
            }
            div {
                label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                    "Weight (kg)"
                }
                input
                    type="number"
                    name="weight_kg"
                    placeholder="80"
                    min="0"
                    style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;";
            }
        }

        div style="display: grid; grid-template-columns: 1fr 1fr; gap: 1rem; margin-bottom: 1rem;" {
            div {
                label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                    "Position"
                }
                select
                    name="position"
                    style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;"
                {
                    option value="" { "Select position..." }
                    option value="Forward" { "Forward" }
                    option value="Defense" { "Defense" }
                    option value="Goalie" { "Goalie" }
                }
            }
            div {
                label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                    "Shoots/Catches"
                }
                select
                    name="shoots"
                    style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;"
                {
                    option value="" { "Select..." }
                    option value="Left" { "Left" }
                    option value="Right" { "Right" }
                }
            }
        }
    };

    modal_form_multipart(
        "player-modal",
        &t.messages.players_create_title().to_string(),
        error,
        "/players",
        form_fields,
        &t.messages.players_create_submit().to_string(),
    )
}

/// Edit player modal
pub fn player_edit_modal(
    t: &TranslationContext,
    player: &PlayerEntity,
    error: Option<&str>,
    _countries: &[(i64, String)],
) -> Markup {
    let form_fields = html! {
        div style="margin-bottom: 1rem;" {
            label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                (t.messages.players_name_label())
                span style="color: red;" { "*" }
            }
            input
                type="text"
                name="name"
                value=(player.name)
                required
                style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;";
        }

        div style="margin-bottom: 1rem;" {
            label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                (t.messages.form_country())
                span style="color: red;" { "*" }
            }
            country-selector
                name="country_id"
                value=(player.country_id)
                placeholder=(t.messages.players_select_country())
                enabled-only
                required;
        }

        @if let Some(current_photo) = &player.photo_path {
            div style="margin-bottom: 1rem;" {
                label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                    (t.messages.players_current_photo())
                }
                img
                    src=(current_photo)
                    alt=(t.messages.players_current_photo())
                    style="max-width: 200px; max-height: 200px; border-radius: 8px; border: 1px solid var(--gray-300);"
                    onerror="this.style.display='none'";
            }
        }

        div style="margin-bottom: 1rem;" {
            label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                (t.messages.players_upload_new_photo())
            }
            input
                type="file"
                name="photo_file"
                accept="image/jpeg,image/png,image/gif,image/webp"
                style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;";
            p style="font-size: 0.75rem; color: var(--gray-500); margin-top: 0.25rem;" {
                (t.messages.players_photo_hint())
            }
        }

        div style="margin-bottom: 1.5rem;" {
            label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                (t.messages.players_photo_url())
            }
            input
                type="url"
                name="photo_url"
                value=[player.photo_path.as_ref()]
                placeholder="https://example.com/photo.jpg"
                style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;";
            p style="font-size: 0.75rem; color: var(--gray-500); margin-top: 0.25rem;" {
                (t.messages.players_photo_url_hint())
            }
        }

        // Biographical Information Section
        h3 style="margin: 1.5rem 0 1rem; font-size: 1rem; font-weight: 600; color: var(--gray-700);" {
            "Biographical Information"
        }

        div style="display: grid; grid-template-columns: 1fr 1fr; gap: 1rem; margin-bottom: 1rem;" {
            div {
                label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                    "Date of Birth"
                }
                input
                    type="date"
                    name="birth_date"
                    value=[player.birth_date.as_ref()]
                    style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;";
            }
            div {
                label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                    "Birthplace"
                }
                input
                    type="text"
                    name="birth_place"
                    value=[player.birth_place.as_ref()]
                    placeholder="City, Country"
                    style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;";
            }
        }

        div style="display: grid; grid-template-columns: 1fr 1fr; gap: 1rem; margin-bottom: 1rem;" {
            div {
                label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                    "Height (cm)"
                }
                input
                    type="number"
                    name="height_cm"
                    value=[player.height_cm]
                    placeholder="180"
                    min="0"
                    style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;";
            }
            div {
                label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                    "Weight (kg)"
                }
                input
                    type="number"
                    name="weight_kg"
                    value=[player.weight_kg]
                    placeholder="80"
                    min="0"
                    style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;";
            }
        }

        div style="display: grid; grid-template-columns: 1fr 1fr; gap: 1rem; margin-bottom: 1rem;" {
            div {
                label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                    "Position"
                }
                select
                    name="position"
                    style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;"
                {
                    option value="" selected[player.position.is_none()] { "Select position..." }
                    option value="Forward" selected[player.position.as_deref() == Some("Forward")] { "Forward" }
                    option value="Defense" selected[player.position.as_deref() == Some("Defense")] { "Defense" }
                    option value="Goalie" selected[player.position.as_deref() == Some("Goalie")] { "Goalie" }
                }
            }
            div {
                label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                    "Shoots/Catches"
                }
                select
                    name="shoots"
                    style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;"
                {
                    option value="" selected[player.shoots.is_none()] { "Select..." }
                    option value="Left" selected[player.shoots.as_deref() == Some("Left")] { "Left" }
                    option value="Right" selected[player.shoots.as_deref() == Some("Right")] { "Right" }
                }
            }
        }
    };

    modal_form_multipart(
        "player-modal",
        &t.messages.players_edit_title().to_string(),
        error,
        &format!("/players/{}", player.id),
        form_fields,
        &t.messages.common_save().to_string(),
    )
}
