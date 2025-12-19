var __decorate = (this && this.__decorate) || function (decorators, target, key, desc) {
    var c = arguments.length, r = c < 3 ? target : desc === null ? desc = Object.getOwnPropertyDescriptor(target, key) : desc, d;
    if (typeof Reflect === "object" && typeof Reflect.decorate === "function") r = Reflect.decorate(decorators, target, key, desc);
    else for (var i = decorators.length - 1; i >= 0; i--) if (d = decorators[i]) r = (c < 3 ? d(r) : c > 3 ? d(target, key, r) : d(target, key)) || r;
    return c > 3 && r && Object.defineProperty(target, key, r), r;
};
import { LitElement, html, css } from 'lit';
import { customElement, property } from 'lit/decorators.js';
import './client-data-table.js';
import './badge.js';
import './flag-icon.js';
import './toggle-switch.js';
/**
 * Countries table component
 *
 * Demonstrates how to configure client-data-table for a specific entity.
 * This pattern can be replicated for teams, players, events, etc.
 *
 * @example
 * ```html
 * <countries-table api-endpoint="/api/countries"></countries-table>
 * ```
 */
let CountriesTable = class CountriesTable extends LitElement {
    constructor() {
        super(...arguments);
        this.apiEndpoint = '/api/countries';
        this.pageSize = 20;
    }
    handleToggleChange(e) {
        // Dispatch event to parent if needed
        this.dispatchEvent(new CustomEvent('country-toggled', {
            detail: e.detail,
            bubbles: true,
            composed: true,
        }));
    }
    handleToggleError(e) {
        // Handle toggle error if needed
        console.error('Toggle error:', e.detail);
    }
    get columns() {
        return [
            {
                key: 'id',
                label: 'ID',
                sortable: true,
                width: '80px',
                align: 'center',
            },
            {
                key: 'name',
                label: 'Country',
                sortable: true,
                filterable: true,
                renderer: (value, row) => html `
          <div class="country-name-cell">
            ${row.iso2Code
                    ? html `
                  <flag-icon
                    country-code=${row.iso2Code}
                    country-name=${row.name}
                    size="sm"
                  ></flag-icon>
                `
                    : ''}
            <span>${value}</span>
          </div>
        `,
            },
            {
                key: 'iso2Code',
                label: 'ISO2',
                sortable: true,
                filterable: true,
                width: '100px',
                align: 'center',
                renderer: (value) => value ? html `<code>${value}</code>` : html `<span style="color: var(--gray-400);">-</span>`,
            },
            {
                key: 'iocCode',
                label: 'IOC',
                sortable: true,
                filterable: true,
                width: '100px',
                align: 'center',
                renderer: (value) => value ? html `<code>${value}</code>` : html `<span style="color: var(--gray-400);">-</span>`,
            },
            {
                key: 'iihf',
                label: 'IIHF',
                sortable: true,
                filterable: true,
                width: '100px',
                align: 'center',
                renderer: (value) => value
                    ? html `<hockey-badge variant="primary" text="IIHF"></hockey-badge>`
                    : html `<span style="color: var(--gray-400);">-</span>`,
            },
            {
                key: 'isHistorical',
                label: 'Historical',
                sortable: true,
                filterable: true,
                width: '150px',
                align: 'center',
                renderer: (value, row) => value
                    ? html `
                <hockey-badge
                  variant="warning"
                  text=${row.years || 'Historical'}
                ></hockey-badge>
              `
                    : html `<span style="color: var(--gray-400);">-</span>`,
            },
            {
                key: 'enabled',
                label: 'Status',
                sortable: true,
                filterable: true,
                width: '120px',
                align: 'center',
                renderer: (value, row) => html `
          <toggle-switch
            .checked=${value}
            .entityId=${row.id}
            api-endpoint="/api/countries/${row.id}/toggle"
            @toggle-change=${this.handleToggleChange}
            @toggle-error=${this.handleToggleError}
          ></toggle-switch>
        `,
            },
        ];
    }
    render() {
        return html `
      <client-data-table
        .columns=${this.columns}
        api-endpoint=${this.apiEndpoint}
        page-size=${this.pageSize}
        empty-message="No countries available"
      ></client-data-table>
    `;
    }
};
CountriesTable.styles = css `
    :host {
      display: block;
    }

    .country-name-cell {
      display: flex;
      align-items: center;
      gap: 0.5rem;
    }

    code {
      padding: 0.125rem 0.375rem;
      background-color: var(--gray-100, #f3f4f6);
      border: 1px solid var(--gray-300, #d1d5db);
      border-radius: 3px;
      font-family: 'Courier New', monospace;
      font-size: 0.875rem;
      color: var(--gray-700, #374151);
    }
  `;
__decorate([
    property({ type: String, attribute: 'api-endpoint' })
], CountriesTable.prototype, "apiEndpoint", void 0);
__decorate([
    property({ type: Number, attribute: 'page-size' })
], CountriesTable.prototype, "pageSize", void 0);
CountriesTable = __decorate([
    customElement('countries-table')
], CountriesTable);
export { CountriesTable };
//# sourceMappingURL=countries-table.js.map