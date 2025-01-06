// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { DEFAULT_ORGANIZATION_INFORMATION, expect, msTest } from '@tests/e2e/helpers';

msTest('Test dashboard', async ({ clientArea }) => {
  const title = clientArea.locator('.header-content').locator('.header-title');
  await expect(title).toHaveText('Dashboard');
  const monthSummaryItems = clientArea.locator('.month-summary').locator('.month-summary-item');
  await expect(monthSummaryItems.locator('.month-summary-item__title')).toHaveText(['Active users', 'Used storage']);
  await expect(monthSummaryItems.locator('.month-summary-item__data')).toHaveText(['59', '373 GB']);

  const invoiceContainer = clientArea.locator('.invoices-container');
  await expect(invoiceContainer.locator('.skeleton-loading').nth(0)).toBeHidden();
  const invoices = invoiceContainer.locator('.invoices-list-item');
  await expect(invoices).toHaveCount(3);
  await expect(invoices.nth(0).locator('.invoices-list-item__data')).toHaveText([
    'Dec 1, 2021',
    DEFAULT_ORGANIZATION_INFORMATION.name,
    /^€[\d.,]+$/,
    /^(Paid|In progress|To pay)\s*Download$/,
  ]);
  await expect(invoices.nth(1).locator('.invoices-list-item__data')).toHaveText([
    'Nov 1, 2021',
    DEFAULT_ORGANIZATION_INFORMATION.name,
    /^€[\d.,]+$/,
    /^(Paid|In progress|To pay)\s*Download$/,
  ]);
  await expect(invoices.nth(2).locator('.invoices-list-item__data')).toHaveText([
    'Oct 1, 2021',
    DEFAULT_ORGANIZATION_INFORMATION.name,
    /^€[\d.,]+$/,
    /^(Paid|In progress|To pay)\s*Download$/,
  ]);
  const paymentButton = clientArea.locator('.payment-container').locator('.custom-button');
  await expect(paymentButton).toHaveText('Update');
  await paymentButton.click();
  await expect(title).toHaveText('Payment methods');
});
