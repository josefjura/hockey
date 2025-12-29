# Testing Guide

This document describes the hybrid testing strategy for the hockey management application, covering backend (Rust), frontend components (Lit), and end-to-end testing.

## Quick Start

```bash
# Run all tests
make test-all

# Run specific test suites
make test              # Rust tests only
make test-storybook    # Storybook component tests
make test-e2e          # E2E smoke tests (requires server on :8080)

# Individual commands
cargo test                              # Rust backend
cd web_components && yarn test-storybook  # Storybook
yarn test:e2e                           # E2E
```

## Test Pyramid

```
      ╱ ╲
     ╱E2E╲      ~12 tests  - Critical user flows only
    ╱─────╲
   ╱ Story ╲    ~14+ tests - Component interactions
  ╱ book   ╲
 ╱───────────╲
╱  Backend    ╲  ~100 tests - Service layer + routes
───────────────
```

### Philosophy

- **Backend tests** (Rust): Comprehensive coverage of business logic, data access, and route handlers
- **Component tests** (Storybook): Interaction testing for Lit components with MSW mocking
- **E2E tests** (Playwright): Minimal smoke tests for critical paths (login + page navigation)

## Backend Testing (Rust)

### Service Layer Tests

Service tests use `#[sqlx::test]` for automatic database setup/teardown with in-memory SQLite.

**Pattern:**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[sqlx::test(migrations = "./migrations")]
    async fn test_get_countries_no_filters(pool: SqlitePool) {
        let filters = CountryFilters::default();
        let countries = get_countries(&pool, &filters).await.unwrap();

        assert!(countries.len() > 200);
        assert!(countries.iter().any(|c| c.name == "Canada"));
    }

    #[sqlx::test(migrations = "./migrations", fixtures("users"))]
    async fn test_with_fixture_data(pool: SqlitePool) {
        // Fixtures from tests/fixtures/*.sql are loaded automatically
        let users = get_users(&pool).await.unwrap();
        assert!(!users.is_empty());
    }
}
```

**Key Features:**
- Automatic database creation with migrations
- Each test gets its own isolated database
- Fixtures in `tests/fixtures/*.sql` for test data
- No manual setup/teardown needed

**Reference Implementation:**
- `/home/josef/source/hockey/src/service/countries.rs:150-277` (10 tests)

**Remaining Work:**
- See issues #148 (session tests), #149 (route tests), #150 (service tests)

### Route Handler Tests

Route tests use `axum-test` for integration testing of HTTP endpoints.

**Pattern (TODO - see issue #149):**
```rust
use axum_test::TestServer;
use crate::test_utils::*;

#[sqlx::test(migrations = "./migrations")]
async fn test_teams_list_page(pool: SqlitePool) {
    let app = create_test_app(pool.clone());
    let server = TestServer::new(app).unwrap();
    let session = create_test_session(&pool).await;

    let response = server
        .get("/teams")
        .add_cookie(session_cookie(&session))
        .await;

    response.assert_status_ok();
    response.assert_text_contains("<h1>Teams</h1>");
}

#[sqlx::test(migrations = "./migrations")]
async fn test_teams_htmx_partial(pool: SqlitePool) {
    let app = create_test_app(pool.clone());
    let server = TestServer::new(app).unwrap();
    let session = create_test_session(&pool).await;

    let response = server
        .get("/teams/list")
        .add_cookie(session_cookie(&session))
        .add_header("HX-Request", "true")
        .await;

    response.assert_status_ok();
    // HTMX partials should NOT include full HTML layout
    assert!(!response.text().contains("<html>"));
}
```

**Test Utilities:**
- `src/test_utils.rs` - Shared utilities for route testing
- `create_test_app()` - Create app instance
- `create_test_session()` - Create authenticated session
- `session_cookie()` - Generate session cookie

**What to Test:**
- Full page renders correctly
- HTMX partials return fragments (no layout)
- Authentication required for protected routes
- Form submissions and validation
- Error handling

### Running Backend Tests

```bash
# All tests
cargo test

# Specific test
cargo test test_get_countries_no_filters

# With output
cargo test -- --nocapture

# Single-threaded (for debugging)
cargo test -- --test-threads=1
```

## Component Testing (Storybook Test-Runner)

Storybook test-runner uses Playwright to execute `play` functions in stories, enabling automated interaction testing.

### Why Storybook Instead of Vitest?

- ✅ Leverages existing 68 Storybook stories
- ✅ Documentation + testing in one place
- ✅ Enforces discipline to add new components to Storybook
- ✅ Uses Playwright (same as E2E) - no additional tools
- ✅ MSW mocking already configured
- ✅ Tests components in isolation
- ✅ Visual regression testing potential (future)

### Writing Play Functions

**Pattern:**
```typescript
import { expect, userEvent, within, waitFor } from '@storybook/test';

export const InteractiveStory: Story = {
  render: () => html`<my-component></my-component>`,
  play: async ({ canvasElement }) => {
    // 1. Get component and shadow root
    const component = canvasElement.querySelector('my-component');
    const shadowRoot = component.shadowRoot!;

    // 2. Wait for async data/state
    await waitFor(() => expect(component.data).toBeDefined(), { timeout: 3000 });

    // 3. Test user interactions
    const button = within(shadowRoot).getByText('Click Me');
    await userEvent.click(button);

    // 4. Verify state changes
    await waitFor(() => expect(component.isOpen).toBe(true));

    // 5. Verify DOM updates
    const dialog = within(shadowRoot).getByRole('dialog');
    await expect(dialog).toBeVisible();
  },
};
```

**Key Principles:**
- **Shadow DOM querying**: Use `within(shadowRoot)` or `shadowRoot.querySelector()`
- **Async operations**: Always use `waitFor()` for state changes
- **User simulation**: Use `userEvent` for realistic interactions
- **Appropriate timeouts**: 1000ms for debounced inputs, 6000ms for auto-dismiss
- **MSW mocking**: API calls mocked via MSW handlers in story parameters

**Reference Implementations:**
- `stories/CountrySelector.stories.ts:102-217` (4 play functions)
- `stories/ClientDataTable.stories.ts:195-447` (4 play functions)
- `stories/ConfirmDialog.stories.ts:102-271` (3 play functions)
- `stories/Toast.stories.ts:94-266` (3 play functions)

### Testing Checklist

For each interactive component:
- [ ] Data loading and rendering
- [ ] User interactions (clicks, typing, selections)
- [ ] Form validation (if applicable)
- [ ] Event emission (if applicable)
- [ ] State changes and UI updates
- [ ] Empty/error states (if applicable)

### Running Storybook Tests

```bash
# Development (requires Storybook running on :6006)
cd web_components
yarn storybook          # Terminal 1
yarn test-storybook     # Terminal 2

# Watch mode
yarn test-storybook --watch

# CI mode (builds and tests)
yarn test-storybook:ci
```

### Test-Runner Configuration

`.storybook/test-runner.ts` provides pre/post visit hooks:
- `preVisit`: Setup (e.g., color scheme emulation)
- `postVisit`: Verification (e.g., console error checking)

**Remaining Work:**
- See issue #151 for adding play functions to remaining ~54 stories

## E2E Smoke Tests (Playwright)

Minimal smoke tests focusing on critical user flows only. Complex scenarios should be tested in Storybook or backend tests.

### Test Scope

**DO test:**
- ✅ Login/logout works
- ✅ All main pages load without errors
- ✅ No JavaScript console errors

**DON'T test:**
- ❌ Complex form interactions (test in Storybook)
- ❌ Business logic validation (test in backend)
- ❌ Component behavior (test in Storybook)
- ❌ Data correctness (test in service layer)

### Writing E2E Tests

**Pattern:**
```typescript
import { test, expect } from '@playwright/test';

test.describe('Main Pages', () => {
  test.beforeEach(async ({ page }) => {
    // Login before each test
    await page.goto('/auth/login');
    await page.fill('input[name="email"]', 'admin@example.com');
    await page.fill('input[name="password"]', 'admin');
    await page.click('button[type="submit"]');
    await expect(page).toHaveURL('http://localhost:8080/');
  });

  test('teams page loads', async ({ page }) => {
    await page.goto('/teams');
    await expect(page.locator('h1')).toContainText(/teams/i);
    await expect(page.locator('table')).toBeVisible();
  });
});
```

**Reference Implementation:**
- `tests/e2e/smoke.spec.ts` (12 tests)

### Running E2E Tests

```bash
# Start server first (Terminal 1)
cargo run

# Run tests (Terminal 2)
yarn test:e2e

# UI mode (interactive)
yarn test:e2e:ui

# Debug mode
yarn test:e2e:debug

# Headed mode (see browser)
npx playwright test --headed
```

### Configuration

`playwright.config.ts` settings:
- **baseURL**: `http://localhost:8080`
- **Browsers**: Chromium (can add Firefox/WebKit)
- **Retries**: 2 in CI, 0 locally
- **Workers**: 1 in CI, auto locally
- **Artifacts**: Screenshots on failure, traces on retry

## CI/CD Integration

### GitHub Actions

Tests run automatically on push/PR to master:

1. **Rust tests** - Service layer + route handlers
2. **Storybook tests** - Component interaction tests
3. **E2E tests** - Smoke tests with server running

Artifacts uploaded:
- Playwright HTML report (30-day retention)

See `.github/workflows/ci.yml:74-114` for full configuration.

### Local Pre-commit

```bash
# Run all pre-commit checks
make precommit

# This runs:
# 1. cargo fmt --check
# 2. cargo clippy
# 3. cargo test
```

**Note**: `precommit` currently only runs Rust tests. To run all tests:
```bash
make test-all
```

## Test Coverage

### Current Status

- ✅ Backend: 10 service tests (countries.rs), test utilities created
- ✅ Storybook: 14 play functions across 4 components
- ✅ E2E: 12 smoke tests
- ⏳ Backend: Route tests pending (issue #149)
- ⏳ Backend: Remaining service tests pending (issue #150)
- ⏳ Backend: Session test refactoring pending (issue #148)
- ⏳ Storybook: ~54 stories need play functions (issue #151)

### Target

- **~100+ tests** across the stack
- Service layer: ~50 tests
- Route handlers: ~40 tests
- Storybook play functions: ~70 tests (68 stories)
- E2E smoke tests: ~12 tests

## Best Practices

### General

- **AAA Pattern**: Arrange → Act → Assert
- **Independence**: Tests should not depend on each other
- **Clarity**: Test names should describe what they test
- **Speed**: Keep tests fast (< 5 minutes total in CI)

### Backend Tests

- Use `#[sqlx::test]` for database tests
- Prefer fixtures over manual data creation
- Test both success and error paths
- Verify HTMX partials don't include layout

### Component Tests

- Test user perspective, not implementation details
- Wait for state changes with `waitFor()`
- Use semantic queries (`getByRole`, `getByText`)
- Test Shadow DOM with `within(shadowRoot)`

### E2E Tests

- Keep minimal - only critical paths
- Use `beforeEach` for common setup (login)
- Capture console errors
- Don't test business logic

## Troubleshooting

### Rust Tests

**Database locked error:**
```bash
# Run tests single-threaded
cargo test -- --test-threads=1
```

**sqlx::test not found:**
```bash
# Ensure sqlx is in dev-dependencies with macros feature
# See Cargo.toml
```

### Storybook Tests

**Port 6006 already in use:**
```bash
# Kill existing Storybook
pkill -f storybook
```

**Play function errors:**
- Check Shadow DOM queries use `within(shadowRoot)`
- Verify `waitFor()` timeout is sufficient
- Use `--debug` flag to see browser interactions

### E2E Tests

**Server not running:**
```bash
# Start server first
cargo run
```

**Tests timing out:**
- Increase timeout in `playwright.config.ts`
- Check server is responding on `:8080`
- Verify database migrations ran

**Browser not installed:**
```bash
npx playwright install chromium
```

## Resources

- [sqlx-test docs](https://docs.rs/sqlx/latest/sqlx/attr.test.html)
- [axum-test docs](https://docs.rs/axum-test/)
- [Storybook test-runner](https://github.com/storybookjs/test-runner)
- [@storybook/test docs](https://storybook.js.org/docs/writing-tests)
- [Playwright docs](https://playwright.dev/)

## Contributing

When adding new features:

1. **Add service tests** for data access logic
2. **Add route tests** for HTTP endpoints
3. **Add component to Storybook** with play functions
4. **Update E2E tests** only if adding critical new user flow

Keep test pyramid balanced: many backend tests, some component tests, few E2E tests.
