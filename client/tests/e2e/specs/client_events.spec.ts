// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { answerQuestion, expect, login, logout, msTest } from '@tests/e2e/helpers';

msTest('Revoked event', async ({ usersPage }) => {
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
  await logout(usersPage);
  await login(usersPage, 'Boby McBobFace');
  await usersPage.waitForTimeout(3000);
  const modal = usersPage.locator('.information-modal');
  await expect(modal.locator('.ms-error').nth(0)).toBeVisible();
  await expect(modal.locator('.container-textinfo__text')).toHaveText(
    `You have been revoked from this organization. You can still see some of the data you
 had access to, but most features of Parsec will not work.`,
  );
  await modal.locator('#next-button').click();
  await expect(usersPage).toHaveNotification('You have been revoked from this organization.');
});
