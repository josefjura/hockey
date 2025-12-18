import { LitElement, html, css } from 'lit';
import { customElement, property } from 'lit/decorators.js';
import { Column } from './shared/types.js';
import './client-data-table.js';
import './badge.js';
import './flag-icon.js';
import './toggle-switch.js';

/**
 * Country entity interface
 */
interface Country {
  id: number;
  name: string;
  iihf: boolean;
  iocCode: string | null;
  iso2Code: string | null;
  isHistorical: boolean;
  years: string | null;
  enabled: boolean;
}

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
@customElement('countries-table')
export class CountriesTable extends LitElement {
  static styles = css`
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

  @property({ type: String, attribute: 'api-endpoint' })
  apiEndpoint: string = '/api/countries';

  @property({ type: Number, attribute: 'page-size' })
  pageSize: number = 20;

  private handleToggleChange(e: CustomEvent) {
    // Dispatch event to parent if needed
    this.dispatchEvent(
      new CustomEvent('country-toggled', {
        detail: e.detail,
        bubbles: true,
        composed: true,
      })
    );
  }

  private handleToggleError(e: CustomEvent) {
    // Handle toggle error if needed
    console.error('Toggle error:', e.detail);
  }

  private get columns(): Column<Country>[] {
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
        renderer: (value, row) => html`
          <div class="country-name-cell">
            ${row.iso2Code
              ? html`
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
        renderer: (value) =>
          value ? html`<code>${value}</code>` : html`<span style="color: var(--gray-400);">-</span>`,
      },
      {
        key: 'iocCode',
        label: 'IOC',
        sortable: true,
        filterable: true,
        width: '100px',
        align: 'center',
        renderer: (value) =>
          value ? html`<code>${value}</code>` : html`<span style="color: var(--gray-400);">-</span>`,
      },
      {
        key: 'iihf',
        label: 'IIHF',
        sortable: true,
        filterable: true,
        width: '100px',
        align: 'center',
        renderer: (value) =>
          value
            ? html`<hockey-badge variant="primary" text="IIHF"></hockey-badge>`
            : html`<span style="color: var(--gray-400);">-</span>`,
      },
      {
        key: 'isHistorical',
        label: 'Historical',
        sortable: true,
        filterable: true,
        width: '150px',
        align: 'center',
        renderer: (value, row) =>
          value
            ? html`
                <hockey-badge
                  variant="warning"
                  text=${row.years || 'Historical'}
                ></hockey-badge>
              `
            : html`<span style="color: var(--gray-400);">-</span>`,
      },
      {
        key: 'enabled',
        label: 'Status',
        sortable: true,
        filterable: true,
        width: '120px',
        align: 'center',
        renderer: (value, row) => html`
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
    return html`
      <client-data-table
        .columns=${this.columns}
        api-endpoint=${this.apiEndpoint}
        page-size=${this.pageSize}
        empty-message="No countries available"
      ></client-data-table>
    `;
  }
}

declare global {
  interface HTMLElementTagNameMap {
    'countries-table': CountriesTable;
  }
}
