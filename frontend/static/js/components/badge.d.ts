import { LitElement } from 'lit';
import { BadgeVariant } from './shared/types.js';
/**
 * Badge component for displaying status labels
 *
 * @example
 * ```html
 * <hockey-badge variant="primary" text="IIHF"></hockey-badge>
 * <hockey-badge variant="warning" text="Historical" outlined></hockey-badge>
 * ```
 */
export declare class Badge extends LitElement {
    static styles: import("lit").CSSResult;
    variant: BadgeVariant;
    text: string;
    outlined: boolean;
    render(): import("lit-html").TemplateResult<1>;
}
declare global {
    interface HTMLElementTagNameMap {
        'hockey-badge': Badge;
    }
}
//# sourceMappingURL=badge.d.ts.map