import type { Meta, StoryObj } from '@storybook/web-components-vite';
import { html } from 'lit';
import { http, HttpResponse, delay } from 'msw';
import { expect, userEvent, within, waitFor } from '@storybook/test';
import '../country-selector.js';

// Mock countries data
const mockCountries = [
  { id: 1, name: 'Czech Republic', iihf: true, iocCode: 'CZE', iso2Code: 'CZ', isHistorical: false, years: null, enabled: true },
  { id: 2, name: 'Slovakia', iihf: true, iocCode: 'SVK', iso2Code: 'SK', isHistorical: false, years: null, enabled: true },
  { id: 3, name: 'Finland', iihf: true, iocCode: 'FIN', iso2Code: 'FI', isHistorical: false, years: null, enabled: true },
  { id: 4, name: 'Sweden', iihf: true, iocCode: 'SWE', iso2Code: 'SE', isHistorical: false, years: null, enabled: true },
  { id: 5, name: 'Canada', iihf: true, iocCode: 'CAN', iso2Code: 'CA', isHistorical: false, years: null, enabled: true },
  { id: 6, name: 'United States', iihf: true, iocCode: 'USA', iso2Code: 'US', isHistorical: false, years: null, enabled: true },
  { id: 7, name: 'Russia', iihf: false, iocCode: 'RUS', iso2Code: 'RU', isHistorical: false, years: null, enabled: true },
  { id: 8, name: 'Germany', iihf: true, iocCode: 'GER', iso2Code: 'DE', isHistorical: false, years: null, enabled: true },
  { id: 9, name: 'Switzerland', iihf: true, iocCode: 'SUI', iso2Code: 'CH', isHistorical: false, years: null, enabled: true },
  { id: 10, name: 'Latvia', iihf: true, iocCode: 'LAT', iso2Code: 'LV', isHistorical: false, years: null, enabled: true },
];

// Default MSW handlers
const defaultHandlers = [
  http.get('/api/countries', async ({ request }) => {
    await delay(150);
    const url = new URL(request.url);
    const iihfOnly = url.searchParams.get('iihf_only') === 'true';
    const data = iihfOnly ? mockCountries.filter(c => c.iihf) : mockCountries;
    return HttpResponse.json(data);
  }),
];

const meta: Meta = {
  title: 'Components/CountrySelector',
  component: 'country-selector',
  
  argTypes: {
    name: {
      control: 'text',
      description: 'Form field name for the selector',
    },
    placeholder: {
      control: 'text',
      description: 'Placeholder text when no country is selected',
      table: {
        defaultValue: { summary: 'Select a country' },
      },
    },
    iihfOnly: {
      control: 'boolean',
      description: 'Only show IIHF member countries',
      table: {
        defaultValue: { summary: 'false' },
      },
    },
  },
  parameters: {
    // Use fullscreen layout to avoid position:fixed issues with the dropdown
    layout: 'fullscreen',
    docs: {
      description: {
        component: `
A searchable dropdown selector for choosing countries.

## Usage

\`\`\`html
<country-selector
  name="countryId"
  placeholder="Choose a country"
  iihf-only>
</country-selector>
\`\`\`

## Features

- **Searchable dropdown**: Filter countries by typing
- **Flag display**: Shows country flag next to name
- **IIHF filter**: Optionally show only IIHF member nations
- **Form integration**: Works with standard forms (form-associated custom element)
- **Keyboard navigation**: Full keyboard support

## Note on Storybook

The dropdown uses \`position: fixed\` for proper display in complex layouts. 
Stories use fullscreen layout to ensure the dropdown displays correctly.
        `,
      },
      // Ensure story source is displayed properly
      story: {
        inline: true,
      },
    },
    msw: {
      handlers: defaultHandlers,
    },
  },
  // Fullscreen decorator that provides proper padding and height
  decorators: [
    (story) => html`
      <div style="padding: 2rem; min-height: 500px;">
        ${story()}
      </div>
    `,
  ],
};

export default meta;
type Story = StoryObj;

export const Default: Story = {
  parameters: {
    msw: { handlers: defaultHandlers },
  },
  render: () => html`
    <div style="width: 300px;">
      <country-selector
        name="countryId"
        placeholder="Select a country"
      ></country-selector>
    </div>
  `,
  play: async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    const selector = canvasElement.querySelector('country-selector');

    // Wait for countries to load
    await waitFor(() => expect(selector!.countries.length).toBeGreaterThan(0), { timeout: 3000 });

    // Verify placeholder is shown
    const shadowRoot = selector!.shadowRoot!;
    const placeholder = within(shadowRoot).getByText(/Select a country/i);
    await expect(placeholder).toBeInTheDocument();

    // Verify all 10 mock countries loaded
    await expect(selector!.countries.length).toBe(10);
  },
};

export const WithPreselectedValue: Story = {
  name: 'Pre-selected Value',
  parameters: {
    msw: { handlers: defaultHandlers },
  },
  render: () => html`
    <div style="width: 300px;">
      <country-selector
        name="countryId"
        .value=${1}
      ></country-selector>
    </div>
  `,
  play: async ({ canvasElement }) => {
    const selector = canvasElement.querySelector('country-selector');

    // Wait for data to load and selection to appear
    await waitFor(() => expect(selector!.selectedCountry).toBeTruthy(), { timeout: 3000 });

    // Verify Czech Republic is selected (ID: 1)
    await expect(selector!.selectedCountry!.name).toBe('Czech Republic');
    await expect(selector!.value).toBe(1);

    // Verify it's displayed in the UI
    const shadowRoot = selector!.shadowRoot!;
    const display = within(shadowRoot).getByText('Czech Republic');
    await expect(display).toBeInTheDocument();
  },
};

export const IIHFMembersOnly: Story = {
  name: 'IIHF Members Only',
  parameters: {
    docs: {
      description: {
        story: 'When `iihf-only` is set, only IIHF member nations are shown.',
      },
    },
    msw: { handlers: defaultHandlers },
  },
  render: () => html`
    <div style="width: 300px;">
      <country-selector
        name="countryId"
        placeholder="Select IIHF country"
        iihf-only
      ></country-selector>
    </div>
  `,
  play: async ({ canvasElement }) => {
    const selector = canvasElement.querySelector('country-selector');

    // Wait for filtered countries to load
    await waitFor(() => expect(selector!.countries.length).toBeGreaterThan(0), { timeout: 3000 });

    // Verify only IIHF members are loaded (Russia excluded: iihf=false in mock)
    await expect(selector!.countries.length).toBe(9);

    // Verify all loaded countries are IIHF members
    selector!.countries.forEach(country => {
      expect(country.iihf).toBe(true);
    });

    // Verify Russia is NOT in the list
    const hasRussia = selector!.countries.some(c => c.name === 'Russia');
    await expect(hasRussia).toBe(false);
  },
};

export const CustomPlaceholder: Story = {
  parameters: {
    msw: { handlers: defaultHandlers },
  },
  render: () => html`
    <div style="width: 300px;">
      <country-selector
        name="nationality"
        placeholder="Choose player nationality..."
      ></country-selector>
    </div>
  `,
};

// Loading state
export const Loading: Story = {
  parameters: {
    docs: {
      description: {
        story: 'Loading state while countries are being fetched (3 second delay).',
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
    <div style="width: 300px;">
      <country-selector
        name="countryId"
        placeholder="Loading countries..."
      ></country-selector>
    </div>
  `,
};

// In form context
export const InFormContext: Story = {
  name: 'In Form Context',
  parameters: {
    docs: {
      description: {
        story: 'Country selector used within a form.',
      },
    },
    msw: { handlers: defaultHandlers },
  },
  render: () => html`
    <form style="display: flex; flex-direction: column; gap: 1rem; max-width: 400px; padding: 1.5rem; background: #f9fafb; border-radius: 8px;">
      <div>
        <label style="display: block; margin-bottom: 0.5rem; font-weight: 500; font-size: 0.875rem;">Player Name</label>
        <input type="text" name="playerName" placeholder="Enter name" style="width: 100%; padding: 0.5rem; border: 1px solid #d1d5db; border-radius: 4px; box-sizing: border-box;">
      </div>
      
      <div>
        <label style="display: block; margin-bottom: 0.5rem; font-weight: 500; font-size: 0.875rem;">Country</label>
        <country-selector
          name="countryId"
          placeholder="Select country..."
        ></country-selector>
      </div>
      
      <button type="submit" style="padding: 0.5rem 1rem; background: #3b82f6; color: white; border: none; border-radius: 4px; cursor: pointer; font-weight: 500;">
        Submit
      </button>
    </form>
  `,
};

// Multiple selectors
export const MultipleSelectors: Story = {
  name: 'Multiple Selectors',
  parameters: {
    docs: {
      description: {
        story: 'Multiple country selectors on the same page.',
      },
    },
    msw: { handlers: defaultHandlers },
  },
  render: () => html`
    <div style="display: flex; gap: 2rem; flex-wrap: wrap;">
      <div style="width: 280px;">
        <label style="display: block; margin-bottom: 0.5rem; font-weight: 500; font-size: 0.875rem;">Birth Country</label>
        <country-selector
          name="birthCountry"
          placeholder="Select birth country..."
        ></country-selector>
      </div>
      
      <div style="width: 280px;">
        <label style="display: block; margin-bottom: 0.5rem; font-weight: 500; font-size: 0.875rem;">Current Nationality</label>
        <country-selector
          name="currentNationality"
          placeholder="Select nationality..."
          iihf-only
        ></country-selector>
      </div>
    </div>
  `,
};

// Event handling example
export const WithEventHandling: Story = {
  name: 'With Event Handling',
  parameters: {
    docs: {
      description: {
        story: 'Listen to the `change` event to react to selections.',
      },
    },
    msw: { handlers: defaultHandlers },
  },
  render: () => {
    const handleChange = (e: CustomEvent) => {
      const output = document.getElementById('selection-output');
      if (output) {
        if (e.detail.id) {
          output.textContent = `Selected: ${e.detail.name} (ID: ${e.detail.id})`;
        } else {
          output.textContent = 'No country selected';
        }
      }
    };

    return html`
      <div style="width: 300px;">
        <country-selector
          name="countryId"
          placeholder="Select a country..."
          @change=${handleChange}
        ></country-selector>
        <p id="selection-output" style="margin-top: 1rem; padding: 0.5rem; background: #f3f4f6; border-radius: 4px; font-size: 0.875rem;">
          No country selected
        </p>
      </div>
    `;
  },
  play: async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    const selector = canvasElement.querySelector('country-selector');

    // Wait for countries to load
    await waitFor(() => expect(selector!.countries.length).toBeGreaterThan(0), { timeout: 3000 });

    // Open dropdown
    const shadowRoot = selector!.shadowRoot!;
    const button = within(shadowRoot).getByRole('button');
    await userEvent.click(button);

    // Wait for dropdown to open
    await waitFor(() => {
      const dropdown = shadowRoot.querySelector('.dropdown');
      return expect(dropdown).toBeInTheDocument();
    });

    // Click on first country in the list (Czech Republic)
    const firstCountryButton = shadowRoot.querySelectorAll('.country-item')[1]; // [0] is "No country", [1] is first actual country
    await userEvent.click(firstCountryButton as HTMLElement);

    // Verify the output text updated via event handler
    const output = canvasElement.querySelector('#selection-output');
    await waitFor(() => {
      expect(output!.textContent).toContain('Selected: Czech Republic');
      expect(output!.textContent).toContain('ID: 1');
    });
  },
};
