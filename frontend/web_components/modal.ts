import { LitElement, html, css } from 'lit';
import { customElement, property, state } from 'lit/decorators.js';

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
@customElement('hockey-modal')
export class Modal extends LitElement {
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
      z-index: 1000;
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

    .modal {
      background: white;
      border-radius: 12px;
      padding: 2rem;
      max-width: 500px;
      width: 90%;
      max-height: 90vh;
      overflow-y: auto;
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

    .modal.large {
      max-width: 700px;
    }

    .modal.small {
      max-width: 400px;
    }

    .header {
      display: flex;
      justify-content: space-between;
      align-items: center;
      margin-bottom: 1.5rem;
    }

    .title {
      font-size: 1.5rem;
      font-weight: 700;
      color: var(--gray-900, #111827);
      margin: 0;
    }

    .close-btn {
      background: none;
      border: none;
      cursor: pointer;
      padding: 0.25rem;
      color: var(--gray-500, #6b7280);
      border-radius: 4px;
      transition: all 0.15s;
      display: flex;
      align-items: center;
      justify-content: center;
    }

    .close-btn:hover {
      background: var(--gray-100, #f3f4f6);
      color: var(--gray-700, #374151);
    }

    .close-btn:focus {
      outline: none;
      box-shadow: 0 0 0 2px var(--primary-color, #3b82f6);
    }

    .close-btn svg {
      width: 20px;
      height: 20px;
    }

    .keyboard-hint {
      font-size: 0.75rem;
      color: var(--gray-400, #9ca3af);
      margin-top: 1rem;
      text-align: center;
    }

    kbd {
      display: inline-block;
      padding: 0.125rem 0.375rem;
      font-size: 0.6875rem;
      font-family: monospace;
      background: var(--gray-100, #f3f4f6);
      border: 1px solid var(--gray-300, #d1d5db);
      border-radius: 3px;
      margin: 0 0.125rem;
    }
  `;

  /** Unique ID for this modal */
  @property({ type: String, attribute: 'modal-id' })
  modalId = '';

  /** Modal title */
  @property({ type: String })
  title = '';

  /** Size variant: small, default, large */
  @property({ type: String })
  size: 'small' | 'default' | 'large' = 'default';

  /** Show keyboard hints */
  @property({ type: Boolean, attribute: 'show-hints' })
  showHints = true;

  /** Allow closing by clicking outside */
  @property({ type: Boolean, attribute: 'close-on-outside-click' })
  closeOnOutsideClick = true;

  /** Allow closing with Escape key */
  @property({ type: Boolean, attribute: 'close-on-escape' })
  closeOnEscape = true;

  /** Auto-focus first input */
  @property({ type: Boolean, attribute: 'auto-focus' })
  autoFocus = true;

  private focusableElements: HTMLElement[] = [];
  private firstFocusable?: HTMLElement;
  private lastFocusable?: HTMLElement;
  private previousActiveElement?: HTMLElement;

  connectedCallback() {
    super.connectedCallback();
    // Save currently focused element
    this.previousActiveElement = document.activeElement as HTMLElement;

    // Add keyboard listener
    document.addEventListener('keydown', this.handleKeydown);

    // Prevent body scrolling
    document.body.style.overflow = 'hidden';
  }

  disconnectedCallback() {
    super.disconnectedCallback();
    document.removeEventListener('keydown', this.handleKeydown);
    document.body.style.overflow = '';

    // Restore focus to previous element
    if (this.previousActiveElement) {
      this.previousActiveElement.focus();
    }
  }

  firstUpdated() {
    this.updateFocusableElements();

    if (this.autoFocus) {
      // Focus first input or first focusable element
      requestAnimationFrame(() => {
        const slotted = this.querySelector('input, select, textarea') as HTMLElement;
        if (slotted) {
          slotted.focus();
        } else if (this.firstFocusable) {
          this.firstFocusable.focus();
        }
      });
    }
  }

  render() {
    const sizeClass = this.size !== 'default' ? this.size : '';

    return html`
      <div class="backdrop" @click=${this.handleBackdropClick}>
        <div class="modal ${sizeClass}" @click=${(e: Event) => e.stopPropagation()}>
          <div class="header">
            <h2 class="title">${this.title}</h2>
            <button class="close-btn" @click=${this.close} aria-label="Close modal">
              <svg viewBox="0 0 20 20" fill="currentColor">
                <path
                  fill-rule="evenodd"
                  d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z"
                  clip-rule="evenodd"
                />
              </svg>
            </button>
          </div>

          <slot name="content"></slot>

          ${this.showHints
            ? html`
                <div class="keyboard-hint">
                  <kbd>Esc</kbd> to close &middot; <kbd>Ctrl</kbd>+<kbd>Enter</kbd> to submit
                </div>
              `
            : ''}
        </div>
      </div>
    `;
  }

  private handleKeydown = (e: KeyboardEvent) => {
    // Escape to close
    if (e.key === 'Escape' && this.closeOnEscape) {
      e.preventDefault();
      this.close();
      return;
    }

    // Ctrl/Cmd + Enter to submit
    if (e.key === 'Enter' && (e.ctrlKey || e.metaKey)) {
      const form = this.querySelector('form');
      if (form) {
        e.preventDefault();
        const submitBtn = form.querySelector(
          'button[type="submit"], input[type="submit"]'
        ) as HTMLElement;
        if (submitBtn) {
          submitBtn.click();
        } else {
          form.requestSubmit();
        }
      }
      return;
    }

    // Tab key for focus trapping
    if (e.key === 'Tab') {
      this.handleTabKey(e);
    }
  };

  private handleTabKey(e: KeyboardEvent) {
    this.updateFocusableElements();

    if (this.focusableElements.length === 0) return;

    if (e.shiftKey) {
      // Shift + Tab
      if (document.activeElement === this.firstFocusable) {
        e.preventDefault();
        this.lastFocusable?.focus();
      }
    } else {
      // Tab
      if (document.activeElement === this.lastFocusable) {
        e.preventDefault();
        this.firstFocusable?.focus();
      }
    }
  }

  private updateFocusableElements() {
    // Get all focusable elements in the modal and slotted content
    const modalElements = Array.from(
      this.shadowRoot?.querySelectorAll(
        'button, [href], input, select, textarea, [tabindex]:not([tabindex="-1"])'
      ) || []
    ) as HTMLElement[];

    const slottedElements = Array.from(
      this.querySelectorAll(
        'button, [href], input, select, textarea, [tabindex]:not([tabindex="-1"])'
      ) || []
    ) as HTMLElement[];

    this.focusableElements = [...modalElements, ...slottedElements].filter(
      el => !el.hasAttribute('disabled') && el.offsetParent !== null
    );

    this.firstFocusable = this.focusableElements[0];
    this.lastFocusable = this.focusableElements[this.focusableElements.length - 1];
  }

  private handleBackdropClick = (e: Event) => {
    if (this.closeOnOutsideClick && e.target === e.currentTarget) {
      this.close();
    }
  };

  close() {
    this.dispatchEvent(
      new CustomEvent('modal-close', {
        bubbles: true,
        composed: true,
      })
    );
    // Remove the element from DOM
    this.remove();
  }
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
export function initLegacyModalKeyboardSupport() {
  document.addEventListener('keydown', (e: KeyboardEvent) => {
    // Find any open modal
    const modal = document.querySelector('.modal-backdrop') as HTMLElement;
    if (!modal) return;

    // Escape to close
    if (e.key === 'Escape') {
      e.preventDefault();
      modal.remove();
      return;
    }

    // Ctrl/Cmd + Enter to submit
    if (e.key === 'Enter' && (e.ctrlKey || e.metaKey)) {
      const form = modal.querySelector('form');
      if (form) {
        e.preventDefault();
        const submitBtn = form.querySelector(
          'button[type="submit"], input[type="submit"]'
        ) as HTMLElement;
        if (submitBtn) {
          submitBtn.click();
        }
      }
    }
  });

  // Auto-focus first input when modal is added
  const observer = new MutationObserver(mutations => {
    for (const mutation of mutations) {
      for (const node of mutation.addedNodes) {
        if (node instanceof HTMLElement) {
          const modal = node.classList?.contains('modal-backdrop')
            ? node
            : node.querySelector?.('.modal-backdrop');
          if (modal) {
            // Focus first input
            const firstInput = modal.querySelector(
              'input:not([type="hidden"]), select, textarea'
            ) as HTMLElement;
            if (firstInput) {
              requestAnimationFrame(() => firstInput.focus());
            }
          }
        }
      }
    }
  });

  observer.observe(document.body, { childList: true, subtree: true });
}

// Auto-initialize legacy support
if (typeof document !== 'undefined') {
  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', initLegacyModalKeyboardSupport);
  } else {
    initLegacyModalKeyboardSupport();
  }
}
