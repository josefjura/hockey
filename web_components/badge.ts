import { LitElement, html, css } from 'lit';
import { customElement, property } from 'lit/decorators.js';
import { BadgeVariant } from './shared/types.js';

/**
 * Badge component for displaying status labels
 *
 * @example
 * ```html
 * <hockey-badge variant="primary" text="IIHF"></hockey-badge>
 * <hockey-badge variant="warning" text="Historical" outlined></hockey-badge>
 * ```
 */
@customElement('hockey-badge')
export class Badge extends LitElement {
  static styles = css`
    :host {
      display: inline-block;
    }

    .badge {
      display: inline-flex;
      align-items: center;
      padding: 0.25rem 0.75rem;
      font-size: 0.75rem;
      font-weight: 600;
      line-height: 1;
      border-radius: 9999px;
      text-transform: uppercase;
      letter-spacing: 0.025em;
      white-space: nowrap;
    }

    /* Filled variants */
    .badge-primary {
      background-color: #3b82f6;
      color: white;
    }

    .badge-success {
      background-color: #10b981;
      color: white;
    }

    .badge-warning {
      background-color: #f59e0b;
      color: white;
    }

    .badge-danger {
      background-color: #ef4444;
      color: white;
    }

    .badge-info {
      background-color: #06b6d4;
      color: white;
    }

    .badge-default {
      background-color: var(--gray-200, #e5e7eb);
      color: var(--gray-700, #374151);
    }

    /* Outlined variants */
    .badge-primary.outlined {
      background-color: transparent;
      color: #3b82f6;
      border: 1px solid #3b82f6;
    }

    .badge-success.outlined {
      background-color: transparent;
      color: #10b981;
      border: 1px solid #10b981;
    }

    .badge-warning.outlined {
      background-color: transparent;
      color: #f59e0b;
      border: 1px solid #f59e0b;
    }

    .badge-danger.outlined {
      background-color: transparent;
      color: #ef4444;
      border: 1px solid #ef4444;
    }

    .badge-info.outlined {
      background-color: transparent;
      color: #06b6d4;
      border: 1px solid #06b6d4;
    }

    .badge-default.outlined {
      background-color: transparent;
      color: var(--gray-600, #4b5563);
      border: 1px solid var(--gray-300, #d1d5db);
    }
  `;

  @property({ type: String })
  variant: BadgeVariant = 'default';

  @property({ type: String })
  text: string = '';

  @property({ type: Boolean })
  outlined: boolean = false;

  render() {
    const classes = `badge badge-${this.variant} ${this.outlined ? 'outlined' : ''}`;

    return html`
      <span class=${classes}>
        ${this.text}
      </span>
    `;
  }
}

declare global {
  interface HTMLElementTagNameMap {
    'hockey-badge': Badge;
  }
}
