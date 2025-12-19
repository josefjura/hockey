var __decorate = (this && this.__decorate) || function (decorators, target, key, desc) {
    var c = arguments.length, r = c < 3 ? target : desc === null ? desc = Object.getOwnPropertyDescriptor(target, key) : desc, d;
    if (typeof Reflect === "object" && typeof Reflect.decorate === "function") r = Reflect.decorate(decorators, target, key, desc);
    else for (var i = decorators.length - 1; i >= 0; i--) if (d = decorators[i]) r = (c < 3 ? d(r) : c > 3 ? d(target, key, r) : d(target, key)) || r;
    return c > 3 && r && Object.defineProperty(target, key, r), r;
};
import { LitElement, html, css } from 'lit';
import { customElement, property, state } from 'lit/decorators.js';
import { post } from './shared/api-client.js';
/**
 * Toggle switch component with API integration
 *
 * Features:
 * - Optimistic updates (instant UI feedback)
 * - Automatic rollback on error
 * - Loading state during API call
 * - Keyboard accessible
 *
 * @example
 * ```html
 * <toggle-switch
 *   .checked=${true}
 *   .entityId=${123}
 *   api-endpoint="/api/countries/123/toggle"
 *   label="Enabled">
 * </toggle-switch>
 * ```
 */
let ToggleSwitch = class ToggleSwitch extends LitElement {
    constructor() {
        super(...arguments);
        this.checked = false;
        this.disabled = false;
        this.apiEndpoint = '';
        this.entityId = 0;
        this.label = '';
        this.loading = false;
        this.error = '';
        this.previousValue = false;
    }
    async handleToggle() {
        if (this.disabled || this.loading || !this.apiEndpoint) {
            return;
        }
        // Store previous value for rollback
        this.previousValue = this.checked;
        // Optimistic update
        this.checked = !this.checked;
        this.loading = true;
        this.error = '';
        // Dispatch change event immediately (optimistic)
        this.dispatchEvent(new CustomEvent('toggle-change', {
            detail: {
                checked: this.checked,
                entityId: this.entityId,
            },
            bubbles: true,
            composed: true,
        }));
        // Call API
        const response = await post(this.apiEndpoint);
        this.loading = false;
        if (response.error) {
            // Rollback on error
            this.checked = this.previousValue;
            this.error = response.error.message || 'Failed to update';
            // Dispatch rollback event
            this.dispatchEvent(new CustomEvent('toggle-error', {
                detail: {
                    error: this.error,
                    entityId: this.entityId,
                },
                bubbles: true,
                composed: true,
            }));
            // Clear error after 3 seconds
            setTimeout(() => {
                this.error = '';
            }, 3000);
        }
        else {
            // Success - the optimistic update was correct
            this.dispatchEvent(new CustomEvent('toggle-success', {
                detail: {
                    checked: this.checked,
                    entityId: this.entityId,
                },
                bubbles: true,
                composed: true,
            }));
        }
    }
    handleKeyDown(e) {
        if (e.key === 'Enter' || e.key === ' ') {
            e.preventDefault();
            this.handleToggle();
        }
    }
    render() {
        const containerClass = `toggle-switch ${this.disabled ? 'disabled' : ''} ${this.loading ? 'loading' : ''}`;
        return html `
      <div class="toggle-container">
        <label class=${containerClass} @keydown=${this.handleKeyDown}>
          <input
            type="checkbox"
            .checked=${this.checked}
            .disabled=${this.disabled || this.loading}
            @change=${this.handleToggle}
            role="switch"
            aria-checked=${this.checked}
            aria-label=${this.label || 'Toggle'}
          />
          <span class="toggle-slider"></span>
        </label>
        ${this.label ? html `<span class="toggle-label">${this.label}</span>` : ''}
      </div>
      ${this.error ? html `<div class="error-message">${this.error}</div>` : ''}
    `;
    }
};
ToggleSwitch.styles = css `
    :host {
      display: inline-block;
    }

    .toggle-container {
      display: inline-flex;
      align-items: center;
      gap: 0.5rem;
    }

    .toggle-label {
      font-size: 0.875rem;
      color: var(--gray-700, #374151);
      user-select: none;
    }

    .toggle-switch {
      position: relative;
      display: inline-block;
      width: 44px;
      height: 24px;
      cursor: pointer;
    }

    .toggle-switch.disabled {
      opacity: 0.5;
      cursor: not-allowed;
    }

    .toggle-switch input {
      opacity: 0;
      width: 0;
      height: 0;
    }

    .toggle-slider {
      position: absolute;
      cursor: pointer;
      top: 0;
      left: 0;
      right: 0;
      bottom: 0;
      background-color: var(--gray-300, #d1d5db);
      transition: 0.2s;
      border-radius: 24px;
    }

    .toggle-slider:before {
      position: absolute;
      content: '';
      height: 18px;
      width: 18px;
      left: 3px;
      bottom: 3px;
      background-color: white;
      transition: 0.2s;
      border-radius: 50%;
    }

    input:checked + .toggle-slider {
      background-color: #3b82f6;
    }

    input:checked + .toggle-slider:before {
      transform: translateX(20px);
    }

    input:focus + .toggle-slider {
      box-shadow: 0 0 0 2px rgba(59, 130, 246, 0.3);
    }

    .toggle-switch.loading .toggle-slider {
      opacity: 0.7;
    }

    .toggle-switch.loading .toggle-slider:before {
      animation: pulse 1s ease-in-out infinite;
    }

    @keyframes pulse {
      0%, 100% {
        opacity: 1;
      }
      50% {
        opacity: 0.5;
      }
    }

    .error-message {
      color: #ef4444;
      font-size: 0.75rem;
      margin-top: 0.25rem;
    }
  `;
__decorate([
    property({ type: Boolean })
], ToggleSwitch.prototype, "checked", void 0);
__decorate([
    property({ type: Boolean })
], ToggleSwitch.prototype, "disabled", void 0);
__decorate([
    property({ type: String, attribute: 'api-endpoint' })
], ToggleSwitch.prototype, "apiEndpoint", void 0);
__decorate([
    property({ type: Number, attribute: 'entity-id' })
], ToggleSwitch.prototype, "entityId", void 0);
__decorate([
    property({ type: String })
], ToggleSwitch.prototype, "label", void 0);
__decorate([
    state()
], ToggleSwitch.prototype, "loading", void 0);
__decorate([
    state()
], ToggleSwitch.prototype, "error", void 0);
ToggleSwitch = __decorate([
    customElement('toggle-switch')
], ToggleSwitch);
export { ToggleSwitch };
//# sourceMappingURL=toggle-switch.js.map