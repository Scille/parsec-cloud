// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { answerQuestion, expect, login, logout, mockLibParsec, msTest, sendEvent } from '@tests/e2e/helpers';

msTest('Revoked event', async ({ usersPage }) => {
  msTest.setTimeout(60_000);

  // Revoke Boby
  const item = usersPage.locator('#users-page-user-list').getByRole('listitem').nth(1);
  await item.click({ button: 'right' });
  const menu = usersPage.locator('#user-context-menu');
  await expect(menu).toBeVisible();
  await menu.getByRole('listitem').nth(1).click();
  await answerQuestion(usersPage, true, {
    expectedTitleText: 'Revoke this user?',
    expectedQuestionText:
      'This will revoke Boby McBobFace, preventing them from accessing this organization. Are you sure you want to proceed?',
    expectedPositiveText: 'Revoke',
    expectedNegativeText: 'Cancel',
  });
  await expect(usersPage).toShowToast('Boby McBobFace has been revoked. They can no longer access this organization.', 'Success');
  await expect(item).toHaveTheClass('revoked');

  // Revoke Malloryy
  const item2 = usersPage.locator('#users-page-user-list').getByRole('listitem').nth(2);
  await item2.click({ button: 'right' });
  await menu.getByRole('listitem').nth(1).click();
  await answerQuestion(usersPage, true, {
    expectedTitleText: 'Revoke this user?',
    expectedQuestionText:
      'This will revoke Malloryy McMalloryFace, preventing them from accessing this organization. Are you sure you want to proceed?',
    expectedPositiveText: 'Revoke',
    expectedNegativeText: 'Cancel',
  });
  await expect(usersPage).toShowToast('Malloryy McMalloryFace has been revoked. They can no longer access this organization.', 'Success');
  await expect(item2).toHaveTheClass('revoked');

  await logout(usersPage);
  const cards = usersPage.locator('.organization-list').locator('.organization-card');
  await expect(cards).toHaveCount(3);

  // Login to Boby: two devices exist and should be archived
  await login(usersPage, 'Boby McBobFace');
  const modal = usersPage.locator('.information-modal');
  await expect(modal).toBeVisible({ timeout: 10000 });
  await expect(modal.locator('.ms-error').nth(0)).toBeVisible();
  await expect(modal.locator('.container-textinfo__text')).toHaveText(
    `You have been revoked from this organization. You will be logged out and won't be able to access it anymore. If you think this is
 a mistake or have any issues, please contact an administrator.`,
  );
  await modal.locator('#next-button').click();
  await expect(usersPage).toBeHomePage();
  await expect(modal).not.toBeVisible();
  await expect(cards).toHaveCount(2);

  // Login to Malloryy: a single device exists and should be archived
  await login(usersPage, 'Malloryy McMalloryFace');
  await expect(modal).toBeVisible({ timeout: 10000 });
  await expect(modal.locator('.ms-error').nth(0)).toBeVisible();
  await expect(modal.locator('.container-textinfo__text')).toHaveText(
    `You have been revoked from this organization. You will be logged out and won't be able to access it anymore. If you think this is
 a mistake or have any issues, please contact an administrator.`,
  );
  await modal.locator('#next-button').click();
  await expect(usersPage).toBeHomePage();
  await expect(modal).not.toBeVisible();
  await expect(cards).toHaveCount(1);
});

msTest('Test must accept TOS event', async ({ connected }) => {
  await mockLibParsec(connected, [
    {
      name: 'clientGetTos',
      result: {
        ok: true,
        value: { perLocaleUrls: [['en', 'http://invalid']], updatedOn: 1337 },
      },
      valueConverter: [['perLocaleUrls', 'toMap']],
    },
  ]);

  const modal = connected.locator('.modal-tos');
  await expect(modal).toBeHidden();
  await sendEvent(connected, 'ClientEventMustAcceptTos');
  await expect(modal).toBeVisible();
  await expect(modal.locator('#next-button')).toBeTrulyDisabled();
  await modal.locator('.ms-checkbox').check();
  await expect(modal.locator('#next-button')).toBeTrulyEnabled();
  await modal.locator('#next-button').click();
  await expect(modal).toBeHidden();
});

msTest('Test incompatible server event', async ({ connected }) => {
  await sendEvent(connected, 'ClientEventIncompatibleServer');
  await connected.waitForTimeout(2000);
  await expect(connected).toShowInformationModal(
    'Your application and the server are not compatible. \
Online features will not be available. \
Please check that your application is up-to-date, \
and if so, contact an administrator.',
    'Error',
  );
});

msTest('Test revoked self user event', async ({ connected }) => {
  await sendEvent(connected, 'ClientEventRevokedSelfUser');
  await connected.waitForTimeout(2000);
  await expect(connected).toShowInformationModal(
    "You have been revoked from this organization. \
You will be logged out and won't be able to access it anymore. \
If you think this is a mistake or have any issues, please contact an administrator.",
    'Error',
  );
  await expect(connected).toBeHomePage();
});
