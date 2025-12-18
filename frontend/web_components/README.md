# Web Components Documentation

This directory contains Lit-based web components for the hockey management application.

## Table of Contents

- [Client Data Table](#client-data-table)
- [Countries Table](#countries-table)
- [Other Components](#other-components)

---

## Client Data Table

A fully client-side data table component with filtering, sorting, and pagination.

### When to Use

**Use client-data-table when:**
- Dataset is **small to medium** (< 500 rows)
- Data changes infrequently
- Fast filtering/sorting without server round-trips is desired
- The API returns all data at once

**Use server-side HTMX tables when:**
- Dataset is **large** (> 500 rows)
- Real-time data updates
- Complex server-side filtering logic
- Bandwidth/performance constraints

### Features

✅ Client-side sorting (all columns sortable by default)
✅ Client-side filtering (debounced search)
✅ Client-side pagination with ellipsis
✅ Column configuration (sortable, filterable, alignment, width)
✅ Custom cell renderers (for actions, badges, etc.)
✅ Loading, error, and empty states
✅ Responsive design
✅ Type-safe with TypeScript

### Basic Usage

#### 1. Define Your Columns

```typescript
import { Column } from './shared/types.js';

interface Country {
  id: number;
  name: string;
  iso2Code: string;
  iihfMember: boolean;
}

const columns: Column<Country>[] = [
  {
    key: 'id',
    label: 'ID',
    width: '80px',
    align: 'right'
  },
  {
    key: 'name',
    label: 'Country Name',
    sortable: true,
    filterable: true
  },
  {
    key: 'iso2Code',
    label: 'ISO Code',
    width: '100px'
  },
  {
    key: 'iihfMember',
    label: 'IIHF',
    align: 'center',
    renderer: (value) => html`
      ${value ? '✓' : '✗'}
    `
  }
];
```

#### 2. Use in Maud Template (Rust)

```rust
use maud::{html, Markup};

pub fn countries_page() -> Markup {
    html! {
        div class="card" {
            h1 { "Countries" }

            // Use the component
            client-data-table
                api-endpoint="/api/countries"
                page-size="20"
                empty-message="No countries found" {}
        }
    }
}
```

#### 3. Create API Endpoint

The component expects a JSON array response:

```rust
// routes/api/countries.rs
pub async fn countries_api(
    State(state): State<AppState>,
) -> impl IntoResponse {
    let countries = countries::get_all_countries(&state.db)
        .await
        .unwrap_or_default();

    Json(countries)
}
```

### Advanced Usage

#### Custom Cell Renderers

```typescript
{
  key: 'actions',
  label: 'Actions',
  align: 'right',
  sortable: false,
  filterable: false,
  renderer: (_, row) => html`
    <button class="btn btn-sm" @click=${() => editCountry(row.id)}>
      Edit
    </button>
    <button class="btn btn-sm btn-danger" @click=${() => deleteCountry(row.id)}>
      Delete
    </button>
  `
}
```

#### Conditional Rendering

```typescript
{
  key: 'status',
  label: 'Status',
  renderer: (value, row) => {
    if (row.available) {
      return html`<span class="badge badge-success">Active</span>`;
    }
    return html`<span class="badge badge-secondary">Inactive</span>`;
  }
}
```

### Properties

| Property | Type | Default | Description |
|----------|------|---------|-------------|
| `api-endpoint` | string | required | API endpoint that returns JSON array |
| `page-size` | number | 20 | Rows per page |
| `empty-message` | string | "No data available" | Message when no results |
| `columns` | Column[] | [] | Column configuration (set via JS) |
| `filters` | Record<string, any> | undefined | Additional filters (set via JS) |

### Column Configuration

```typescript
interface Column<T> {
  key: string;              // Object property key
  label: string;            // Column header text
  sortable?: boolean;       // Enable sorting (default: true)
  filterable?: boolean;     // Include in search (default: true)
  width?: string;           // CSS width (e.g., "100px", "20%")
  align?: 'left' | 'center' | 'right';  // Text alignment
  renderer?: (value: any, row: T) => TemplateResult;  // Custom cell renderer
}
```

### Performance Considerations

- **Optimal**: < 100 rows - instant filtering/sorting
- **Good**: 100-300 rows - smooth performance
- **Acceptable**: 300-500 rows - slight lag on older devices
- **Not recommended**: > 500 rows - use server-side tables

The component includes built-in performance monitoring. When loading > 100 rows,
check browser console (debug level) for timing metrics:

```
[client-data-table] Loaded 250 rows in 45.23ms, filtered/sorted in 2.15ms
```

#### Performance Testing

To test with large datasets, you can create a test endpoint:

```rust
// For testing only - generates synthetic data
pub async fn test_large_dataset() -> impl IntoResponse {
    let data: Vec<_> = (1..=500)
        .map(|i| json!({
            "id": i,
            "name": format!("Item {}", i),
            "value": i * 10,
            "status": if i % 2 == 0 { "active" } else { "inactive" }
        }))
        .collect();
    Json(data)
}
```

Typical performance on modern hardware (2020+ laptop):
- 100 rows: ~20ms load, <1ms filter/sort
- 250 rows: ~45ms load, ~2ms filter/sort
- 500 rows: ~85ms load, ~5ms filter/sort

### Events

The component dispatches custom events:

```typescript
// When data is loaded
element.addEventListener('data-loaded', (e) => {
  console.log('Total records:', e.detail.total);
});
```

### Styling

The component uses CSS custom properties for theming:

```css
client-data-table {
  --gray-50: #f9fafb;
  --gray-200: #e5e7eb;
  --gray-300: #d1d5db;
  --gray-400: #9ca3af;
  --gray-500: #6b7280;
  --gray-600: #4b5563;
  --gray-700: #374151;
  --gray-900: #111827;
}
```

---

## Countries Table

A specialized implementation of `client-data-table` for the countries management page.

### Usage

```rust
countries-table
    api-endpoint="/api/countries"
    page-size="20" {}
```

This component:
- Pre-configures columns for countries data
- Adds custom renderers for flags, IIHF badges, and actions
- Includes toggle switches for availability
- Handles edit/delete actions with HTMX

See `countries-table.ts` for implementation details.

---

## Other Components

### Country Selector
Searchable dropdown for selecting countries.

### Flag Icon
Displays country flags with fallback handling.

### Toggle Switch
Styled checkbox for boolean values.

### Badge
Status badges with color variants.

---

## Development

### Building Components

```bash
cd web_components
npm install
npm run build
```

Components are compiled to `static/js/components/`.

### Adding a New Component

1. Create `my-component.ts` in `web_components/`
2. Implement using Lit:
   ```typescript
   import { LitElement, html, css } from 'lit';
   import { customElement, property } from 'lit/decorators.js';

   @customElement('my-component')
   export class MyComponent extends LitElement {
     @property({ type: String })
     label = '';

     render() {
       return html`<div>${this.label}</div>`;
     }
   }
   ```
3. Build: `npm run build`
4. Import in layout.rs:
   ```rust
   script type="module" src="/static/js/components/my-component.js" {}
   ```

### Testing

```bash
npm test
```

---

## Best Practices

1. **Choose the right table type**:
   - Small datasets (< 500 rows): `client-data-table`
   - Large datasets (> 500 rows): Server-side HTMX tables

2. **Column configuration**:
   - Disable sorting for action columns: `sortable: false`
   - Disable filtering for non-text columns: `filterable: false`
   - Use custom renderers for complex data

3. **Performance**:
   - Keep row count under 500 for client-side tables
   - Use pagination (20-50 rows per page recommended)
   - Minimize custom renderer complexity

4. **Accessibility**:
   - Provide meaningful `empty-message` text
   - Use semantic HTML in custom renderers
   - Ensure buttons have clear labels

5. **Error handling**:
   - Always provide error states in API endpoints
   - Component shows error UI with retry button
   - Log errors for debugging
