// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { expect, MsPage, msTest, setupNewPage } from '@tests/e2e/helpers';

const SIMPLE_LOGO = `<?xml version="1.0" encoding="UTF-8" standalone="no"?>
<svg xmlns="http://www.w3.org/2000/svg" width="500" height="500">
<circle cx="250" cy="250" r="210" fill="#fff" stroke="#000" stroke-width="8"/>
</svg>`;

msTest('Use custom branding', async ({ context }) => {
  const page = (await context.newPage()) as MsPage;

  page.route('**/custom/custom_en-US.json', async (route, request) => {
    if (request.method().toUpperCase() === 'GET') {
      await route.fulfill({
        status: 200,
        json: {
          HomePage: {
            topbar: {
              welcome: 'App with Custom Branding',
            },
          },
        },
      });
    } else {
      await route.fulfill({
        status: 404,
      });
    }
  });

  page.route('**/custom/logo.svg', async (route, request) => {
    if (request.method().toUpperCase() === 'GET') {
      await route.fulfill({
        status: 200,
        contentType: 'image/svg+xml',
        body: Buffer.from(SIMPLE_LOGO, 'utf-8'),
      });
    } else {
      route.fulfill({
        status: 404,
      });
    }
  });

  await setupNewPage(page, { withCustomBranding: true });

  await expect(page.locator('.homepage-content').locator('.topbar-left-text__title')).toHaveText('App with Custom Branding');
  const logo = await page.locator('.sidebar-container').locator('.logo-img').locator('svg').innerHTML();
  expect(logo).toBe('\n<circle cx="250" cy="250" r="210" fill="#fff" stroke="#000" stroke-width="8"></circle>\n');
  await expect(page.locator('.sidebar-container').locator('.sidebar-bottom').locator('.sidebar-tagline')).toHaveText('Powered by');
  await expect(page.locator('.sidebar-container').locator('.sidebar-bottom').locator('.logo-icon')).toBeVisible();

  const newTabPromise = page.waitForEvent('popup');
  await page.locator('.sidebar-container').locator('.sidebar-bottom').locator('.logo-icon').click();
  const newTab = await newTabPromise;
  await expect(newTab).toHaveURL(new RegExp('^https://parsec\\.cloud/.+$'));
  await newTab.close();
  await page.release();
});
