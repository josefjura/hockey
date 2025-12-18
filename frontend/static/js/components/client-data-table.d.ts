import { LitElement } from 'lit';
import { Column } from './shared/types.js';
/**
 * Client-side data table with filtering, sorting, and pagination
 *
 * All operations are performed client-side for optimal performance with small to medium datasets.
 *
 * **Performance Characteristics:**
 * - Optimal: < 100 rows (instant filtering/sorting)
 * - Good: 100-300 rows (smooth performance on all devices)
 * - Acceptable: 300-500 rows (slight lag on older devices)
 * - Not recommended: > 500 rows (use server-side tables instead)
 *
 * The component loads all data once and performs filtering, sorting, and pagination
 * client-side using JavaScript. This provides instant responsiveness but requires
 * the entire dataset to be loaded into browser memory.
 *
 * **When to use:**
 * - Small datasets that change infrequently
 * - Fast filtering/sorting without server round-trips
 * - Reference data (countries, categories, settings)
 *
 * **When to use server-side tables:**
 * - Large datasets (> 500 rows)
 * - Real-time data that updates frequently
 * - Complex server-side filtering/aggregation
 *
 * @example
 * ```html
 * <client-data-table
 *   .columns=${columns}
 *   api-endpoint="/api/countries"
 *   page-size="20"
 *   empty-message="No countries found">
 * </client-data-table>
 * ```
 */
export declare class ClientDataTable<T = any> extends LitElement {
    static styles: import("lit").CSSResult;
    apiEndpoint: string;
    columns: Column<T>[];
    pageSize: number;
    emptyMessage: string;
    filters?: Record<string, any>;
    private data;
    private filteredData;
    private loading;
    private error;
    private searchQuery;
    private sortConfig;
    private currentPage;
    private searchTimeout;
    connectedCallback(): void;
    private loadData;
    private applyFiltersAndSort;
    private handleSearch;
    private handleSort;
    private changePage;
    private get paginationInfo();
    private get totalPages();
    private get paginatedData();
    private renderSortIcon;
    private renderCell;
    private renderPaginationButtons;
    render(): import("lit-html").TemplateResult<1>;
}
declare global {
    interface HTMLElementTagNameMap {
        'client-data-table': ClientDataTable;
    }
}
//# sourceMappingURL=client-data-table.d.ts.map