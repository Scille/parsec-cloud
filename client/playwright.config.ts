// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { defineConfig, devices } from '@playwright/test';
import dotenv from 'dotenv';

/**
 * Read environment variables from file.
 * https://github.com/motdotla/dotenv
 */
dotenv.config({ path: '.env.playwright' });

const IN_CI = !!process.env.CI;

/**
 * See https://playwright.dev/docs/test-configuration.
 */
export default defineConfig({
  testDir: './tests/e2e',
  /* Run tests in files in parallel */
  fullyParallel: true,
  /* Fail the build on CI if you accidentally left test.only in the source code. */
  forbidOnly: IN_CI,
  /* Retry on CI only */
  retries: IN_CI ? 2 : 0,
  /* Opt out of parallel tests on CI. */
  workers: IN_CI ? 5 : '50%',
  /* Reporter to use. See https://playwright.dev/docs/test-reporters */
  reporter: IN_CI ? 'blob' : 'list',
  webServer: {
    command: 'npm run dev -- --port 8080',
    url: IN_CI ? 'http://localhost:8080' : 'https://localhost:8080',
    ignoreHTTPSErrors: IN_CI ? false : true,
    reuseExistingServer: true,
  },
  /* Shared settings for all the projects below. See https://playwright.dev/docs/api/class-testoptions. */
  use: {
    /* Base URL to use in actions like `await page.goto('/')`. */
    baseURL: IN_CI ? 'http://localhost:8080' : 'https://localhost:8080',

    /* Ignore HTTPS errors for self-signed certificates */
    ignoreHTTPSErrors: IN_CI ? false : true,

    /* Collect trace when retrying the failed test. See https://playwright.dev/docs/trace-viewer */
    trace: 'on-first-retry',
  },
  /* Leaving the empty array to make it easier to ignore tests */
  testIgnore: [],
  /* Configure projects for major browsers */
  projects: [
    {
      name: 'Google Chrome',
      use: {
        ...devices['Desktop Chrome'],
        // We use chrome over chromium because we need some media codec during e2e test for the file viewer feature.
        channel: 'chrome',
      },
    },

    // {
    //   name: 'firefox',
    //   use: { ...devices['Desktop Firefox'] },
    // },

    // {
    //   name: 'webkit',
    //   use: { ...devices['Desktop Safari'] },
    // },

    /* Test against mobile viewports. */
    // {
    //   name: 'Mobile Chrome',
    //   use: { ...devices['Pixel 5'] },
    // },
    // {
    //   name: 'Mobile Safari',
    //   use: { ...devices['iPhone 12'] },
    // },

    /* Test against branded browsers. */
    // {
    //   name: 'Microsoft Edge',
    //   use: { ...devices['Desktop Edge'], channel: 'msedge' },
    // },
    // {
    //   name: 'Google Chrome',
    //   use: { ...devices['Desktop Chrome'], channel: 'chrome' },
    // },
  ],
});
