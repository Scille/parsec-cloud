// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { expect, msTest } from '@tests/e2e/helpers';

msTest('User list default state', async ({ organizationPage }) => {
  const container = organizationPage.locator('.org-info-container');
  const configContainer = container.locator('.org-config');
  const usersContainer = container.locator('.org-user');

  await expect(configContainer.locator('.org-config-list-item').locator('.org-info-item-title')).toHaveText([
    'External profile',
    'User limit (excluding users with External profile)',
    'Server address',
  ]);
  await expect(configContainer.locator('.org-config-list-item').nth(0).locator('.org-config-list-item__value')).toHaveText(['Enabled']);
  await expect(configContainer.locator('.org-config-list-item').nth(1).locator('.org-config-list-item__value')).toHaveText(['Unlimited']);
  await expect(configContainer.locator('.org-config-list-item').nth(2).locator('.server-address-value__text')).toHaveText(
    'parsec3://example.com/MyOrg',
  );

  await expect(usersContainer.locator('.user-active-header').locator('.user-active-header__title')).toHaveText('Active');
  await expect(usersContainer.locator('.user-active-header').locator('.title-h4')).toHaveText('5');
  await expect(usersContainer.locator('.user-active-list').locator('.label-profile')).toHaveText(['Administrator', 'Member', 'External']);
  await expect(usersContainer.locator('.user-active-list').locator('.user-active-list-item__value')).toHaveText(['3', '3', '2']);
  await expect(usersContainer.locator('.user-revoked-header').locator('.user-revoked-header__title')).toHaveText('Revoked');
  await expect(usersContainer.locator('.user-revoked-header').locator('.title-h4')).toHaveText('3');
});
