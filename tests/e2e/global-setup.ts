import { execSync } from 'child_process';
import { FullConfig } from '@playwright/test';

/**
 * Global setup for E2E tests
 * Runs once before all tests to prepare the test environment
 */
async function globalSetup(config: FullConfig) {
  console.log('Setting up E2E test environment...');

  const dbUrl = process.env.DATABASE_URL || 'sqlite:./hockey.db';

  // Create/reset test database
  console.log(`Creating test database: ${dbUrl}`);
  execSync('sqlx database create', { stdio: 'inherit', env: { ...process.env, DATABASE_URL: dbUrl } });

  // Run migrations
  console.log('Running migrations...');
  execSync('sqlx migrate run', { stdio: 'inherit', env: { ...process.env, DATABASE_URL: dbUrl } });

  // Ensure test admin user exists with correct password
  console.log('Setting up test admin user...');
  execSync('cargo run --bin create_admin --quiet -- admin@example.com "Test Admin" admin --force', {
    stdio: 'pipe',
    env: { ...process.env, DATABASE_URL: dbUrl }
  });
  console.log('✅ Test admin user ready (admin@example.com / admin)');

  console.log('E2E test environment ready!');
}

export default globalSetup;
