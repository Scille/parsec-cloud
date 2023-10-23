// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { test, expect, Page } from '@playwright/test';

async function newTestbed(page: Page, template?: 'coolorg' | 'empty') {
  // const TESTBED_SERVER_URL = process.env.TESTBED_SERVER_URL;
  const TESTBED_SERVER_URL = 'parsec://127.0.0.1:6770?no_ssl=true';
  if (TESTBED_SERVER_URL === undefined) {
      throw new Error('Environ variable `TESTBED_SERVER_URL` must be defined to use testbed');
  }

  // `page.evaluate` runs inside the web page, hence why we pass a function with
  // parameters instead of a closure.
  return await page.evaluate(async ([template, testbed_server_url]) => {
    console.log("window.nextStageHook", template, testbed_server_url);
      // Next part of the mess to initialize the app, should be replaced by the init script
      const [libparsec, nextStage] = window.nextStageHook();

      const configPath = await libparsec.testNewTestbed(template, testbed_server_url);
      // assert.isDefined(configPath);

      // Final part of the mess to initialize the app
      // Force locale to en-US
      await nextStage(configPath, 'en-US');

      return configPath;
  }, [template, TESTBED_SERVER_URL]);
}

test('Opens the settings dialog', async ({ page }) => {
    // TODO: this init script would be very handy to avoid the big mess currently
    // in `main.ts` to configure the application in test mode ^^
    await page.addInitScript(() => {
      // Needed by `main.ts`
      window.Cypress = "dummy";
    });

    // Output all console logs from the webpage \o/
    page.on('console', msg => console.log('> ', msg.text()));

    // Actual start of the test

    await page.goto('/home');  // Default url is configured in `playwright.config.ts`

    const configPath = await newTestbed(page, 'coolorg');

    // Opens the settings dialog
    await page.locator('.sidebar-footer__version').click();

    await expect(page.locator('.about-modal')).toBeVisible();
    await expect(page.locator('.about-modal').locator('.title-h2')).toContainText('About');
});
