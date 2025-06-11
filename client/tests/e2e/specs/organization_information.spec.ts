// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { answerQuestion, DisplaySize, expect, msTest } from '@tests/e2e/helpers';

for (const displaySize of [DisplaySize.Large, DisplaySize.Small]) {
  msTest(`Org info default state on ${displaySize} display`, async ({ organizationPage }) => {
    const container = organizationPage.locator('.organization-page-content');
    const configContainer = container.locator('.organization-info');
    const usersContainer = container.locator('.organization-users');
    await usersContainer.locator('#invitations-button').isVisible();

    if (displaySize === DisplaySize.Small) {
      await organizationPage.setDisplaySize(DisplaySize.Small);
      await expect(usersContainer.locator('.gradient-button-text').locator('.button-large')).toHaveText('pending invitation');
      await usersContainer.locator('.user-invite-button').isVisible();
      await usersContainer.locator('.card-header__button').click();
    }

    await expect(configContainer.locator('.info-list-item').locator('.info-list-item__title')).toHaveText([
      'External profile',
      'User limit (excluding users with External profile)',
      'Server address',
    ]);
    await expect(configContainer.locator('.info-list-item').nth(0).locator('.info-list-item__value')).toHaveText(['Enabled']);
    await expect(configContainer.locator('.info-list-item').nth(1).locator('.info-list-item__value')).toHaveText(['Unlimited']);
    await expect(configContainer.locator('.info-list-item').nth(2).locator('.server-address-value__text')).toHaveText(/^parsec3:\/\/.+$/);

    await expect(usersContainer.locator('.users-list-item').nth(0).locator('.users-list-item__title')).toHaveText('3');
    await expect(usersContainer.locator('.users-list-item').nth(0).locator('.users-list-item__description')).toHaveText('Active');
    await expect(usersContainer.locator('.users-list-item').nth(1).locator('.users-list-item__title')).toHaveText('0');
    await expect(usersContainer.locator('.users-list-item').nth(1).locator('.users-list-item__description')).toHaveText('Revoked');
    await expect(usersContainer.locator('.users-list-item').nth(2).locator('.users-list-item__title')).toHaveText('0');
    await expect(usersContainer.locator('.users-list-item').nth(2).locator('.users-list-item__description')).toHaveText('Frozen');
    await expect(usersContainer.locator('.user-active-list').locator('.label-profile')).toHaveText(['Administrator', 'Member', 'External']);
    await expect(usersContainer.locator('.user-active-list').locator('.user-active-list-item__value')).toHaveText(['1', '1', '1']);
  });

  msTest(`Org info after one revocation ${displaySize} display`, async ({ usersPage }) => {
    const sidebarButtons = usersPage
      .locator('.sidebar')
      .locator('.manage-organization')
      .locator('.list-sidebar-content')
      .getByRole('listitem');

    if (displaySize === DisplaySize.Small) {
      await usersPage.setDisplaySize(DisplaySize.Small);
      const userSmall = usersPage.locator('#users-page-user-list').getByRole('listitem').nth(1);
      await userSmall.hover();
      await userSmall.locator('.options-button').click();
      await usersPage.locator('.user-context-sheet-modal').getByRole('listitem').nth(0).click();
      await usersPage.locator('.question-modal').locator('ion-button').nth(1).click();
      await expect(usersPage).toShowToast('Boby McBobFace has been revoked. They can no longer access this organization.', 'Success');
    } else {
      const user = usersPage.locator('#users-page-user-list').getByRole('listitem').nth(1);
      await user.hover();
      await user.locator('.options-button').click();
      await usersPage.locator('.user-context-menu').getByRole('listitem').nth(1).click();
      await answerQuestion(usersPage, true);
      await expect(usersPage).toShowToast('Boby McBobFace has been revoked. They can no longer access this organization.', 'Success');
      await sidebarButtons.nth(1).click();
    }

    const container = usersPage.locator('.organization-page');
    const usersContainer = container.locator('.organization-users');
    if (displaySize === DisplaySize.Small) {
      await usersPage.locator('.tab-bar-menu').locator('.tab-bar-menu-button').nth(2).click();
    }
    await expect(usersContainer.locator('.users-list-item').nth(0).locator('.users-list-item__description')).toHaveText('Active');
    await expect(usersContainer.locator('.users-list-item').nth(0).locator('.users-list-item__title')).toHaveText('2');
    await expect(usersContainer.locator('.users-list-item').nth(1).locator('.users-list-item__description')).toHaveText('Revoked');
    await expect(usersContainer.locator('.users-list-item').nth(1).locator('.users-list-item__title')).toHaveText('1');
    await expect(usersContainer.locator('.user-active-list').locator('.label-profile')).toHaveText(['Administrator', 'Member', 'External']);
    await expect(usersContainer.locator('.user-active-list').locator('.user-active-list-item__value')).toHaveText(['1', '0', '1']);
  });
}
