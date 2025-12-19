import { LitElement } from 'lit';
/**
 * Modal component with keyboard shortcuts and focus management
 *
 * Features:
 * - Escape key to close
 * - Click outside to close
 * - Focus trap within modal
 * - Auto-focus first input
 * - Ctrl/Cmd+Enter to submit forms
 *
 * @example
 * ```html
 * <hockey-modal modal-id="my-modal" title="Edit Item">
 *   <form slot="content">
 *     <input type="text" name="name">
 *     <button type="submit">Save</button>
 *   </form>
 * </hockey-modal>
 * ```
 */
export declare class Modal extends LitElement {
    static styles: import("lit").CSSResult;
    /** Unique ID for this modal */
    modalId: string;
    /** Modal title */
    title: string;
    /** Size variant: small, default, large */
    size: 'small' | 'default' | 'large';
    /** Show keyboard hints */
    showHints: boolean;
    /** Allow closing by clicking outside */
    closeOnOutsideClick: boolean;
    /** Allow closing with Escape key */
    closeOnEscape: boolean;
    /** Auto-focus first input */
    autoFocus: boolean;
    private focusableElements;
    private firstFocusable?;
    private lastFocusable?;
    private previousActiveElement?;
    connectedCallback(): void;
    disconnectedCallback(): void;
    firstUpdated(): void;
    render(): import("lit-html").TemplateResult<1>;
    private handleKeydown;
    private handleTabKey;
    private updateFocusableElements;
    private handleBackdropClick;
    close(): void;
}
declare global {
    interface HTMLElementTagNameMap {
        'hockey-modal': Modal;
    }
}
/**
 * Initialize keyboard shortcuts for legacy modal elements
 *
 * This adds keyboard handling to existing modal elements that use the
 * .modal-backdrop class pattern. Call this after page load or include
 * it in your layout.
 */
export declare function initLegacyModalKeyboardSupport(): void;
//# sourceMappingURL=modal.d.ts.map