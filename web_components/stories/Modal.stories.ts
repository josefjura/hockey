import type { Meta, StoryObj } from '@storybook/web-components-vite';
import { html } from 'lit';
import '../modal.js';

const meta: Meta = {
	title: 'Components/Modal',
	component: 'hockey-modal',

	argTypes: {
		title: {
			control: 'text',
			description: 'Modal title',
		},
		size: {
			control: 'select',
			options: ['small', 'default', 'large'],
			description: 'Size variant of the modal',
			table: {
				defaultValue: { summary: 'default' },
			},
		},
		showHints: {
			control: 'boolean',
			description: 'Show keyboard shortcut hints',
			table: {
				defaultValue: { summary: 'true' },
			},
		},
		closeOnEscape: {
			control: 'boolean',
			description: 'Allow closing with Escape key',
			table: {
				defaultValue: { summary: 'true' },
			},
		},
		closeOnOutsideClick: {
			control: 'boolean',
			description: 'Allow closing by clicking outside',
			table: {
				defaultValue: { summary: 'true' },
			},
		},
		autoFocus: {
			control: 'boolean',
			description: 'Auto-focus first input when modal opens',
			table: {
				defaultValue: { summary: 'true' },
			},
		},
	},
	parameters: {
		docs: {
			description: {
				component: `
A modal component with built-in keyboard shortcuts and focus management.

## Features
- **Escape** to close
- **Ctrl/Cmd + Enter** to submit forms
- Focus trap within modal (Tab key cycles through focusable elements)
- Auto-focus first input on open
- Restore focus to previous element on close
- Click outside to close

## Usage

\`\`\`html
<hockey-modal modal-id="my-modal" title="Edit Item">
  <form slot="content">
    <input type="text" name="name">
    <button type="submit">Save</button>
  </form>
</hockey-modal>
\`\`\`

## Sizes
- **small**: 400px max-width
- **default**: 500px max-width
- **large**: 700px max-width
        `,
			},
		},
	},
};

export default meta;
type Story = StoryObj;

// Helper to show a modal
const showModal = (options: {
	title: string;
	size?: string;
	showHints?: boolean;
	content: string;
}) => {
	// Remove any existing modals
	document.querySelectorAll('hockey-modal').forEach(m => m.remove());

	const modal = document.createElement('hockey-modal');
	modal.setAttribute('modal-id', 'demo-modal');
	modal.setAttribute('title', options.title);
	if (options.size) modal.setAttribute('size', options.size);
	if (options.showHints === false) modal.removeAttribute('show-hints');

	const content = document.createElement('div');
	content.setAttribute('slot', 'content');
	content.innerHTML = options.content;
	modal.appendChild(content);

	document.body.appendChild(modal);
};

// Default modal with form
export const Default: Story = {
	render: () => html`
    <button
      style="padding: 0.5rem 1rem; background: #3b82f6; color: white; border: none; border-radius: 6px; cursor: pointer;"
      @click=${() =>
			showModal({
				title: 'Edit Profile',
				content: `
          <form style="display: flex; flex-direction: column; gap: 1rem;">
            <div>
              <label style="display: block; margin-bottom: 0.25rem; font-weight: 500;">Name</label>
              <input type="text" value="John Doe" style="width: 100%; padding: 0.5rem; border: 1px solid #d1d5db; border-radius: 6px;">
            </div>
            <div>
              <label style="display: block; margin-bottom: 0.25rem; font-weight: 500;">Email</label>
              <input type="email" value="john@example.com" style="width: 100%; padding: 0.5rem; border: 1px solid #d1d5db; border-radius: 6px;">
            </div>
            <div style="display: flex; gap: 0.5rem; justify-content: flex-end; margin-top: 0.5rem;">
              <button type="button" style="padding: 0.5rem 1rem; background: white; border: 1px solid #d1d5db; border-radius: 6px; cursor: pointer;">Cancel</button>
              <button type="submit" style="padding: 0.5rem 1rem; background: #3b82f6; color: white; border: none; border-radius: 6px; cursor: pointer;">Save Changes</button>
            </div>
          </form>
        `,
			})}
    >
      Open Modal
    </button>
  `,
};

// Size variants
export const Sizes: Story = {
	render: () => html`
    <div style="display: flex; gap: 0.5rem;">
      <button
        style="padding: 0.5rem 1rem; background: #6b7280; color: white; border: none; border-radius: 6px; cursor: pointer;"
        @click=${() =>
			showModal({
				title: 'Small Modal',
				size: 'small',
				content: '<p>This is a small modal (400px max-width).</p>',
			})}
      >
        Small
      </button>
      <button
        style="padding: 0.5rem 1rem; background: #6b7280; color: white; border: none; border-radius: 6px; cursor: pointer;"
        @click=${() =>
			showModal({
				title: 'Default Modal',
				size: 'default',
				content: '<p>This is the default modal size (500px max-width).</p>',
			})}
      >
        Default
      </button>
      <button
        style="padding: 0.5rem 1rem; background: #6b7280; color: white; border: none; border-radius: 6px; cursor: pointer;"
        @click=${() =>
			showModal({
				title: 'Large Modal',
				size: 'large',
				content: '<p>This is a large modal (700px max-width). Good for complex forms or content that needs more space.</p>',
			})}
      >
        Large
      </button>
    </div>
  `,
};

// Without keyboard hints
export const NoHints: Story = {
	render: () => html`
    <button
      style="padding: 0.5rem 1rem; background: #3b82f6; color: white; border: none; border-radius: 6px; cursor: pointer;"
      @click=${() =>
			showModal({
				title: 'Clean Modal',
				showHints: false,
				content: '<p>This modal does not show keyboard hints at the bottom.</p>',
			})}
    >
      Open Modal (No Hints)
    </button>
  `,
};

// With complex form
export const ComplexForm: Story = {
	render: () => html`
    <button
      style="padding: 0.5rem 1rem; background: #3b82f6; color: white; border: none; border-radius: 6px; cursor: pointer;"
      @click=${() =>
			showModal({
				title: 'Create New Event',
				size: 'large',
				content: `
          <form style="display: flex; flex-direction: column; gap: 1rem;">
            <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 1rem;">
              <div>
                <label style="display: block; margin-bottom: 0.25rem; font-weight: 500;">Event Name *</label>
                <input type="text" placeholder="Enter event name" style="width: 100%; padding: 0.5rem; border: 1px solid #d1d5db; border-radius: 6px;">
              </div>
              <div>
                <label style="display: block; margin-bottom: 0.25rem; font-weight: 500;">Country</label>
                <select style="width: 100%; padding: 0.5rem; border: 1px solid #d1d5db; border-radius: 6px;">
                  <option>Select country...</option>
                  <option>Czech Republic</option>
                  <option>Slovakia</option>
                  <option>Canada</option>
                </select>
              </div>
            </div>
            <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 1rem;">
              <div>
                <label style="display: block; margin-bottom: 0.25rem; font-weight: 500;">Start Date</label>
                <input type="date" style="width: 100%; padding: 0.5rem; border: 1px solid #d1d5db; border-radius: 6px;">
              </div>
              <div>
                <label style="display: block; margin-bottom: 0.25rem; font-weight: 500;">End Date</label>
                <input type="date" style="width: 100%; padding: 0.5rem; border: 1px solid #d1d5db; border-radius: 6px;">
              </div>
            </div>
            <div>
              <label style="display: block; margin-bottom: 0.25rem; font-weight: 500;">Description</label>
              <textarea rows="3" placeholder="Enter description..." style="width: 100%; padding: 0.5rem; border: 1px solid #d1d5db; border-radius: 6px; resize: vertical;"></textarea>
            </div>
            <div style="display: flex; gap: 0.5rem; justify-content: flex-end; margin-top: 0.5rem; padding-top: 1rem; border-top: 1px solid #e5e7eb;">
              <button type="button" style="padding: 0.5rem 1rem; background: white; border: 1px solid #d1d5db; border-radius: 6px; cursor: pointer;">Cancel</button>
              <button type="submit" style="padding: 0.5rem 1rem; background: #3b82f6; color: white; border: none; border-radius: 6px; cursor: pointer;">Create Event</button>
            </div>
          </form>
        `,
			})}
    >
      Open Complex Form Modal
    </button>
  `,
};

// Focus management demo
export const FocusManagement: Story = {
	render: () => html`
    <div>
      <p style="margin-bottom: 1rem; color: #666;">
        Try using Tab key to cycle through focusable elements. Focus will be
        trapped within the modal.
      </p>
      <button
        style="padding: 0.5rem 1rem; background: #3b82f6; color: white; border: none; border-radius: 6px; cursor: pointer;"
        @click=${() =>
			showModal({
				title: 'Focus Trap Demo',
				content: `
          <div style="display: flex; flex-direction: column; gap: 1rem;">
            <p>Press Tab to cycle through these elements:</p>
            <input type="text" placeholder="First input" style="padding: 0.5rem; border: 1px solid #d1d5db; border-radius: 6px;">
            <input type="text" placeholder="Second input" style="padding: 0.5rem; border: 1px solid #d1d5db; border-radius: 6px;">
            <div style="display: flex; gap: 0.5rem;">
              <button type="button" style="padding: 0.5rem 1rem; background: white; border: 1px solid #d1d5db; border-radius: 6px; cursor: pointer;">Button 1</button>
              <button type="button" style="padding: 0.5rem 1rem; background: white; border: 1px solid #d1d5db; border-radius: 6px; cursor: pointer;">Button 2</button>
              <button type="button" style="padding: 0.5rem 1rem; background: #3b82f6; color: white; border: none; border-radius: 6px; cursor: pointer;">Button 3</button>
            </div>
          </div>
        `,
			})}
    >
      Open Focus Demo
    </button>
    </div>
  `,
};
