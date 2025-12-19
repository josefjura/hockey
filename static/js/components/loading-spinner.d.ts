import { LitElement } from 'lit';
/**
 * Loading spinner component with multiple size and style variants
 *
 * @example
 * ```html
 * <hockey-loading-spinner></hockey-loading-spinner>
 * <hockey-loading-spinner size="lg" label="Loading events..."></hockey-loading-spinner>
 * <hockey-loading-spinner variant="dots"></hockey-loading-spinner>
 * ```
 */
export declare class LoadingSpinner extends LitElement {
    static styles: import("lit").CSSResult;
    /** Size of the spinner: sm, md, lg, xl */
    size: 'sm' | 'md' | 'lg' | 'xl';
    /** Visual variant: circle or dots */
    variant: 'circle' | 'dots';
    /** Optional label text to display */
    label?: string;
    /** Layout direction for label */
    layout: 'vertical' | 'horizontal';
    render(): import("lit-html").TemplateResult<1>;
    private renderCircle;
    private renderDots;
}
declare global {
    interface HTMLElementTagNameMap {
        'hockey-loading-spinner': LoadingSpinner;
    }
}
//# sourceMappingURL=loading-spinner.d.ts.map