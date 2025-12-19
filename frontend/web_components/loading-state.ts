import { LitElement, html, css } from 'lit';
import { customElement, property } from 'lit/decorators.js';
import './loading-spinner.js';

/**
 * Loading state component for page sections or full page loading
 *
 * @example
 * ```html
 * <hockey-loading-state label="Loading events..."></hockey-loading-state>
 * <hockey-loading-state variant="inline" size="sm"></hockey-loading-state>
 * ```
 */
@customElement('hockey-loading-state')
export class LoadingState extends LitElement {
	static styles = css`
    :host {
      display: block;
    }

    /* Full container loading state */
    .loading-container {
      display: flex;
      flex-direction: column;
      align-items: center;
      justify-content: center;
      padding: 3rem 2rem;
      gap: 1rem;
    }

    .loading-container.minimal {
      padding: 1.5rem 1rem;
    }

    /* Inline loading state */
    .loading-inline {
      display: inline-flex;
      align-items: center;
      gap: 0.5rem;
    }

    /* Skeleton loading */
    .skeleton {
      background: linear-gradient(
        90deg,
        var(--gray-200, #e5e7eb) 25%,
        var(--gray-100, #f3f4f6) 50%,
        var(--gray-200, #e5e7eb) 75%
      );
      background-size: 200% 100%;
      animation: skeleton-pulse 1.5s ease-in-out infinite;
      border-radius: 4px;
    }

    .skeleton-text {
      height: 1rem;
      width: 100%;
    }

    .skeleton-text.short {
      width: 60%;
    }

    .skeleton-text.medium {
      width: 80%;
    }

    .skeleton-heading {
      height: 1.5rem;
      width: 50%;
      margin-bottom: 0.5rem;
    }

    .skeleton-row {
      display: flex;
      gap: 1rem;
      margin-bottom: 0.75rem;
    }

    .skeleton-cell {
      height: 2.5rem;
      flex: 1;
    }

    .skeleton-table {
      width: 100%;
    }

    @keyframes skeleton-pulse {
      0% {
        background-position: 200% 0;
      }
      100% {
        background-position: -200% 0;
      }
    }
  `;

	/** Display variant: container (centered), inline, or skeleton */
	@property({ type: String })
	variant: 'container' | 'inline' | 'skeleton' = 'container';

	/** Size passed to spinner */
	@property({ type: String })
	size: 'sm' | 'md' | 'lg' | 'xl' = 'lg';

	/** Optional loading label */
	@property({ type: String })
	label?: string;

	/** Number of skeleton rows to show */
	@property({ type: Number })
	skeletonRows = 5;

	/** Use minimal padding for container variant */
	@property({ type: Boolean })
	minimal = false;

	render() {
		switch (this.variant) {
			case 'inline':
				return this.renderInline();
			case 'skeleton':
				return this.renderSkeleton();
			default:
				return this.renderContainer();
		}
	}

	private renderContainer() {
		return html`
      <div class="loading-container ${this.minimal ? 'minimal' : ''}">
        <hockey-loading-spinner
          size=${this.size}
          label=${this.label || ''}
        ></hockey-loading-spinner>
      </div>
    `;
	}

	private renderInline() {
		return html`
      <span class="loading-inline">
        <hockey-loading-spinner
          size=${this.size}
          layout="horizontal"
          label=${this.label || ''}
        ></hockey-loading-spinner>
      </span>
    `;
	}

	private renderSkeleton() {
		return html`
      <div class="skeleton-table">
        <div class="skeleton skeleton-heading"></div>
        ${Array(this.skeletonRows)
				.fill(null)
				.map(
					() => html`
              <div class="skeleton-row">
                <div class="skeleton skeleton-cell"></div>
                <div class="skeleton skeleton-cell"></div>
                <div class="skeleton skeleton-cell"></div>
                <div class="skeleton skeleton-cell" style="flex: 0.5;"></div>
              </div>
            `
				)}
      </div>
    `;
	}
}

declare global {
	interface HTMLElementTagNameMap {
		'hockey-loading-state': LoadingState;
	}
}
