// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { expect, fillInputModal, login, msTest } from '@tests/e2e/helpers';

msTest('Display account invitations', async ({ parsecAccountLoggedIn }) => {
  // Get the account email, check that we don't have an invitation
  await parsecAccountLoggedIn.locator('.profile-header-homepage').click();
  await expect(parsecAccountLoggedIn.locator('.profile-header-homepage-popover')).toBeVisible();
  const email = (await parsecAccountLoggedIn
    .locator('.profile-header-homepage-popover')
    .locator('.header-list-email')
    .textContent()) as string;
  await parsecAccountLoggedIn.locator('.profile-header-homepage-popover').locator('ion-backdrop').click();
  const invitations = parsecAccountLoggedIn.locator('.homepage-content').locator('.account-invitations').locator('.account-invitation');
  await expect(invitations).toHaveCount(0);

  // Open a second tab, no account, invite using the email
  const noAccountPage = await parsecAccountLoggedIn.openNewTab({ withParsecAccount: false });
  await expect(noAccountPage.locator('.organization-card')).toHaveCount(3);
  await login(noAccountPage, 'Alicey McAliceFace');
  await noAccountPage.locator('.sidebar').locator('#manageOrganization').click();
  await expect(noAccountPage).toHavePageTitle('Users');
  await expect(noAccountPage).toBeUserPage();
  await noAccountPage.locator('#activate-users-ms-action-bar').getByText('Invite a user').click();
  await fillInputModal(noAccountPage, email);
  await expect(noAccountPage).toShowToast(`An invitation to join the organization has been sent to ${email}.`, 'Success');
  await noAccountPage.locator('.topbar').locator('#invitations-button').click();
  const popover = noAccountPage.locator('.invitations-list-popover');
  const inv = popover.locator('.invitation-list-item').nth(1);
  await inv.locator('.invitation-header__greet-button').click();
  const greetModal = noAccountPage.locator('.greet-organization-modal');
  await expect(greetModal).toBeVisible();
  await noAccountPage.release();

  // Account tab, check if the invitation appeared
  await parsecAccountLoggedIn.waitForTimeout(500);
  await expect(parsecAccountLoggedIn.locator('.account-invitations__title')).toHaveText('One invitation pending');
  await expect(invitations).toHaveCount(1);
  await invitations.nth(0).click();
  const joinModal = parsecAccountLoggedIn.locator('.join-organization-modal');
  await expect(joinModal).toBeVisible();
  await expect(joinModal.locator('#next-button')).toBeVisible();
});
