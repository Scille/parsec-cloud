// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { answerQuestion, expect, getClipboardText, inviteUsers, msTest, setWriteClipboardPermission } from '@tests/e2e/helpers';

msTest('Profile popover default state', async ({ connected }) => {
  await expect(connected.locator('.topbar').locator('#invitations-button').locator('.unread-count')).toHaveText('1');

  await expect(connected.locator('.invitations-list-popover')).toBeHidden();
  await connected.locator('.topbar').locator('#invitations-button').click();
  const popover = connected.locator('.invitations-list-popover');
  await expect(popover).toBeVisible();
  await expect(popover.locator('.invitations-list-header__title')).toHaveText('Invitations');
  await expect(popover.locator('.invitations-list-header__counter')).toHaveText('1');
  await expect(popover.locator('.invitations-list-header__button')).toHaveText('Invite a new member');
  const invitations = popover.locator('.invitation-list-item');
  await expect(invitations).toHaveCount(1);
  await expect(invitations.locator('.invitation-header__email')).toHaveText('zack@example.invalid');
  await expect(invitations.locator('.invitation-footer__date')).toHaveText('Jan 7, 2000', { useInnerText: true });
  const firstInv = invitations.nth(0);
  await expect(firstInv.locator('.copy-link')).toBeVisible();
  await expect(firstInv.locator('.send-email')).toBeVisible();
  await expect(firstInv.locator('.manage-button')).toHaveCount(3);
});

msTest('Copy invitation link', async ({ connected }) => {
  await connected.locator('.topbar').locator('#invitations-button').click();
  const popover = connected.locator('.invitations-list-popover');
  const inv = popover.locator('.invitation-list-item').nth(0);
  await inv.locator('.copy-link').click();
  await expect(connected).toShowToast('Failed to copy the link. Your browser or device does not seem to support copy/paste.', 'Error');
  await setWriteClipboardPermission(connected.context(), true);
  await inv.hover();
  await inv.locator('.copy-link').click();
  await expect(connected).toShowToast('Invitation link has been copied to clipboard.', 'Info');
  expect(await getClipboardText(connected)).toMatch(/^parsec3:\/\/.+$/);
});

msTest('Resend email', async ({ connected }) => {
  await connected.locator('.topbar').locator('#invitations-button').click();
  const popover = connected.locator('.invitations-list-popover');
  const inv = popover.locator('.invitation-list-item').nth(0);
  await inv.locator('.send-email').click();
  await answerQuestion(connected, true, {
    expectedTitleText: 'Resend the link by email',
    expectedQuestionText: 'The user should already have received an email with the link. Are you sure you want to send another one?',
    expectedPositiveText: 'Resend email',
    expectedNegativeText: 'No',
  });
  await expect(connected).toShowToast('An invitation to join the organization has been sent to zack@example.invalid.', 'Success');
  await expect(inv.locator('.send-email')).toBeTrulyDisabled();
});

msTest('Cancel invitation - no', async ({ connected }) => {
  await connected.locator('.topbar').locator('#invitations-button').click();
  const popover = connected.locator('.invitations-list-popover');
  const inv = popover.locator('.invitation-list-item').nth(0);
  await inv.hover();
  await inv.locator('.cancel-button').click();
  await answerQuestion(connected, false, {
    expectedTitleText: 'Cancel invitation',
    expectedQuestionText:
      'The invitation sent to zack@example.invalid and the invitation link \
    will no longer be valid. Are you sure you want to continue?',
    expectedPositiveText: 'Delete invitation',
    expectedNegativeText: 'Keep invitation',
  });
});

msTest('Cancel invitation - yes', async ({ connected }) => {
  await connected.locator('.topbar').locator('#invitations-button').click();
  const popover = connected.locator('.invitations-list-popover');
  const inv = popover.locator('.invitation-list-item').nth(0);
  await inv.hover();
  await inv.locator('.cancel-button').click();
  await answerQuestion(connected, true);
  await expect(connected).toShowToast('Invitation has been cancelled.', 'Success');
});

msTest('Invite new user', async ({ connected }) => {
  await connected.locator('.topbar').locator('#invitations-button').click();
  const popover = connected.locator('.invitations-list-popover');
  await popover.locator('.invitations-list-header__button').click();
  await expect(connected).toBeInvitationPage();
  // cspell:disable-next-line
  await inviteUsers(connected, 'zana@wraeclast');
  // cspell:disable-next-line
  await expect(connected).toShowToast('An invitation to join the organization has been sent to zana@wraeclast.', 'Success');
  await connected.locator('.topbar .back-button').click();
  await expect(connected).toBeWorkspacePage();
  await expect(connected.locator('.topbar').locator('#invitations-button').locator('.unread-count')).toHaveText('2');

  await expect(connected.locator('.invitations-list-popover')).toBeHidden();
  await connected.locator('.topbar').locator('#invitations-button').click();
  await expect(popover).toBeVisible();
  await expect(popover.locator('.invitations-list-header__title')).toHaveText('Invitations');
  await expect(popover.locator('.invitations-list-header__counter')).toHaveText('2');
  const invitations = popover.locator('.invitation-list-item');
  await expect(invitations).toHaveCount(2);
  // cspell:disable-next-line
  await expect(invitations.locator('.invitation-header__email')).toHaveText(['zack@example.invalid', 'zana@wraeclast']);
  await expect(invitations.locator('.invitation-footer__date')).toHaveText(['Jan 7, 2000', 'now'], { useInnerText: true });
});

msTest('Invite user with already existing email', async ({ connected }) => {
  await connected.locator('.topbar').locator('#invitations-button').click();
  const popover = connected.locator('.invitations-list-popover');
  await popover.locator('.invitations-list-header__button').click();
  await expect(connected).toBeInvitationPage();
  await inviteUsers(connected, 'bob@example.com');
  await expect(connected).toShowToast('The email bob@example.com is already used by someone in this organization.', 'Warning');
});
