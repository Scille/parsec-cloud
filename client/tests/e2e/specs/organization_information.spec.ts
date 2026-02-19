// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { answerQuestion, DisplaySize, expect, getClipboardText, msTest } from '@tests/e2e/helpers';

for (const displaySize of [DisplaySize.Large, DisplaySize.Small]) {
  msTest(`Org info default state on ${displaySize} display`, async ({ organizationPage }) => {
    const container = organizationPage.locator('.organization-page-content');
    const configContainer = container.locator('.organization-info');
    const usersContainer = container.locator('.organization-users');
    const inviteButton = usersContainer.locator('.user-invite-button');
    const storageContainer = container.locator('.organization-storage');
    const emailInvitationCard = usersContainer.locator('.invitation-card-list-item').nth(0);
    const linkRequestCard = usersContainer.locator('.invitation-card-list-item').nth(1);
    await usersContainer.locator('#invitations-button').isVisible();

    if (displaySize === DisplaySize.Small) {
      await organizationPage.setDisplaySize(DisplaySize.Small);
    }

    await expect(emailInvitationCard).toBeVisible();
    await expect(emailInvitationCard.locator('.invitation-card-list-item__title')).toContainText('Email invitations');
    await expect(emailInvitationCard.locator('.invitation-card-list-item__number')).toHaveText('1');
    await expect(linkRequestCard).toBeVisible();
    await expect(linkRequestCard.locator('.invitation-card-list-item__title')).toContainText('Link requests');
    await expect(linkRequestCard.locator('.invitation-card-list-item__number')).toHaveText('0');
    await expect(inviteButton).toBeVisible();

    if (displaySize === DisplaySize.Small) {
      await organizationPage.locator('.tab-bar-menu').locator('.tab-bar-menu-button').nth(2).click();
    }

    await expect(configContainer.locator('.info-list-item').locator('.info-list-item__title')).toHaveText([
      'External profile',
      'User limit (excluding users with External profile)',
      'Server address',
    ]);
    await expect(configContainer.locator('.info-list-item').nth(0).locator('.info-list-item__value')).toHaveText(['Enabled']);
    await expect(configContainer.locator('.info-list-item').nth(1).locator('.info-list-item__value')).toHaveText(['Unlimited']);
    await expect(configContainer.locator('.info-list-item').nth(2).locator('.server-address-value__text')).toHaveText(/^parsec3:\/\/.+$/);

    const userCategories = usersContainer.locator('.users-list-item');
    await expect(userCategories.nth(0).locator('.users-list-item__title')).toHaveText('3');
    await expect(userCategories.nth(0).locator('.users-list-item__description')).toHaveText('Active');
    await expect(userCategories.nth(1).locator('.users-list-item__title')).toHaveText('0');
    await expect(userCategories.nth(1).locator('.users-list-item__description')).toHaveText('Revoked');
    await expect(userCategories.nth(2).locator('.users-list-item__title')).toHaveText('0');
    await expect(userCategories.nth(2).locator('.users-list-item__description')).toHaveText('Frozen');
    await expect(usersContainer.locator('.user-active-list').locator('.label-profile')).toHaveText(['Administrator', 'Member', 'External']);
    await expect(usersContainer.locator('.user-active-list').locator('.user-active-list-item__value')).toHaveText(['1', '1', '1']);

    await expect(storageContainer.locator('.storage-list-item__value').nth(0)).toHaveText(/^\d+(\.\d{2})? (B|KB|MB)$/);
    await expect(storageContainer.locator('.storage-list-item__value').nth(1)).toHaveText(/^\d+(\.\d{2})? (B|KB|MB)$/);
  });

  msTest(`Org info default state on ${displaySize} display on standard user`, async ({ organizationPageStandard }) => {
    const container = organizationPageStandard.locator('.organization-page-content');
    const configContainer = container.locator('.organization-info');
    const usersContainer = container.locator('.organization-users');
    const inviteButton = usersContainer.locator('.user-invite-button');
    const storageContainer = container.locator('.organization-storage');
    const emailInvitationCard = usersContainer.locator('.invitation-card-list-item').nth(0);
    const PkiRequestCard = usersContainer.locator('.invitation-card-list-item').nth(1);
    await usersContainer.locator('#invitations-button').isVisible();

    if (displaySize === DisplaySize.Small) {
      await organizationPageStandard.setDisplaySize(DisplaySize.Small);
    }

    await expect(emailInvitationCard).toBeHidden();
    await expect(PkiRequestCard).toBeHidden();
    await expect(inviteButton).toBeHidden();

    if (displaySize === DisplaySize.Small) {
      await organizationPageStandard.locator('.tab-bar-menu').locator('.tab-bar-menu-button').nth(2).click();
    }

    await expect(configContainer.locator('.info-list-item').locator('.info-list-item__title')).toHaveText([
      'External profile',
      'User limit (excluding users with External profile)',
      'Server address',
    ]);
    await expect(configContainer.locator('.info-list-item').nth(0).locator('.info-list-item__value')).toHaveText(['Enabled']);
    await expect(configContainer.locator('.info-list-item').nth(1).locator('.info-list-item__value')).toHaveText(['Unlimited']);
    await expect(configContainer.locator('.info-list-item').nth(2).locator('.server-address-value__text')).toHaveText(/^parsec3:\/\/.+$/);

    const userCategories = usersContainer.locator('.users-list-item');
    await expect(userCategories.nth(0).locator('.users-list-item__title')).toHaveText('3');
    await expect(userCategories.nth(0).locator('.users-list-item__description')).toHaveText('Active');
    await expect(userCategories.nth(1).locator('.users-list-item__title')).toHaveText('0');
    await expect(userCategories.nth(1).locator('.users-list-item__description')).toHaveText('Revoked');
    await expect(userCategories.nth(2).locator('.users-list-item__title')).toHaveText('0');
    await expect(userCategories.nth(2).locator('.users-list-item__description')).toHaveText('Frozen');
    await expect(usersContainer.locator('.user-active-list').locator('.label-profile')).toHaveText(['Administrator', 'Member', 'External']);
    await expect(usersContainer.locator('.user-active-list').locator('.user-active-list-item__value')).toHaveText(['1', '1', '1']);

    await expect(storageContainer.locator('.storage-list-item__value').nth(0)).toHaveText(/^\d+(\.\d{2})? (B|KB|MB)$/);
    await expect(storageContainer.locator('.storage-list-item__value').nth(1)).toHaveText(/^\d+(\.\d{2})? (B|KB|MB)$/);
  });

  msTest(`Org info after one revocation ${displaySize} display`, async ({ usersPage }) => {
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
      await usersPage.locator('.sidebar').locator('#sidebar-organization-information').click();
      await expect(usersPage).toBeOrganizationPage();
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

  msTest(`Go to invitation page ${displaySize} display`, async ({ connected }) => {
    const container = connected.locator('.organization-page-content');
    const usersContainer = container.locator('.organization-users');
    const sidebar = connected.locator('.sidebar');

    if (displaySize === DisplaySize.Small) {
      await connected.setDisplaySize(DisplaySize.Small);
      await connected.locator('.tab-bar-menu').locator('.tab-bar-menu-button').nth(2).click();
    } else {
      await sidebar.locator('.sidebar-content-organization-button').nth(2).click();
    }
    await expect(usersContainer.locator('.invitation-card-list')).toBeVisible();
    await expect(usersContainer.locator('.invitation-card-list-item__title').nth(0)).toHaveText('Email invitations');
    await expect(usersContainer.locator('.invitation-card-list-item__title').nth(1)).toHaveText('Link requests');
    await usersContainer.locator('.invitation-card-list-item').nth(0).click();
    await expect(connected).toBeInvitationPage();

    if (displaySize === DisplaySize.Small) {
      await expect(connected.locator('.switch-view-button')).toContainText('Email invitation');
      await connected.locator('#tab-bar').locator('.tab-bar-menu-button').nth(2).click();
      await expect(usersContainer.locator('.invitation-card-list')).toBeVisible();
      await usersContainer.locator('.invitation-card-list-item').nth(1).click();
      await expect(connected).toBeInvitationPage();
      await expect(connected.locator('.switch-view-button')).toContainText('Link requests');
    } else {
      await expect(connected.locator('.toggle-view').locator('.email-button')).toHaveTheClass('active');
      await sidebar.locator('.sidebar-content-organization-button').nth(2).click();
      await expect(usersContainer.locator('.invitation-card-list')).toBeVisible();
      await usersContainer.locator('.invitation-card-list-item').nth(1).click();
      await expect(connected).toBeInvitationPage();
      await expect(connected.locator('.toggle-view').locator('.pki-button')).toHaveTheClass('active');
    }
  });
}

msTest('No access to organization info as external', async ({ workspacesExternal }) => {
  await expect(workspacesExternal.locator('.sidebar').locator('#sidebar-organization-information')).toBeHidden();
  await workspacesExternal.setDisplaySize(DisplaySize.Small);
  const tabBarButtons = workspacesExternal.locator('#tab-bar').locator('.tab-bar-menu-button');
  await expect(tabBarButtons.nth(2)).toHaveText('Organization');
  await expect(tabBarButtons.nth(2)).toBeHidden();
});

msTest('Copy server address', async ({ organizationPage }) => {
  const btn = organizationPage.locator('.organization-info').locator('#copy-link-btn');
  await btn.click();
  await expect(organizationPage).toShowToast(
    'Failed to copy the organization address. Your browser or device does not seem to support copy/paste.',
    'Error',
  );
  // Try again with permissions
  await organizationPage.context().grantPermissions(['clipboard-write']);
  await btn.click();
  await expect(btn).toBeHidden();
  await expect(organizationPage.locator('.organization-info').locator('.server-address-value__copied')).toBeVisible();
  await expect(organizationPage.locator('.organization-info').locator('.server-address-value__copied')).toHaveText('Copied');

  expect(await getClipboardText(organizationPage)).toMatch(/^parsec3:\/\/.+$/);
});
