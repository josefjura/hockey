import type { Meta, StoryObj } from '@storybook/web-components-vite';
import { html } from 'lit';
import '../loading-spinner.js';

const meta: Meta = {
  title: 'Components/LoadingSpinner',
  component: 'hockey-loading-spinner',

  argTypes: {
    size: {
      control: 'select',
      options: ['sm', 'md', 'lg', 'xl'],
      description: 'Size of the spinner',
      table: {
        defaultValue: { summary: 'md' },
      },
    },
    variant: {
      control: 'select',
      options: ['circle', 'dots'],
      description: 'Visual style of the spinner',
      table: {
        defaultValue: { summary: 'circle' },
      },
    },
    label: {
      control: 'text',
      description: 'Optional label text to display with the spinner',
    },
    layout: {
      control: 'select',
      options: ['vertical', 'horizontal'],
      description: 'Layout direction for spinner and label',
      table: {
        defaultValue: { summary: 'vertical' },
      },
    },
  },
  parameters: {
    docs: {
      description: {
        component: `
A loading spinner component with multiple size and style variants.

## Usage

\`\`\`html
<hockey-loading-spinner></hockey-loading-spinner>
<hockey-loading-spinner size="lg" label="Loading..."></hockey-loading-spinner>
<hockey-loading-spinner variant="dots"></hockey-loading-spinner>
\`\`\`

## Sizes
- **sm**: 16px - for inline/button loading
- **md**: 24px - default size
- **lg**: 40px - for section loading
- **xl**: 56px - for page loading

## Variants
- **circle**: Classic spinning circle
- **dots**: Bouncing dots animation
        `,
      },
    },
  },
};

export default meta;
type Story = StoryObj;

// Basic circle spinner
export const Default: Story = {
  render: () => html`<hockey-loading-spinner></hockey-loading-spinner>`,
};

// Size variants
export const Sizes: Story = {
  render: () => html`
    <div style="display: flex; align-items: center; gap: 2rem;">
      <div style="text-align: center;">
        <hockey-loading-spinner size="sm"></hockey-loading-spinner>
        <p style="margin-top: 0.5rem; font-size: 0.75rem; color: #666;">sm</p>
      </div>
      <div style="text-align: center;">
        <hockey-loading-spinner size="md"></hockey-loading-spinner>
        <p style="margin-top: 0.5rem; font-size: 0.75rem; color: #666;">md</p>
      </div>
      <div style="text-align: center;">
        <hockey-loading-spinner size="lg"></hockey-loading-spinner>
        <p style="margin-top: 0.5rem; font-size: 0.75rem; color: #666;">lg</p>
      </div>
      <div style="text-align: center;">
        <hockey-loading-spinner size="xl"></hockey-loading-spinner>
        <p style="margin-top: 0.5rem; font-size: 0.75rem; color: #666;">xl</p>
      </div>
    </div>
  `,
};

// Dots variant
export const DotsVariant: Story = {
  render: () => html`
    <div style="display: flex; align-items: center; gap: 2rem;">
      <hockey-loading-spinner variant="dots" size="sm"></hockey-loading-spinner>
      <hockey-loading-spinner variant="dots" size="md"></hockey-loading-spinner>
      <hockey-loading-spinner variant="dots" size="lg"></hockey-loading-spinner>
      <hockey-loading-spinner variant="dots" size="xl"></hockey-loading-spinner>
    </div>
  `,
};

// With label
export const WithLabel: Story = {
  render: () => html`
    <div style="display: flex; gap: 3rem;">
      <hockey-loading-spinner size="lg" label="Loading..."></hockey-loading-spinner>
      <hockey-loading-spinner
        size="md"
        label="Please wait"
        layout="horizontal"
      ></hockey-loading-spinner>
    </div>
  `,
};

// Interactive playground
export const Playground: Story = {
  args: {
    size: 'md',
    variant: 'circle',
    label: 'Loading...',
    layout: 'vertical',
  },
  render: args => html`
    <hockey-loading-spinner
      size=${args.size}
      variant=${args.variant}
      label=${args.label}
      layout=${args.layout}
    ></hockey-loading-spinner>
  `,
};
