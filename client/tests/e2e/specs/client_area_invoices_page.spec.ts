// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { MockBms, expect, msTest } from '@tests/e2e/helpers';

msTest('List the invoices', async ({ clientArea }) => {
  await MockBms.mockGetInvoices(clientArea);

  const title = clientArea.locator('.header-content').locator('.header-title');
  await clientArea.locator('.menu-client').locator('.menu-client-list').getByRole('listitem').nth(2).click();
  await expect(title).toHaveText('Invoices');
  const containers = clientArea.locator('.invoices-year');
  await expect(containers).toHaveCount(3);
  await expect(containers.locator('.invoices-year-text')).toHaveText(['2021', '2020', '2019']);
  await expect(containers.nth(0).locator('.invoices-year-content-list-item')).toHaveCount(12);
  await expect(containers.nth(1).locator('.invoices-year-content-list-item')).toHaveCount(12);
  await expect(containers.nth(2).locator('.invoices-year-content-list-item')).toHaveCount(12);
  await expect(containers.nth(0).locator('.invoices-year-content-list-item').nth(0).locator('ion-text')).toHaveText([
    'Dec 2021',
    '2021-12',
    clientArea.orgInfo.name,
    /^€[\d.,]+$/,
    /^(Paid|In progress|To pay)\s*Download$/,
  ]);
  const yearFilterButton = clientArea.locator('.invoices-header-filter').locator('.invoices-header-filter-button').nth(0);
  const monthFilterButton = clientArea.locator('.invoices-header-filter').locator('.invoices-header-filter-button').nth(1);
  const popover = clientArea.locator('.time-filter-popover');
  await expect(popover).toBeHidden();
  await yearFilterButton.click();
  await expect(popover).toBeVisible();
  await expect(popover.locator('.time-list-item')).toHaveText(['2021', '2020', '2019']);
  await popover.locator('ion-backdrop').click();
  await expect(popover).toBeHidden();

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
  await popover.locator('ion-backdrop').click();
  await expect(popover).toBeHidden();
});

msTest('List the invoices generic error', async ({ clientArea }) => {
  await MockBms.mockGetInvoices(clientArea, {}, { GET: { errors: { status: 400 } } });

  const title = clientArea.locator('.header-content').locator('.header-title');
  await clientArea.locator('.menu-client').locator('.menu-client-list').getByRole('listitem').nth(2).click();
  await expect(title).toHaveText('Invoices');
  await expect(clientArea.locator('.invoices-year:visible')).toHaveCount(0);
  await expect(clientArea.locator('.main-content').locator('.form-error')).toHaveText('Could not retrieve your invoices.');
  await expect(clientArea.locator('.main-content').locator('.no-invoices')).toBeHidden();
});

msTest('List the invoices timeout', async ({ clientArea }) => {
  await MockBms.mockGetInvoices(clientArea, {}, { GET: { timeout: true } });

  const title = clientArea.locator('.header-content').locator('.header-title');
  await clientArea.locator('.menu-client').locator('.menu-client-list').getByRole('listitem').nth(2).click();
  await expect(title).toHaveText('Invoices');
  await expect(clientArea.locator('.invoices-year:visible')).toHaveCount(0);
  await expect(clientArea.locator('.main-content').locator('.form-error')).toHaveText('Could not retrieve your invoices.');
  await expect(clientArea.locator('.main-content').locator('.no-invoices')).toBeHidden();
});

msTest('Empty invoice list', async ({ clientArea }) => {
  await MockBms.mockGetInvoices(clientArea, { count: 0 });
  const title = clientArea.locator('.header-content').locator('.header-title');
  await clientArea.locator('.menu-client').locator('.menu-client-list').getByRole('listitem').nth(2).click();
  await expect(title).toHaveText('Invoices');
  await expect(clientArea.locator('.invoices-year:visible')).toHaveCount(0);
  await expect(clientArea.locator('.main-content').locator('.form-error')).toBeHidden();
  await expect(clientArea.locator('.main-content').locator('.no-invoices')).toBeVisible();
  await expect(clientArea.locator('.main-content').locator('.no-invoices')).toHaveText('You do not have any invoice yet.');
});

msTest('Filter the invoices', async ({ clientArea }) => {
  await MockBms.mockGetInvoices(clientArea);

  const title = clientArea.locator('.header-content').locator('.header-title');
  await clientArea.locator('.menu-client').locator('.menu-client-list').getByRole('listitem').nth(2).click();
  await expect(title).toHaveText('Invoices');
  const containers = clientArea.locator('.invoices-year:visible');
  await expect(containers).toHaveCount(3);
  await expect(containers.nth(0).locator('.invoices-year-content-list-item')).toHaveCount(12);
  const yearFilterButton = clientArea.locator('.invoices-header-filter').locator('.invoices-header-filter-button').nth(0);
  const monthFilterButton = clientArea.locator('.invoices-header-filter').locator('.invoices-header-filter-button').nth(1);
  const popover = clientArea.locator('.time-filter-popover');
  await expect(popover).toBeHidden();

  const choices = clientArea.locator('.selected-choice').locator('.selected-choice-item');
  await expect(choices).toHaveCount(0);

  // Filter by years
  await yearFilterButton.click();
  await expect(popover).toBeVisible();
  const years = popover.locator('.time-list-item');
  await years.nth(1).click();
  await expect(choices).toHaveText('2020');
  await expect(years.nth(1)).toHaveTheClass('selected');
  await expect(containers).toHaveCount(1);
  await expect(containers.locator('.invoices-year-text')).toHaveText('2020');
  await expect(containers.locator('.invoices-year-content-list-item')).toHaveCount(12);
  // Add a second year
  await years.nth(2).click();
  await expect(choices).toHaveText(['2020', '2019']);
  await expect(years.nth(2)).toHaveTheClass('selected');
  await expect(containers).toHaveCount(2);
  await expect(containers.locator('.invoices-year-text')).toHaveText(['2020', '2019']);
  await expect(containers.nth(0).locator('.invoices-year-content-list-item')).toHaveCount(12);
  await expect(containers.nth(1).locator('.invoices-year-content-list-item')).toHaveCount(12);
  // Remove the first year
  await years.nth(1).click();
  await expect(choices).toHaveText(['2019']);
  await expect(years.nth(1)).not.toHaveTheClass('selected');
  await expect(containers).toHaveCount(1);
  await expect(containers.locator('.invoices-year-text')).toHaveText('2019');
  await expect(containers.locator('.invoices-year-content-list-item')).toHaveCount(12);

  await popover.locator('ion-backdrop').click();
  await expect(popover).toBeHidden();

  // Add months filtering
  await monthFilterButton.click();
  await expect(popover).toBeVisible();
  const months = popover.locator('.time-list-item');
  await months.nth(1).click();
  await expect(choices).toHaveText(['2019', 'Feb']);
  await expect(months.nth(1)).toHaveTheClass('selected');
  await expect(containers).toHaveCount(1);
  await expect(containers.locator('.invoices-year-content-list-item:visible')).toHaveCount(1);
  await expect(containers.locator('.invoices-year-content-list-item:visible').locator('ion-text').nth(0)).toHaveText('Feb 2019');

  // Add a second month
  await months.nth(3).click();
  await expect(choices).toHaveText(['2019', 'Feb', 'Apr']);
  await expect(months.nth(3)).toHaveTheClass('selected');
  await expect(containers).toHaveCount(1);
  await expect(containers.locator('.invoices-year-content-list-item:visible')).toHaveCount(2);
  await expect(containers.locator('.invoices-year-content-list-item:visible').nth(0).locator('ion-text').nth(0)).toHaveText('Apr 2019');
  await expect(containers.locator('.invoices-year-content-list-item:visible').nth(1).locator('ion-text').nth(0)).toHaveText('Feb 2019');

  // Remove the first month
  await years.nth(1).click();
  await expect(choices).toHaveText(['2019', 'Apr']);
  await expect(years.nth(1)).not.toHaveTheClass('selected');
  await expect(containers).toHaveCount(1);
  await expect(containers.locator('.invoices-year-content-list-item:visible')).toHaveCount(1);
  await expect(containers.locator('.invoices-year-content-list-item:visible').locator('ion-text').nth(0)).toHaveText('Apr 2019');

  await popover.locator('ion-backdrop').click();
  await expect(popover).toBeHidden();

  // Remove year
  await choices.nth(0).locator('.selected-choice-item__icon').click();
  await expect(choices).toHaveText('Apr');
  await expect(containers).toHaveCount(3);
  await expect(containers.nth(0).locator('.invoices-year-content-list-item:visible')).toHaveCount(1);
  await expect(containers.nth(1).locator('.invoices-year-content-list-item:visible')).toHaveCount(1);
  await expect(containers.nth(2).locator('.invoices-year-content-list-item:visible')).toHaveCount(1);

  // Remove last filter
  await choices.nth(0).locator('.selected-choice-item__icon').click();
  await expect(choices).toHaveCount(0);
  await expect(containers).toHaveCount(3);
  await expect(containers.nth(0).locator('.invoices-year-content-list-item:visible')).toHaveCount(12);
  await expect(containers.nth(1).locator('.invoices-year-content-list-item:visible')).toHaveCount(12);
  await expect(containers.nth(2).locator('.invoices-year-content-list-item:visible')).toHaveCount(12);
});
