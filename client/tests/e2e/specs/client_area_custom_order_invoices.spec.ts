// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { clientAreaSwitchOrganization, expect, MockBms, msTest } from '@tests/e2e/helpers';

msTest('Test all orgs', async ({ clientAreaCustomOrder }) => {
  const title = clientAreaCustomOrder.locator('.header-content').locator('.header-title');

  await clientAreaCustomOrder.locator('.menu-client').locator('.menu-client-list').getByRole('listitem').nth(3).click();
  await expect(title).toHaveText('Invoices');
  const content = clientAreaCustomOrder.locator('.client-page-invoices');
  const invoices = content.locator('.invoices-year-content-list-item');
  await expect(invoices).toHaveCount(30);
  await expect(content.locator('.no-invoices')).toBeHidden();
  for (const invoice of await invoices.all()) {
    await expect(invoice.locator('.invoices-organization')).toHaveText(/^BlackMesa(-2)?$/);
    await expect(invoice.locator('.invoices-amount')).toHaveText(/^€\d+\.\d{2}$/);
    await expect(invoice.locator('.badge-status')).toHaveText(/^To pay|Paid|Void|In progress|Uncollectible$/);
  }
});

msTest('Test only one org', async ({ clientAreaCustomOrder }) => {
  const title = clientAreaCustomOrder.locator('.header-content').locator('.header-title');

  await clientAreaSwitchOrganization(clientAreaCustomOrder, clientAreaCustomOrder.orgInfo.name);

  await clientAreaCustomOrder.locator('.menu-client').locator('.menu-client-list').getByRole('listitem').nth(3).click();
  await expect(title).toHaveText('Invoices');
  const content = clientAreaCustomOrder.locator('.client-page-invoices');
  const invoices = content.locator('.invoices-year-content-list-item');
  await expect(invoices).toHaveCount(15);
  await expect(content.locator('.no-invoices')).toBeHidden();
  for (const invoice of await invoices.all()) {
    await expect(invoice.locator('.invoices-organization')).toHaveText('BlackMesa');
    await expect(invoice.locator('.invoices-amount')).toHaveText(/^€\d+\.\d{2}$/);
    await expect(invoice.locator('.badge-status')).toHaveText(/^To pay|Paid|Void|In progress|Uncollectible$/);
  }
});

msTest('Test filter date', async ({ clientAreaCustomOrder }) => {
  const title = clientAreaCustomOrder.locator('.header-content').locator('.header-title');

  await clientAreaCustomOrder.locator('.menu-client').locator('.menu-client-list').getByRole('listitem').nth(3).click();
  await expect(title).toHaveText('Invoices');

  const containers = clientAreaCustomOrder.locator('.invoices-year:visible');
  await expect(containers).toHaveCount(2);
  await expect(containers.locator('.invoices-year-text')).toHaveText(['2025', '2024']);

  const yearFilterButton = clientAreaCustomOrder.locator('.invoices-header-filter').locator('.invoices-header-filter-button').nth(0);
  const monthFilterButton = clientAreaCustomOrder.locator('.invoices-header-filter').locator('.invoices-header-filter-button').nth(1);
  const popover = clientAreaCustomOrder.locator('.time-filter-popover');
  await expect(popover).toBeHidden();

  await yearFilterButton.click();
  await expect(popover).toBeVisible();
  await expect(popover.locator('.time-list-item')).toHaveText(['2025', '2024']);

  await popover.locator('.time-list-item').nth(1).click();
  await popover.locator('ion-backdrop').click();
  await expect(popover).toBeHidden();

  await expect(containers).toHaveCount(1);
  await expect(containers.locator('.invoices-year-text')).toHaveText('2024');
  await expect(containers.locator('.invoices-year-content-list-item:visible')).toHaveCount(24);

  await monthFilterButton.click();
  await expect(popover).toBeVisible();
  await expect(popover.locator('.time-list-item')).toHaveText([
    'Jan',
    'Feb',
    'Mar',
    'Apr',
    'May',
    'Jun',
    'Jul',
    'Aug',
    'Sep',
    'Oct',
    'Nov',
    'Dec',
  ]);
  await popover.locator('.time-list-item').nth(1).click();
  await popover.locator('.time-list-item').nth(3).click();
  await popover.locator('.time-list-item').nth(4).click();
  await popover.locator('.time-list-item').nth(10).click();
  await popover.locator('ion-backdrop').click();
  await expect(popover).toBeHidden();
  await expect(containers.locator('.invoices-year-content-list-item:visible')).toHaveCount(8);
});

msTest('Test no invoices', async ({ clientAreaCustomOrder }) => {
  await MockBms.mockGetCustomOrderInvoices(clientAreaCustomOrder, {}, { empty: true });
  const title = clientAreaCustomOrder.locator('.header-content').locator('.header-title');

  await clientAreaCustomOrder.locator('.menu-client').locator('.menu-client-list').getByRole('listitem').nth(3).click();
  await expect(title).toHaveText('Invoices');
  const content = clientAreaCustomOrder.locator('.client-page-invoices');
  await expect(content.locator('.invoices-year-content-list-item')).toHaveCount(0);
  await expect(content.locator('.no-invoices')).toBeVisible();
  await expect(content.locator('.no-invoices')).toHaveText("You don't have any invoice yet.");
});

for (const orgMode of ['oneOrg', 'allOrgs']) {
  msTest(`List the invoices for ${orgMode} generic error`, async ({ clientAreaCustomOrder }) => {
    await MockBms.mockGetCustomOrderInvoices(clientAreaCustomOrder, { POST: { errors: { status: 400 } } });

    if (orgMode === 'orgOrg') {
      await clientAreaSwitchOrganization(clientAreaCustomOrder, clientAreaCustomOrder.orgInfo.name);
    }

    const title = clientAreaCustomOrder.locator('.header-content').locator('.header-title');
    await clientAreaCustomOrder.locator('.menu-client').locator('.menu-client-list').getByRole('listitem').nth(3).click();
    await expect(title).toHaveText('Invoices');
    const content = clientAreaCustomOrder.locator('.client-page-invoices');
    await expect(content.locator('.no-invoices')).toBeHidden();
    await expect(content.locator('.invoices-year-content-list-item')).toHaveCount(0);
    await expect(clientAreaCustomOrder.locator('.main-content').locator('.form-error')).toHaveText('Could not retrieve your invoices.');
    await expect(clientAreaCustomOrder.locator('.main-content').locator('.no-invoices')).toBeHidden();
  });

  msTest(`List the invoices for ${orgMode} timeout`, async ({ clientAreaCustomOrder }) => {
    await MockBms.mockGetCustomOrderInvoices(clientAreaCustomOrder, { POST: { timeout: true } });

    if (orgMode === 'orgOrg') {
      await clientAreaSwitchOrganization(clientAreaCustomOrder, clientAreaCustomOrder.orgInfo.name);
    }

    const title = clientAreaCustomOrder.locator('.header-content').locator('.header-title');
    await clientAreaCustomOrder.locator('.menu-client').locator('.menu-client-list').getByRole('listitem').nth(3).click();
    await expect(title).toHaveText('Invoices');
    const content = clientAreaCustomOrder.locator('.client-page-invoices');
    await expect(content.locator('.no-invoices')).toBeHidden();
    await expect(content.locator('.invoices-year-content-list-item')).toHaveCount(0);
    await expect(clientAreaCustomOrder.locator('.main-content').locator('.form-error')).toHaveText('Could not retrieve your invoices.');
    await expect(clientAreaCustomOrder.locator('.main-content').locator('.no-invoices')).toBeHidden();
  });
}
