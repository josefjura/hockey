import { LitElement, html, css } from 'lit';
import { customElement, property, state } from 'lit/decorators.js';

export type ToastVariant = 'success' | 'error' | 'warning' | 'info';
export type ToastPosition = 'top-right' | 'top-left' | 'bottom-right' | 'bottom-left' | 'top-center' | 'bottom-center';

interface ToastItem {
	id: string;
	message: string;
	variant: ToastVariant;
	duration: number;
	dismissible: boolean;
}

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
@customElement('hockey-toast-container')
export class ToastContainer extends LitElement {
	static styles = css`
    :host {
      position: fixed;
      z-index: 9999;
      pointer-events: none;
      display: flex;
      flex-direction: column;
      gap: 0.5rem;
      max-width: 400px;
      padding: 1rem;
    }

    /* Position variants */
    :host([position='top-right']) {
      top: 0;
      right: 0;
    }

    :host([position='top-left']) {
      top: 0;
      left: 0;
    }

    :host([position='bottom-right']) {
      bottom: 0;
      right: 0;
    }

    :host([position='bottom-left']) {
      bottom: 0;
      left: 0;
    }

    :host([position='top-center']) {
      top: 0;
      left: 50%;
      transform: translateX(-50%);
    }

    :host([position='bottom-center']) {
      bottom: 0;
      left: 50%;
      transform: translateX(-50%);
    }
  `;

	/** Position of the toast container */
	@property({ type: String, reflect: true })
	position: ToastPosition = 'top-right';

	/** Default duration for toasts in milliseconds */
	@property({ type: Number })
	defaultDuration = 4000;

	/**
	 * Active toasts (exposed for observability in tests)
	 *
	 * Note: This is intentionally a @property instead of @state to allow
	 * tests to observe toast state. This follows Lit's recommended pattern
	 * for testing components with encapsulated shadow DOM.
	 * @readonly - Should not be modified directly by consumers
	 */
	@property({ type: Array })
	toasts: ToastItem[] = [];

	private toastCounter = 0;

	render() {
		return html`
      ${this.toasts.map(
			toast => html`
          <hockey-toast
            .toastId=${toast.id}
            .message=${toast.message}
            .variant=${toast.variant}
            .duration=${toast.duration}
            .dismissible=${toast.dismissible}
            @toast-dismiss=${this.handleDismiss}
          ></hockey-toast>
        `
		)}
    `;
	}

	/**
	 * Show a toast notification
	 */
	show(options: {
		message: string;
		variant?: ToastVariant;
		duration?: number;
		dismissible?: boolean;
	}): string {
		const id = `toast-${++this.toastCounter}`;
		const toast: ToastItem = {
			id,
			message: options.message,
			variant: options.variant || 'info',
			duration: options.duration ?? this.defaultDuration,
			dismissible: options.dismissible ?? true,
		};

		this.toasts = [...this.toasts, toast];

		// Auto-remove after duration (if duration > 0)
		if (toast.duration > 0) {
			setTimeout(() => this.dismiss(id), toast.duration);
		}

		return id;
	}

	/** Show a success toast */
	success(message: string, duration?: number): string {
		return this.show({ message, variant: 'success', duration });
	}

	/** Show an error toast */
	error(message: string, duration?: number): string {
		return this.show({ message, variant: 'error', duration: duration ?? 6000 });
	}

	/** Show a warning toast */
	warning(message: string, duration?: number): string {
		return this.show({ message, variant: 'warning', duration });
	}

	/** Show an info toast */
	info(message: string, duration?: number): string {
		return this.show({ message, variant: 'info', duration });
	}

	/** Dismiss a specific toast by ID */
	dismiss(id: string): void {
		this.toasts = this.toasts.filter(t => t.id !== id);
	}

	/** Dismiss all toasts */
	dismissAll(): void {
		this.toasts = [];
	}

	private handleDismiss(e: CustomEvent<{ id: string }>) {
		this.dismiss(e.detail.id);
	}
}

/**
 * Individual toast notification component
 */
@customElement('hockey-toast')
export class Toast extends LitElement {
	static styles = css`
    :host {
      display: block;
      pointer-events: auto;
    }

    .toast {
      display: flex;
      align-items: flex-start;
      gap: 0.75rem;
      padding: 1rem;
      border-radius: 8px;
      box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
      animation: slideIn 0.3s ease-out;
      min-width: 280px;
      max-width: 100%;
    }

    .toast.exiting {
      animation: slideOut 0.2s ease-in forwards;
    }

    @keyframes slideIn {
      from {
        opacity: 0;
        transform: translateX(100%);
      }
      to {
        opacity: 1;
        transform: translateX(0);
      }
    }

    @keyframes slideOut {
      from {
        opacity: 1;
        transform: translateX(0);
      }
      to {
        opacity: 0;
        transform: translateX(100%);
      }
    }

    /* Variant styles */
    .toast-success {
      background: var(--success-color);
      color: white;
    }

    .toast-error {
      background: var(--danger-color);
      color: white;
    }

    .toast-warning {
      background: var(--warning-color);
      color: white;
    }

    .toast-info {
      background: var(--primary-color);
      color: white;
    }

    .icon {
      flex-shrink: 0;
      width: 20px;
      height: 20px;
    }

    .message {
      flex: 1;
      font-size: 0.875rem;
      font-weight: 500;
      line-height: 1.4;
    }

    .dismiss-btn {
      flex-shrink: 0;
      background: none;
      border: none;
      color: inherit;
      cursor: pointer;
      padding: 0;
      opacity: 0.7;
      transition: opacity 0.2s;
      display: flex;
      align-items: center;
      justify-content: center;
    }

    .dismiss-btn:hover {
      opacity: 1;
    }

    .dismiss-btn svg {
      width: 16px;
      height: 16px;
    }
  `;

	@property({ type: String })
	toastId = '';

	@property({ type: String })
	message = '';

	@property({ type: String })
	variant: ToastVariant = 'info';

	@property({ type: Number })
	duration = 4000;

	@property({ type: Boolean })
	dismissible = true;

	@state()
	private exiting = false;

	render() {
		return html`
      <div class="toast toast-${this.variant} ${this.exiting ? 'exiting' : ''}">
        <span class="icon">${this.getIcon()}</span>
        <span class="message">${this.message}</span>
        ${this.dismissible
				? html`
              <button class="dismiss-btn" @click=${this.handleDismiss} aria-label="Dismiss">
                <svg viewBox="0 0 20 20" fill="currentColor">
                  <path
                    fill-rule="evenodd"
                    d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z"
                    clip-rule="evenodd"
                  />
                </svg>
              </button>
            `
				: ''}
      </div>
    `;
	}

	private getIcon() {
		switch (this.variant) {
			case 'success':
				return html`
          <svg viewBox="0 0 20 20" fill="currentColor">
            <path
              fill-rule="evenodd"
              d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z"
              clip-rule="evenodd"
            />
          </svg>
        `;
			case 'error':
				return html`
          <svg viewBox="0 0 20 20" fill="currentColor">
            <path
              fill-rule="evenodd"
              d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z"
              clip-rule="evenodd"
            />
          </svg>
        `;
			case 'warning':
				return html`
          <svg viewBox="0 0 20 20" fill="currentColor">
            <path
              fill-rule="evenodd"
              d="M8.257 3.099c.765-1.36 2.722-1.36 3.486 0l5.58 9.92c.75 1.334-.213 2.98-1.742 2.98H4.42c-1.53 0-2.493-1.646-1.743-2.98l5.58-9.92zM11 13a1 1 0 11-2 0 1 1 0 012 0zm-1-8a1 1 0 00-1 1v3a1 1 0 002 0V6a1 1 0 00-1-1z"
              clip-rule="evenodd"
            />
          </svg>
        `;
			case 'info':
			default:
				return html`
          <svg viewBox="0 0 20 20" fill="currentColor">
            <path
              fill-rule="evenodd"
              d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7-4a1 1 0 11-2 0 1 1 0 012 0zM9 9a1 1 0 000 2v3a1 1 0 001 1h1a1 1 0 100-2v-3a1 1 0 00-1-1H9z"
              clip-rule="evenodd"
            />
          </svg>
        `;
		}
	}

	private handleDismiss() {
		this.exiting = true;
		// Wait for animation to complete before dispatching
		setTimeout(() => {
			this.dispatchEvent(
				new CustomEvent('toast-dismiss', {
					detail: { id: this.toastId },
					bubbles: true,
					composed: true,
				})
			);
		}, 200);
	}
}

declare global {
	interface HTMLElementTagNameMap {
		'hockey-toast-container': ToastContainer;
		'hockey-toast': Toast;
	}
}
