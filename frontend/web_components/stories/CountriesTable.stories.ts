import type { Meta, StoryObj } from '@storybook/web-components-vite';
import { html } from 'lit';
import { http, HttpResponse, delay } from 'msw';
import '../countries-table.js';

// Mock countries data
const mockCountries = [
  { id: 1, name: 'Czech Republic', iihf: true, iocCode: 'CZE', iso2Code: 'CZ', isHistorical: false, years: null, enabled: true },
  { id: 2, name: 'Slovakia', iihf: true, iocCode: 'SVK', iso2Code: 'SK', isHistorical: false, years: null, enabled: true },
  { id: 3, name: 'Finland', iihf: true, iocCode: 'FIN', iso2Code: 'FI', isHistorical: false, years: null, enabled: true },
  { id: 4, name: 'Sweden', iihf: true, iocCode: 'SWE', iso2Code: 'SE', isHistorical: false, years: null, enabled: true },
  { id: 5, name: 'Canada', iihf: true, iocCode: 'CAN', iso2Code: 'CA', isHistorical: false, years: null, enabled: true },
  { id: 6, name: 'United States', iihf: true, iocCode: 'USA', iso2Code: 'US', isHistorical: false, years: null, enabled: true },
  { id: 7, name: 'Russia', iihf: false, iocCode: 'RUS', iso2Code: 'RU', isHistorical: false, years: null, enabled: false },
  { id: 8, name: 'Soviet Union', iihf: false, iocCode: 'URS', iso2Code: null, isHistorical: true, years: '1946-1991', enabled: false },
  { id: 9, name: 'Germany', iihf: true, iocCode: 'GER', iso2Code: 'DE', isHistorical: false, years: null, enabled: true },
  { id: 10, name: 'Switzerland', iihf: true, iocCode: 'SUI', iso2Code: 'CH', isHistorical: false, years: null, enabled: true },
  { id: 11, name: 'Latvia', iihf: true, iocCode: 'LAT', iso2Code: 'LV', isHistorical: false, years: null, enabled: true },
  { id: 12, name: 'Denmark', iihf: true, iocCode: 'DEN', iso2Code: 'DK', isHistorical: false, years: null, enabled: true },
  { id: 13, name: 'Norway', iihf: true, iocCode: 'NOR', iso2Code: 'NO', isHistorical: false, years: null, enabled: true },
  { id: 14, name: 'Austria', iihf: true, iocCode: 'AUT', iso2Code: 'AT', isHistorical: false, years: null, enabled: true },
  { id: 15, name: 'France', iihf: true, iocCode: 'FRA', iso2Code: 'FR', isHistorical: false, years: null, enabled: true },
  { id: 16, name: 'Belarus', iihf: false, iocCode: 'BLR', iso2Code: 'BY', isHistorical: false, years: null, enabled: false },
  { id: 17, name: 'Kazakhstan', iihf: true, iocCode: 'KAZ', iso2Code: 'KZ', isHistorical: false, years: null, enabled: true },
  { id: 18, name: 'Poland', iihf: true, iocCode: 'POL', iso2Code: 'PL', isHistorical: false, years: null, enabled: true },
  { id: 19, name: 'Hungary', iihf: true, iocCode: 'HUN', iso2Code: 'HU', isHistorical: false, years: null, enabled: true },
  { id: 20, name: 'Great Britain', iihf: true, iocCode: 'GBR', iso2Code: 'GB', isHistorical: false, years: null, enabled: true },
  { id: 21, name: 'East Germany', iihf: false, iocCode: 'GDR', iso2Code: null, isHistorical: true, years: '1949-1990', enabled: false },
  { id: 22, name: 'Czechoslovakia', iihf: false, iocCode: 'TCH', iso2Code: null, isHistorical: true, years: '1920-1992', enabled: false },
];

const meta: Meta = {
  title: 'Tables/CountriesTable',
  component: 'countries-table',
  
  argTypes: {
    apiEndpoint: {
      control: 'text',
      description: 'API endpoint to fetch countries from',
      table: {
        defaultValue: { summary: '/api/countries' },
      },
    },
    pageSize: {
      control: { type: 'select' },
      options: [10, 20, 50],
      description: 'Number of rows per page',
      table: {
        defaultValue: { summary: '20' },
      },
    },
  },
  parameters: {
    docs: {
      description: {
        component: `
A pre-configured data table specifically for displaying countries.

This component wraps \`client-data-table\` with country-specific column configuration including:

- Country name with flag icon
- ISO and IOC codes
- IIHF membership badge
- Historical country indicator with year ranges
- Enable/disable toggle

## Usage

\`\`\`html
<countries-table api-endpoint="/api/countries"></countries-table>
\`\`\`

## When to Use

The countries table is used on the **Management** page for administering country data.
Since there are typically fewer than 250 countries, client-side filtering and sorting
provides excellent UX without needing server-side pagination.
        `,
      },
    },
  },
};

export default meta;
type Story = StoryObj;

// Default view with all countries
export const Default: Story = {
  parameters: {
    msw: {
      handlers: [
        http.get('/api/countries', async () => {
          await delay(200);
          return HttpResponse.json(mockCountries);
        }),
        http.post('/api/countries/:id/toggle', async ({ params }) => {
          await delay(300);
          return HttpResponse.json({ success: true });
        }),
      ],
    },
  },
  render: () => html`
    <div style="width: 100%; max-width: 1200px;">
      <countries-table api-endpoint="/api/countries"></countries-table>
    </div>
  `,
};

// Loading state
export const Loading: Story = {
  name: 'Loading State',
  parameters: {
    docs: {
      description: {
        story: 'Shows the loading state while countries are being fetched.',
      },
    },
    msw: {
      handlers: [
        http.get('/api/countries', async () => {
          await delay(3000);
          return HttpResponse.json(mockCountries);
        }),
      ],
    },
  },
  render: () => html`
    <div style="width: 100%; max-width: 1200px;">
      <countries-table api-endpoint="/api/countries"></countries-table>
    </div>
  `,
};

// Empty state
export const Empty: Story = {
  name: 'Empty State',
  parameters: {
    docs: {
      description: {
        story: 'What the table looks like when no countries are returned.',
      },
    },
    msw: {
      handlers: [
        http.get('/api/countries', async () => {
          await delay(200);
          return HttpResponse.json([]);
        }),
      ],
    },
  },
  render: () => html`
    <div style="width: 100%; max-width: 1200px;">
      <countries-table api-endpoint="/api/countries"></countries-table>
    </div>
  `,
};

// Error state
export const Error: Story = {
  name: 'Error State',
  parameters: {
    docs: {
      description: {
        story: 'Error display when the API fails.',
      },
    },
    msw: {
      handlers: [
        http.get('/api/countries', async () => {
          await delay(200);
          return HttpResponse.json({ error: 'Database connection failed' }, { status: 500 });
        }),
      ],
    },
  },
  render: () => html`
    <div style="width: 100%; max-width: 1200px;">
      <countries-table api-endpoint="/api/countries"></countries-table>
    </div>
  `,
};

// Only IIHF members
export const IIHFMembersOnly: Story = {
  name: 'IIHF Members Only',
  parameters: {
    docs: {
      description: {
        story: 'Filtered view showing only IIHF member nations.',
      },
    },
    msw: {
      handlers: [
        http.get('/api/countries', async () => {
          await delay(200);
          return HttpResponse.json(mockCountries.filter(c => c.iihf));
        }),
        http.post('/api/countries/:id/toggle', async () => {
          await delay(300);
          return HttpResponse.json({ success: true });
        }),
      ],
    },
  },
  render: () => html`
    <div style="width: 100%; max-width: 1200px;">
      <p style="margin-bottom: 1rem; padding: 0.75rem; background: #eff6ff; border-radius: 4px; color: #1e40af;">
        ℹ️ Showing only IIHF member nations
      </p>
      <countries-table api-endpoint="/api/countries"></countries-table>
    </div>
  `,
};

// Historical countries focus
export const HistoricalCountries: Story = {
  name: 'Historical Countries',
  parameters: {
    docs: {
      description: {
        story: 'View focused on historical countries that no longer exist (Soviet Union, East Germany, Czechoslovakia).',
      },
    },
    msw: {
      handlers: [
        http.get('/api/countries', async () => {
          await delay(200);
          return HttpResponse.json(mockCountries.filter(c => c.isHistorical));
        }),
      ],
    },
  },
  render: () => html`
    <div style="width: 100%; max-width: 1200px;">
      <p style="margin-bottom: 1rem; padding: 0.75rem; background: #fef3c7; border-radius: 4px; color: #92400e;">
        ⏰ Historical countries - these nations no longer exist but have historical records
      </p>
      <countries-table api-endpoint="/api/countries"></countries-table>
    </div>
  `,
};

// Toggle error demonstration
export const ToggleError: Story = {
  name: 'Toggle Error Handling',
  parameters: {
    docs: {
      description: {
        story: 'Demonstrates error handling when the toggle API fails. Try clicking a toggle to see the rollback behavior.',
      },
    },
    msw: {
      handlers: [
        http.get('/api/countries', async () => {
          await delay(200);
          return HttpResponse.json(mockCountries.slice(0, 5));
        }),
        http.post('/api/countries/:id/toggle', async () => {
          await delay(500);
          return HttpResponse.json({ error: 'Permission denied' }, { status: 403 });
        }),
      ],
    },
  },
  render: () => html`
    <div style="width: 100%; max-width: 1200px;">
      <p style="margin-bottom: 1rem; padding: 0.75rem; background: #fef2f2; border-radius: 4px; color: #991b1b;">
        ⚠️ Toggle operations will fail - watch the toggle revert after clicking
      </p>
      <countries-table api-endpoint="/api/countries"></countries-table>
    </div>
  `,
};

// Small page size
export const SmallPageSize: Story = {
  name: 'Small Page Size',
  args: {
    pageSize: 5,
  },
  parameters: {
    docs: {
      description: {
        story: 'Countries table with a smaller page size to show pagination.',
      },
    },
    msw: {
      handlers: [
        http.get('/api/countries', async () => {
          await delay(200);
          return HttpResponse.json(mockCountries);
        }),
        http.post('/api/countries/:id/toggle', async () => {
          await delay(300);
          return HttpResponse.json({ success: true });
        }),
      ],
    },
  },
  render: (args) => html`
    <div style="width: 100%; max-width: 1200px;">
      <countries-table api-endpoint="/api/countries" page-size=${args.pageSize || 5}></countries-table>
    </div>
  `,
};

// In page context
export const InPageContext: Story = {
  name: 'In Page Context',
  parameters: {
    layout: 'fullscreen',
    docs: {
      description: {
        story: 'How the countries table appears within the Management page context.',
      },
    },
    msw: {
      handlers: [
        http.get('/api/countries', async () => {
          await delay(200);
          return HttpResponse.json(mockCountries);
        }),
        http.post('/api/countries/:id/toggle', async () => {
          await delay(300);
          return HttpResponse.json({ success: true });
        }),
      ],
    },
  },
  render: () => html`
    <div style="background: #f3f4f6; min-height: 100vh; padding: 2rem;">
      <div style="max-width: 1400px; margin: 0 auto;">
        <header style="margin-bottom: 2rem;">
          <h1 style="font-size: 1.5rem; font-weight: 600; color: #111827;">Management</h1>
          <p style="color: #6b7280; margin-top: 0.25rem;">Manage countries and application settings</p>
        </header>
        
        <div style="background: white; border-radius: 8px; box-shadow: 0 1px 3px rgba(0,0,0,0.1); padding: 1.5rem;">
          <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 1rem;">
            <h2 style="font-size: 1.125rem; font-weight: 600;">Countries</h2>
            <button style="background: #3b82f6; color: white; padding: 0.5rem 1rem; border: none; border-radius: 4px; cursor: pointer; font-weight: 500;">
              + Add Country
            </button>
          </div>
          <countries-table api-endpoint="/api/countries"></countries-table>
        </div>
      </div>
    </div>
  `,
};
