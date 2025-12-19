import { LitElement } from 'lit';
import './loading-spinner.js';
/**
 * Loading state component for page sections or full page loading
 *
 * @example
 * ```html
 * <hockey-loading-state label="Loading events..."></hockey-loading-state>
 * <hockey-loading-state variant="inline" size="sm"></hockey-loading-state>
 * ```
 */
export declare class LoadingState extends LitElement {
    static styles: import("lit").CSSResult;
    /** Display variant: container (centered), inline, or skeleton */
    variant: 'container' | 'inline' | 'skeleton';
    /** Size passed to spinner */
    size: 'sm' | 'md' | 'lg' | 'xl';
    /** Optional loading label */
    label?: string;
    /** Number of skeleton rows to show */
    skeletonRows: number;
    /** Use minimal padding for container variant */
    minimal: boolean;
    render(): import("lit-html").TemplateResult<1>;
    private renderContainer;
    private renderInline;
    private renderSkeleton;
}
declare global {
    interface HTMLElementTagNameMap {
        'hockey-loading-state': LoadingState;
    }
}
//# sourceMappingURL=loading-state.d.ts.map