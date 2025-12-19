import { LitElement } from 'lit';
export type ToastVariant = 'success' | 'error' | 'warning' | 'info';
export type ToastPosition = 'top-right' | 'top-left' | 'bottom-right' | 'bottom-left' | 'top-center' | 'bottom-center';
/**
 * Toast notification container - manages and displays toast notifications
 *
 * @example
 * ```html
 * <hockey-toast-container position="top-right"></hockey-toast-container>
 * ```
 *
 * @example JavaScript usage:
 * ```javascript
 * // Get the container
 * const container = document.querySelector('hockey-toast-container');
 *
 * // Show different types of toasts
 * container.success('Item saved successfully!');
 * container.error('Failed to save item');
 * container.warning('This action cannot be undone');
 * container.info('New updates available');
 *
 * // Or use the show method with options
 * container.show({
 *   message: 'Custom toast',
 *   variant: 'success',
 *   duration: 5000,
 *   dismissible: true
 * });
 * ```
 */
export declare class ToastContainer extends LitElement {
    static styles: import("lit").CSSResult;
    /** Position of the toast container */
    position: ToastPosition;
    /** Default duration for toasts in milliseconds */
    defaultDuration: number;
    private toasts;
    private toastCounter;
    render(): import("lit-html").TemplateResult<1>;
    /**
     * Show a toast notification
     */
    show(options: {
        message: string;
        variant?: ToastVariant;
        duration?: number;
        dismissible?: boolean;
    }): string;
    /** Show a success toast */
    success(message: string, duration?: number): string;
    /** Show an error toast */
    error(message: string, duration?: number): string;
    /** Show a warning toast */
    warning(message: string, duration?: number): string;
    /** Show an info toast */
    info(message: string, duration?: number): string;
    /** Dismiss a specific toast by ID */
    dismiss(id: string): void;
    /** Dismiss all toasts */
    dismissAll(): void;
    private handleDismiss;
}
/**
 * Individual toast notification component
 */
export declare class Toast extends LitElement {
    static styles: import("lit").CSSResult;
    toastId: string;
    message: string;
    variant: ToastVariant;
    duration: number;
    dismissible: boolean;
    private exiting;
    render(): import("lit-html").TemplateResult<1>;
    private getIcon;
    private handleDismiss;
}
declare global {
    interface HTMLElementTagNameMap {
        'hockey-toast-container': ToastContainer;
        'hockey-toast': Toast;
    }
}
//# sourceMappingURL=toast.d.ts.map