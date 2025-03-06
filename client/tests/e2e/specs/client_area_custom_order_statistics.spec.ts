// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { MockBms, clientAreaSwitchOrganization, expect, msTest } from '@tests/e2e/helpers';

msTest('Test initial status', async ({ clientAreaCustomOrder }) => {
  const title = clientAreaCustomOrder.locator('.header-content').locator('.header-title');

  await clientAreaSwitchOrganization(clientAreaCustomOrder, 'BlackMesa');

  await clientAreaCustomOrder.locator('.menu-client').locator('.menu-client-list').getByRole('listitem').nth(1).click();
  await expect(title).toHaveText('Statistics');
  const page = clientAreaCustomOrder.locator('.client-page-statistics');
  const error = page.locator('.statistics-error');
  await expect(error).toBeHidden();
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

msTest('Test initial status for all orgs', async ({ clientAreaCustomOrder }) => {
  const orgSelector = clientAreaCustomOrder.locator('.sidebar-header').locator('.organization-card-header').locator('.card-header-title');
  await expect(orgSelector).toHaveText('All organizations');
  await clientAreaCustomOrder.locator('.menu-client').locator('.menu-client-list').getByRole('listitem').nth(1).click();

  const title = clientAreaCustomOrder.locator('.header-content').locator('.header-title');
  await expect(title).toHaveText('Statistics');

  const container = clientAreaCustomOrder.locator('.client-page-statistics');
  const error = container.locator('.statistics-error');
  await expect(error).toBeHidden();

  const orgChoice = container.locator('.organization-choice-title');
  await expect(orgChoice).toBeVisible();
  // eslint-disable-next-line max-len
  await expect(orgChoice).toHaveText(
    'You have multiple organizations. Please select one from the top-left button or select it below in order to check its statistics.',
  );

  await expect(container.locator('.organization-list')).toBeVisible();
  const orgs = container.locator('.organization-list').locator('.organization-list-item');
  await expect(orgs).toHaveText(['BlackMesa', 'BlackMesa-2']);
  await orgs.nth(1).click();
  await expect(orgChoice).toBeHidden();
  await expect(container.locator('.organization-list')).toBeHidden();
  await expect(orgSelector).toHaveText('BlackMesa-2');
});

msTest('Custom order stats generic error', async ({ clientAreaCustomOrder }) => {
  await MockBms.mockOrganizationStats(clientAreaCustomOrder, {}, { GET: { errors: { status: 400 } } });

  await clientAreaSwitchOrganization(clientAreaCustomOrder, 'BlackMesa');
  await clientAreaCustomOrder.locator('.menu-client').locator('.menu-client-list').getByRole('listitem').nth(1).click();

  const container = clientAreaCustomOrder.locator('.client-page-statistics');
  const error = container.locator('.statistics-error');
  await expect(error).toBeVisible();
  await expect(error).toHaveText('Failed to retrieve organization data.');
});

msTest('Custom order stats timeout error', async ({ clientAreaCustomOrder }) => {
  await MockBms.mockOrganizationStats(clientAreaCustomOrder, {}, { GET: { timeout: true } });

  await clientAreaSwitchOrganization(clientAreaCustomOrder, 'BlackMesa');
  await clientAreaCustomOrder.locator('.menu-client').locator('.menu-client-list').getByRole('listitem').nth(1).click();

  const container = clientAreaCustomOrder.locator('.client-page-statistics');
  const error = container.locator('.statistics-error');
  await expect(error).toBeVisible();
  await expect(error).toHaveText('Failed to retrieve organization data.');
});
