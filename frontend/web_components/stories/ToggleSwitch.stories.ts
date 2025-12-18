import type { Meta, StoryObj } from '@storybook/web-components-vite';
import { html } from 'lit';
import { http, HttpResponse, delay } from 'msw';
import '../toggle-switch.js';

// Default handler for toggle API
const defaultToggleHandler = http.post(/\/api\/.*toggle.*/, async () => {
  await delay(200);
  return HttpResponse.json({ success: true });
});

const meta: Meta = {
  title: 'Components/ToggleSwitch',
  component: 'toggle-switch',
  
  argTypes: {
    checked: {
      control: 'boolean',
      description: 'Whether the toggle is checked/on',
      table: {
        defaultValue: { summary: 'false' },
      },
    },
    label: {
      control: 'text',
      description: 'Label text displayed next to the toggle',
    },
    disabled: {
      control: 'boolean',
      description: 'Whether the toggle is disabled',
      table: {
        defaultValue: { summary: 'false' },
      },
    },
    apiEndpoint: {
      control: 'text',
      description: 'API endpoint to POST toggle changes to',
      table: {
        type: { summary: 'string' },
      },
    },
    entityId: {
      control: 'number',
      description: 'ID of the entity being toggled (passed to API)',
    },
  },
  parameters: {
    docs: {
      description: {
        component: `
A toggle switch component with API integration for persisting state changes.

## Usage

\`\`\`html
<toggle-switch
  .checked=\${true}
  .entityId=\${123}
  api-endpoint="/api/countries/123/toggle"
  label="Enabled">
</toggle-switch>
\`\`\`

## Features

- **Optimistic updates**: UI responds immediately
- **Automatic rollback**: Reverts on API error
- **Loading state**: Shows spinner during API call
- **Keyboard accessible**: Space/Enter to toggle
- **Form integration**: Works with form submissions

## Events

- \`toggle-change\`: Fired when toggle state changes successfully
- \`toggle-error\`: Fired when API call fails
        `,
      },
    },
    msw: {
      handlers: [defaultToggleHandler],
    },
  },
};

export default meta;
type Story = StoryObj;

// Basic states
export const Off: Story = {
  args: {
    checked: false,
    label: 'Disabled',
  },
  parameters: {
    msw: { handlers: [defaultToggleHandler] },
  },
  render: (args) => html`
    <toggle-switch
      ?checked=${args.checked}
      label=${args.label}
      api-endpoint="/api/toggle"
    ></toggle-switch>
  `,
};

export const On: Story = {
  args: {
    checked: true,
    label: 'Enabled',
  },
  parameters: {
    msw: { handlers: [defaultToggleHandler] },
  },
  render: (args) => html`
    <toggle-switch
      ?checked=${args.checked}
      label=${args.label}
      api-endpoint="/api/toggle"
    ></toggle-switch>
  `,
};

export const Disabled: Story = {
  args: {
    checked: true,
    label: 'Cannot change',
    disabled: true,
  },
  render: (args) => html`
    <toggle-switch
      ?checked=${args.checked}
      label=${args.label}
      ?disabled=${args.disabled}
    ></toggle-switch>
  `,
};

// Without label
export const NoLabel: Story = {
  name: 'Without Label',
  args: {
    checked: true,
  },
  parameters: {
    msw: { handlers: [defaultToggleHandler] },
  },
  render: (args) => html`
    <toggle-switch
      ?checked=${args.checked}
      api-endpoint="/api/toggle"
    ></toggle-switch>
  `,
};

// Simulated loading state (slow API)
export const SlowAPI: Story = {
  name: 'Slow API Response',
  args: {
    checked: false,
    label: 'Click to see loading',
  },
  parameters: {
    docs: {
      description: {
        story: 'Demonstrates the loading spinner when API response is slow (2 second delay).',
      },
    },
    msw: {
      handlers: [
        http.post('/api/toggle-slow', async () => {
          await delay(2000);
          return HttpResponse.json({ success: true, enabled: true });
        }),
      ],
    },
  },
  render: (args) => html`
    <toggle-switch
      ?checked=${args.checked}
      label=${args.label}
      api-endpoint="/api/toggle-slow"
    ></toggle-switch>
  `,
};

// API error scenario
export const APIError: Story = {
  name: 'API Error (Rollback)',
  args: {
    checked: false,
    label: 'Click to see error rollback',
  },
  parameters: {
    docs: {
      description: {
        story: 'Demonstrates automatic rollback when the API returns an error. Toggle will briefly change then revert.',
      },
    },
    msw: {
      handlers: [
        http.post('/api/toggle-error', async () => {
          await delay(500);
          return HttpResponse.json({ error: 'Permission denied' }, { status: 403 });
        }),
      ],
    },
  },
  render: (args) => html`
    <toggle-switch
      ?checked=${args.checked}
      label=${args.label}
      api-endpoint="/api/toggle-error"
    ></toggle-switch>
  `,
};

// Multiple toggles
export const MultipleToggles: Story = {
  name: 'Multiple Toggles',
  parameters: {
    docs: {
      description: {
        story: 'Multiple independent toggle switches, as they might appear in a settings panel.',
      },
    },
    msw: { handlers: [defaultToggleHandler] },
  },
  render: () => html`
    <div style="display: flex; flex-direction: column; gap: 1rem;">
      <toggle-switch checked label="Email notifications" api-endpoint="/api/settings/email"></toggle-switch>
      <toggle-switch label="SMS notifications" api-endpoint="/api/settings/sms"></toggle-switch>
      <toggle-switch checked label="Push notifications" api-endpoint="/api/settings/push"></toggle-switch>
      <toggle-switch label="Marketing emails" api-endpoint="/api/settings/marketing"></toggle-switch>
    </div>
  `,
};

// Real-world usage: Country toggle
export const CountryToggle: Story = {
  name: 'Country Enable (Real Example)',
  parameters: {
    docs: {
      description: {
        story: 'How the toggle appears in the countries table for enabling/disabling countries.',
      },
    },
    msw: { handlers: [defaultToggleHandler] },
  },
  render: () => html`
    <div style="display: flex; flex-direction: column; gap: 0.75rem; padding: 1rem; background: #f9fafb; border-radius: 8px;">
      <div style="display: flex; align-items: center; justify-content: space-between; padding: 0.5rem; background: white; border-radius: 4px;">
        <span style="display: flex; align-items: center; gap: 0.5rem;">
          🇨🇿 Czech Republic
        </span>
        <toggle-switch checked api-endpoint="/api/countries/1/toggle" .entityId=${1}></toggle-switch>
      </div>
      <div style="display: flex; align-items: center; justify-content: space-between; padding: 0.5rem; background: white; border-radius: 4px;">
        <span style="display: flex; align-items: center; gap: 0.5rem;">
          🇷🇺 Russia
        </span>
        <toggle-switch api-endpoint="/api/countries/7/toggle" .entityId=${7}></toggle-switch>
      </div>
      <div style="display: flex; align-items: center; justify-content: space-between; padding: 0.5rem; background: white; border-radius: 4px;">
        <span style="display: flex; align-items: center; gap: 0.5rem;">
          🇫🇮 Finland
        </span>
        <toggle-switch checked api-endpoint="/api/countries/3/toggle" .entityId=${3}></toggle-switch>
      </div>
    </div>
  `,
};
