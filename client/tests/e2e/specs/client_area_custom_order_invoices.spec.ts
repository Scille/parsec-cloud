// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { clientAreaSwitchOrganization, DEFAULT_ORGANIZATION_INFORMATION, expect, msTest } from '@tests/e2e/helpers';

msTest('Test all orgs', async ({ clientAreaCustomOrder }) => {
  const title = clientAreaCustomOrder.locator('.header-content').locator('.header-title');

  await clientAreaCustomOrder.locator('.menu-client').locator('.menu-client-list').getByRole('listitem').nth(3).click();
  await expect(title).toHaveText('Invoices');
  const content = clientAreaCustomOrder.locator('.client-page-invoices');
  const invoices = content.locator('.invoices-year-content-list-item');
  await expect(invoices).toHaveCount(24);
});

msTest('Test only one org', async ({ clientAreaCustomOrder }) => {
  const title = clientAreaCustomOrder.locator('.header-content').locator('.header-title');

  await clientAreaSwitchOrganization(clientAreaCustomOrder, DEFAULT_ORGANIZATION_INFORMATION.name);

  await clientAreaCustomOrder.locator('.menu-client').locator('.menu-client-list').getByRole('listitem').nth(3).click();
  await expect(title).toHaveText('Invoices');
  const content = clientAreaCustomOrder.locator('.client-page-invoices');
  const invoices = content.locator('.invoices-year-content-list-item');
  await expect(invoices).toHaveCount(12);
});
