import { defineConfig } from '@playwright/test';

export default defineConfig({
  testDir: './e2e',
  fullyParallel: false,
  forbidOnly: !!process.env.CI,
  retries: 0,
  workers: 1,
  reporter: 'list',
  use: {
    baseURL: 'http://localhost:5173',
    trace: 'on-first-retry',
  },
  webServer: [
    {
      command: 'cd .. && DATABASE_URL=sqlite:e2e_test.db cargo run',
      url: 'http://localhost:3000/api/auth/me',
      reuseExistingServer: !process.env.CI,
      timeout: 120_000,
      // The backend returns 401 for /api/auth/me when not logged in, which is expected
      ignoreHTTPSErrors: true,
    },
    {
      command: 'npm run dev',
      url: 'http://localhost:5173',
      reuseExistingServer: !process.env.CI,
      timeout: 30_000,
    },
  ],
});
