// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { expect, msTest } from '@tests/e2e/helpers';

msTest('Org info default state', async ({ organizationPage }) => {
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
    /^parsec3:\/\/.+$/,
  );

  await expect(usersContainer.locator('.user-active-header').locator('.user-active-header__title')).toHaveText('Active');
  await expect(usersContainer.locator('.user-active-header').locator('.title-h4')).toHaveText('3');
  await expect(usersContainer.locator('.user-active-list').locator('.label-profile')).toHaveText(['Administrator', 'Member', 'External']);
  await expect(usersContainer.locator('.user-active-list').locator('.user-active-list-item__value')).toHaveText(['1', '1', '1']);
  await expect(usersContainer.locator('.user-revoked-header').locator('.user-revoked-header__title')).toHaveText('Revoked');
  await expect(usersContainer.locator('.user-revoked-header').locator('.title-h4')).toHaveText('0');
});

msTest('Org info after one revocation', async ({ usersPage }) => {
  const sidebarButtons = usersPage
    .locator('.sidebar')
    .locator('.manage-organization')
    .locator('.list-sidebar-content')
    .getByRole('listitem');
  await sidebarButtons.nth(0).click();

  const user = usersPage.locator('#users-page-user-list').getByRole('listitem').nth(1);
  await user.hover();
  await user.locator('.options-button').click();
  await usersPage.locator('.user-context-menu').getByRole('listitem').nth(1).click();
  await usersPage.locator('.question-modal').locator('ion-button').nth(2).click();

  await sidebarButtons.nth(1).click();
  const container = usersPage.locator('.org-info-container');
  const usersContainer = container.locator('.org-user');
  await expect(usersContainer.locator('.user-active-header').locator('.user-active-header__title')).toHaveText('Active');
  await expect(usersContainer.locator('.user-active-header').locator('.title-h4')).toHaveText('2');
  await expect(usersContainer.locator('.user-active-list').locator('.label-profile')).toHaveText(['Administrator', 'Member', 'External']);
  await expect(usersContainer.locator('.user-active-list').locator('.user-active-list-item__value')).toHaveText(['1', '0', '1']);
  await expect(usersContainer.locator('.user-revoked-header').locator('.user-revoked-header__title')).toHaveText('Revoked');
  await expect(usersContainer.locator('.user-revoked-header').locator('.title-h4')).toHaveText('1');
});
