import { LitElement, html, css } from 'lit';
import { customElement, property, state } from 'lit/decorators.js';
import { repeat } from 'lit/directives/repeat.js';
import { Column, SortConfig, PaginationInfo } from './shared/types.js';
import { get } from './shared/api-client.js';

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
@customElement('client-data-table')
export class ClientDataTable<T = any> extends LitElement {
  static styles = css`
    :host {
      display: block;
    }

    .table-container {
      width: 100%;
      overflow-x: auto;
    }

    .filters {
      margin-bottom: 1rem;
      padding: 1rem;
      background-color: var(--gray-50, #f9fafb);
      border-radius: 8px;
    }

    .search-input {
      width: 100%;
      max-width: 400px;
      padding: 0.5rem;
      border: 1px solid var(--gray-300, #d1d5db);
      border-radius: 4px;
      font-size: 0.875rem;
    }

    .search-input:focus {
      outline: 2px solid var(--primary-color);
      outline-offset: -2px;
    }

    table {
      width: 100%;
      border-collapse: collapse;
      background-color: white;
      border: 1px solid var(--gray-200, #e5e7eb);
      border-radius: 8px;
    }

    thead {
      background-color: var(--gray-50, #f9fafb);
      border-bottom: 2px solid var(--gray-200, #e5e7eb);
    }

    th {
      padding: 0.75rem 1rem;
      text-align: left;
      font-weight: 600;
      font-size: 0.875rem;
      color: var(--gray-700, #374151);
      white-space: nowrap;
    }

    th.text-center {
      text-align: center;
    }

    th.text-right {
      text-align: right;
    }

    .sortable-header {
      cursor: pointer;
      user-select: none;
      display: inline-flex;
      align-items: center;
      gap: 0.25rem;
    }

    .sortable-header:hover {
      color: var(--primary-color);
    }

    .sort-icon {
      font-size: 0.75rem;
      opacity: 0.5;
    }

    .sort-icon.active {
      opacity: 1;
      color: var(--primary-color);
    }

    td {
      padding: 0.75rem 1rem;
      border-top: 1px solid var(--gray-200, #e5e7eb);
      font-size: 0.875rem;
      color: var(--gray-900, #111827);
    }

    td.text-center {
      text-align: center;
    }

    td.text-right {
      text-align: right;
    }

    tbody tr:hover {
      background-color: var(--gray-50, #f9fafb);
    }

    .loading {
      text-align: center;
      padding: 3rem;
      color: var(--gray-500, #6b7280);
    }

    .spinner {
      display: inline-block;
      width: 40px;
      height: 40px;
      border: 4px solid var(--gray-200, #e5e7eb);
      border-top-color: var(--primary-color);
      border-radius: 50%;
      animation: spin 0.8s linear infinite;
    }

    @keyframes spin {
      to { transform: rotate(360deg); }
    }

    .empty {
      text-align: center;
      padding: 3rem;
      color: var(--gray-500, #6b7280);
    }

    .empty h3 {
      font-size: 1.25rem;
      font-weight: 600;
      margin-bottom: 0.5rem;
      color: var(--gray-700, #374151);
    }

    .pagination {
      display: flex;
      align-items: center;
      justify-content: space-between;
      margin-top: 1rem;
      padding: 1rem;
      border-top: 1px solid var(--gray-200, #e5e7eb);
    }

    .pagination-info {
      font-size: 0.875rem;
      color: var(--gray-600, #4b5563);
    }

    .pagination-controls {
      display: flex;
      align-items: center;
      gap: 0.5rem;
    }

    .page-button {
      padding: 0.5rem 0.75rem;
      border: 1px solid var(--gray-300, #d1d5db);
      background-color: white;
      border-radius: 4px;
      font-size: 0.875rem;
      cursor: pointer;
      transition: all 0.2s;
    }

    .page-button:hover:not(:disabled) {
      background-color: var(--gray-50, #f9fafb);
      border-color: var(--gray-400, #9ca3af);
    }

    .page-button:disabled {
      opacity: 0.5;
      cursor: not-allowed;
    }

    .page-button.active {
      background-color: var(--primary-color);
      color: white;
      border-color: var(--primary-color);
    }

    .error {
      padding: 1rem;
      margin: 1rem 0;
      background-color: #fee2e2; /* TODO: Add CSS variable for light danger bg */
      border: 1px solid var(--danger-color);
      border-radius: 4px;
      color: #991b1b; /* TODO: Add CSS variable for dark danger text */
    }
  `;

  @property({ type: String, attribute: 'api-endpoint' })
  apiEndpoint: string = '';

  @property({ type: Array })
  columns: Column<T>[] = [];

  @property({ type: Number, attribute: 'page-size' })
  pageSize: number = 20;

  @property({ type: String, attribute: 'empty-message' })
  emptyMessage: string = 'No data available';

  @property({ type: Object })
  filters?: Record<string, any>;

  @state()
  private data: T[] = [];

  @state()
  private filteredData: T[] = [];

  @state()
  private loading: boolean = false;

  @state()
  private error: string = '';

  @state()
  private searchQuery: string = '';

  @state()
  private sortConfig: SortConfig | null = null;

  @state()
  private currentPage: number = 1;

  private searchTimeout: number | null = null;

  connectedCallback() {
    super.connectedCallback();
    this.loadData();
  }

  private async loadData() {
    if (!this.apiEndpoint) {
      return;
    }

    this.loading = true;
    this.error = '';

    const startTime = performance.now();
    const response = await get<T[]>(this.apiEndpoint);

    this.loading = false;

    if (response.error) {
      this.error = response.error.message || 'Failed to load data';
      return;
    }

    this.data = response.data || [];
    const loadTime = performance.now() - startTime;

    const filterStartTime = performance.now();
    this.applyFiltersAndSort();
    const filterTime = performance.now() - filterStartTime;

    // Log performance metrics (helpful for debugging large datasets)
    if (this.data.length > 100) {
      console.debug(
        `[client-data-table] Loaded ${this.data.length} rows in ${loadTime.toFixed(2)}ms, ` +
        `filtered/sorted in ${filterTime.toFixed(2)}ms`
      );
    }

    this.dispatchEvent(
      new CustomEvent('data-loaded', {
        detail: { total: this.data.length },
        bubbles: true,
        composed: true,
      })
    );
  }

  private applyFiltersAndSort() {
    let result = [...this.data];

    // Apply search filter
    if (this.searchQuery.trim()) {
      const query = this.searchQuery.toLowerCase();
      result = result.filter((row) => {
        return this.columns
          .filter((col) => col.filterable !== false)
          .some((col) => {
            const value = (row as any)[col.key];
            if (value === null || value === undefined) return false;
            return String(value).toLowerCase().includes(query);
          });
      });
    }

    // Apply sorting
    if (this.sortConfig) {
      result.sort((a, b) => {
        const aVal = (a as any)[this.sortConfig!.key];
        const bVal = (b as any)[this.sortConfig!.key];

        if (aVal === bVal) return 0;

        let comparison = 0;
        if (aVal === null || aVal === undefined) comparison = 1;
        else if (bVal === null || bVal === undefined) comparison = -1;
        else if (typeof aVal === 'number' && typeof bVal === 'number') {
          comparison = aVal - bVal;
        } else {
          comparison = String(aVal).localeCompare(String(bVal));
        }

        return this.sortConfig!.order === 'asc' ? comparison : -comparison;
      });
    }

    this.filteredData = result;

    // Reset to first page if filter changed
    if (this.currentPage > this.totalPages) {
      this.currentPage = Math.max(1, this.totalPages);
    }
  }

  private handleSearch(e: Event) {
    const input = e.target as HTMLInputElement;
    const value = input.value;

    // Debounce search
    if (this.searchTimeout) {
      clearTimeout(this.searchTimeout);
    }

    this.searchTimeout = window.setTimeout(() => {
      this.searchQuery = value;
      this.currentPage = 1;
      this.applyFiltersAndSort();
    }, 300);
  }

  private handleSort(columnKey: string) {
    const column = this.columns.find((col) => col.key === columnKey);
    if (!column || column.sortable === false) {
      return;
    }

    if (this.sortConfig?.key === columnKey) {
      // Toggle sort direction
      this.sortConfig = {
        key: columnKey,
        order: this.sortConfig.order === 'asc' ? 'desc' : 'asc',
      };
    } else {
      // New sort column
      this.sortConfig = { key: columnKey, order: 'asc' };
    }

    this.applyFiltersAndSort();
  }

  private changePage(page: number) {
    if (page < 1 || page > this.totalPages) {
      return;
    }
    this.currentPage = page;
  }

  private get paginationInfo(): PaginationInfo {
    const totalItems = this.filteredData.length;
    const totalPages = Math.max(1, Math.ceil(totalItems / this.pageSize));

    return {
      currentPage: this.currentPage,
      pageSize: this.pageSize,
      totalItems,
      totalPages,
    };
  }

  private get totalPages(): number {
    return this.paginationInfo.totalPages;
  }

  private get paginatedData(): T[] {
    const start = (this.currentPage - 1) * this.pageSize;
    const end = start + this.pageSize;
    return this.filteredData.slice(start, end);
  }

  private renderSortIcon(columnKey: string) {
    if (this.sortConfig?.key !== columnKey) {
      return html`<span class="sort-icon">↕</span>`;
    }

    return html`
      <span class="sort-icon active">
        ${this.sortConfig.order === 'asc' ? '↑' : '↓'}
      </span>
    `;
  }

  private renderCell(column: Column<T>, row: T) {
    const value = (row as any)[column.key];

    if (column.renderer) {
      return column.renderer(value, row);
    }

    if (value === null || value === undefined) {
      return html`<span style="color: var(--gray-400);">-</span>`;
    }

    return html`${value}`;
  }

  private renderPaginationButtons() {
    const { currentPage, totalPages } = this.paginationInfo;
    const buttons: number[] = [];

    // Always show first page
    buttons.push(1);

    // Calculate range around current page
    let start = Math.max(2, currentPage - 1);
    let end = Math.min(totalPages - 1, currentPage + 1);

    // Add ellipsis after first page if needed
    if (start > 2) {
      buttons.push(-1); // -1 represents ellipsis
    }

    // Add pages around current page
    for (let i = start; i <= end; i++) {
      buttons.push(i);
    }

    // Add ellipsis before last page if needed
    if (end < totalPages - 1) {
      buttons.push(-1); // -1 represents ellipsis
    }

    // Always show last page if there is more than one page
    if (totalPages > 1) {
      buttons.push(totalPages);
    }

    return buttons.map((page, index) => {
      if (page === -1) {
        return html`<span key=${index} style="padding: 0 0.5rem;">...</span>`;
      }

      return html`
        <button
          key=${page}
          class="page-button ${page === currentPage ? 'active' : ''}"
          @click=${() => this.changePage(page)}
        >
          ${page}
        </button>
      `;
    });
  }

  render() {
    if (this.loading) {
      return html`
        <div class="loading">
          <div class="spinner"></div>
          <p style="margin-top: 1rem;">Loading data...</p>
        </div>
      `;
    }

    if (this.error) {
      return html`
        <div class="error">
          <strong>Error:</strong> ${this.error}
          <button
            @click=${this.loadData}
            style="margin-left: 1rem; padding: 0.25rem 0.5rem; border: none; background: #991b1b; color: white; border-radius: 4px; cursor: pointer;"
          >
            Retry
          </button>
        </div>
      `;
    }

    if (this.data.length === 0) {
      return html`
        <div class="empty">
          <h3>No Data</h3>
          <p>${this.emptyMessage}</p>
        </div>
      `;
    }

    const { currentPage, pageSize, totalItems, totalPages } = this.paginationInfo;
    const start = (currentPage - 1) * pageSize + 1;
    const end = Math.min(currentPage * pageSize, totalItems);

    return html`
      <!-- Search Filter -->
      <div class="filters">
        <input
          type="text"
          class="search-input"
          placeholder="Search..."
          @input=${this.handleSearch}
        />
      </div>

      <!-- Table -->
      <div class="table-container">
        <table>
          <thead>
            <tr>
              ${this.columns.map(
                (column) => html`
                  <th
                    class=${column.align ? `text-${column.align}` : ''}
                    style=${column.width ? `width: ${column.width}` : ''}
                  >
                    ${column.sortable !== false
                      ? html`
                          <div
                            class="sortable-header"
                            @click=${() => this.handleSort(column.key)}
                          >
                            ${column.label}
                            ${this.renderSortIcon(column.key)}
                          </div>
                        `
                      : html`${column.label}`}
                  </th>
                `
              )}
            </tr>
          </thead>
          <tbody>
            ${this.filteredData.length === 0
              ? html`
                  <tr>
                    <td colspan=${this.columns.length} style="text-align: center; padding: 2rem; color: var(--gray-500);">
                      No results match your search
                    </td>
                  </tr>
                `
              : repeat(
                  this.paginatedData,
                  (row: any) => row.id || JSON.stringify(row),
                  (row) => html`
                    <tr>
                      ${this.columns.map(
                        (column) => html`
                          <td class=${column.align ? `text-${column.align}` : ''}>
                            ${this.renderCell(column, row)}
                          </td>
                        `
                      )}
                    </tr>
                  `
                )}
          </tbody>
        </table>
      </div>

      <!-- Pagination -->
      ${totalPages > 1
        ? html`
            <div class="pagination">
              <div class="pagination-info">
                Showing ${start} to ${end} of ${totalItems} entries
              </div>
              <div class="pagination-controls">
                <button
                  class="page-button"
                  ?disabled=${currentPage === 1}
                  @click=${() => this.changePage(currentPage - 1)}
                >
                  Previous
                </button>
                ${this.renderPaginationButtons()}
                <button
                  class="page-button"
                  ?disabled=${currentPage === totalPages}
                  @click=${() => this.changePage(currentPage + 1)}
                >
                  Next
                </button>
              </div>
            </div>
          `
        : ''}
    `;
  }
}

declare global {
  interface HTMLElementTagNameMap {
    'client-data-table': ClientDataTable;
  }
}
