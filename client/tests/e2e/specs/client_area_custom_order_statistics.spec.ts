// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { expect, msTest } from '@tests/e2e/helpers';

msTest('Test initial status', async ({ clientAreaCustomOrder }) => {
  const title = clientAreaCustomOrder.locator('.header-content').locator('.header-title');

  await clientAreaCustomOrder.locator('.menu-client').locator('.menu-client-list').getByRole('listitem').nth(1).click();
  await expect(title).toHaveText('Statistics');
  const page = clientAreaCustomOrder.locator('.client-page-statistics');
  const active = page.locator('.users-cards-list').nth(0);
  const activeUserItems = active.locator('.users-cards-list-item').locator('.users-cards-list-item-text');
  await expect(activeUserItems).toHaveText(['4Administrators', '54Members', '1External']);

  const revoked = page.locator('.users-cards-list').nth(1);
  const revokedUserItems = revoked.locator('.users-cards-list-item').locator('.users-cards-list-item-text');
  await expect(revokedUserItems).toHaveText(['1Administrator', '1Member', '142Externals']);

  const storage = page.locator('.storage-data');
  const storageGlobal = storage.locator('.storage-data-global');
  await expect(storageGlobal.locator('ion-text')).toHaveText(['373 GB', 'of which', '373 GBdata', '381 MBmetadata']);
  const consumption = storage.locator('.storage-data-consumption');
  await expect(consumption.locator('.ms-warning')).toBeVisible();
  await expect(consumption.locator('.ms-warning')).toHaveText('You have reached your storage limit.');
});
