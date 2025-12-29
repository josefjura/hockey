import type { Meta, StoryObj } from '@storybook/web-components-vite';
import { html } from 'lit';
import { expect, userEvent, within, waitFor } from '@storybook/test';
import '../confirm-dialog.js';
import type { ConfirmDialog } from '../confirm-dialog.js';

const meta: Meta = {
	title: 'Components/ConfirmDialog',
	component: 'hockey-confirm-dialog',

	parameters: {
		docs: {
			description: {
				component: `
A confirmation dialog component for replacing browser \`confirm()\`.

## Usage

\`\`\`html
<hockey-confirm-dialog></hockey-confirm-dialog>
\`\`\`

\`\`\`javascript
const dialog = document.querySelector('hockey-confirm-dialog');

// Promise-based usage
const confirmed = await dialog.show({
  title: 'Delete Item',
  message: 'Are you sure you want to delete this item?',
  variant: 'danger'
});

if (confirmed) {
  // User clicked confirm
}

// With callbacks
dialog.show({
  title: 'Confirm Action',
  message: 'This action cannot be undone.',
  confirmText: 'Yes, proceed',
  cancelText: 'No, cancel',
  variant: 'warning',
  onConfirm: () => console.log('Confirmed!'),
  onCancel: () => console.log('Cancelled')
});
\`\`\`

## HTMX Integration

Use \`hx-confirm-custom\` attribute instead of \`hx-confirm\`:

\`\`\`html
<button
  hx-post="/delete/123"
  hx-confirm-custom='{"title": "Delete Item", "message": "Are you sure?", "variant": "danger"}'
>Delete</button>
\`\`\`

## Variants
- **danger**: Red - for destructive actions like delete
- **warning**: Amber - for actions that need caution
- **info**: Blue - for informational confirmations
        `,
			},
		},
	},
};

export default meta;
type Story = StoryObj;

// Helper to get dialog
const getDialog = (): ConfirmDialog | null => {
	return document.querySelector('hockey-confirm-dialog');
};

// Danger confirmation (delete)
export const Danger: Story = {
	render: () => html`
    <hockey-confirm-dialog></hockey-confirm-dialog>

    <button
      style="padding: 0.5rem 1rem; background: #ef4444; color: white; border: none; border-radius: 6px; cursor: pointer;"
      @click=${async () => {
			const confirmed = await getDialog()?.show({
				title: 'Delete Item',
				message:
					'Are you sure you want to delete this item? This action cannot be undone.',
				variant: 'danger',
				confirmText: 'Delete',
				cancelText: 'Cancel',
			});
			if (confirmed) {
				alert('Item deleted!');
			}
		}}
    >
      Delete Item
    </button>
  `,
	play: async ({ canvasElement }) => {
		const canvas = within(canvasElement);
		const dialog = canvasElement.querySelector('hockey-confirm-dialog') as ConfirmDialog;

		// Verify dialog exists
		await expect(dialog).toBeInTheDocument();

		// Initially, dialog should not be visible
		const shadowRoot = dialog.shadowRoot!;
		let dialogElement = shadowRoot.querySelector('dialog') as HTMLDialogElement;
		await expect(dialogElement.open).toBe(false);

		// Click the "Delete Item" button to open dialog
		const triggerButton = canvas.getByText('Delete Item');
		await userEvent.click(triggerButton);

		// Wait for dialog to open
		await waitFor(() => {
			dialogElement = shadowRoot.querySelector('dialog') as HTMLDialogElement;
			return expect(dialogElement.open).toBe(true);
		});

		// Verify dialog content
		const title = within(shadowRoot).getByText('Delete Item');
		await expect(title).toBeInTheDocument();

		const message = within(shadowRoot).getByText(/This action cannot be undone/i);
		await expect(message).toBeInTheDocument();

		// Verify buttons exist
		const confirmButton = within(shadowRoot).getByText('Delete');
		const cancelButton = within(shadowRoot).getByText('Cancel');
		await expect(confirmButton).toBeInTheDocument();
		await expect(cancelButton).toBeInTheDocument();

		// Click confirm button
		await userEvent.click(confirmButton);

		// Wait for dialog to close
		await waitFor(() => {
			dialogElement = shadowRoot.querySelector('dialog') as HTMLDialogElement;
			return expect(dialogElement.open).toBe(false);
		});
	},
};

// Warning confirmation
export const Warning: Story = {
	render: () => html`
    <hockey-confirm-dialog></hockey-confirm-dialog>

    <button
      style="padding: 0.5rem 1rem; background: #f59e0b; color: white; border: none; border-radius: 6px; cursor: pointer;"
      @click=${async () => {
			const confirmed = await getDialog()?.show({
				title: 'Discard Changes',
				message:
					'You have unsaved changes. Are you sure you want to leave this page?',
				variant: 'warning',
				confirmText: 'Discard',
				cancelText: 'Stay',
			});
			if (confirmed) {
				alert('Changes discarded!');
			}
		}}
    >
      Discard Changes
    </button>
  `,
	play: async ({ canvasElement }) => {
		const canvas = within(canvasElement);
		const dialog = canvasElement.querySelector('hockey-confirm-dialog') as ConfirmDialog;

		// Click the "Discard Changes" button to open dialog
		const triggerButton = canvas.getByText('Discard Changes');
		await userEvent.click(triggerButton);

		// Wait for dialog to open
		const shadowRoot = dialog.shadowRoot!;
		await waitFor(() => {
			const dialogElement = shadowRoot.querySelector('dialog') as HTMLDialogElement;
			return expect(dialogElement.open).toBe(true);
		});

		// Verify warning variant content
		const title = within(shadowRoot).getByText('Discard Changes');
		await expect(title).toBeInTheDocument();

		const message = within(shadowRoot).getByText(/unsaved changes/i);
		await expect(message).toBeInTheDocument();

		// Verify custom button text
		const discardButton = within(shadowRoot).getByText('Discard');
		const stayButton = within(shadowRoot).getByText('Stay');
		await expect(discardButton).toBeInTheDocument();
		await expect(stayButton).toBeInTheDocument();

		// Click cancel button (Stay)
		await userEvent.click(stayButton);

		// Wait for dialog to close
		await waitFor(() => {
			const dialogElement = shadowRoot.querySelector('dialog') as HTMLDialogElement;
			return expect(dialogElement.open).toBe(false);
		});
	},
};

// Info confirmation
export const Info: Story = {
	render: () => html`
    <hockey-confirm-dialog></hockey-confirm-dialog>

    <button
      style="padding: 0.5rem 1rem; background: #3b82f6; color: white; border: none; border-radius: 6px; cursor: pointer;"
      @click=${async () => {
			const confirmed = await getDialog()?.show({
				title: 'Enable Notifications',
				message:
					'Would you like to receive email notifications for important updates?',
				variant: 'info',
				confirmText: 'Enable',
				cancelText: 'Not now',
			});
			if (confirmed) {
				alert('Notifications enabled!');
			}
		}}
    >
      Enable Notifications
    </button>
  `,
	play: async ({ canvasElement }) => {
		const canvas = within(canvasElement);
		const dialog = canvasElement.querySelector('hockey-confirm-dialog') as ConfirmDialog;

		// Click the "Enable Notifications" button to open dialog
		const triggerButton = canvas.getByText('Enable Notifications');
		await userEvent.click(triggerButton);

		// Wait for dialog to open
		const shadowRoot = dialog.shadowRoot!;
		await waitFor(() => {
			const dialogElement = shadowRoot.querySelector('dialog') as HTMLDialogElement;
			return expect(dialogElement.open).toBe(true);
		});

		// Verify info variant content
		const title = within(shadowRoot).getByText('Enable Notifications');
		await expect(title).toBeInTheDocument();

		const message = within(shadowRoot).getByText(/email notifications/i);
		await expect(message).toBeInTheDocument();

		// Verify custom button text for info variant
		const enableButton = within(shadowRoot).getByText('Enable');
		const notNowButton = within(shadowRoot).getByText('Not now');
		await expect(enableButton).toBeInTheDocument();
		await expect(notNowButton).toBeInTheDocument();

		// Click confirm button
		await userEvent.click(enableButton);

		// Wait for dialog to close
		await waitFor(() => {
			const dialogElement = shadowRoot.querySelector('dialog') as HTMLDialogElement;
			return expect(dialogElement.open).toBe(false);
		});
	},
};

// All variants
export const AllVariants: Story = {
	render: () => html`
    <hockey-confirm-dialog></hockey-confirm-dialog>

    <div style="display: flex; flex-wrap: wrap; gap: 0.5rem;">
      <button
        style="padding: 0.5rem 1rem; background: #ef4444; color: white; border: none; border-radius: 6px; cursor: pointer;"
        @click=${() =>
			getDialog()?.show({
				title: 'Delete Confirmation',
				message: 'This is a danger/delete confirmation dialog.',
				variant: 'danger',
			})}
      >
        Danger
      </button>
      <button
        style="padding: 0.5rem 1rem; background: #f59e0b; color: white; border: none; border-radius: 6px; cursor: pointer;"
        @click=${() =>
			getDialog()?.show({
				title: 'Warning Confirmation',
				message: 'This is a warning confirmation dialog.',
				variant: 'warning',
			})}
      >
        Warning
      </button>
      <button
        style="padding: 0.5rem 1rem; background: #3b82f6; color: white; border: none; border-radius: 6px; cursor: pointer;"
        @click=${() =>
			getDialog()?.show({
				title: 'Info Confirmation',
				message: 'This is an info confirmation dialog.',
				variant: 'info',
			})}
      >
        Info
      </button>
    </div>
  `,
};

// With callbacks
export const WithCallbacks: Story = {
	render: () => html`
    <hockey-confirm-dialog></hockey-confirm-dialog>

    <div>
      <p style="margin-bottom: 1rem; color: #666;">
        Check the console for callback messages
      </p>
      <button
        style="padding: 0.5rem 1rem; background: #6366f1; color: white; border: none; border-radius: 6px; cursor: pointer;"
        @click=${() =>
			getDialog()?.show({
				title: 'Confirm Action',
				message: 'This will log to console based on your choice.',
				variant: 'info',
				onConfirm: () => console.log('✅ Confirmed!'),
				onCancel: () => console.log('❌ Cancelled!'),
			})}
      >
        Show Dialog with Callbacks
      </button>
    </div>
  `,
};

// Custom button text
export const CustomButtons: Story = {
	render: () => html`
    <hockey-confirm-dialog></hockey-confirm-dialog>

    <button
      style="padding: 0.5rem 1rem; background: #10b981; color: white; border: none; border-radius: 6px; cursor: pointer;"
      @click=${async () => {
			const confirmed = await getDialog()?.show({
				title: 'Publish Article',
				message:
					'Your article will be visible to all users. Ready to publish?',
				variant: 'info',
				confirmText: 'Yes, Publish Now',
				cancelText: 'Save as Draft',
			});
			alert(confirmed ? 'Article published!' : 'Saved as draft');
		}}
    >
      Publish Article
    </button>
  `,
};
