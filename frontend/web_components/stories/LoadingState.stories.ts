import type { Meta, StoryObj } from '@storybook/web-components-vite';
import { html } from 'lit';
import '../loading-state.js';

const meta: Meta = {
	title: 'Components/LoadingState',
	component: 'hockey-loading-state',

	argTypes: {
		variant: {
			control: 'select',
			options: ['container', 'inline', 'skeleton'],
			description: 'Display variant of the loading state',
			table: {
				defaultValue: { summary: 'container' },
			},
		},
		size: {
			control: 'select',
			options: ['sm', 'md', 'lg', 'xl'],
			description: 'Size of the spinner (for container/inline variants)',
			table: {
				defaultValue: { summary: 'lg' },
			},
		},
		label: {
			control: 'text',
			description: 'Optional loading label',
		},
		skeletonRows: {
			control: 'number',
			description: 'Number of skeleton rows (for skeleton variant)',
			table: {
				defaultValue: { summary: '5' },
			},
		},
		minimal: {
			control: 'boolean',
			description: 'Use minimal padding for container variant',
			table: {
				defaultValue: { summary: 'false' },
			},
		},
	},
	parameters: {
		docs: {
			description: {
				component: `
A loading state component for page sections or full page loading.

## Usage

\`\`\`html
<hockey-loading-state label="Loading events..."></hockey-loading-state>
<hockey-loading-state variant="inline" size="sm"></hockey-loading-state>
<hockey-loading-state variant="skeleton" skeletonRows="5"></hockey-loading-state>
\`\`\`

## Variants
- **container**: Centered spinner with optional label (for content areas)
- **inline**: Inline spinner for buttons or inline loading
- **skeleton**: Animated skeleton placeholder for tables
        `,
			},
		},
	},
};

export default meta;
type Story = StoryObj;

// Default container loading
export const Default: Story = {
	render: () => html`
    <div
      style="border: 1px solid #e5e7eb; border-radius: 8px; min-height: 300px;"
    >
      <hockey-loading-state label="Loading data..."></hockey-loading-state>
    </div>
  `,
};

// Container variant
export const Container: Story = {
	render: () => html`
    <div
      style="border: 1px solid #e5e7eb; border-radius: 8px; min-height: 300px;"
    >
      <hockey-loading-state
        variant="container"
        size="xl"
        label="Loading events..."
      ></hockey-loading-state>
    </div>
  `,
};

// Inline variant
export const Inline: Story = {
	render: () => html`
    <div style="display: flex; align-items: center; gap: 1rem;">
      <span>Status:</span>
      <hockey-loading-state
        variant="inline"
        size="sm"
        label="Checking..."
      ></hockey-loading-state>
    </div>
  `,
};

// Skeleton variant
export const Skeleton: Story = {
	render: () => html`
    <div style="border: 1px solid #e5e7eb; border-radius: 8px; padding: 1rem;">
      <hockey-loading-state
        variant="skeleton"
        skeletonRows="5"
      ></hockey-loading-state>
    </div>
  `,
};

// Skeleton with fewer rows
export const SkeletonCompact: Story = {
	render: () => html`
    <div style="border: 1px solid #e5e7eb; border-radius: 8px; padding: 1rem;">
      <hockey-loading-state
        variant="skeleton"
        skeletonRows="3"
      ></hockey-loading-state>
    </div>
  `,
};

// Minimal container
export const MinimalContainer: Story = {
	render: () => html`
    <div style="border: 1px solid #e5e7eb; border-radius: 8px;">
      <hockey-loading-state
        variant="container"
        minimal
        label="Loading..."
      ></hockey-loading-state>
    </div>
  `,
};

// Interactive playground
export const Playground: Story = {
	args: {
		variant: 'container',
		size: 'lg',
		label: 'Loading...',
		skeletonRows: 5,
		minimal: false,
	},
	render: args => html`
    <div
      style="border: 1px solid #e5e7eb; border-radius: 8px; min-height: 200px; padding: 1rem;"
    >
      <hockey-loading-state
        variant=${args.variant}
        size=${args.size}
        label=${args.label}
        skeletonRows=${args.skeletonRows}
        ?minimal=${args.minimal}
      ></hockey-loading-state>
    </div>
  `,
};
