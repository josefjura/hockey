import { LitElement } from 'lit';
/**
 * Toggle switch component with API integration
 *
 * Features:
 * - Optimistic updates (instant UI feedback)
 * - Automatic rollback on error
 * - Loading state during API call
 * - Keyboard accessible
 *
 * @example
 * ```html
 * <toggle-switch
 *   .checked=${true}
 *   .entityId=${123}
 *   api-endpoint="/api/countries/123/toggle"
 *   label="Enabled">
 * </toggle-switch>
 * ```
 */
export declare class ToggleSwitch extends LitElement {
    static styles: import("lit").CSSResult;
    checked: boolean;
    disabled: boolean;
    apiEndpoint: string;
    entityId: number;
    label: string;
    private loading;
    private error;
    private previousValue;
    private handleToggle;
    private handleKeyDown;
    render(): import("lit-html").TemplateResult<1>;
}
declare global {
    interface HTMLElementTagNameMap {
        'toggle-switch': ToggleSwitch;
    }
}
//# sourceMappingURL=toggle-switch.d.ts.map