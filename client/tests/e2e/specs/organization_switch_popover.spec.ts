// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { expect, msTest } from '@tests/e2e/helpers';

msTest('Open organization switch popover', async ({ connected }) => {
  const popoverButton = connected.locator('.sidebar').locator('.sidebar-header').locator('.organization-card-header-desktop');
  const popover = connected.locator('.popover-switch');

  await expect(popoverButton.locator('.organization-text')).toHaveText(/^Org\d+$/);
  await expect(popover).toBeHidden();
  await popoverButton.click();
  await expect(popover).toBeVisible();
  await expect(popover.locator('.current-organization').locator('.organization-name')).toHaveText(/^Org\d+$/);
});

msTest('Back to log in page', async ({ connected }) => {
  connected.locator('.sidebar').locator('.sidebar-header').locator('.organization-card-header-desktop').click();
  const popover = connected.locator('.popover-switch');
  const switchButton = popover.locator('.section-buttons');
  await expect(switchButton).toHaveText('Switch organization');
  await switchButton.click();
  await expect(connected).toBeHomePage();
});
