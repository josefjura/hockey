use maud::{html, Markup};

use crate::common::pagination::{PagedResult, SortOrder};

/// Generate page numbers for pagination with ellipsis for large page counts
///
/// Returns a vec of page numbers where 0 represents "..." (ellipsis).
/// Always shows first page, last page, and pages around current page.
///
/// Examples:
/// - Total 5 pages: [1, 2, 3, 4, 5]
/// - Current page 1 of 20: [1, 2, 3, 0, 20]
/// - Current page 10 of 20: [1, 0, 9, 10, 11, 0, 20]
pub fn pagination_pages(current_page: usize, total_pages: usize) -> Vec<usize> {
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

/// Render pagination controls for a paged result
///
/// This is a generic pagination component that works with any entity type.
/// Callers provide URL builders for navigation links.
pub fn pagination_controls<T, F>(
    result: &PagedResult<T>,
    entity_name: &str,
    build_page_url: F,
) -> Markup
where
    F: Fn(usize) -> String,
{
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
                " " (entity_name)
            }

            // Page buttons
            @if result.total_pages > 1 {
                div style="display: flex; gap: 0.5rem;" {
                    // Previous button
                    @if result.has_previous {
                        button
                            class="btn btn-sm"
                            hx-get=(build_page_url(result.page - 1))
                            hx-target=(format!("#{}-table", entity_name))
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
                                hx-get=(build_page_url(page))
                                hx-target=(format!("#{}-table", entity_name))
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
                            hx-get=(build_page_url(result.page + 1))
                            hx-target=(format!("#{}-table", entity_name))
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

/// Render a sortable table header with sort indicators
///
/// Generic sortable header component that shows sort direction indicators.
/// Callers provide the URL for toggling sort on this column.
pub fn sortable_header<F>(
    label: &str,
    is_active: bool,
    current_order: &SortOrder,
    entity_name: &str,
    build_sort_url: F,
) -> Markup
where
    F: Fn() -> String,
{
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
            hx-get=(build_sort_url())
            hx-target=(format!("#{}-table", entity_name))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pagination_pages_small() {
        assert_eq!(pagination_pages(1, 5), vec![1, 2, 3, 4, 5]);
        assert_eq!(pagination_pages(3, 5), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_pagination_pages_large_at_start() {
        assert_eq!(pagination_pages(1, 20), vec![1, 2, 0, 20]);
        assert_eq!(pagination_pages(2, 20), vec![1, 2, 3, 0, 20]);
    }

    #[test]
    fn test_pagination_pages_large_in_middle() {
        assert_eq!(pagination_pages(10, 20), vec![1, 0, 9, 10, 11, 0, 20]);
    }

    #[test]
    fn test_pagination_pages_large_at_end() {
        assert_eq!(pagination_pages(19, 20), vec![1, 0, 18, 19, 20]);
        assert_eq!(pagination_pages(20, 20), vec![1, 0, 19, 20]);
    }
}
