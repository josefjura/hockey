use maud::{html, Markup};

use crate::common::pagination::{PagedResult, SortOrder};
use crate::service::matches::{MatchDetailEntity, MatchEntity, MatchFilters, ScoreEventEntity, SortField};
use crate::views::components::crud::{empty_state, modal_form, page_header, pagination, table_actions};

/// Main matches page with table and filters
pub fn matches_page(
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
            (page_header(
                "Matches",
                "Manage and view all matches in the system.",
                "/matches/new",
                "+ New Match"
            ))

            // Filters
            div style="margin-bottom: 1.5rem; padding: 1rem; background: var(--gray-50); border-radius: 8px;" {
                form hx-get="/matches/list" hx-target="#matches-table" hx-swap="outerHTML" hx-trigger="submit, change delay:300ms" {
                    div style="display: grid; grid-template-columns: repeat(3, 1fr) auto; gap: 1rem; align-items: end;" {
                        // Season filter
                        div {
                            label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                                "Season"
                            }
                            select
                                name="season_id"
                                style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;"
                            {
                                option value="" { "All seasons" }
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
                                "Team"
                            }
                            select
                                name="team_id"
                                style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;"
                            {
                                option value="" { "All teams" }
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
                                "Status"
                            }
                            select
                                name="status"
                                style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;"
                            {
                                option value="" { "All statuses" }
                                option
                                    value="scheduled"
                                    selected[filters.status.as_deref() == Some("scheduled")]
                                {
                                    "Scheduled"
                                }
                                option
                                    value="in_progress"
                                    selected[filters.status.as_deref() == Some("in_progress")]
                                {
                                    "In Progress"
                                }
                                option
                                    value="finished"
                                    selected[filters.status.as_deref() == Some("finished")]
                                {
                                    "Finished"
                                }
                                option
                                    value="cancelled"
                                    selected[filters.status.as_deref() == Some("cancelled")]
                                {
                                    "Cancelled"
                                }
                            }
                        }

                        // Clear button
                        div {
                            button
                                type="button"
                                class="btn"
                                style="background: white; border: 1px solid var(--gray-300);"
                                hx-get="/matches/list"
                                hx-target="#matches-table"
                                hx-swap="outerHTML"
                            {
                                "Clear"
                            }
                        }
                    }

                    // Date range filters (second row)
                    div style="display: grid; grid-template-columns: repeat(2, 1fr); gap: 1rem; margin-top: 1rem;" {
                        div {
                            label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                                "From Date"
                            }
                            input
                                type="date"
                                name="date_from"
                                value=[filters.date_from.as_ref()]
                                style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;";
                        }

                        div {
                            label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                                "To Date"
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
            (match_list_content(result, filters, sort_field, sort_order))

            // Modal container
            div id="modal-container" {}
        }
    }
}

/// Matches table content (for HTMX updates)
pub fn match_list_content(
    result: &PagedResult<MatchEntity>,
    filters: &MatchFilters,
    sort_field: &SortField,
    sort_order: &SortOrder,
) -> Markup {
    html! {
        div id="matches-table" {
            @if result.items.is_empty() {
                (empty_state(
                    "matches",
                    filters.season_id.is_some() || filters.team_id.is_some() || filters.status.is_some() || filters.date_from.is_some() || filters.date_to.is_some()
                ))
            } @else {
                table class="table" {
                    thead {
                        tr {
                            th {
                                (sortable_header(
                                    "Date",
                                    &SortField::Date,
                                    sort_field,
                                    sort_order,
                                    filters,
                                ))
                            }
                            th {
                                (sortable_header(
                                    "Event",
                                    &SortField::Event,
                                    sort_field,
                                    sort_order,
                                    filters,
                                ))
                            }
                            th { "Match" }
                            th { "Score" }
                            th {
                                (sortable_header(
                                    "Status",
                                    &SortField::Status,
                                    sort_field,
                                    sort_order,
                                    filters,
                                ))
                            }
                            th style="text-align: right;" { "Actions" }
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
                                                img
                                                    src=(format!("https://flagcdn.com/w40/{}.png", iso2.to_lowercase()))
                                                    alt=(match_item.home_team_name)
                                                    style="width: 20px; height: 15px; object-fit: cover; border: 1px solid var(--gray-300);"
                                                    onerror="this.style.display='none'";
                                            }
                                            span { (match_item.home_team_name) }
                                        }
                                        div style="display: flex; align-items: center; gap: 0.5rem;" {
                                            @if let Some(iso2) = &match_item.away_team_country_iso2 {
                                                img
                                                    src=(format!("https://flagcdn.com/w40/{}.png", iso2.to_lowercase()))
                                                    alt=(match_item.away_team_name)
                                                    style="width: 20px; height: 15px; object-fit: cover; border: 1px solid var(--gray-300);"
                                                    onerror="this.style.display='none'";
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
                                (table_actions(
                                    &format!("/matches/{}", match_item.id),
                                    &build_delete_url(match_item.id, filters, sort_field, sort_order),
                                    "matches-table",
                                    "match"
                                ))
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
    let mut url = format!("/matches/list?sort={}&order={}", field.as_str(), order.as_str());

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
fn build_pagination_url(
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

/// Match detail page with score tracking
pub fn match_detail_page(detail: &MatchDetailEntity) -> Markup {
    let match_info = &detail.match_info;

    html! {
        div class="card" {
            // Header with back button and action buttons
            div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 1.5rem;" {
                div style="display: flex; align-items: center; gap: 1rem;" {
                    a
                        href="/matches"
                        class="btn"
                        style="background: white; border: 1px solid var(--gray-300);"
                    {
                        "← Back to Matches"
                    }
                    h1 style="font-size: 2rem; font-weight: 700; margin: 0;" {
                        "Match Detail"
                    }
                }
                div style="display: flex; gap: 0.5rem;" {
                    button
                        class="btn btn-primary"
                        hx-get=(format!("/matches/{}/edit", match_info.id))
                        hx-target="#modal-container"
                        hx-swap="innerHTML"
                    {
                        "Edit Match"
                    }
                    button
                        class="btn btn-danger"
                        hx-post=(format!("/matches/{}/delete", match_info.id))
                        hx-confirm="Are you sure you want to delete this match? This will also delete all score events."
                    {
                        "Delete Match"
                    }
                }
            }

            // Match Info Card
            div style="margin-bottom: 2rem; padding: 1.5rem; background: var(--gray-50); border-radius: 8px;" {
                // Match Score
                div style="text-align: center; margin-bottom: 1.5rem;" {
                    div style="display: flex; justify-content: center; align-items: center; gap: 2rem; margin-bottom: 1rem;" {
                        // Home Team
                        div style="flex: 1; text-align: right;" {
                            div style="display: flex; justify-content: flex-end; align-items: center; gap: 0.5rem; margin-bottom: 0.5rem;" {
                                @if let Some(iso2) = &match_info.home_team_country_iso2 {
                                    img
                                        src=(format!("https://flagcdn.com/w40/{}.png", iso2.to_lowercase()))
                                        alt=(match_info.home_team_name)
                                        style="width: 24px; height: 18px; object-fit: cover; border: 1px solid var(--gray-300);"
                                        onerror="this.style.display='none'";
                                }
                                span style="font-size: 1.5rem; font-weight: 600;" {
                                    (match_info.home_team_name)
                                }
                            }
                        }

                        // Score
                        div style="font-size: 3rem; font-weight: 700; padding: 0 2rem;" {
                            (detail.home_score_total)
                            " : "
                            (detail.away_score_total)
                        }

                        // Away Team
                        div style="flex: 1; text-align: left;" {
                            div style="display: flex; justify-content: flex-start; align-items: center; gap: 0.5rem; margin-bottom: 0.5rem;" {
                                span style="font-size: 1.5rem; font-weight: 600;" {
                                    (match_info.away_team_name)
                                }
                                @if let Some(iso2) = &match_info.away_team_country_iso2 {
                                    img
                                        src=(format!("https://flagcdn.com/w40/{}.png", iso2.to_lowercase()))
                                        alt=(match_info.away_team_name)
                                        style="width: 24px; height: 18px; object-fit: cover; border: 1px solid var(--gray-300);"
                                        onerror="this.style.display='none'";
                                }
                            }
                        }
                    }

                    // Status Badge
                    div {
                        (status_badge(&match_info.status))
                    }
                }

                // Match Information Grid
                div style="display: grid; grid-template-columns: repeat(2, 1fr); gap: 1rem; margin-top: 1.5rem; padding-top: 1.5rem; border-top: 1px solid var(--gray-200);" {
                    div {
                        div style="color: var(--gray-600); font-size: 0.875rem; margin-bottom: 0.25rem;" {
                            "Event"
                        }
                        div style="font-weight: 500;" {
                            @if let Some(event_name) = &match_info.event_name {
                                (event_name)
                            } @else {
                                span style="color: var(--gray-400); font-style: italic;" { "Unknown" }
                            }
                        }
                    }
                    div {
                        div style="color: var(--gray-600); font-size: 0.875rem; margin-bottom: 0.25rem;" {
                            "Season"
                        }
                        div style="font-weight: 500;" {
                            @if let Some(season_name) = &match_info.season_name {
                                (season_name)
                            } @else {
                                span style="color: var(--gray-400); font-style: italic;" { "Unknown" }
                            }
                        }
                    }
                    div {
                        div style="color: var(--gray-600); font-size: 0.875rem; margin-bottom: 0.25rem;" {
                            "Date"
                        }
                        div style="font-weight: 500;" {
                            @if let Some(date) = &match_info.match_date {
                                (format_date(date))
                            } @else {
                                span style="color: var(--gray-400); font-style: italic;" { "Not scheduled" }
                            }
                        }
                    }
                    div {
                        div style="color: var(--gray-600); font-size: 0.875rem; margin-bottom: 0.25rem;" {
                            "Venue"
                        }
                        div style="font-weight: 500;" {
                            @if let Some(venue) = &match_info.venue {
                                (venue)
                            } @else {
                                span style="color: var(--gray-400); font-style: italic;" { "TBD" }
                            }
                        }
                    }
                }
            }

            // Score Breakdown
            div style="margin-bottom: 2rem;" {
                h2 style="font-size: 1.5rem; font-weight: 700; margin-bottom: 1rem;" {
                    "Score Breakdown"
                }
                div style="display: grid; grid-template-columns: repeat(2, 1fr); gap: 1rem;" {
                    // Home Team Breakdown
                    div style="padding: 1rem; background: var(--gray-50); border-radius: 8px;" {
                        div style="font-weight: 600; margin-bottom: 0.5rem;" {
                            (match_info.home_team_name)
                        }
                        div style="display: flex; justify-content: space-between; padding: 0.5rem 0; border-bottom: 1px solid var(--gray-200);" {
                            span { "Identified goals" }
                            span style="font-weight: 600;" { (detail.home_score_identified) }
                        }
                        div style="display: flex; justify-content: space-between; padding: 0.5rem 0; border-bottom: 1px solid var(--gray-200);" {
                            span { "Unidentified goals" }
                            span style="font-weight: 600;" { (match_info.home_score_unidentified) }
                        }
                        div style="display: flex; justify-content: space-between; padding: 0.5rem 0; font-size: 1.125rem;" {
                            span style="font-weight: 600;" { "Total" }
                            span style="font-weight: 700;" { (detail.home_score_total) }
                        }
                    }

                    // Away Team Breakdown
                    div style="padding: 1rem; background: var(--gray-50); border-radius: 8px;" {
                        div style="font-weight: 600; margin-bottom: 0.5rem;" {
                            (match_info.away_team_name)
                        }
                        div style="display: flex; justify-content: space-between; padding: 0.5rem 0; border-bottom: 1px solid var(--gray-200);" {
                            span { "Identified goals" }
                            span style="font-weight: 600;" { (detail.away_score_identified) }
                        }
                        div style="display: flex; justify-content: space-between; padding: 0.5rem 0; border-bottom: 1px solid var(--gray-200);" {
                            span { "Unidentified goals" }
                            span style="font-weight: 600;" { (match_info.away_score_unidentified) }
                        }
                        div style="display: flex; justify-content: space-between; padding: 0.5rem 0; font-size: 1.125rem;" {
                            span style="font-weight: 600;" { "Total" }
                            span style="font-weight: 700;" { (detail.away_score_total) }
                        }
                    }
                }
            }

            // Score Events (Goals)
            div {
                div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 1rem;" {
                    h2 style="font-size: 1.5rem; font-weight: 700; margin: 0;" {
                        "Goals"
                    }
                    button
                        class="btn btn-primary"
                        hx-get=(format!("/matches/{}/score-events/new", match_info.id))
                        hx-target="#modal-container"
                        hx-swap="innerHTML"
                    {
                        "+ Add Goal"
                    }
                }

                @if detail.score_events.is_empty() {
                    div style="padding: 3rem; text-align: center; color: var(--gray-500); background: var(--gray-50); border-radius: 8px;" {
                        p { "No goals recorded yet." }
                        p style="margin-top: 0.5rem; font-size: 0.875rem;" {
                            "Click 'Add Goal' to record scoring events."
                        }
                    }
                } @else {
                    (score_events_list(&detail.score_events, match_info.home_team_id, match_info.away_team_id))
                }
            }

            // Modal container
            div id="modal-container" {}
        }
    }
}

/// Render score events list
fn score_events_list(events: &[ScoreEventEntity], home_team_id: i64, _away_team_id: i64) -> Markup {
    html! {
        div style="border: 1px solid var(--gray-200); border-radius: 8px; overflow: hidden;" {
            @for event in events {
                div style=(format!(
                    "display: flex; justify-content: space-between; align-items: center; padding: 1rem; border-bottom: 1px solid var(--gray-200); {}",
                    if event.team_id == home_team_id {
                        "background: linear-gradient(90deg, rgba(59, 130, 246, 0.05) 0%, transparent 100%);"
                    } else {
                        "background: linear-gradient(270deg, rgba(239, 68, 68, 0.05) 0%, transparent 100%);"
                    }
                )) {
                    // Time and Period
                    div style="min-width: 100px;" {
                        div style="font-weight: 600; color: var(--gray-900);" {
                            (period_name(event.period))
                        }
                        @if let (Some(min), Some(sec)) = (event.time_minutes, event.time_seconds) {
                            div style="font-size: 0.875rem; color: var(--gray-600);" {
                                (format!("{}:{:02}", min, sec))
                            }
                        }
                    }

                    // Team
                    div style="flex: 1;" {
                        div style="font-weight: 600; color: var(--gray-900);" {
                            (event.team_name)
                        }
                        @if let Some(goal_type) = &event.goal_type {
                            div style="font-size: 0.875rem; color: var(--gray-600);" {
                                (format_goal_type(goal_type))
                            }
                        }
                    }

                    // Scorer and Assists
                    div style="flex: 2;" {
                        div {
                            span style="font-weight: 600;" { "Goal: " }
                            @if let Some(scorer_name) = &event.scorer_name {
                                span { (scorer_name) }
                            } @else {
                                span style="color: var(--gray-400); font-style: italic;" { "Unknown" }
                            }
                        }
                        @if event.assist1_id.is_some() || event.assist2_id.is_some() {
                            div style="font-size: 0.875rem; color: var(--gray-600); margin-top: 0.25rem;" {
                                span { "Assists: " }
                                @if let Some(assist1_name) = &event.assist1_name {
                                    span { (assist1_name) }
                                    @if event.assist2_name.is_some() {
                                        span { ", " }
                                    }
                                }
                                @if let Some(assist2_name) = &event.assist2_name {
                                    span { (assist2_name) }
                                }
                            }
                        }
                    }

                    // Actions
                    div {
                        button
                            class="btn btn-sm"
                            hx-get=(format!("/matches/score-events/{}/edit", event.id))
                            hx-target="#modal-container"
                            hx-swap="innerHTML"
                            style="margin-right: 0.5rem;"
                        {
                            "Edit"
                        }
                        button
                            class="btn btn-sm btn-danger"
                            hx-post=(format!("/matches/score-events/{}/delete", event.id))
                            hx-confirm="Are you sure you want to delete this goal?"
                        {
                            "Delete"
                        }
                    }
                }
            }
        }
    }
}

/// Format match status as a badge
fn status_badge(status: &str) -> Markup {
    let text = match status {
        "scheduled" => "Scheduled",
        "in_progress" => "In Progress",
        "finished" => "Finished",
        "cancelled" => "Cancelled",
        _ => status,
    };

    html! {
        span
            style=(format!(
                "display: inline-block; padding: 0.25rem 0.75rem; border-radius: 9999px; font-size: 0.875rem; font-weight: 500; {}",
                match status {
                    "scheduled" => "color: #1e40af; background: #dbeafe;",
                    "in_progress" => "color: #15803d; background: #dcfce7;",
                    "finished" => "color: #374151; background: #f3f4f6;",
                    "cancelled" => "color: #b91c1c; background: #fee2e2;",
                    _ => "color: #374151; background: #f3f4f6;",
                }
            ))
        {
            (text)
        }
    }
}

/// Format period number to readable name
fn period_name(period: i32) -> &'static str {
    match period {
        1 => "1st Period",
        2 => "2nd Period",
        3 => "3rd Period",
        4 => "Overtime",
        5 => "Shootout",
        _ => "Unknown",
    }
}

/// Format goal type to readable text
fn format_goal_type(goal_type: &str) -> String {
    match goal_type {
        "even_strength" => "Even Strength".to_string(),
        "power_play" => "Power Play".to_string(),
        "short_handed" => "Short Handed".to_string(),
        "penalty_shot" => "Penalty Shot".to_string(),
        "empty_net" => "Empty Net".to_string(),
        _ => goal_type.to_string(),
    }
}

/// Format ISO date to readable format
fn format_date(date: &str) -> String {
    // Simple formatting - just display the date part
    if let Some(date_part) = date.split('T').next() {
        date_part.to_string()
    } else {
        date.to_string()
    }
}

/// Create match modal
pub fn match_create_modal(
    error: Option<&str>,
    seasons: &[(i64, String)],
    teams: &[(i64, String)],
) -> Markup {
    let form_fields = html! {
        div style="display: grid; grid-template-columns: 1fr 1fr; gap: 1rem; margin-bottom: 1rem;" {
            div {
                label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                    "Season"
                    span style="color: red;" { "*" }
                }
                select
                    name="season_id"
                    required
                    style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;"
                {
                    option value="" { "Select season" }
                    @for (id, name) in seasons {
                        option value=(id) { (name) }
                    }
                }
            }

            div {
                label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                    "Status"
                    span style="color: red;" { "*" }
                }
                select
                    name="status"
                    required
                    style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;"
                {
                    option value="scheduled" selected { "Scheduled" }
                    option value="in_progress" { "In Progress" }
                    option value="finished" { "Finished" }
                    option value="cancelled" { "Cancelled" }
                }
            }
        }

        div style="display: grid; grid-template-columns: 1fr 1fr; gap: 1rem; margin-bottom: 1rem;" {
            div {
                label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                    "Home Team"
                    span style="color: red;" { "*" }
                }
                select
                    name="home_team_id"
                    required
                    style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;"
                {
                    option value="" { "Select home team" }
                    @for (id, name) in teams {
                        option value=(id) { (name) }
                    }
                }
            }

            div {
                label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                    "Away Team"
                    span style="color: red;" { "*" }
                }
                select
                    name="away_team_id"
                    required
                    style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;"
                {
                    option value="" { "Select away team" }
                    @for (id, name) in teams {
                        option value=(id) { (name) }
                    }
                }
            }
        }

        div style="display: grid; grid-template-columns: 1fr 1fr; gap: 1rem; margin-bottom: 1rem;" {
            div {
                label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                    "Home Score"
                }
                input
                    type="number"
                    name="home_score_unidentified"
                    value="0"
                    min="0"
                    style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;";
            }

            div {
                label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                    "Away Score"
                }
                input
                    type="number"
                    name="away_score_unidentified"
                    value="0"
                    min="0"
                    style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;";
            }
        }

        div style="margin-bottom: 1rem;" {
            label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                "Match Date"
            }
            input
                type="datetime-local"
                name="match_date"
                style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;";
        }

        div style="margin-bottom: 1.5rem;" {
            label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                "Venue"
            }
            input
                type="text"
                name="venue"
                placeholder="Enter venue name..."
                style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;";
        }
    };

    modal_form(
        "match-modal",
        "Create Match",
        error,
        "/matches",
        form_fields,
        "Create Match"
    )
}

/// Edit match modal
pub fn match_edit_modal(
    match_entity: &MatchEntity,
    error: Option<&str>,
    seasons: &[(i64, String)],
    teams: &[(i64, String)],
) -> Markup {
    let form_fields = html! {
        div style="display: grid; grid-template-columns: 1fr 1fr; gap: 1rem; margin-bottom: 1rem;" {
            div {
                label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                    "Season"
                    span style="color: red;" { "*" }
                }
                select
                    name="season_id"
                    required
                    style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;"
                {
                    @for (id, name) in seasons {
                        option
                            value=(id)
                            selected[*id == match_entity.season_id]
                        {
                            (name)
                        }
                    }
                }
            }

            div {
                label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                    "Status"
                    span style="color: red;" { "*" }
                }
                select
                    name="status"
                    required
                    style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;"
                {
                    option value="scheduled" selected[match_entity.status == "scheduled"] { "Scheduled" }
                    option value="in_progress" selected[match_entity.status == "in_progress"] { "In Progress" }
                    option value="finished" selected[match_entity.status == "finished"] { "Finished" }
                    option value="cancelled" selected[match_entity.status == "cancelled"] { "Cancelled" }
                }
            }
        }

        div style="display: grid; grid-template-columns: 1fr 1fr; gap: 1rem; margin-bottom: 1rem;" {
            div {
                label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                    "Home Team"
                    span style="color: red;" { "*" }
                }
                select
                    name="home_team_id"
                    required
                    style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;"
                {
                    @for (id, name) in teams {
                        option
                            value=(id)
                            selected[*id == match_entity.home_team_id]
                        {
                            (name)
                        }
                    }
                }
            }

            div {
                label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                    "Away Team"
                    span style="color: red;" { "*" }
                }
                select
                    name="away_team_id"
                    required
                    style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;"
                {
                    @for (id, name) in teams {
                        option
                            value=(id)
                            selected[*id == match_entity.away_team_id]
                        {
                            (name)
                        }
                    }
                }
            }
        }

        div style="display: grid; grid-template-columns: 1fr 1fr; gap: 1rem; margin-bottom: 1rem;" {
            div {
                label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                    "Home Score"
                }
                input
                    type="number"
                    name="home_score_unidentified"
                    value=(match_entity.home_score_unidentified)
                    min="0"
                    style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;";
            }

            div {
                label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                    "Away Score"
                }
                input
                    type="number"
                    name="away_score_unidentified"
                    value=(match_entity.away_score_unidentified)
                    min="0"
                    style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;";
            }
        }

        div style="margin-bottom: 1rem;" {
            label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                "Match Date"
            }
            input
                type="datetime-local"
                name="match_date"
                value=[match_entity.match_date.as_ref()]
                style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;";
        }

        div style="margin-bottom: 1.5rem;" {
            label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                "Venue"
            }
            input
                type="text"
                name="venue"
                value=[match_entity.venue.as_ref()]
                placeholder="Enter venue name..."
                style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;";
        }
    };

    modal_form(
        "match-modal",
        "Edit Match",
        error,
        &format!("/matches/{}", match_entity.id),
        form_fields,
        "Save Changes"
    )
}

/// Create score event modal
pub fn score_event_create_modal(
    error: Option<&str>,
    match_info: &MatchEntity,
    home_players: &[(i64, String)],
    away_players: &[(i64, String)],
) -> Markup {
    let form_fields = html! {
        div style="margin-bottom: 1rem;" {
            label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                "Team"
                span style="color: red;" { "*" }
            }
            select
                name="team_id"
                required
                style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;"
            {
                option value=(match_info.home_team_id) { (match_info.home_team_name) " (Home)" }
                option value=(match_info.away_team_id) { (match_info.away_team_name) " (Away)" }
            }
        }

        div style="display: grid; grid-template-columns: 1fr 1fr 1fr; gap: 1rem; margin-bottom: 1rem;" {
            div {
                label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                    "Period"
                    span style="color: red;" { "*" }
                }
                select
                    name="period"
                    required
                    style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;"
                {
                    option value="1" selected { "1st Period" }
                    option value="2" { "2nd Period" }
                    option value="3" { "3rd Period" }
                    option value="4" { "Overtime" }
                    option value="5" { "Shootout" }
                }
            }

            div {
                label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                    "Minutes"
                }
                input
                    type="number"
                    name="time_minutes"
                    min="0"
                    max="60"
                    placeholder="0"
                    style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;";
            }

            div {
                label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                    "Seconds"
                }
                input
                    type="number"
                    name="time_seconds"
                    min="0"
                    max="59"
                    placeholder="0"
                    style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;";
            }
        }

        div style="margin-bottom: 1rem;" {
            label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                "Goal Type"
            }
            select
                name="goal_type"
                style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;"
            {
                option value="" { "Select type (optional)" }
                option value="even_strength" { "Even Strength" }
                option value="power_play" { "Power Play" }
                option value="short_handed" { "Short Handed" }
                option value="penalty_shot" { "Penalty Shot" }
                option value="empty_net" { "Empty Net" }
            }
        }

        div style="margin-bottom: 1rem;" {
            label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                "Scorer"
            }
            select
                name="scorer_id"
                style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;"
            {
                option value="" { "Unknown scorer" }
                optgroup label=(match_info.home_team_name) {
                    @for (id, name) in home_players {
                        option value=(id) { (name) }
                    }
                }
                optgroup label=(match_info.away_team_name) {
                    @for (id, name) in away_players {
                        option value=(id) { (name) }
                    }
                }
            }
        }

        div style="margin-bottom: 1rem;" {
            label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                "First Assist"
            }
            select
                name="assist1_id"
                style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;"
            {
                option value="" { "No assist" }
                optgroup label=(match_info.home_team_name) {
                    @for (id, name) in home_players {
                        option value=(id) { (name) }
                    }
                }
                optgroup label=(match_info.away_team_name) {
                    @for (id, name) in away_players {
                        option value=(id) { (name) }
                    }
                }
            }
        }

        div style="margin-bottom: 1.5rem;" {
            label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                "Second Assist"
            }
            select
                name="assist2_id"
                style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;"
            {
                option value="" { "No assist" }
                optgroup label=(match_info.home_team_name) {
                    @for (id, name) in home_players {
                        option value=(id) { (name) }
                    }
                }
                optgroup label=(match_info.away_team_name) {
                    @for (id, name) in away_players {
                        option value=(id) { (name) }
                    }
                }
            }
        }
    };

    modal_form(
        "score-event-modal",
        "Add Goal",
        error,
        &format!("/matches/{}/score-events", match_info.id),
        form_fields,
        "Add Goal"
    )
}

/// Edit score event modal
pub fn score_event_edit_modal(
    error: Option<&str>,
    score_event: &ScoreEventEntity,
    match_info: &MatchEntity,
    home_players: &[(i64, String)],
    away_players: &[(i64, String)],
) -> Markup {
    let form_fields = html! {
        div style="margin-bottom: 1rem;" {
            label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                "Team"
                span style="color: red;" { "*" }
            }
            select
                name="team_id"
                required
                style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;"
            {
                option
                    value=(match_info.home_team_id)
                    selected[score_event.team_id == match_info.home_team_id]
                {
                    (match_info.home_team_name) " (Home)"
                }
                option
                    value=(match_info.away_team_id)
                    selected[score_event.team_id == match_info.away_team_id]
                {
                    (match_info.away_team_name) " (Away)"
                }
            }
        }

        div style="display: grid; grid-template-columns: 1fr 1fr 1fr; gap: 1rem; margin-bottom: 1rem;" {
            div {
                label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                    "Period"
                    span style="color: red;" { "*" }
                }
                select
                    name="period"
                    required
                    style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;"
                {
                    option value="1" selected[score_event.period == 1] { "1st Period" }
                    option value="2" selected[score_event.period == 2] { "2nd Period" }
                    option value="3" selected[score_event.period == 3] { "3rd Period" }
                    option value="4" selected[score_event.period == 4] { "Overtime" }
                    option value="5" selected[score_event.period == 5] { "Shootout" }
                }
            }

            div {
                label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                    "Minutes"
                }
                input
                    type="number"
                    name="time_minutes"
                    value=[score_event.time_minutes.as_ref()]
                    min="0"
                    max="60"
                    placeholder="0"
                    style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;";
            }

            div {
                label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                    "Seconds"
                }
                input
                    type="number"
                    name="time_seconds"
                    value=[score_event.time_seconds.as_ref()]
                    min="0"
                    max="59"
                    placeholder="0"
                    style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;";
            }
        }

        div style="margin-bottom: 1rem;" {
            label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                "Goal Type"
            }
            select
                name="goal_type"
                style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;"
            {
                option value="" selected[score_event.goal_type.is_none()] { "Select type (optional)" }
                option value="even_strength" selected[score_event.goal_type.as_deref() == Some("even_strength")] { "Even Strength" }
                option value="power_play" selected[score_event.goal_type.as_deref() == Some("power_play")] { "Power Play" }
                option value="short_handed" selected[score_event.goal_type.as_deref() == Some("short_handed")] { "Short Handed" }
                option value="penalty_shot" selected[score_event.goal_type.as_deref() == Some("penalty_shot")] { "Penalty Shot" }
                option value="empty_net" selected[score_event.goal_type.as_deref() == Some("empty_net")] { "Empty Net" }
            }
        }

        div style="margin-bottom: 1rem;" {
            label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                "Scorer"
            }
            select
                name="scorer_id"
                style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;"
            {
                option value="" selected[score_event.scorer_id.is_none()] { "Unknown scorer" }
                optgroup label=(match_info.home_team_name) {
                    @for (id, name) in home_players {
                        option value=(id) selected[score_event.scorer_id == Some(*id)] { (name) }
                    }
                }
                optgroup label=(match_info.away_team_name) {
                    @for (id, name) in away_players {
                        option value=(id) selected[score_event.scorer_id == Some(*id)] { (name) }
                    }
                }
            }
        }

        div style="margin-bottom: 1rem;" {
            label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                "First Assist"
            }
            select
                name="assist1_id"
                style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;"
            {
                option value="" selected[score_event.assist1_id.is_none()] { "No assist" }
                optgroup label=(match_info.home_team_name) {
                    @for (id, name) in home_players {
                        option value=(id) selected[score_event.assist1_id == Some(*id)] { (name) }
                    }
                }
                optgroup label=(match_info.away_team_name) {
                    @for (id, name) in away_players {
                        option value=(id) selected[score_event.assist1_id == Some(*id)] { (name) }
                    }
                }
            }
        }

        div style="margin-bottom: 1.5rem;" {
            label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                "Second Assist"
            }
            select
                name="assist2_id"
                style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;"
            {
                option value="" selected[score_event.assist2_id.is_none()] { "No assist" }
                optgroup label=(match_info.home_team_name) {
                    @for (id, name) in home_players {
                        option value=(id) selected[score_event.assist2_id == Some(*id)] { (name) }
                    }
                }
                optgroup label=(match_info.away_team_name) {
                    @for (id, name) in away_players {
                        option value=(id) selected[score_event.assist2_id == Some(*id)] { (name) }
                    }
                }
            }
        }
    };

    modal_form(
        "score-event-modal",
        "Edit Goal",
        error,
        &format!("/matches/score-events/{}", score_event.id),
        form_fields,
        "Save Changes"
    )
}
