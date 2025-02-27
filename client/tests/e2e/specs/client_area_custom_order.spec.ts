// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { expect, msTest } from '@tests/e2e/helpers';

msTest('Test initial status', async ({ clientAreaCustomOrder }) => {
  const title = clientAreaCustomOrder.locator('.header-content').locator('.header-title');
  await expect(title).toHaveText('Contract');

  const pages = [
    { button: 'Contract', title: 'Contract', url: 'contracts' },
    { button: 'Statistics', title: 'Statistics', url: 'custom-order-statistics' },
    { button: 'Orders', title: 'Orders', url: 'orders' },
    { button: 'Billing details', title: 'Billing details', url: 'custom-order-billing-details' },
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
