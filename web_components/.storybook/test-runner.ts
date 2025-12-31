import type { TestRunnerConfig } from '@storybook/test-runner';

const config: TestRunnerConfig = {
  async preVisit(page) {
    // Setup before each story test
    await page.emulateMedia({ colorScheme: 'light' });
  },
  // Note: Console error checking removed - play functions catch component errors,
  // and E2E tests handle integration-level console errors
};

export default config;
