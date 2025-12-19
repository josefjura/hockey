import type { Meta, StoryObj } from '@storybook/web-components-vite';
import { html } from 'lit';
import '../toast.js';
import type { ToastContainer } from '../toast.js';

const meta: Meta = {
  title: 'Components/Toast',
  component: 'hockey-toast-container',

  parameters: {
    docs: {
      description: {
        component: `
A toast notification system for displaying feedback messages.

## Usage

\`\`\`html
<hockey-toast-container position="top-right"></hockey-toast-container>
\`\`\`

\`\`\`javascript
const container = document.querySelector('hockey-toast-container');

// Show different types of toasts
container.success('Item saved successfully!');
container.error('Failed to save item');
container.warning('This action cannot be undone');
container.info('New updates available');

// Or use the show method with options
container.show({
  message: 'Custom toast',
  variant: 'success',
  duration: 5000,
  dismissible: true
});
\`\`\`

## Positions
- top-right (default)
- top-left
- top-center
- bottom-right
- bottom-left
- bottom-center
        `,
      },
    },
  },
};

export default meta;
type Story = StoryObj;

// Helper to get toast container
const getContainer = (): ToastContainer | null => {
  return document.querySelector('hockey-toast-container');
};

// Default with demo buttons
export const Default: Story = {
  render: () => html`
    <hockey-toast-container position="top-right"></hockey-toast-container>

    <div style="display: flex; flex-wrap: wrap; gap: 0.5rem;">
      <button
        style="padding: 0.5rem 1rem; background: #10b981; color: white; border: none; border-radius: 6px; cursor: pointer;"
        @click=${() => getContainer()?.success('Operation completed successfully!')}
      >
        Show Success
      </button>
      <button
        style="padding: 0.5rem 1rem; background: #ef4444; color: white; border: none; border-radius: 6px; cursor: pointer;"
        @click=${() => getContainer()?.error('Something went wrong. Please try again.')}
      >
        Show Error
      </button>
      <button
        style="padding: 0.5rem 1rem; background: #f59e0b; color: white; border: none; border-radius: 6px; cursor: pointer;"
        @click=${() => getContainer()?.warning('This action cannot be undone.')}
      >
        Show Warning
      </button>
      <button
        style="padding: 0.5rem 1rem; background: #3b82f6; color: white; border: none; border-radius: 6px; cursor: pointer;"
        @click=${() => getContainer()?.info('New updates are available.')}
      >
        Show Info
      </button>
    </div>
  `,
};

// All variants
export const Variants: Story = {
  render: () => html`
    <hockey-toast-container position="top-right"></hockey-toast-container>

    <div style="display: flex; flex-direction: column; gap: 1rem;">
      <p style="color: #666;">Click buttons to see different toast variants:</p>
      <div style="display: flex; flex-wrap: wrap; gap: 0.5rem;">
        <button
          style="padding: 0.5rem 1rem; background: #10b981; color: white; border: none; border-radius: 6px; cursor: pointer;"
          @click=${() => getContainer()?.success('Success! Your changes have been saved.')}
        >
          Success Toast
        </button>
        <button
          style="padding: 0.5rem 1rem; background: #ef4444; color: white; border: none; border-radius: 6px; cursor: pointer;"
          @click=${() =>
            getContainer()?.error('Error! Failed to connect to the server.')}
        >
          Error Toast
        </button>
        <button
          style="padding: 0.5rem 1rem; background: #f59e0b; color: white; border: none; border-radius: 6px; cursor: pointer;"
          @click=${() =>
            getContainer()?.warning('Warning! You have unsaved changes.')}
        >
          Warning Toast
        </button>
        <button
          style="padding: 0.5rem 1rem; background: #3b82f6; color: white; border: none; border-radius: 6px; cursor: pointer;"
          @click=${() =>
            getContainer()?.info('Info: Your session will expire in 5 minutes.')}
        >
          Info Toast
        </button>
      </div>
    </div>
  `,
};

// Multiple toasts
export const MultipleToasts: Story = {
  render: () => html`
    <hockey-toast-container position="top-right"></hockey-toast-container>

    <button
      style="padding: 0.75rem 1.5rem; background: #6366f1; color: white; border: none; border-radius: 6px; cursor: pointer;"
      @click=${() => {
        const container = getContainer();
        container?.success('First notification');
        setTimeout(() => container?.info('Second notification'), 300);
        setTimeout(() => container?.warning('Third notification'), 600);
      }}
    >
      Show Multiple Toasts
    </button>
  `,
};

// Custom duration
export const CustomDuration: Story = {
  render: () => html`
    <hockey-toast-container
      position="top-right"
      defaultDuration="2000"
    ></hockey-toast-container>

    <div style="display: flex; gap: 0.5rem;">
      <button
        style="padding: 0.5rem 1rem; background: #3b82f6; color: white; border: none; border-radius: 6px; cursor: pointer;"
        @click=${() =>
          getContainer()?.show({ message: 'Quick toast (2s)', variant: 'info', duration: 2000 })}
      >
        2 Second Toast
      </button>
      <button
        style="padding: 0.5rem 1rem; background: #3b82f6; color: white; border: none; border-radius: 6px; cursor: pointer;"
        @click=${() =>
          getContainer()?.show({
            message: 'Long toast (10s)',
            variant: 'info',
            duration: 10000,
          })}
      >
        10 Second Toast
      </button>
      <button
        style="padding: 0.5rem 1rem; background: #3b82f6; color: white; border: none; border-radius: 6px; cursor: pointer;"
        @click=${() =>
          getContainer()?.show({
            message: 'Persistent toast (click to dismiss)',
            variant: 'warning',
            duration: 0,
          })}
      >
        Persistent Toast
      </button>
    </div>
  `,
};

// Different positions
export const Positions: Story = {
  render: () => html`
    <div style="display: flex; flex-wrap: wrap; gap: 0.5rem;">
      <button
        style="padding: 0.5rem 1rem; background: #6b7280; color: white; border: none; border-radius: 6px; cursor: pointer;"
        @click=${() => {
          const existing = document.querySelector('hockey-toast-container');
          existing?.remove();
          const container = document.createElement('hockey-toast-container');
          container.setAttribute('position', 'top-right');
          document.body.appendChild(container);
          (container as ToastContainer).info('Top Right Position');
        }}
      >
        Top Right
      </button>
      <button
        style="padding: 0.5rem 1rem; background: #6b7280; color: white; border: none; border-radius: 6px; cursor: pointer;"
        @click=${() => {
          const existing = document.querySelector('hockey-toast-container');
          existing?.remove();
          const container = document.createElement('hockey-toast-container');
          container.setAttribute('position', 'top-left');
          document.body.appendChild(container);
          (container as ToastContainer).info('Top Left Position');
        }}
      >
        Top Left
      </button>
      <button
        style="padding: 0.5rem 1rem; background: #6b7280; color: white; border: none; border-radius: 6px; cursor: pointer;"
        @click=${() => {
          const existing = document.querySelector('hockey-toast-container');
          existing?.remove();
          const container = document.createElement('hockey-toast-container');
          container.setAttribute('position', 'bottom-right');
          document.body.appendChild(container);
          (container as ToastContainer).info('Bottom Right Position');
        }}
      >
        Bottom Right
      </button>
      <button
        style="padding: 0.5rem 1rem; background: #6b7280; color: white; border: none; border-radius: 6px; cursor: pointer;"
        @click=${() => {
          const existing = document.querySelector('hockey-toast-container');
          existing?.remove();
          const container = document.createElement('hockey-toast-container');
          container.setAttribute('position', 'top-center');
          document.body.appendChild(container);
          (container as ToastContainer).info('Top Center Position');
        }}
      >
        Top Center
      </button>
    </div>
  `,
};
