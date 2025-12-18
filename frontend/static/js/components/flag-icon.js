var __decorate = (this && this.__decorate) || function (decorators, target, key, desc) {
    var c = arguments.length, r = c < 3 ? target : desc === null ? desc = Object.getOwnPropertyDescriptor(target, key) : desc, d;
    if (typeof Reflect === "object" && typeof Reflect.decorate === "function") r = Reflect.decorate(decorators, target, key, desc);
    else for (var i = decorators.length - 1; i >= 0; i--) if (d = decorators[i]) r = (c < 3 ? d(r) : c > 3 ? d(target, key, r) : d(target, key)) || r;
    return c > 3 && r && Object.defineProperty(target, key, r), r;
};
import { LitElement, html, css } from 'lit';
import { customElement, property, state } from 'lit/decorators.js';
/**
 * Flag icon component for displaying country flags
 *
 * @example
 * ```html
 * <flag-icon country-code="cz" country-name="Czech Republic"></flag-icon>
 * <flag-icon country-code="us" country-name="United States" size="lg"></flag-icon>
 * ```
 */
let FlagIcon = class FlagIcon extends LitElement {
    constructor() {
        super(...arguments);
        this.countryCode = '';
        this.countryName = '';
        this.size = 'md';
        this.imageError = false;
    }
    handleImageError() {
        this.imageError = true;
    }
    getInitials() {
        if (!this.countryName)
            return '??';
        const words = this.countryName.trim().split(/\s+/);
        if (words.length === 1) {
            return words[0].substring(0, 2).toUpperCase();
        }
        return (words[0][0] + words[words.length - 1][0]).toUpperCase();
    }
    render() {
        if (!this.countryCode || this.imageError) {
            return html `
        <div class="flag-placeholder size-${this.size}" title=${this.countryName || 'Unknown'}>
          ${this.getInitials()}
        </div>
      `;
        }
        const flagUrl = `https://flagcdn.com/w${this.size === 'lg' ? '80' : this.size === 'md' ? '40' : '20'}/${this.countryCode.toLowerCase()}.png`;
        return html `
      <div class="flag-container">
        <img
          class="flag-icon size-${this.size}"
          src=${flagUrl}
          alt=${this.countryName || `${this.countryCode} flag`}
          title=${this.countryName || this.countryCode}
          loading="lazy"
          @error=${this.handleImageError}
        />
      </div>
    `;
    }
};
FlagIcon.styles = css `
    :host {
      display: inline-block;
    }

    .flag-container {
      display: inline-flex;
      align-items: center;
      justify-content: center;
    }

    .flag-icon {
      object-fit: cover;
      border: 1px solid var(--gray-300, #d1d5db);
      border-radius: 2px;
    }

    .flag-icon.size-sm {
      width: 20px;
      height: 15px;
    }

    .flag-icon.size-md {
      width: 32px;
      height: 24px;
    }

    .flag-icon.size-lg {
      width: 48px;
      height: 36px;
    }

    .flag-placeholder {
      display: inline-flex;
      align-items: center;
      justify-content: center;
      background-color: var(--gray-200, #e5e7eb);
      color: var(--gray-600, #4b5563);
      font-size: 0.75rem;
      font-weight: 600;
      border-radius: 2px;
    }

    .flag-placeholder.size-sm {
      width: 20px;
      height: 15px;
      font-size: 0.625rem;
    }

    .flag-placeholder.size-md {
      width: 32px;
      height: 24px;
      font-size: 0.75rem;
    }

    .flag-placeholder.size-lg {
      width: 48px;
      height: 36px;
      font-size: 1rem;
    }
  `;
__decorate([
    property({ type: String, attribute: 'country-code' })
], FlagIcon.prototype, "countryCode", void 0);
__decorate([
    property({ type: String, attribute: 'country-name' })
], FlagIcon.prototype, "countryName", void 0);
__decorate([
    property({ type: String })
], FlagIcon.prototype, "size", void 0);
__decorate([
    state()
], FlagIcon.prototype, "imageError", void 0);
FlagIcon = __decorate([
    customElement('flag-icon')
], FlagIcon);
export { FlagIcon };
//# sourceMappingURL=flag-icon.js.map