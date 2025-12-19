var __decorate = (this && this.__decorate) || function (decorators, target, key, desc) {
    var c = arguments.length, r = c < 3 ? target : desc === null ? desc = Object.getOwnPropertyDescriptor(target, key) : desc, d;
    if (typeof Reflect === "object" && typeof Reflect.decorate === "function") r = Reflect.decorate(decorators, target, key, desc);
    else for (var i = decorators.length - 1; i >= 0; i--) if (d = decorators[i]) r = (c < 3 ? d(r) : c > 3 ? d(target, key, r) : d(target, key)) || r;
    return c > 3 && r && Object.defineProperty(target, key, r), r;
};
import { LitElement, html, css } from 'lit';
import { customElement, property } from 'lit/decorators.js';
/**
 * Loading spinner component with multiple size and style variants
 *
 * @example
 * ```html
 * <hockey-loading-spinner></hockey-loading-spinner>
 * <hockey-loading-spinner size="lg" label="Loading events..."></hockey-loading-spinner>
 * <hockey-loading-spinner variant="dots"></hockey-loading-spinner>
 * ```
 */
let LoadingSpinner = class LoadingSpinner extends LitElement {
    constructor() {
        super(...arguments);
        /** Size of the spinner: sm, md, lg, xl */
        this.size = 'md';
        /** Visual variant: circle or dots */
        this.variant = 'circle';
        /** Layout direction for label */
        this.layout = 'vertical';
    }
    render() {
        return html `
      <div class="spinner-container ${this.layout === 'horizontal' ? 'horizontal' : ''}">
        ${this.variant === 'circle' ? this.renderCircle() : this.renderDots()}
        ${this.label ? html `<span class="label ${this.size}">${this.label}</span>` : ''}
      </div>
    `;
    }
    renderCircle() {
        return html `<div class="spinner-circle ${this.size}"></div>`;
    }
    renderDots() {
        return html `
      <div class="spinner-dots ${this.size}">
        <div class="dot"></div>
        <div class="dot"></div>
        <div class="dot"></div>
      </div>
    `;
    }
};
LoadingSpinner.styles = css `
    :host {
      display: inline-flex;
      align-items: center;
      justify-content: center;
      gap: 0.5rem;
    }

    /* Spinner container */
    .spinner-container {
      display: flex;
      flex-direction: column;
      align-items: center;
      gap: 0.75rem;
    }

    .spinner-container.horizontal {
      flex-direction: row;
    }

    /* Circular spinner */
    .spinner-circle {
      border-radius: 50%;
      border: 2px solid var(--gray-200, #e5e7eb);
      border-top-color: var(--primary-color, #3b82f6);
      animation: spin 0.8s linear infinite;
    }

    .spinner-circle.sm {
      width: 16px;
      height: 16px;
      border-width: 2px;
    }

    .spinner-circle.md {
      width: 24px;
      height: 24px;
      border-width: 2px;
    }

    .spinner-circle.lg {
      width: 40px;
      height: 40px;
      border-width: 3px;
    }

    .spinner-circle.xl {
      width: 56px;
      height: 56px;
      border-width: 4px;
    }

    /* Dots spinner */
    .spinner-dots {
      display: flex;
      gap: 4px;
    }

    .spinner-dots .dot {
      background-color: var(--primary-color, #3b82f6);
      border-radius: 50%;
      animation: bounce 1.4s infinite ease-in-out both;
    }

    .spinner-dots.sm .dot {
      width: 6px;
      height: 6px;
    }

    .spinner-dots.md .dot {
      width: 8px;
      height: 8px;
    }

    .spinner-dots.lg .dot {
      width: 12px;
      height: 12px;
    }

    .spinner-dots.xl .dot {
      width: 16px;
      height: 16px;
    }

    .spinner-dots .dot:nth-child(1) {
      animation-delay: -0.32s;
    }

    .spinner-dots .dot:nth-child(2) {
      animation-delay: -0.16s;
    }

    .spinner-dots .dot:nth-child(3) {
      animation-delay: 0s;
    }

    /* Label */
    .label {
      font-size: 0.875rem;
      color: var(--gray-600, #4b5563);
    }

    .label.sm {
      font-size: 0.75rem;
    }

    .label.lg,
    .label.xl {
      font-size: 1rem;
    }

    /* Animations */
    @keyframes spin {
      to {
        transform: rotate(360deg);
      }
    }

    @keyframes bounce {
      0%,
      80%,
      100% {
        transform: scale(0);
      }
      40% {
        transform: scale(1);
      }
    }

    /* Overlay variant */
    :host([overlay]) {
      position: absolute;
      inset: 0;
      background: rgba(255, 255, 255, 0.8);
      z-index: 10;
    }
  `;
__decorate([
    property({ type: String })
], LoadingSpinner.prototype, "size", void 0);
__decorate([
    property({ type: String })
], LoadingSpinner.prototype, "variant", void 0);
__decorate([
    property({ type: String })
], LoadingSpinner.prototype, "label", void 0);
__decorate([
    property({ type: String })
], LoadingSpinner.prototype, "layout", void 0);
LoadingSpinner = __decorate([
    customElement('hockey-loading-spinner')
], LoadingSpinner);
export { LoadingSpinner };
//# sourceMappingURL=loading-spinner.js.map