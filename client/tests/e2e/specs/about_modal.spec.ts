// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { Locator } from '@playwright/test';
import { expect, MsPage, msTest } from '@tests/e2e/helpers';

async function openLink(page: MsPage, button: Locator, expectedUrl: RegExp): Promise<void> {
  const newTabPromise = page.waitForEvent('popup');
  await button.click();
  const newTab = await newTabPromise;
  await expect(newTab).toHaveURL(expectedUrl);
  await newTab.close();
}

msTest('Opens the about dialog', async ({ home }) => {
  await home.locator('#trigger-version-button').click();

  const modal = home.locator('.about-modal');
  await expect(modal).toBeVisible();
  await expect(modal.locator('.ms-modal-header__title')).toContainText('About');
  const titles = modal.locator('.app-info-key');
  const values = modal.locator('.app-info-value');
  await expect(titles).toHaveText(['Version', 'Developer', 'License', 'Project']);
  await expect(values).toHaveText([/^ v[a-z0-9-.+]+$/, 'Parsec Cloud', 'BUSL-1.1', ' GitHub ']);

  await openLink(home, values.nth(1), /^https:\/\/parsec\.cloud\/en\/?.+$/);
  await openLink(home, values.nth(2), /^https:\/\/raw\.githubusercontent\.com\/Scille\/parsec-cloud\/.+\/LICENSE.*$/);
  await openLink(home, values.nth(3), /^https:\/\/github\.com\/Scille\/parsec-cloud$/);
  await openLink(home, modal.locator('.changelog-btn'), /^https:\/\/docs\.parsec\.cloud\/.+$/);

  await modal.locator('.closeBtn').click();
  await expect(modal).toBeHidden();
});
