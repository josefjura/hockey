import type { TestRunnerConfig } from '@storybook/test-runner';

const config: TestRunnerConfig = {
  async preVisit(page) {
    // Setup before each story test
    await page.emulateMedia({ colorScheme: 'light' });
  },
  async postVisit(page) {
    // Check for console errors after each story
    const logs = await page.evaluate(() => {
      return (window as any).__storybook_test_logs || [];
    });

    const errors = logs.filter((log: any) => log.level === 'error');
    if (errors.length > 0) {
      throw new Error(`Console errors in story: ${JSON.stringify(errors)}`);
    }
  },
};

export default config;
