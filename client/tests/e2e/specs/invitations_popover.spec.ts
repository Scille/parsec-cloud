// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { answerQuestion, expect, fillInputModal, getClipboardText, msTest, setWriteClipboardPermission } from '@tests/e2e/helpers';

msTest('Profile popover default state', async ({ connected }) => {
  await expect(connected.locator('.topbar').locator('#invitations-button')).toHaveText('2 invitations');

  await expect(connected.locator('.invitations-list-popover')).toBeHidden();
  await connected.locator('.topbar').locator('#invitations-button').click();
  const popover = connected.locator('.invitations-list-popover');
  await expect(popover).toBeVisible();
  await expect(popover.locator('.invitations-list-header__title')).toHaveText('Invitations');
  await expect(popover.locator('.invitations-list-header__counter')).toHaveText('2');
  await expect(popover.locator('.invitations-list-header__button')).toHaveText('Invite a new member');
  const invitations = popover.locator('.invitation-list-item');
  await expect(invitations).toHaveCount(2);
  await expect(invitations.locator('.invitation-email')).toHaveText(['shadowheart@swordcoast.faerun', 'gale@waterdeep.faerun']);
  await expect(invitations.locator('.invitation-actions-date')).toHaveText(['now', 'now'], { useInnerText: true });
  const firstInv = invitations.nth(0);
  await expect(firstInv.locator('.copy-link')).toBeHidden();
  await firstInv.hover();
  await expect(firstInv.locator('.copy-link')).toBeVisible();
  await expect(firstInv.locator('.invitation-actions-buttons').locator('ion-button')).toHaveText(['Cancel', 'Greet']);
});

msTest('Copy invitation link', async ({ connected }) => {
  await connected.locator('.topbar').locator('#invitations-button').click();
  const popover = connected.locator('.invitations-list-popover');
  const inv = popover.locator('.invitation-list-item').nth(0);
  await inv.hover();
  await inv.locator('.copy-link').click();
  await expect(connected).toShowToast('Failed to copy the link. Your browser or device does not seem to support copy/paste.', 'Error');
  await setWriteClipboardPermission(connected.context(), true);
  await inv.hover();
  await inv.locator('.copy-link').click();
  await expect(connected).toShowToast('Invitation link has been copied to clipboard.', 'Info');
  expect(await getClipboardText(connected)).toMatch(/^parsec3:\/\/.+$/);
});

msTest('Cancel invitation cancel', async ({ connected }) => {
  await connected.locator('.topbar').locator('#invitations-button').click();
  const popover = connected.locator('.invitations-list-popover');
  const inv = popover.locator('.invitation-list-item').nth(0);
  await inv.hover();
  await inv.locator('.invitation-actions-buttons').locator('ion-button').nth(0).click();
  await answerQuestion(connected, false, {
    expectedTitleText: 'Cancel invitation',
    expectedQuestionText:
      'The invitation sent to shadowheart@swordcoast.faerun and the invitation link \
    will no longer be valid. Are you sure you want to continue?',
    expectedPositiveText: 'Cancel invitation',
    expectedNegativeText: 'Keep invitation',
  });
});

msTest('Cancel invitation', async ({ connected }) => {
  await connected.locator('.topbar').locator('#invitations-button').click();
  const popover = connected.locator('.invitations-list-popover');
  const inv = popover.locator('.invitation-list-item').nth(0);
  await inv.hover();
  await inv.locator('.invitation-actions-buttons').locator('ion-button').nth(0).click();
  await answerQuestion(connected, true);
  await expect(connected).toShowToast('Invitation has been cancelled.', 'Success');
});

msTest('Invite new user', async ({ connected }) => {
  await connected.locator('.topbar').locator('#invitations-button').click();
  const popover = connected.locator('.invitations-list-popover');
  await popover.locator('.invitations-list-header__button').click();
  await expect(connected).toBeUserPage();
  // cspell:disable-next-line
  await fillInputModal(connected, 'zana@wraeclast');
  // cspell:disable-next-line
  await expect(connected).toShowToast('An invitation to join the organization has been sent to zana@wraeclast.', 'Success');
});

msTest('Invite user with already existing email', async ({ connected }) => {
  await connected.locator('.topbar').locator('#invitations-button').click();
  const popover = connected.locator('.invitations-list-popover');
  await popover.locator('.invitations-list-header__button').click();
  await expect(connected).toBeUserPage();
  await fillInputModal(connected, 'jaheira@gmail.com');
  await expect(connected).toShowToast('The email jaheira@gmail.com is already used by someone in this organization.', 'Error');
});
