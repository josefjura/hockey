import type { Meta, StoryObj } from '@storybook/web-components-vite';
import { html } from 'lit';
import '../flag-icon.js';

const meta: Meta = {
  title: 'Components/FlagIcon',
  component: 'flag-icon',
  
  argTypes: {
    countryCode: {
      control: 'text',
      description: 'ISO 3166-1 alpha-2 country code (e.g., "CZ", "US")',
      table: {
        type: { summary: 'string' },
      },
    },
    countryName: {
      control: 'text',
      description: 'Country name for alt text and fallback display',
      table: {
        type: { summary: 'string' },
      },
    },
    size: {
      control: 'select',
      options: ['sm', 'md', 'lg'],
      description: 'Size variant of the flag icon',
      table: {
        defaultValue: { summary: 'md' },
      },
    },
  },
  parameters: {
    docs: {
      description: {
        component: `
A flag icon component for displaying country flags with graceful fallback.

## Usage

\`\`\`html
<flag-icon country-code="cz" country-name="Czech Republic"></flag-icon>
<flag-icon country-code="us" country-name="United States" size="lg"></flag-icon>
\`\`\`

## Features

- Displays country flags from SVG files
- Automatic fallback to country initials if image fails to load
- Three size variants: small, medium, and large
- Accessible with proper alt text

## Size Reference

| Size | Dimensions |
|------|------------|
| sm   | 20x15px    |
| md   | 32x24px    |
| lg   | 48x36px    |
        `,
      },
    },
  },
};

export default meta;
type Story = StoryObj;

export const Default: Story = {
  args: {
    countryCode: 'CZ',
    countryName: 'Czech Republic',
    size: 'md',
  },
  render: (args) => html`
    <flag-icon
      country-code=${args.countryCode}
      country-name=${args.countryName}
      size=${args.size}
    ></flag-icon>
  `,
};

export const SmallSize: Story = {
  args: {
    countryCode: 'FI',
    countryName: 'Finland',
    size: 'sm',
  },
  render: (args) => html`
    <flag-icon
      country-code=${args.countryCode}
      country-name=${args.countryName}
      size=${args.size}
    ></flag-icon>
  `,
};

export const MediumSize: Story = {
  args: {
    countryCode: 'SE',
    countryName: 'Sweden',
    size: 'md',
  },
  render: (args) => html`
    <flag-icon
      country-code=${args.countryCode}
      country-name=${args.countryName}
      size=${args.size}
    ></flag-icon>
  `,
};

export const LargeSize: Story = {
  args: {
    countryCode: 'CA',
    countryName: 'Canada',
    size: 'lg',
  },
  render: (args) => html`
    <flag-icon
      country-code=${args.countryCode}
      country-name=${args.countryName}
      size=${args.size}
    ></flag-icon>
  `,
};

// Fallback behavior when image doesn't load
export const FallbackNoCode: Story = {
  name: 'Fallback (No Country Code)',
  args: {
    countryCode: '',
    countryName: 'Unknown Country',
    size: 'md',
  },
  parameters: {
    docs: {
      description: {
        story: 'When no country code is provided, the component displays initials from the country name.',
      },
    },
  },
  render: (args) => html`
    <flag-icon
      country-code=${args.countryCode}
      country-name=${args.countryName}
      size=${args.size}
    ></flag-icon>
  `,
};

export const FallbackInvalidCode: Story = {
  name: 'Fallback (Invalid Code)',
  args: {
    countryCode: 'XX',
    countryName: 'Invalid Country',
    size: 'md',
  },
  parameters: {
    docs: {
      description: {
        story: 'When an invalid country code is provided and the image fails to load, initials are shown.',
      },
    },
  },
  render: (args) => html`
    <flag-icon
      country-code=${args.countryCode}
      country-name=${args.countryName}
      size=${args.size}
    ></flag-icon>
  `,
};

// Size comparison
export const SizeComparison: Story = {
  parameters: {
    docs: {
      description: {
        story: 'Comparison of all three size variants side by side.',
      },
    },
  },
  render: () => html`
    <div style="display: flex; align-items: center; gap: 1rem;">
      <div style="text-align: center;">
        <flag-icon country-code="US" country-name="United States" size="sm"></flag-icon>
        <div style="font-size: 0.75rem; color: #6b7280; margin-top: 0.25rem;">Small</div>
      </div>
      <div style="text-align: center;">
        <flag-icon country-code="US" country-name="United States" size="md"></flag-icon>
        <div style="font-size: 0.75rem; color: #6b7280; margin-top: 0.25rem;">Medium</div>
      </div>
      <div style="text-align: center;">
        <flag-icon country-code="US" country-name="United States" size="lg"></flag-icon>
        <div style="font-size: 0.75rem; color: #6b7280; margin-top: 0.25rem;">Large</div>
      </div>
    </div>
  `,
};

// Multiple countries showcase
export const HockeyNations: Story = {
  name: 'Hockey Nations',
  parameters: {
    docs: {
      description: {
        story: 'Flags of major hockey nations commonly seen in the application.',
      },
    },
  },
  render: () => html`
    <div style="display: flex; flex-wrap: wrap; gap: 1rem;">
      ${[
        { code: 'CZ', name: 'Czech Republic' },
        { code: 'SK', name: 'Slovakia' },
        { code: 'FI', name: 'Finland' },
        { code: 'SE', name: 'Sweden' },
        { code: 'CA', name: 'Canada' },
        { code: 'US', name: 'United States' },
        { code: 'RU', name: 'Russia' },
        { code: 'DE', name: 'Germany' },
        { code: 'CH', name: 'Switzerland' },
        { code: 'LV', name: 'Latvia' },
      ].map(
        (country) => html`
          <div style="display: flex; align-items: center; gap: 0.5rem; padding: 0.5rem; background: #f9fafb; border-radius: 4px;">
            <flag-icon country-code=${country.code} country-name=${country.name} size="sm"></flag-icon>
            <span style="font-size: 0.875rem;">${country.name}</span>
          </div>
        `
      )}
    </div>
  `,
};

// In context example
export const InTableContext: Story = {
  name: 'Table Row Context',
  parameters: {
    docs: {
      description: {
        story: 'How the flag icon appears in a typical table row, alongside country name.',
      },
    },
  },
  render: () => html`
    <div style="display: flex; flex-direction: column; gap: 0.5rem;">
      <div style="display: flex; align-items: center; gap: 0.5rem; padding: 0.75rem; border-bottom: 1px solid #e5e7eb;">
        <flag-icon country-code="CZ" country-name="Czech Republic" size="sm"></flag-icon>
        <span>Czech Republic</span>
      </div>
      <div style="display: flex; align-items: center; gap: 0.5rem; padding: 0.75rem; border-bottom: 1px solid #e5e7eb;">
        <flag-icon country-code="SK" country-name="Slovakia" size="sm"></flag-icon>
        <span>Slovakia</span>
      </div>
      <div style="display: flex; align-items: center; gap: 0.5rem; padding: 0.75rem; border-bottom: 1px solid #e5e7eb;">
        <flag-icon country-code="FI" country-name="Finland" size="sm"></flag-icon>
        <span>Finland</span>
      </div>
    </div>
  `,
};
