import { defineConfig, devices } from '@playwright/test';

/**
 * Playwright configuration for E2E smoke tests
 *
 * These tests focus on critical user flows only:
 * - Login/logout works
 * - All main pages load without errors
 * - No JavaScript console errors
 *
 * Complex scenarios should be tested in Storybook or backend tests.
 */
export default defineConfig({
  testDir: './tests/e2e',
  fullyParallel: true,
  forbidOnly: !!process.env.CI,
  retries: process.env.CI ? 2 : 0,
  workers: process.env.CI ? 1 : undefined,
  reporter: 'html',
  globalSetup: './tests/e2e/global-setup.ts',
  use: {
    baseURL: 'http://localhost:8080',
    trace: 'on-first-retry',
    screenshot: 'only-on-failure',
  },
  projects: [
    {
      name: 'chromium',
      use: { ...devices['Desktop Chrome'] },
    },
  ],
  // Start dev server before tests (disabled by default - start manually)
  // Uncomment for CI or automated runs
  // webServer: {
  //   command: 'cargo run',
  //   url: 'http://localhost:8080',
  //   reuseExistingServer: !process.env.CI,
  //   timeout: 120_000,
  // },
});
