import { LitElement, html, css } from 'lit';
import { customElement, property, state } from 'lit/decorators.js';
import { ComponentSize } from './shared/types.js';

/**
 * Flag icon component for displaying country flags
 *
 * @example
 * ```html
 * <flag-icon country-code="cz" country-name="Czech Republic"></flag-icon>
 * <flag-icon country-code="us" country-name="United States" size="lg"></flag-icon>
 * ```
 */
@customElement('flag-icon')
export class FlagIcon extends LitElement {
  static styles = css`
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

  @property({ type: String, attribute: 'country-code' })
  countryCode: string = '';

  @property({ type: String, attribute: 'country-name' })
  countryName: string = '';

  @property({ type: String })
  size: ComponentSize = 'md';

  @state()
  private imageError: boolean = false;

  private handleImageError() {
    this.imageError = true;
  }

  private getInitials(): string {
    if (!this.countryName) return '??';

    const words = this.countryName.trim().split(/\s+/);
    if (words.length === 1) {
      return words[0].substring(0, 2).toUpperCase();
    }
    return (words[0][0] + words[words.length - 1][0]).toUpperCase();
  }

  render() {
    if (!this.countryCode || this.imageError) {
      return html`
        <div class="flag-placeholder size-${this.size}" title=${this.countryName || 'Unknown'}>
          ${this.getInitials()}
        </div>
      `;
    }

    const flagUrl = `https://flagcdn.com/w${this.size === 'lg' ? '80' : this.size === 'md' ? '40' : '20'}/${this.countryCode.toLowerCase()}.png`;

    return html`
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
}

declare global {
  interface HTMLElementTagNameMap {
    'flag-icon': FlagIcon;
  }
}
