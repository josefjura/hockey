use maud::{html, Markup};

use crate::common::pagination::{PagedResult, SortOrder};
use crate::i18n::TranslationContext;
use crate::service::matches::{MatchEntity, MatchFilters, SortField};
use crate::views::components::confirm::{confirm_attrs, ConfirmVariant};
use crate::views::components::crud::{empty_state_i18n, page_header_i18n, pagination};

use super::detail_page::{format_date, status_badge};

/// Main matches page with table and filters
pub fn matches_page(
    t: &TranslationContext,
    result: &PagedResult<MatchEntity>,
    filters: &MatchFilters,
    sort_field: &SortField,
    sort_order: &SortOrder,
    seasons: &[(i64, String)],
    teams: &[(i64, String)],
) -> Markup {
    html! {
        div class="card" {
            // Header with title and create button
            (page_header_i18n(
                &t.messages.matches_title().to_string(),
                &t.messages.matches_description().to_string(),
                "/matches/new",
                &t.messages.matches_new().to_string()
            ))

            // Filters
            div style="margin-bottom: 1.5rem; padding: 1rem; background: var(--gray-50); border-radius: 8px;" {
                form hx-get="/matches/list" hx-target="#matches-table" hx-swap="outerHTML" hx-trigger="submit, change delay:300ms" {
                    div style="display: grid; grid-template-columns: repeat(3, 1fr) auto; gap: 1rem; align-items: end;" {
                        // Season filter
                        div {
                            label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                                (t.messages.matches_filter_season())
                            }
                            select
                                name="season_id"
                                style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;"
                            {
                                option value="" { (t.messages.matches_all_seasons()) }
                                @for (id, name) in seasons {
                                    option
                                        value=(id)
                                        selected[filters.season_id == Some(*id)]
                                    {
                                        (name)
                                    }
                                }
                            }
                        }

                        // Team filter
                        div {
                            label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                                (t.messages.matches_filter_team())
                            }
                            select
                                name="team_id"
                                style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;"
                            {
                                option value="" { (t.messages.matches_all_teams()) }
                                @for (id, name) in teams {
                                    option
                                        value=(id)
                                        selected[filters.team_id == Some(*id)]
                                    {
                                        (name)
                                    }
                                }
                            }
                        }

                        // Status filter
                        div {
                            label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                                (t.messages.matches_filter_status())
                            }
                            select
                                name="status"
                                style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;"
                            {
                                option value="" { (t.messages.matches_all_statuses()) }
                                option
                                    value="scheduled"
                                    selected[filters.status.as_deref() == Some("scheduled")]
                                {
                                    (t.messages.matches_status_scheduled())
                                }
                                option
                                    value="in_progress"
                                    selected[filters.status.as_deref() == Some("in_progress")]
                                {
                                    (t.messages.matches_status_in_progress())
                                }
                                option
                                    value="finished"
                                    selected[filters.status.as_deref() == Some("finished")]
                                {
                                    (t.messages.matches_status_finished())
                                }
                                option
                                    value="cancelled"
                                    selected[filters.status.as_deref() == Some("cancelled")]
                                {
                                    (t.messages.matches_status_cancelled())
                                }
                            }
                        }

                        // Clear button
                        div {
                            button
                                type="button"
                                class="btn btn-secondary"
                                hx-get="/matches/list"
                                hx-target="#matches-table"
                                hx-swap="outerHTML"
                            {
                                (t.messages.common_clear())
                            }
                        }
                    }

                    // Date range filters (second row)
                    div style="display: grid; grid-template-columns: repeat(2, 1fr); gap: 1rem; margin-top: 1rem;" {
                        div {
                            label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                                (t.messages.matches_filter_date_from())
                            }
                            input
                                type="date"
                                name="date_from"
                                value=[filters.date_from.as_ref()]
                                style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;";
                        }

                        div {
                            label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                                (t.messages.matches_filter_date_to())
                            }
                            input
                                type="date"
                                name="date_to"
                                value=[filters.date_to.as_ref()]
                                style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;";
                        }
                    }
                }
            }

            // Table
            (match_list_content(t, result, filters, sort_field, sort_order))

            // Modal container
            div id="modal-container" {}
        }
    }
}

/// Matches table content (for HTMX updates)
pub fn match_list_content(
    t: &TranslationContext,
    result: &PagedResult<MatchEntity>,
    filters: &MatchFilters,
    sort_field: &SortField,
    sort_order: &SortOrder,
) -> Markup {
    html! {
        div id="matches-table" class="loading-overlay" {
            // Loading spinner overlay
            div class="loading-spinner-overlay" {
                hockey-loading-spinner size="lg" {}
            }

            @if result.items.is_empty() {
                (empty_state_i18n(
                    &t.messages.matches_empty_title().to_string(),
                    &t.messages.matches_empty_message().to_string(),
                    filters.season_id.is_some() || filters.team_id.is_some() || filters.status.is_some() || filters.date_from.is_some() || filters.date_to.is_some()
                ))
            } @else {
                table class="table" {
                    thead {
                        tr {
                            th {
                                (sortable_header(
                                    &t.messages.matches_date().to_string(),
                                    &SortField::Date,
                                    sort_field,
                                    sort_order,
                                    filters,
                                ))
                            }
                            th {
                                (sortable_header(
                                    &t.messages.events_title().to_string(),
                                    &SortField::Event,
                                    sort_field,
                                    sort_order,
                                    filters,
                                ))
                            }
                            th { (t.messages.nav_matches()) }
                            th { (t.messages.matches_score()) }
                            th {
                                (sortable_header(
                                    &t.messages.matches_status().to_string(),
                                    &SortField::Status,
                                    sort_field,
                                    sort_order,
                                    filters,
                                ))
                            }
                            th style="text-align: right;" { (t.messages.common_actions()) }
                        }
                    }
                    tbody {
                        @for match_item in &result.items {
                            tr {
                                // Date
                                td {
                                    @if let Some(date) = &match_item.match_date {
                                        (format_date(date))
                                    } @else {
                                        span style="color: var(--gray-400); font-style: italic;" { "TBD" }
                                    }
                                }

                                // Event
                                td {
                                    @if let Some(event_name) = &match_item.event_name {
                                        (event_name)
                                    } @else {
                                        span style="color: var(--gray-400); font-style: italic;" { "-" }
                                    }
                                }

                                // Match (teams)
                                td {
                                    div style="display: flex; flex-direction: column; gap: 0.25rem;" {
                                        div style="display: flex; align-items: center; gap: 0.5rem;" {
                                            @if let Some(iso2) = &match_item.home_team_country_iso2 {
                                                flag-icon
                                                    country-code=(iso2.to_lowercase())
                                                    country-name=(match_item.home_team_name)
                                                    size="sm";
                                            }
                                            span { (match_item.home_team_name) }
                                        }
                                        div style="display: flex; align-items: center; gap: 0.5rem;" {
                                            @if let Some(iso2) = &match_item.away_team_country_iso2 {
                                                flag-icon
                                                    country-code=(iso2.to_lowercase())
                                                    country-name=(match_item.away_team_name)
                                                    size="sm";
                                            }
                                            span { (match_item.away_team_name) }
                                        }
                                    }
                                }

                                // Score
                                td {
                                    div style="font-weight: 600; font-size: 1.1rem;" {
                                        (match_item.home_score_unidentified)
                                        " : "
                                        (match_item.away_score_unidentified)
                                    }
                                }

                                // Status
                                td {
                                    (status_badge(&match_item.status))
                                }

                                // Actions
                                td style="text-align: right;" {
                                    a
                                        href=(format!("/matches/{}", match_item.id))
                                        class="btn btn-sm"
                                        style="margin-right: 0.5rem;"
                                    {
                                        (t.messages.matches_view())
                                    }
                                    button
                                        class="btn btn-sm"
                                        hx-get=(format!("/matches/{}/edit", match_item.id))
                                        hx-target="#modal-container"
                                        hx-swap="innerHTML"
                                        style="margin-right: 0.5rem;"
                                    {
                                        (t.messages.common_edit())
                                    }
                                    button
                                        class="btn btn-sm btn-danger"
                                        hx-post=(build_delete_url(match_item.id, filters, sort_field, sort_order))
                                        hx-target="#matches-table"
                                        hx-swap="outerHTML"
                                        hx-confirm-custom=(confirm_attrs(
                                            &t.messages.matches_delete().to_string(),
                                            &t.messages.matches_confirm_delete().to_string(),
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
                (pagination(
                    result,
                    "matches",
                    |page| build_pagination_url(page, result.page_size, filters, sort_field, sort_order),
                    "matches-table"
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
    filters: &MatchFilters,
) -> Markup {
    // Determine if this column is currently sorted
    let is_active = matches!(
        (field, current_sort),
        (SortField::Date, SortField::Date)
            | (SortField::Status, SortField::Status)
            | (SortField::Event, SortField::Event)
    );

    // If this column is active, toggle the order; otherwise default to DESC for date, ASC for others
    let next_order = if is_active {
        current_order.toggle()
    } else {
        match field {
            SortField::Date => SortOrder::Desc,
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
            hx-target="#matches-table"
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
fn build_sort_url(field: &SortField, order: &SortOrder, filters: &MatchFilters) -> String {
    let mut url = format!(
        "/matches/list?sort={}&order={}",
        field.as_str(),
        order.as_str()
    );

    if let Some(season_id) = filters.season_id {
        url.push_str(&format!("&season_id={}", season_id));
    }

    if let Some(team_id) = filters.team_id {
        url.push_str(&format!("&team_id={}", team_id));
    }

    if let Some(status) = &filters.status {
        url.push_str(&format!("&status={}", urlencoding::encode(status)));
    }

    if let Some(date_from) = &filters.date_from {
        url.push_str(&format!("&date_from={}", urlencoding::encode(date_from)));
    }

    if let Some(date_to) = &filters.date_to {
        url.push_str(&format!("&date_to={}", urlencoding::encode(date_to)));
    }

    url
}

/// Helper to build pagination URLs
pub fn build_pagination_url(
    page: usize,
    page_size: usize,
    filters: &MatchFilters,
    sort_field: &SortField,
    sort_order: &SortOrder,
) -> String {
    let mut url = format!(
        "/matches/list?page={}&page_size={}&sort={}&order={}",
        page,
        page_size,
        sort_field.as_str(),
        sort_order.as_str()
    );

    if let Some(season_id) = filters.season_id {
        url.push_str(&format!("&season_id={}", season_id));
    }

    if let Some(team_id) = filters.team_id {
        url.push_str(&format!("&team_id={}", team_id));
    }

    if let Some(status) = &filters.status {
        url.push_str(&format!("&status={}", urlencoding::encode(status)));
    }

    if let Some(date_from) = &filters.date_from {
        url.push_str(&format!("&date_from={}", urlencoding::encode(date_from)));
    }

    if let Some(date_to) = &filters.date_to {
        url.push_str(&format!("&date_to={}", urlencoding::encode(date_to)));
    }

    url
}

/// Helper to build delete URLs with current filters
fn build_delete_url(
    id: i64,
    filters: &MatchFilters,
    sort_field: &SortField,
    sort_order: &SortOrder,
) -> String {
    let mut url = format!(
        "/matches/{}/delete?sort={}&order={}",
        id,
        sort_field.as_str(),
        sort_order.as_str()
    );

    if let Some(season_id) = filters.season_id {
        url.push_str(&format!("&season_id={}", season_id));
    }

    if let Some(team_id) = filters.team_id {
        url.push_str(&format!("&team_id={}", team_id));
    }

    if let Some(status) = &filters.status {
        url.push_str(&format!("&status={}", urlencoding::encode(status)));
    }

    if let Some(date_from) = &filters.date_from {
        url.push_str(&format!("&date_from={}", urlencoding::encode(date_from)));
    }

    if let Some(date_to) = &filters.date_to {
        url.push_str(&format!("&date_to={}", urlencoding::encode(date_to)));
    }

    url
}
