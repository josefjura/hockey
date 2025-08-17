export type PaginatedResponse<T> = {
	items: T[];
	total: number;
	page: number;
	page_size: number;
	total_pages: number;
	has_next: boolean;
	has_previous: boolean;
}