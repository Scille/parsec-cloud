// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { clientAreaSwitchOrganization, expect, MockBms, msTest } from '@tests/e2e/helpers';

msTest('Test initial status', async ({ clientAreaCustomOrder }) => {
  const title = clientAreaCustomOrder.locator('.topbar').locator('.topbar-left-text__title');
  await expect(title).toHaveText('Orders');

  const pages = [
    { button: 'Orders', title: 'Orders', url: 'orders' },
    { button: 'My profile', title: 'My profile', url: 'personal-data' },
    { button: 'Contract', title: 'Contract', url: 'contracts' },
    { button: 'Statistics', title: 'Statistics', url: 'custom-order-statistics' },
    { button: 'Invoices', title: 'Invoices', url: 'custom-order-invoices' },
  ];

  await expect(clientAreaCustomOrder.locator('.sidebar-header').locator('.card-header-title')).toBeVisible();
  const menuButtons = clientAreaCustomOrder.locator('.menu-client').locator('.menu-client-list').getByRole('listitem');
  const buttonTexts = pages.map((texts) => texts.button);
  await expect(menuButtons).toHaveText(buttonTexts);
  for (let i = 0; i < pages.length; i++) {
    await menuButtons.nth(i).click();
    await expect(title).toHaveText(pages[i].title);
    // eslint-disable-next-line max-len
    const urlMatch = `https?://[a-z:0-9.]+/clientArea\\?(?:organization=[a-f0-9-]+&)?(?:page=${pages[i].url})&?(?:organization=[a-f0-9-]+)?`;
    await expect(clientAreaCustomOrder).toHaveURL(new RegExp(urlMatch));
  }
});

msTest('Test sidebar goto org', async ({ clientAreaCustomOrder }) => {
  const title = clientAreaCustomOrder.locator('.topbar').locator('.topbar-left-text__title');
  await expect(title).toHaveText('Orders');
  const gotoButton = clientAreaCustomOrder.locator('.sidebar').locator('.organization-card-button');
  await expect(gotoButton).toBeHidden();
  await MockBms.mockOrganizationStatus(clientAreaCustomOrder, { isBootstrapped: false });
  await clientAreaSwitchOrganization(clientAreaCustomOrder, 'BlackMesa-2');
  await expect(gotoButton).toBeHidden();
  await MockBms.mockOrganizationStatus(clientAreaCustomOrder, { isBootstrapped: true });
  await clientAreaSwitchOrganization(clientAreaCustomOrder, 'BlackMesa');
  await expect(gotoButton).toBeVisible();
  await gotoButton.click();
  await expect(clientAreaCustomOrder).toBeHomePage();
  await expect(clientAreaCustomOrder).toHaveURL('/home?bmsOrganizationId=BlackMesa');
});
