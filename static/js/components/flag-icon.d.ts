import { LitElement } from 'lit';
import { ComponentSize } from './shared/types.js';
/**
 * Flag icon component for displaying country flags
 *
 * @example
 * ```html
 * <flag-icon country-code="cz" country-name="Czech Republic"></flag-icon>
 * <flag-icon country-code="us" country-name="United States" size="lg"></flag-icon>
 * ```
 */
export declare class FlagIcon extends LitElement {
    static styles: import("lit").CSSResult;
    countryCode: string;
    countryName: string;
    size: ComponentSize;
    private imageError;
    private handleImageError;
    private getInitials;
    render(): import("lit-html").TemplateResult<1>;
}
declare global {
    interface HTMLElementTagNameMap {
        'flag-icon': FlagIcon;
    }
}
//# sourceMappingURL=flag-icon.d.ts.map