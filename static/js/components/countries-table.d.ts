import { LitElement } from 'lit';
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
export declare class CountriesTable extends LitElement {
    static styles: import("lit").CSSResult;
    apiEndpoint: string;
    pageSize: number;
    private handleToggleChange;
    private handleToggleError;
    private get columns();
    render(): import("lit-html").TemplateResult<1>;
}
declare global {
    interface HTMLElementTagNameMap {
        'countries-table': CountriesTable;
    }
}
//# sourceMappingURL=countries-table.d.ts.map