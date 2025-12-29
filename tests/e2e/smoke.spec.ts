import { test, expect } from '@playwright/test';

/**
 * E2E Smoke Tests
 *
 * Purpose: Catch critical integration issues only
 * Scope: Login + page navigation + console errors
 *
 * DO NOT add complex scenarios here - those belong in:
 * - Storybook play functions (component interactions)
 * - Backend route tests (business logic)
 */

test.describe('Authentication', () => {
  test('login page loads', async ({ page }) => {
    await page.goto('/auth/login');
    await expect(page).toHaveURL(/.*login/);
    await expect(page.locator('h1').first()).toContainText(/sign in/i);
    await expect(page.locator('input[name="email"]')).toBeVisible();
    await expect(page.locator('input[name="password"]')).toBeVisible();
    await expect(page.locator('button[type="submit"]')).toBeVisible();
  });

  test('login flow works with valid credentials', async ({ page }) => {
    await page.goto('/auth/login');

    // Fill in login form (adjust credentials as needed for test user)
    await page.fill('input[name="email"]', 'admin@example.com');
    await page.fill('input[name="password"]', 'admin');
    await page.click('button[type="submit"]');

    // Should redirect to dashboard
    await expect(page).toHaveURL('http://localhost:8080/');
    await expect(page.locator('h1').first()).toContainText(/dashboard/i);
  });

  test('logout works', async ({ page }) => {
    // Login first
    await page.goto('/auth/login');
    await page.fill('input[name="email"]', 'admin@example.com');
    await page.fill('input[name="password"]', 'admin');
    await page.click('button[type="submit"]');
    await expect(page).toHaveURL('http://localhost:8080/');

    // Find and click logout (adjust selector based on your UI)
    await page.click('text=Logout');
    await expect(page).toHaveURL(/.*login/);
  });
});

test.describe('Main Pages Load', () => {
  test.beforeEach(async ({ page }) => {
    // Login before each test
    await page.goto('/auth/login');
    await page.fill('input[name="email"]', 'admin@example.com');
    await page.fill('input[name="password"]', 'admin');
    await page.click('button[type="submit"]');
    await expect(page).toHaveURL('http://localhost:8080/');
  });

  test('dashboard loads successfully', async ({ page }) => {
    await expect(page.locator('h1').first()).toContainText(/dashboard/i);
    await expect(page).toHaveURL('http://localhost:8080/');
  });

  test('teams page loads and displays table', async ({ page }) => {
    await page.goto('/teams');
    await expect(page.locator('h1').first()).toContainText(/teams/i);

    // Wait for table to load (adjust selector based on your UI)
    await expect(page.locator('table')).toBeVisible();
  });

  test('players page loads and displays table', async ({ page }) => {
    await page.goto('/players');
    await expect(page.locator('h1').first()).toContainText(/players/i);
    await expect(page.locator('table')).toBeVisible();
  });

  test('events page loads', async ({ page }) => {
    await page.goto('/events');
    await expect(page.locator('h1').first()).toContainText(/events/i);
  });

  test('seasons page loads', async ({ page }) => {
    await page.goto('/seasons');
    await expect(page.locator('h1').first()).toContainText(/seasons/i);
  });

  test('matches page loads', async ({ page }) => {
    await page.goto('/matches');
    await expect(page.locator('h1').first()).toContainText(/matches/i);
  });

  test('management page loads', async ({ page }) => {
    await page.goto('/management');
    await expect(page.locator('h1').first()).toContainText(/management/i);
  });
});

test.describe('Console Errors', () => {
  test('no JavaScript errors on any page', async ({ page }) => {
    const errors: string[] = [];

    // Capture console errors
    page.on('console', (msg) => {
      if (msg.type() === 'error') {
        errors.push(msg.text());
      }
    });

    // Login
    await page.goto('/auth/login');
    await page.fill('input[name="email"]', 'admin@example.com');
    await page.fill('input[name="password"]', 'admin');
    await page.click('button[type="submit"]');
    await page.waitForURL('http://localhost:8080/');

    // Visit all main pages
    const pages = ['/teams', '/players', '/events', '/seasons', '/matches', '/management'];

    for (const path of pages) {
      await page.goto(`http://localhost:8080${path}`);
      await page.waitForLoadState('networkidle');
    }

    // Assert no errors
    expect(errors).toHaveLength(0);
  });
});
