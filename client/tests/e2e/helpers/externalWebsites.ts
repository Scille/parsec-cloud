// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { expect, Locator } from '@playwright/test';
import { MsContext, MsPage } from '@tests/e2e/helpers/types';

const EXTERNAL_URLS = [
  'parsec.cloud',
  'raw.githubusercontent.com',
  'github.com',
  'docs.parsec.cloud',
  'sign-dev.parsec.cloud',
  'sign.parsec.cloud',
  'www.proconnect.gouv.fr',
];

export async function mockExternalWebsites(context: MsContext): Promise<void> {
  for (const host of EXTERNAL_URLS) {
    await context.route(`**://${host}/**`, async (route) => {
      await route.fulfill({
        status: 200,
        contentType: 'text/html',
        body: `
<!DOCTYPE html>
<html>
<head>
    <title>MOCKED</title>
</head>
<body>
    <div>${route.request().url()}</div>
</body>
</html>
`,
      });
    });
  }
}

export async function openExternalLink(page: MsPage, clickable: Locator, expectedUrl: RegExp): Promise<void> {
  const newTabPromise = page.waitForEvent('popup');
  await clickable.click();
  const newTab = await newTabPromise;
  await expect(newTab).toHaveURL(expectedUrl);
  await expect(newTab).toHaveTitle('MOCKED');
  await expect(newTab.locator('div')).toHaveText(expectedUrl);
  await newTab.close();
}
