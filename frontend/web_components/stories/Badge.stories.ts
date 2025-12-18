import type { Meta, StoryObj } from '@storybook/web-components-vite';
import { html } from 'lit';
import '../badge.js';

const meta: Meta = {
  title: 'Components/Badge',
  component: 'hockey-badge',
  
  argTypes: {
    variant: {
      control: 'select',
      options: ['primary', 'success', 'warning', 'danger', 'info', 'default'],
      description: 'The visual style variant of the badge',
      table: {
        defaultValue: { summary: 'default' },
      },
    },
    text: {
      control: 'text',
      description: 'The text content displayed in the badge',
    },
    outlined: {
      control: 'boolean',
      description: 'Whether to use an outlined style instead of filled',
      table: {
        defaultValue: { summary: 'false' },
      },
    },
  },
  parameters: {
    docs: {
      description: {
        component: `
A badge component for displaying status labels, tags, and indicators.

## Usage

\`\`\`html
<hockey-badge variant="primary" text="IIHF"></hockey-badge>
<hockey-badge variant="warning" text="Historical" outlined></hockey-badge>
\`\`\`

## Variants

- **primary**: Blue - for main/default status
- **success**: Green - for positive states
- **warning**: Amber - for caution/historical items  
- **danger**: Red - for errors or critical states
- **info**: Cyan - for informational content
- **default**: Gray - for neutral/secondary items
        `,
      },
    },
  },
};

export default meta;
type Story = StoryObj;

// Basic stories
export const Primary: Story = {
  args: {
    variant: 'primary',
    text: 'IIHF',
  },
  render: (args) => html`
    <hockey-badge
      variant=${args.variant}
      text=${args.text}
      ?outlined=${args.outlined}
    ></hockey-badge>
  `,
};

export const Success: Story = {
  args: {
    variant: 'success',
    text: 'Active',
  },
  render: (args) => html`
    <hockey-badge
      variant=${args.variant}
      text=${args.text}
      ?outlined=${args.outlined}
    ></hockey-badge>
  `,
};

export const Warning: Story = {
  args: {
    variant: 'warning',
    text: 'Historical',
  },
  render: (args) => html`
    <hockey-badge
      variant=${args.variant}
      text=${args.text}
      ?outlined=${args.outlined}
    ></hockey-badge>
  `,
};

export const Danger: Story = {
  args: {
    variant: 'danger',
    text: 'Disabled',
  },
  render: (args) => html`
    <hockey-badge
      variant=${args.variant}
      text=${args.text}
      ?outlined=${args.outlined}
    ></hockey-badge>
  `,
};

export const Info: Story = {
  args: {
    variant: 'info',
    text: 'New',
  },
  render: (args) => html`
    <hockey-badge
      variant=${args.variant}
      text=${args.text}
      ?outlined=${args.outlined}
    ></hockey-badge>
  `,
};

export const Default: Story = {
  args: {
    variant: 'default',
    text: 'Draft',
  },
  render: (args) => html`
    <hockey-badge
      variant=${args.variant}
      text=${args.text}
      ?outlined=${args.outlined}
    ></hockey-badge>
  `,
};

// Outlined variants
export const OutlinedPrimary: Story = {
  args: {
    variant: 'primary',
    text: 'IIHF Member',
    outlined: true,
  },
  render: (args) => html`
    <hockey-badge
      variant=${args.variant}
      text=${args.text}
      ?outlined=${args.outlined}
    ></hockey-badge>
  `,
};

export const OutlinedWarning: Story = {
  args: {
    variant: 'warning',
    text: 'Historical',
    outlined: true,
  },
  render: (args) => html`
    <hockey-badge
      variant=${args.variant}
      text=${args.text}
      ?outlined=${args.outlined}
    ></hockey-badge>
  `,
};

// Showcase all variants
export const AllVariants: Story = {
  parameters: {
    docs: {
      description: {
        story: 'All available badge variants displayed together.',
      },
    },
  },
  render: () => html`
    <div style="display: flex; gap: 0.5rem; flex-wrap: wrap; align-items: center;">
      <hockey-badge variant="primary" text="Primary"></hockey-badge>
      <hockey-badge variant="success" text="Success"></hockey-badge>
      <hockey-badge variant="warning" text="Warning"></hockey-badge>
      <hockey-badge variant="danger" text="Danger"></hockey-badge>
      <hockey-badge variant="info" text="Info"></hockey-badge>
      <hockey-badge variant="default" text="Default"></hockey-badge>
    </div>
  `,
};

export const AllOutlined: Story = {
  parameters: {
    docs: {
      description: {
        story: 'All badge variants with outlined style.',
      },
    },
  },
  render: () => html`
    <div style="display: flex; gap: 0.5rem; flex-wrap: wrap; align-items: center;">
      <hockey-badge variant="primary" text="Primary" outlined></hockey-badge>
      <hockey-badge variant="success" text="Success" outlined></hockey-badge>
      <hockey-badge variant="warning" text="Warning" outlined></hockey-badge>
      <hockey-badge variant="danger" text="Danger" outlined></hockey-badge>
      <hockey-badge variant="info" text="Info" outlined></hockey-badge>
      <hockey-badge variant="default" text="Default" outlined></hockey-badge>
    </div>
  `,
};

// Real-world usage examples
export const CountryStatusExample: Story = {
  name: 'Country Status (Real Example)',
  parameters: {
    docs: {
      description: {
        story: 'How badges are used in the countries table to indicate IIHF membership and historical status.',
      },
    },
  },
  render: () => html`
    <div style="display: flex; flex-direction: column; gap: 1rem;">
      <div style="display: flex; align-items: center; gap: 0.5rem;">
        <span style="width: 150px;">Czech Republic:</span>
        <hockey-badge variant="primary" text="IIHF"></hockey-badge>
      </div>
      <div style="display: flex; align-items: center; gap: 0.5rem;">
        <span style="width: 150px;">Soviet Union:</span>
        <hockey-badge variant="warning" text="Historical" outlined></hockey-badge>
        <hockey-badge variant="default" text="1946-1991"></hockey-badge>
      </div>
      <div style="display: flex; align-items: center; gap: 0.5rem;">
        <span style="width: 150px;">Russia:</span>
        <hockey-badge variant="danger" text="Disabled"></hockey-badge>
      </div>
    </div>
  `,
};
