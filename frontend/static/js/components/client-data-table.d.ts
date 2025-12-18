import { LitElement } from 'lit';
import { Column } from './shared/types.js';
/**
 * Client-side data table with filtering, sorting, and pagination
 *
 * All operations are performed client-side for optimal performance with small to medium datasets.
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