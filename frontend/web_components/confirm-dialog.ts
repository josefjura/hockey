import { LitElement, html, css } from 'lit';
import { customElement, property, state } from 'lit/decorators.js';

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
@customElement('hockey-confirm-dialog')
export class ConfirmDialog extends LitElement {
	static styles = css`
    :host {
      display: contents;
    }

    .backdrop {
      position: fixed;
      inset: 0;
      background: rgba(0, 0, 0, 0.5);
      display: flex;
      align-items: center;
      justify-content: center;
      z-index: 10000;
      animation: fadeIn 0.15s ease-out;
    }

    @keyframes fadeIn {
      from {
        opacity: 0;
      }
      to {
        opacity: 1;
      }
    }

    .dialog {
      background: white;
      border-radius: 12px;
      padding: 1.5rem;
      max-width: 400px;
      width: 90%;
      box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.1), 0 10px 10px -5px rgba(0, 0, 0, 0.04);
      animation: slideIn 0.2s ease-out;
    }

    @keyframes slideIn {
      from {
        opacity: 0;
        transform: scale(0.95) translateY(-10px);
      }
      to {
        opacity: 1;
        transform: scale(1) translateY(0);
      }
    }

    .icon-container {
      width: 48px;
      height: 48px;
      border-radius: 50%;
      display: flex;
      align-items: center;
      justify-content: center;
      margin: 0 auto 1rem;
    }

    .icon-container.danger {
      background: #fef2f2;
      color: #ef4444;
    }

    .icon-container.warning {
      background: #fffbeb;
      color: #f59e0b;
    }

    .icon-container.info {
      background: #eff6ff;
      color: #3b82f6;
    }

    .icon-container svg {
      width: 24px;
      height: 24px;
    }

    .title {
      font-size: 1.125rem;
      font-weight: 600;
      color: var(--gray-900, #111827);
      text-align: center;
      margin-bottom: 0.5rem;
    }

    .message {
      font-size: 0.875rem;
      color: var(--gray-600, #4b5563);
      text-align: center;
      line-height: 1.5;
      margin-bottom: 1.5rem;
    }

    .actions {
      display: flex;
      gap: 0.75rem;
      justify-content: center;
    }

    .btn {
      padding: 0.625rem 1.25rem;
      border-radius: 6px;
      font-size: 0.875rem;
      font-weight: 500;
      cursor: pointer;
      transition: all 0.15s;
      border: none;
      min-width: 100px;
    }

    .btn-cancel {
      background: white;
      border: 1px solid var(--gray-300, #d1d5db);
      color: var(--gray-700, #374151);
    }

    .btn-cancel:hover {
      background: var(--gray-50, #f9fafb);
    }

    .btn-confirm {
      color: white;
    }

    .btn-confirm.danger {
      background: #ef4444;
    }

    .btn-confirm.danger:hover {
      background: #dc2626;
    }

    .btn-confirm.warning {
      background: #f59e0b;
    }

    .btn-confirm.warning:hover {
      background: #d97706;
    }

    .btn-confirm.info {
      background: #3b82f6;
    }

    .btn-confirm.info:hover {
      background: #2563eb;
    }

    .btn:focus {
      outline: none;
      box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.3);
    }
  `;

	@state()
	private isOpen = false;

	@state()
	private options: ConfirmOptions = {
		title: '',
		message: '',
		confirmText: 'Confirm',
		cancelText: 'Cancel',
		variant: 'danger',
	};

	private resolvePromise?: (value: boolean) => void;

	render() {
		if (!this.isOpen) {
			return html``;
		}

		return html`
      <div class="backdrop" @click=${this.handleBackdropClick} @keydown=${this.handleKeydown}>
        <div class="dialog" @click=${(e: Event) => e.stopPropagation()}>
          <div class="icon-container ${this.options.variant}">${this.getIcon()}</div>
          <h3 class="title">${this.options.title}</h3>
          <p class="message">${this.options.message}</p>
          <div class="actions">
            <button class="btn btn-cancel" @click=${this.handleCancel}>
              ${this.options.cancelText}
            </button>
            <button class="btn btn-confirm ${this.options.variant}" @click=${this.handleConfirm}>
              ${this.options.confirmText}
            </button>
          </div>
        </div>
      </div>
    `;
	}

	private getIcon() {
		switch (this.options.variant) {
			case 'danger':
				return html`
          <svg viewBox="0 0 20 20" fill="currentColor">
            <path
              fill-rule="evenodd"
              d="M9 2a1 1 0 00-.894.553L7.382 4H4a1 1 0 000 2v10a2 2 0 002 2h8a2 2 0 002-2V6a1 1 0 100-2h-3.382l-.724-1.447A1 1 0 0011 2H9zM7 8a1 1 0 012 0v6a1 1 0 11-2 0V8zm5-1a1 1 0 00-1 1v6a1 1 0 102 0V8a1 1 0 00-1-1z"
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

	/**
	 * Show the confirmation dialog
	 * @returns Promise that resolves to true if confirmed, false if cancelled
	 */
	show(options: ConfirmOptions): Promise<boolean> {
		this.options = {
			confirmText: 'Confirm',
			cancelText: 'Cancel',
			variant: 'danger',
			...options,
		};
		this.isOpen = true;

		// Focus trap - focus the cancel button by default
		this.updateComplete.then(() => {
			const cancelBtn = this.shadowRoot?.querySelector('.btn-cancel') as HTMLButtonElement;
			cancelBtn?.focus();
		});

		return new Promise(resolve => {
			this.resolvePromise = resolve;
		});
	}

	private handleConfirm = async () => {
		if (this.options.onConfirm) {
			await this.options.onConfirm();
		}
		this.close(true);
	};

	private handleCancel = () => {
		if (this.options.onCancel) {
			this.options.onCancel();
		}
		this.close(false);
	};

	private handleBackdropClick = () => {
		this.handleCancel();
	};

	private handleKeydown = (e: KeyboardEvent) => {
		if (e.key === 'Escape') {
			this.handleCancel();
		} else if (e.key === 'Enter') {
			e.preventDefault();
			this.handleConfirm();
		}
	};

	private close(result: boolean) {
		this.isOpen = false;
		if (this.resolvePromise) {
			this.resolvePromise(result);
			this.resolvePromise = undefined;
		}
	}
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
export function setupHtmxConfirmation() {
	document.body.addEventListener('htmx:confirm', (e: Event) => {
		const evt = e as CustomEvent;
		const elt = evt.detail.elt as HTMLElement;
		const confirmCustom = elt.getAttribute('hx-confirm-custom');

		if (!confirmCustom) {
			return; // Use default hx-confirm behavior
		}

		// Prevent the default confirmation
		evt.preventDefault();

		try {
			const options = JSON.parse(confirmCustom) as ConfirmOptions;
			const dialog = document.querySelector('hockey-confirm-dialog') as ConfirmDialog;

			if (dialog) {
				dialog.show(options).then(confirmed => {
					if (confirmed) {
						evt.detail.issueRequest();
					}
				});
			} else {
				console.warn('hockey-confirm-dialog not found in document');
				// Fallback to native confirm
				if (confirm(options.message)) {
					evt.detail.issueRequest();
				}
			}
		} catch (error) {
			console.error('Error parsing hx-confirm-custom:', error);
			// Fallback to native confirm
			if (confirm(confirmCustom)) {
				evt.detail.issueRequest();
			}
		}
	});
}

// Auto-setup when the module loads
if (typeof document !== 'undefined') {
	if (document.readyState === 'loading') {
		document.addEventListener('DOMContentLoaded', setupHtmxConfirmation);
	} else {
		setupHtmxConfirmation();
	}
}
