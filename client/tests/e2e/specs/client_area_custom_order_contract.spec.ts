// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { expect, msTest } from '@tests/e2e/helpers';
import { DateTime } from 'luxon';

msTest('Test initial status', async ({ clientAreaCustomOrder }) => {
  const title = clientAreaCustomOrder.locator('.header-content').locator('.header-title');
  await expect(title).toHaveText('Contract');

  const container = clientAreaCustomOrder.locator('.client-contract-page');

  await expect(container.locator('.ms-error').nth(0)).toBeVisible();
  await expect(container.locator('.ms-error').nth(0)).toHaveText(
    'You have exceeded the number of members specified in your contract. Please consider deleting members.',
  );

  await expect(container.locator('.ms-error').nth(1)).toBeVisible();
  await expect(container.locator('.ms-error').nth(1)).toHaveText(
    'You have exceeded the storage capacity of your contract. Remember to delete files.',
  );

  await expect(container.locator('.contract-header-title__text')).toHaveText('Contract n°FACT001');

  const contract = container.locator('.contract-main');
  await expect(contract.locator('.item-content-date__date').nth(0)).toHaveText('Apr 7, 1988');
  await expect(contract.locator('.item-content-date__date').nth(1)).toHaveText(DateTime.now().plus({ year: 1 }).toFormat('LLL d, yyyy'));

  await expect(contract.locator('.data-number').nth(0)).toHaveText('32');
  await expect(contract.locator('.data-number').nth(1)).toHaveText('50');
  await expect(contract.locator('.data-number').nth(2)).toHaveText('100');

  await expect(contract.locator('.data-number').nth(3)).toHaveText('2 or 200 GB');

  const org = container.locator('.organization-users');
  const admins = org.locator('.admins');
  await expect(admins.locator('.item-active__number')).toHaveText('4 / 32 members');
  await expect(admins.locator('.progress-text')).toHaveText('28 remaining');
  const standards = org.locator('.members');
  await expect(standards.locator('.item-active__number')).toHaveText('54 / 50 members');
  await expect(standards.locator('.progress-text')).toHaveText('0 remaining');
  const externals = org.locator('.externals');
  await expect(externals.locator('.item-active__number')).toHaveText('1 / 100 members');
  await expect(externals.locator('.progress-text')).toHaveText('99 remaining');

  const storage = container.locator('.organization-storage');
  await expect(storage.locator('.item-active__number')).toHaveText('373 GB / 200 GB');
  await expect(storage.locator('.progress-text')).toHaveText('186% used');
});
