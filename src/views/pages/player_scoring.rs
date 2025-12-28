use maud::{html, Markup};

use crate::common::pagination::{PagedResult, SortOrder};
use crate::i18n::TranslationContext;
use crate::service::players::{
    PlayerEntity, PlayerScoringEventEntity, PlayerScoringFilters, PlayerSeasonStats,
    ScoringEventSortField,
};
use crate::views::components::crud::{empty_state_i18n, pagination};

/// Main player scoring page with stats and table
#[allow(clippy::too_many_arguments)] // View functions commonly need many template parameters
pub fn player_scoring_page(
    t: &TranslationContext,
    player: &PlayerEntity,
    season_stats: &[PlayerSeasonStats],
    result: &PagedResult<PlayerScoringEventEntity>,
    filters: &PlayerScoringFilters,
    sort_field: &ScoringEventSortField,
    sort_order: &SortOrder,
    seasons: &[(i64, String)],
    teams: &[(i64, String)],
) -> Markup {
    html! {
        div class="card" {
            // Header with back button
            div style="display: flex; align-items: center; gap: 1rem; margin-bottom: 1.5rem;" {
                a
                    href=(format!("/players/{}", player.id))
                    class="btn btn-secondary"
                {
                    (format!("← {}", t.messages.players_back_to_detail()))
                }
                h1 style="font-size: 2rem; font-weight: 700; margin: 0;" {
                    (player.name)
                    " - "
                    (t.messages.player_scoring_title())
                }
            }

            // Season statistics summary
            (season_stats_summary(t, season_stats))

            // Filters
            (scoring_filters(t, player.id, filters, seasons, teams))

            // Table
            (player_scoring_list_content(t, player.id, result, filters, sort_field, sort_order))
        }
    }
}

/// Season statistics summary cards
fn season_stats_summary(t: &TranslationContext, season_stats: &[PlayerSeasonStats]) -> Markup {
    html! {
        @if season_stats.is_empty() {
            div style="padding: 1.5rem; margin-bottom: 1.5rem; background: var(--gray-50); border-radius: 8px; text-align: center; color: var(--gray-500);" {
                (t.messages.player_scoring_empty_title())
            }
        } @else {
            div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(250px, 1fr)); gap: 1rem; margin-bottom: 1.5rem;" {
                @for stats in season_stats {
                    div style="padding: 1.5rem; background: linear-gradient(135deg, var(--primary-color) 0%, var(--primary-dark) 100%); border-radius: 8px; color: white;" {
                        // Season and Event name
                        div style="font-size: 0.875rem; opacity: 0.9; margin-bottom: 0.75rem;" {
                            @if let Some(display_name) = &stats.season_display_name {
                                (display_name)
                            } @else {
                                (stats.season_year)
                            }
                            br;
                            span style="font-size: 0.75rem; opacity: 0.8;" {
                                (&stats.event_name)
                            }
                        }

                        // Stats grid
                        div style="display: grid; grid-template-columns: repeat(3, 1fr); gap: 1rem; margin-top: 1rem;" {
                            // Goals
                            div {
                                div style="font-size: 0.75rem; opacity: 0.8;" {
                                    (t.messages.player_scoring_total_goals())
                                }
                                div style="font-size: 2rem; font-weight: 700;" {
                                    (stats.goals)
                                }
                            }

                            // Assists
                            div {
                                div style="font-size: 0.75rem; opacity: 0.8;" {
                                    (t.messages.player_scoring_total_assists())
                                }
                                div style="font-size: 2rem; font-weight: 700;" {
                                    (stats.assists)
                                }
                            }

                            // Points
                            div {
                                div style="font-size: 0.75rem; opacity: 0.8;" {
                                    (t.messages.player_scoring_total_points())
                                }
                                div style="font-size: 2rem; font-weight: 700;" {
                                    (stats.points)
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Filters section
fn scoring_filters(
    t: &TranslationContext,
    player_id: i64,
    filters: &PlayerScoringFilters,
    seasons: &[(i64, String)],
    teams: &[(i64, String)],
) -> Markup {
    html! {
        div style="margin-bottom: 1.5rem; padding: 1rem; background: var(--gray-50); border-radius: 8px;" {
            form
                hx-get=(format!("/players/{}/scoring/list", player_id))
                hx-target="#player-scoring-table"
                hx-swap="outerHTML"
                hx-trigger="submit, change delay:300ms"
            {
                div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 1rem; align-items: end;" {
                    // Event type filter
                    div {
                        label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                            (t.messages.player_scoring_filter_event_type())
                        }
                        select
                            name="event_type"
                            style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;"
                        {
                            option value="" selected[filters.event_type.is_none()] {
                                (t.messages.player_scoring_all_events())
                            }
                            option
                                value="goals"
                                selected[filters.event_type.as_deref() == Some("goals")]
                            {
                                (t.messages.player_scoring_goals_only())
                            }
                            option
                                value="assists"
                                selected[filters.event_type.as_deref() == Some("assists")]
                            {
                                (t.messages.player_scoring_assists_only())
                            }
                        }
                    }

                    // Season filter
                    div {
                        label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                            (t.messages.player_scoring_filter_season())
                        }
                        select
                            name="season_id"
                            style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;"
                        {
                            option value="" { (t.messages.player_scoring_all_seasons()) }
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
                            (t.messages.player_scoring_filter_team())
                        }
                        select
                            name="team_id"
                            style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;"
                        {
                            option value="" { (t.messages.player_scoring_all_teams()) }
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

                    // Clear button
                    div {
                        button
                            type="button"
                            class="btn btn-secondary"
                            hx-get=(format!("/players/{}/scoring/list", player_id))
                            hx-target="#player-scoring-table"
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
                            (t.messages.player_scoring_filter_date_from())
                        }
                        input
                            type="date"
                            name="date_from"
                            value=[filters.date_from.as_ref()]
                            style="width: 100%; padding: 0.5rem; border: 1px solid var(--gray-300); border-radius: 4px;";
                    }

                    div {
                        label style="display: block; margin-bottom: 0.5rem; font-weight: 500;" {
                            (t.messages.player_scoring_filter_date_to())
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
    }
}

/// Scoring events table content (for HTMX updates)
pub fn player_scoring_list_content(
    t: &TranslationContext,
    player_id: i64,
    result: &PagedResult<PlayerScoringEventEntity>,
    filters: &PlayerScoringFilters,
    sort_field: &ScoringEventSortField,
    sort_order: &SortOrder,
) -> Markup {
    html! {
        div id="player-scoring-table" class="loading-overlay" {
            // Loading spinner overlay
            div class="loading-spinner-overlay" {
                hockey-loading-spinner size="lg" {}
            }

            @if result.items.is_empty() {
                (empty_state_i18n(
                    &t.messages.player_scoring_empty_title().to_string(),
                    &t.messages.player_scoring_empty_message().to_string(),
                    filters.event_type.is_some() || filters.season_id.is_some() || filters.team_id.is_some() || filters.date_from.is_some() || filters.date_to.is_some()
                ))
            } @else {
                table class="table" {
                    thead {
                        tr {
                            th {
                                (sortable_header(
                                    &t.messages.player_scoring_date().to_string(),
                                    &ScoringEventSortField::Date,
                                    sort_field,
                                    sort_order,
                                    player_id,
                                    filters,
                                ))
                            }
                            th { (t.messages.player_scoring_match()) }
                            th {
                                (sortable_header(
                                    &t.messages.player_scoring_event_type().to_string(),
                                    &ScoringEventSortField::EventType,
                                    sort_field,
                                    sort_order,
                                    player_id,
                                    filters,
                                ))
                            }
                            th { (t.messages.player_scoring_team()) }
                            th {
                                (sortable_header(
                                    &t.messages.player_scoring_period().to_string(),
                                    &ScoringEventSortField::Period,
                                    sort_field,
                                    sort_order,
                                    player_id,
                                    filters,
                                ))
                            }
                            th { (t.messages.player_scoring_time()) }
                            th { (t.messages.player_scoring_goal_type()) }
                            th { (t.messages.player_scoring_details()) }
                        }
                    }
                    tbody {
                        @for event in &result.items {
                            tr {
                                // Date
                                td {
                                    @if let Some(date) = &event.match_date {
                                        (format_date(date))
                                    } @else {
                                        span style="color: var(--gray-400); font-style: italic;" { "TBD" }
                                    }
                                }

                                // Match
                                td {
                                    a
                                        href=(format!("/matches/{}", event.match_id))
                                        style="text-decoration: none; color: inherit;"
                                    {
                                        div style="display: flex; flex-direction: column; gap: 0.25rem;" {
                                            div style="display: flex; align-items: center; gap: 0.5rem; font-size: 0.875rem;" {
                                                @if let Some(iso2) = &event.home_team_iso2 {
                                                    flag-icon
                                                        country-code=(iso2.to_lowercase())
                                                        country-name=(&event.home_team_name)
                                                        size="sm" {}
                                                }
                                                span { (&event.home_team_name) }
                                            }
                                            div style="display: flex; align-items: center; gap: 0.5rem; font-size: 0.875rem;" {
                                                @if let Some(iso2) = &event.away_team_iso2 {
                                                    flag-icon
                                                        country-code=(iso2.to_lowercase())
                                                        country-name=(&event.away_team_name)
                                                        size="sm" {}
                                                }
                                                span { (&event.away_team_name) }
                                            }
                                        }
                                    }
                                }

                                // Event Type
                                td {
                                    (event_type_badge(&event.event_type, t))
                                }

                                // Team scored for
                                td {
                                    div style="display: flex; align-items: center; gap: 0.5rem;" {
                                        @if let Some(iso2) = &event.team_iso2 {
                                            flag-icon
                                                country-code=(iso2.to_lowercase())
                                                country-name=(&event.team_name)
                                                size="sm" {}
                                        }
                                        span { (&event.team_name) }
                                    }
                                }

                                // Period
                                td {
                                    (format_period(event.period))
                                }

                                // Time
                                td {
                                    (format_time(event.time_minutes, event.time_seconds))
                                }

                                // Goal Type
                                td {
                                    @if let Some(goal_type) = &event.goal_type {
                                        (format_goal_type(goal_type, t))
                                    } @else {
                                        span style="color: var(--gray-400); font-style: italic;" { "-" }
                                    }
                                }

                                // Details (Scorer + Assists)
                                td style="font-size: 0.875rem;" {
                                    @if let Some(scorer_name) = &event.scorer_name {
                                        div {
                                            strong { (t.messages.player_scoring_scorer()) ": " }
                                            (scorer_name)
                                        }
                                    }
                                    @if let Some(assist1_name) = &event.assist1_name {
                                        div {
                                            strong { (t.messages.player_scoring_assist_1()) ": " }
                                            (assist1_name)
                                        }
                                    }
                                    @if let Some(assist2_name) = &event.assist2_name {
                                        div {
                                            strong { (t.messages.player_scoring_assist_2()) ": " }
                                            (assist2_name)
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // Pagination
                (pagination(
                    result,
                    "events",
                    |page| build_pagination_url(player_id, page, result.page_size, filters, sort_field, sort_order),
                    "player-scoring-table"
                ))
            }
        }
    }
}

/// Sortable table header
fn sortable_header(
    label: &str,
    field: &ScoringEventSortField,
    current_sort: &ScoringEventSortField,
    current_order: &SortOrder,
    player_id: i64,
    filters: &PlayerScoringFilters,
) -> Markup {
    let is_active = field == current_sort;

    let next_order = if is_active {
        current_order.toggle()
    } else {
        match field {
            ScoringEventSortField::Date => SortOrder::Desc,
            _ => SortOrder::Asc,
        }
    };

    let url = build_sort_url(player_id, field, &next_order, filters);

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
            hx-target="#player-scoring-table"
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

/// Event type badge
fn event_type_badge(event_type: &str, t: &TranslationContext) -> Markup {
    let (label, color) = match event_type {
        "goal" => (
            t.messages.player_scoring_goal().to_string(),
            "var(--primary-color)",
        ),
        "assist_primary" => (
            t.messages.player_scoring_assist_primary().to_string(),
            "#10b981",
        ),
        "assist_secondary" => (
            t.messages.player_scoring_assist_secondary().to_string(),
            "#6366f1",
        ),
        _ => ("Unknown".to_string(), "var(--gray-500)"),
    };

    html! {
        span style=(format!("padding: 0.25rem 0.75rem; border-radius: 9999px; font-size: 0.75rem; font-weight: 600; color: white; background: {};", color)) {
            (label)
        }
    }
}

/// Format period (1, 2, 3, OT, SO)
fn format_period(period: i32) -> String {
    match period {
        1 => "1st".to_string(),
        2 => "2nd".to_string(),
        3 => "3rd".to_string(),
        4 => "OT".to_string(),
        5 => "SO".to_string(),
        _ => period.to_string(),
    }
}

/// Format time (minutes:seconds)
fn format_time(minutes: Option<i32>, seconds: Option<i32>) -> String {
    match (minutes, seconds) {
        (Some(m), Some(s)) => format!("{}:{:02}", m, s),
        (Some(m), None) => format!("{}:00", m),
        _ => "-".to_string(),
    }
}

/// Format goal type for display
fn format_goal_type(goal_type: &str, t: &TranslationContext) -> String {
    match goal_type {
        "even_strength" => t.messages.player_scoring_even_strength().to_string(),
        "power_play" => t.messages.player_scoring_power_play().to_string(),
        "short_handed" => t.messages.player_scoring_short_handed().to_string(),
        "penalty_shot" => t.messages.player_scoring_penalty_shot().to_string(),
        "empty_net" => t.messages.player_scoring_empty_net().to_string(),
        _ => goal_type.to_string(),
    }
}

/// Format date for display
fn format_date(date_str: &str) -> String {
    // Parse ISO date and format as "Jan 15, 2024"
    // For now, just return the raw string since we don't have chrono dependency
    date_str.to_string()
}

/// Build sort URL
fn build_sort_url(
    player_id: i64,
    field: &ScoringEventSortField,
    order: &SortOrder,
    filters: &PlayerScoringFilters,
) -> String {
    let mut url = format!(
        "/players/{}/scoring/list?sort={}&order={}",
        player_id,
        field.as_str(),
        order.as_str()
    );

    if let Some(event_type) = &filters.event_type {
        url.push_str(&format!("&event_type={}", urlencoding::encode(event_type)));
    }

    if let Some(season_id) = filters.season_id {
        url.push_str(&format!("&season_id={}", season_id));
    }

    if let Some(team_id) = filters.team_id {
        url.push_str(&format!("&team_id={}", team_id));
    }

    if let Some(date_from) = &filters.date_from {
        url.push_str(&format!("&date_from={}", urlencoding::encode(date_from)));
    }

    if let Some(date_to) = &filters.date_to {
        url.push_str(&format!("&date_to={}", urlencoding::encode(date_to)));
    }

    url
}

/// Build pagination URL
fn build_pagination_url(
    player_id: i64,
    page: usize,
    page_size: usize,
    filters: &PlayerScoringFilters,
    sort_field: &ScoringEventSortField,
    sort_order: &SortOrder,
) -> String {
    let mut url = format!(
        "/players/{}/scoring/list?page={}&page_size={}&sort={}&order={}",
        player_id,
        page,
        page_size,
        sort_field.as_str(),
        sort_order.as_str()
    );

    if let Some(event_type) = &filters.event_type {
        url.push_str(&format!("&event_type={}", urlencoding::encode(event_type)));
    }

    if let Some(season_id) = filters.season_id {
        url.push_str(&format!("&season_id={}", season_id));
    }

    if let Some(team_id) = filters.team_id {
        url.push_str(&format!("&team_id={}", team_id));
    }

    if let Some(date_from) = &filters.date_from {
        url.push_str(&format!("&date_from={}", urlencoding::encode(date_from)));
    }

    if let Some(date_to) = &filters.date_to {
        url.push_str(&format!("&date_to={}", urlencoding::encode(date_to)));
    }

    url
}
