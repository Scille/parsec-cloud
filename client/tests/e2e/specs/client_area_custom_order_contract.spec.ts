// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { clientAreaNavigateTo, clientAreaSwitchOrganization, expect, MockBms, msTest } from '@tests/e2e/helpers';

msTest('Test initial status for an org', async ({ clientAreaCustomOrder }) => {
  const title = clientAreaCustomOrder.locator('.header-content').locator('.header-title');
  await expect(title).toHaveText('Orders');

  await clientAreaSwitchOrganization(clientAreaCustomOrder, 'BlackMesa');
  await clientAreaNavigateTo(clientAreaCustomOrder, 'Contract');

  const container = clientAreaCustomOrder.locator('.client-contract-page');
  const error = container.locator('.form-error');
  await expect(error).toBeHidden();

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
  await expect(contract.locator('.item-content-date__date').nth(0)).toHaveText('Jan 3, 2024');
  await expect(contract.locator('.item-content-date__date').nth(1)).toHaveText('Feb 3, 2024');

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

msTest('Test initial status for all orgs', async ({ clientAreaCustomOrder }) => {
  const orgSelector = clientAreaCustomOrder.locator('.sidebar-header').locator('.organization-card-header').locator('.card-header-title');
  await expect(orgSelector).toHaveText('All organizations');

  const title = clientAreaCustomOrder.locator('.header-content').locator('.header-title');
  await expect(title).toHaveText('Orders');
  await clientAreaNavigateTo(clientAreaCustomOrder, 'Contract');

  const container = clientAreaCustomOrder.locator('.client-contract-page');
  const error = container.locator('.form-error');
  await expect(error).toBeHidden();

  const orgChoice = container.locator('.organization-choice-title');
  await expect(orgChoice).toBeVisible();
  await expect(orgChoice).toHaveText(
    'You have multiple organizations. Please select one from the top-left button or select it below in order to check the contract.',
  );

  await expect(container.locator('.organization-list')).toBeVisible();
  const orgs = container.locator('.organization-list').locator('.organization-list-item');
  await expect(orgs).toHaveText(['BlackMesa', 'BlackMesa-2']);
  await orgs.nth(1).click();
  await expect(orgChoice).toBeHidden();
  await expect(container.locator('.organization-list')).toBeHidden();
  await expect(orgSelector).toHaveText('BlackMesa-2');
});

msTest('Custom order contract stats generic error', async ({ clientAreaCustomOrder }) => {
  await MockBms.mockOrganizationStats(clientAreaCustomOrder, {}, { GET: { errors: { status: 400 } } });

  await clientAreaSwitchOrganization(clientAreaCustomOrder, 'BlackMesa');
  await clientAreaNavigateTo(clientAreaCustomOrder, 'Contract');

  const container = clientAreaCustomOrder.locator('.client-contract-page');
  const error = container.locator('.form-error');
  await expect(error).toBeVisible();
  await expect(error).toHaveText('Failed to retrieve the information');
});

msTest('Custom order contract stats timeout error', async ({ clientAreaCustomOrder }) => {
  await MockBms.mockOrganizationStats(clientAreaCustomOrder, {}, { GET: { timeout: true } });

  await clientAreaSwitchOrganization(clientAreaCustomOrder, 'BlackMesa');
  await clientAreaNavigateTo(clientAreaCustomOrder, 'Contract');

  const container = clientAreaCustomOrder.locator('.client-contract-page');
  const error = container.locator('.form-error');
  await expect(error).toBeVisible();
  await expect(error).toHaveText('Failed to retrieve the information');
});

msTest('Custom order contract details generic error', async ({ clientAreaCustomOrder }) => {
  await MockBms.mockCustomOrderDetails(clientAreaCustomOrder, {}, { POST: { errors: { status: 400 } } });

  await clientAreaSwitchOrganization(clientAreaCustomOrder, 'BlackMesa');
  await clientAreaNavigateTo(clientAreaCustomOrder, 'Contract');

  const container = clientAreaCustomOrder.locator('.client-contract-page');
  const error = container.locator('.form-error');
  await expect(error).toBeVisible();
  await expect(error).toHaveText('Failed to retrieve the information');
});

msTest('Custom order contract details timeout error', async ({ clientAreaCustomOrder }) => {
  await MockBms.mockCustomOrderDetails(clientAreaCustomOrder, {}, { POST: { timeout: true } });

  await clientAreaSwitchOrganization(clientAreaCustomOrder, 'BlackMesa');
  await clientAreaNavigateTo(clientAreaCustomOrder, 'Contract');

  const container = clientAreaCustomOrder.locator('.client-contract-page');
  const error = container.locator('.form-error');
  await expect(error).toBeVisible();
  await expect(error).toHaveText('Failed to retrieve the information');
});
