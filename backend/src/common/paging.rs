use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct Paging {
    pub page: usize,
    pub page_size: usize,
}

impl Paging {
    pub fn new(page: usize, page_size: usize) -> Self {
        Self {
            page: page.max(1),                    // Ensure page is at least 1
            page_size: page_size.min(100).max(1), // Limit page size between 1 and 100
        }
    }

    pub fn offset(&self) -> usize {
        (self.page - 1) * self.page_size
    }

    pub fn limit(&self) -> usize {
        self.page_size
    }
}

impl Default for Paging {
    fn default() -> Self {
        Self::new(1, 20)
    }
}

#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct PagedResult<T> {
    pub items: Vec<T>,
    pub total: usize,
    pub page: usize,
    pub page_size: usize,
    pub total_pages: usize,
    pub has_next: bool,
    pub has_previous: bool,
}

/// Generic paged response for API endpoints
#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct PagedResponse<T> {
    pub items: Vec<T>,
    pub total: usize,
    pub page: usize,
    pub page_size: usize,
    pub total_pages: usize,
    pub has_next: bool,
    pub has_previous: bool,
}

impl<T> PagedResponse<T> {
    /// Create a new PagedResponse from a PagedResult and item mapper
    pub fn from_result<U>(result: PagedResult<U>, items: Vec<T>) -> Self {
        Self {
            items,
            total: result.total,
            page: result.page,
            page_size: result.page_size,
            total_pages: result.total_pages,
            has_next: result.has_next,
            has_previous: result.has_previous,
        }
    }

    /// Create a new PagedResponse directly for documentation examples
    pub fn new(
        items: Vec<T>,
        total: usize,
        page: usize,
        page_size: usize,
        total_pages: usize,
        has_next: bool,
        has_previous: bool,
    ) -> Self {
        Self {
            items,
            total,
            page,
            page_size,
            total_pages,
            has_next,
            has_previous,
        }
    }
}

impl<T> PagedResult<T> {
    pub fn new(items: Vec<T>, total: usize, paging: &Paging) -> Self {
        let total_pages = (total + paging.page_size - 1) / paging.page_size;

        Self {
            items,
            total,
            page: paging.page,
            page_size: paging.page_size,
            total_pages,
            has_next: paging.page < total_pages,
            has_previous: paging.page > 1,
        }
    }
}
