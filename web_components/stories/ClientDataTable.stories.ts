import type { Meta, StoryObj } from '@storybook/web-components-vite';
import { html, render } from 'lit';
import { http, HttpResponse, delay } from 'msw';
import { expect, userEvent, within, waitFor } from '@storybook/test';
import '../client-data-table.js';
import type { Column } from '../shared/types.js';

// Sample data types
interface SimpleItem {
  id: number;
  name: string;
  email: string;
  role: string;
  active: boolean;
}

interface Country {
  id: number;
  name: string;
  iihf: boolean;
  iocCode: string | null;
  isHistorical: boolean;
  years: string | null;
}

interface Product {
  id: number;
  name: string;
  quantity: number;
  price: number;
}

// Mock data
const mockSimpleData: SimpleItem[] = [
  { id: 1, name: 'John Doe', email: 'john@example.com', role: 'Admin', active: true },
  { id: 2, name: 'Jane Smith', email: 'jane@example.com', role: 'Editor', active: true },
  { id: 3, name: 'Bob Johnson', email: 'bob@example.com', role: 'Viewer', active: false },
  { id: 4, name: 'Alice Brown', email: 'alice@example.com', role: 'Editor', active: true },
  { id: 5, name: 'Charlie Wilson', email: 'charlie@example.com', role: 'Viewer', active: true },
];

const mockLargeData: SimpleItem[] = Array.from({ length: 100 }, (_, i) => ({
  id: i + 1,
  name: `User ${i + 1}`,
  email: `user${i + 1}@example.com`,
  role: ['Admin', 'Editor', 'Viewer'][i % 3],
  active: i % 4 !== 0,
}));

const mockCountries: Country[] = [
  { id: 1, name: 'Czech Republic', iihf: true, iocCode: 'CZE', isHistorical: false, years: null },
  { id: 2, name: 'Slovakia', iihf: true, iocCode: 'SVK', isHistorical: false, years: null },
  { id: 3, name: 'Finland', iihf: true, iocCode: 'FIN', isHistorical: false, years: null },
  { id: 4, name: 'Sweden', iihf: true, iocCode: 'SWE', isHistorical: false, years: null },
  { id: 5, name: 'Canada', iihf: true, iocCode: 'CAN', isHistorical: false, years: null },
  { id: 6, name: 'United States', iihf: true, iocCode: 'USA', isHistorical: false, years: null },
  { id: 7, name: 'Russia', iihf: false, iocCode: 'RUS', isHistorical: false, years: null },
  { id: 8, name: 'Soviet Union', iihf: false, iocCode: 'URS', isHistorical: true, years: '1946-1991' },
  { id: 9, name: 'Germany', iihf: true, iocCode: 'GER', isHistorical: false, years: null },
  { id: 10, name: 'Switzerland', iihf: true, iocCode: 'SUI', isHistorical: false, years: null },
];

const mockProducts: Product[] = [
  { id: 1, name: 'Widget', quantity: 150, price: 29.99 },
  { id: 2, name: 'Gadget', quantity: 42, price: 149.99 },
  { id: 3, name: 'Doohickey', quantity: 500, price: 9.99 },
  { id: 4, name: 'Thingamajig', quantity: 25, price: 299.99 },
];

// Column definitions
const simpleColumns: Column<SimpleItem>[] = [
  { key: 'id', label: 'ID', sortable: true, width: '80px', align: 'center' },
  { key: 'name', label: 'Name', sortable: true, filterable: true },
  { key: 'email', label: 'Email', sortable: true, filterable: true },
  { key: 'role', label: 'Role', sortable: true, width: '120px' },
  {
    key: 'active',
    label: 'Status',
    sortable: true,
    width: '100px',
    align: 'center',
    renderer: (value: boolean) => value ? '● Active' : '○ Inactive',
  },
];

const countryColumns: Column<Country>[] = [
  { key: 'id', label: 'ID', sortable: true, width: '60px', align: 'center' },
  { key: 'name', label: 'Country', sortable: true, filterable: true },
  { key: 'iocCode', label: 'IOC', width: '80px', align: 'center', renderer: (v) => v || '-' },
  { key: 'iihf', label: 'IIHF', width: '80px', align: 'center', renderer: (v) => v ? '✓' : '-' },
  { key: 'isHistorical', label: 'Status', width: '120px', renderer: (v, row) => v ? `Historical (${row.years})` : 'Active' },
];

const productColumns: Column<Product>[] = [
  { key: 'id', label: 'ID', width: '60px', align: 'center' },
  { key: 'name', label: 'Product', align: 'left' },
  { key: 'quantity', label: 'Qty', width: '100px', align: 'center' },
  { key: 'price', label: 'Price', width: '100px', align: 'right', renderer: (v: number) => `$${v.toFixed(2)}` },
];

/**
 * Helper function to create a table element and set columns programmatically.
 * This is necessary because the `columns` property contains renderer functions
 * that cannot be serialized as HTML attributes.
 */
function createTableElement(
  endpoint: string,
  columns: Column<any>[],
  options: { pageSize?: number; emptyMessage?: string } = {}
): HTMLElement {
  const container = document.createElement('div');
  container.style.width = '100%';
  container.style.maxWidth = '900px';

  const table = document.createElement('client-data-table') as any;
  table.setAttribute('api-endpoint', endpoint);
  if (options.pageSize) {
    table.setAttribute('page-size', options.pageSize.toString());
  }
  if (options.emptyMessage) {
    table.setAttribute('empty-message', options.emptyMessage);
  }

  // Set columns as a JavaScript property after the element is created
  // Use setTimeout to ensure the element is connected to DOM
  setTimeout(() => {
    table.columns = columns;
  }, 0);

  container.appendChild(table);
  return container;
}

// Default handlers
const defaultHandlers = [
  http.get('/api/users', async () => {
    await delay(200);
    return HttpResponse.json(mockSimpleData);
  }),
];

const meta: Meta = {
  title: 'Components/ClientDataTable',
  component: 'client-data-table',
  
  parameters: {
    layout: 'padded',
    docs: {
      description: {
        component: `
A fully client-side data table component with filtering, sorting, and pagination.

## When to Use

**Use client-data-table when:**
- Dataset is **small to medium** (< 500 rows)
- Data changes infrequently
- Fast filtering/sorting without server round-trips is desired

**Use server-side HTMX tables when:**
- Dataset is **large** (> 500 rows)
- Real-time data updates needed
- Complex server-side filtering logic

## Features

✅ Client-side sorting  
✅ Client-side filtering (debounced search)  
✅ Client-side pagination with ellipsis  
✅ Column configuration  
✅ Custom cell renderers  
✅ Loading, error, and empty states  

## Important Note

The \`columns\` property must be set via JavaScript as it can contain renderer functions.
This means stories use \`createTableElement()\` helper to properly set up the table.
        `,
      },
    },
    msw: {
      handlers: defaultHandlers,
    },
  },
};

export default meta;
type Story = StoryObj;

export const Default: Story = {
  parameters: {
    msw: { handlers: defaultHandlers },
  },
  render: () => createTableElement('/api/users', simpleColumns, { pageSize: 10 }),
  play: async ({ canvasElement }) => {
    const table = canvasElement.querySelector('client-data-table') as any;

    // Wait for data to load
    await waitFor(() => expect(table.data.length).toBeGreaterThan(0), { timeout: 3000 });

    // Verify 5 rows loaded from mock
    await expect(table.data.length).toBe(5);

    // Verify table is rendered in shadow DOM
    const shadowRoot = table.shadowRoot!;
    const tableElement = shadowRoot.querySelector('table');
    await expect(tableElement).toBeInTheDocument();

    // Verify correct number of data rows (excluding header)
    const rows = shadowRoot.querySelectorAll('tbody tr');
    await expect(rows.length).toBe(5);

    // Verify first row contains expected data
    const firstRow = rows[0];
    await expect(firstRow.textContent).toContain('John Doe');
    await expect(firstRow.textContent).toContain('john@example.com');
  },
};

export const Loading: Story = {
  name: 'Loading State',
  parameters: {
    docs: {
      description: {
        story: 'Shows the loading spinner while data is being fetched (3 second delay).',
      },
    },
    msw: {
      handlers: [
        http.get('/api/users-slow', async () => {
          await delay(3000);
          return HttpResponse.json(mockSimpleData);
        }),
      ],
    },
  },
  render: () => createTableElement('/api/users-slow', simpleColumns),
};

export const Empty: Story = {
  name: 'Empty State',
  parameters: {
    docs: {
      description: {
        story: 'Shows the empty message when no data is returned.',
      },
    },
    msw: {
      handlers: [
        http.get('/api/users-empty', async () => {
          await delay(200);
          return HttpResponse.json([]);
        }),
      ],
    },
  },
  render: () => createTableElement('/api/users-empty', simpleColumns, {
    emptyMessage: 'No users found. Try adding some!'
  }),
  play: async ({ canvasElement }) => {
    const table = canvasElement.querySelector('client-data-table') as any;

    // Wait for data to load (empty array)
    await waitFor(() => expect(table.data).toBeDefined(), { timeout: 3000 });

    // Verify empty data
    await expect(table.data.length).toBe(0);

    // Verify empty message is displayed in shadow DOM
    const shadowRoot = table.shadowRoot!;
    await waitFor(() => {
      const emptyText = shadowRoot.textContent;
      return expect(emptyText).toContain('No users found');
    }, { timeout: 3000 });
  },
};

export const Error: Story = {
  name: 'Error State',
  parameters: {
    docs: {
      description: {
        story: 'Shows error message when API request fails.',
      },
    },
    msw: {
      handlers: [
        http.get('/api/users-error', async () => {
          await delay(200);
          return HttpResponse.json({ error: 'Server error' }, { status: 500 });
        }),
      ],
    },
  },
  render: () => createTableElement('/api/users-error', simpleColumns),
};

export const WithPagination: Story = {
  name: 'With Pagination',
  parameters: {
    docs: {
      description: {
        story: 'Demonstrates pagination with a larger dataset (100 rows, 10 per page).',
      },
    },
    msw: {
      handlers: [
        http.get('/api/users-many', async () => {
          await delay(200);
          return HttpResponse.json(mockLargeData);
        }),
      ],
    },
  },
  render: () => createTableElement('/api/users-many', simpleColumns, { pageSize: 10 }),
  play: async ({ canvasElement }) => {
    const table = canvasElement.querySelector('client-data-table') as any;

    // Wait for data to load (100 rows)
    await waitFor(() => expect(table.data.length).toBe(100), { timeout: 3000 });

    const shadowRoot = table.shadowRoot!;

    // Verify we're on page 1 initially
    await expect(table.currentPage).toBe(1);

    // Verify only 10 rows are displayed (page size)
    const initialRows = shadowRoot.querySelectorAll('tbody tr');
    await expect(initialRows.length).toBe(10);

    // Verify first row is "User 1"
    await expect(initialRows[0].textContent).toContain('User 1');

    // Find and click the "Next" button (use XPath to find button with text)
    const buttons = Array.from(shadowRoot.querySelectorAll('button'));
    const nextButton = buttons.find(b => b.textContent?.includes('Next'));
    if (nextButton) {
      await userEvent.click(nextButton);
    }

    // Wait for page to update
    await waitFor(() => expect(table.currentPage).toBe(2));

    // Verify we're now on page 2 with different data
    const page2Rows = shadowRoot.querySelectorAll('tbody tr');
    await expect(page2Rows.length).toBe(10);
    await expect(page2Rows[0].textContent).toContain('User 11');

    // Find and click page number "3"
    const allButtons = Array.from(shadowRoot.querySelectorAll('button'));
    const page3Button = allButtons.find(b => b.textContent?.trim() === '3');
    if (page3Button) {
      await userEvent.click(page3Button);
    }

    // Verify we're on page 3
    await waitFor(() => expect(table.currentPage).toBe(3));
    const page3Rows = shadowRoot.querySelectorAll('tbody tr');
    await expect(page3Rows[0].textContent).toContain('User 21');
  },
};

export const CustomRenderers: Story = {
  name: 'Custom Cell Renderers',
  parameters: {
    docs: {
      description: {
        story: 'Columns can have custom renderers for complex cell content.',
      },
    },
    msw: {
      handlers: [
        http.get('/api/countries-table', async () => {
          await delay(200);
          return HttpResponse.json(mockCountries);
        }),
      ],
    },
  },
  render: () => createTableElement('/api/countries-table', countryColumns, { pageSize: 10 }),
};

export const Filtering: Story = {
  name: 'Filtering Demonstration',
  parameters: {
    docs: {
      description: {
        story: 'Try typing in the search box to filter rows. Search for "Admin", "user10", or an email.',
      },
    },
    msw: {
      handlers: [
        http.get('/api/users-filter', async () => {
          await delay(100);
          return HttpResponse.json(mockLargeData.slice(0, 30));
        }),
      ],
    },
  },
  render: () => {
    const container = document.createElement('div');
    container.innerHTML = `
      <p style="margin-bottom: 1rem; color: #6b7280;">
        💡 Try searching for "Admin", "user10", or an email address
      </p>
    `;
    container.appendChild(createTableElement('/api/users-filter', simpleColumns, { pageSize: 10 }));
    return container;
  },
  play: async ({ canvasElement }) => {
    const table = canvasElement.querySelector('client-data-table') as any;

    // Wait for data to load (30 rows from mock)
    await waitFor(() => expect(table.data.length).toBe(30), { timeout: 3000 });

    const shadowRoot = table.shadowRoot!;

    // Verify initial page shows 10 rows (page size)
    const initialRows = shadowRoot.querySelectorAll('tbody tr');
    await expect(initialRows.length).toBe(10);

    // Find the search input (type="text" with class "search-input")
    const searchInput = shadowRoot.querySelector('.search-input') as HTMLInputElement;
    await expect(searchInput).toBeTruthy();

    // Type "Admin" in the search box
    await userEvent.type(searchInput, 'Admin');

    // Wait for filtering to take effect (debounced)
    await waitFor(() => {
      const visibleRows = shadowRoot.querySelectorAll('tbody tr');
      // Should show Admin role users only (10 users with Admin role from 30 total)
      return expect(visibleRows.length).toBeGreaterThan(0) && expect(visibleRows.length).toBeLessThanOrEqual(10);
    }, { timeout: 3000 });

    // Verify filtered rows contain "Admin"
    const filteredRows = shadowRoot.querySelectorAll('tbody tr');
    const firstRow = filteredRows[0];
    await expect(firstRow.textContent).toContain('Admin');
  },
};

export const ColumnAlignment: Story = {
  name: 'Column Alignment',
  parameters: {
    docs: {
      description: {
        story: 'Different text alignments: left (default), center, and right.',
      },
    },
    msw: {
      handlers: [
        http.get('/api/products', async () => {
          await delay(100);
          return HttpResponse.json(mockProducts);
        }),
      ],
    },
  },
  render: () => createTableElement('/api/products', productColumns),
};
