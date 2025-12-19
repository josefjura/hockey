import { LitElement } from 'lit';
export type ConfirmVariant = 'danger' | 'warning' | 'info';
interface ConfirmOptions {
    title: string;
    message: string;
    confirmText?: string;
    cancelText?: string;
    variant?: ConfirmVariant;
    onConfirm?: () => void | Promise<void>;
    onCancel?: () => void;
}
/**
 * Confirmation dialog component for replacing browser confirm()
 *
 * @example JavaScript usage:
 * ```javascript
 * const dialog = document.querySelector('hockey-confirm-dialog');
 *
 * // Simple usage
 * const confirmed = await dialog.show({
 *   title: 'Delete Item',
 *   message: 'Are you sure you want to delete this item?',
 *   variant: 'danger'
 * });
 *
 * if (confirmed) {
 *   // User clicked confirm
 * }
 *
 * // With callbacks
 * dialog.show({
 *   title: 'Confirm Action',
 *   message: 'This action cannot be undone.',
 *   confirmText: 'Yes, proceed',
 *   cancelText: 'No, cancel',
 *   variant: 'warning',
 *   onConfirm: () => console.log('Confirmed!'),
 *   onCancel: () => console.log('Cancelled')
 * });
 * ```
 */
export declare class ConfirmDialog extends LitElement {
    static styles: import("lit").CSSResult;
    private isOpen;
    private options;
    private resolvePromise?;
    render(): import("lit-html").TemplateResult<1>;
    private getIcon;
    /**
     * Show the confirmation dialog
     * @returns Promise that resolves to true if confirmed, false if cancelled
     */
    show(options: ConfirmOptions): Promise<boolean>;
    private handleConfirm;
    private handleCancel;
    private handleBackdropClick;
    private handleKeydown;
    private close;
}
declare global {
    interface HTMLElementTagNameMap {
        'hockey-confirm-dialog': ConfirmDialog;
    }
}
/**
 * HTMX integration for confirmation dialogs
 *
 * Use hx-confirm-custom attribute instead of hx-confirm to use the custom dialog.
 * The attribute value should be a JSON string with title and message:
 *
 * @example
 * ```html
 * <button
 *   hx-post="/delete/123"
 *   hx-confirm-custom='{"title": "Delete Item", "message": "Are you sure?", "variant": "danger"}'
 * >Delete</button>
 * ```
 */
export declare function setupHtmxConfirmation(): void;
export {};
//# sourceMappingURL=confirm-dialog.d.ts.map