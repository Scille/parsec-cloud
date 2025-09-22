// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { answerQuestion, expect, fillInputModal, getClipboardText, msTest, setWriteClipboardPermission } from '@tests/e2e/helpers';

msTest('Email invitations default state', async ({ invitationsPage }) => {
  const viewToggle = invitationsPage.locator('.toggle-view-container');

  await expect(viewToggle.locator('.email-button')).toHaveText('Email invitation1');
  await expect(viewToggle.locator('.pki-button')).toHaveText('PKI requests3');
  await expect(viewToggle.locator('.email-button')).toBeTrulyDisabled();
  await expect(viewToggle.locator('.pki-button')).toBeTrulyEnabled();
  await expect(viewToggle.locator('#invite-user-button')).toBeVisible();
  await expect(viewToggle.locator('#invite-user-button')).toHaveText('Invite a user');
  await expect(viewToggle.locator('#update-root-certificate-button')).toBeHidden();
  await expect(invitationsPage.locator('.invitations-container-list').locator('.invitations-list-header__label')).toHaveText([
    'Email',
    'Sent on',
    '',
  ]);
  const invites = invitationsPage.locator('.invitations-container-list').locator('.invitation-list-item');
  await expect(invites).toHaveCount(1);
  await expect(invites.nth(0).locator('.invitation-email')).toHaveText('zack@example.invalid');
  await expect(invites.nth(0).locator('.invitation-sentOn')).toHaveText('Jan 7, 2000');
  const actions = invites.nth(0).locator('.invitation-actions').locator('ion-button');
  await expect(actions.nth(0)).toHaveText('Greet');
  const tooltip = invitationsPage.locator('.tooltip-popover');
  await expect(tooltip).toBeHidden();

  await actions.nth(1).hover();
  await expect(tooltip).toBeVisible();
  await expect(tooltip).toHaveText('Copy link');
  // Hide the tooltip
  await invites.nth(0).locator('.invitation-email').hover();

  await actions.nth(2).hover();
  await expect(tooltip).toBeVisible();
  await expect(tooltip).toHaveText('Resend email');
  await invites.nth(0).locator('.invitation-email').hover();

  await actions.nth(3).hover();
  await expect(tooltip).toBeVisible();
  await expect(tooltip).toHaveText('Delete invitation');
  await invites.nth(0).locator('.invitation-email').hover();
});

msTest('Email invitations new invite', async ({ invitationsPage }) => {
  const viewToggle = invitationsPage.locator('.toggle-view-container');
  const invites = invitationsPage.locator('.invitations-container-list').locator('.invitation-list-item');
  await expect(invites).toHaveCount(1);
  await viewToggle.locator('#invite-user-button').click();
  await fillInputModal(invitationsPage, 'gordon.freeman@blackmesa.nm');
  await expect(invitationsPage).toShowToast(
    'An invitation to join the organization has been sent to gordon.freeman@blackmesa.nm.',
    'Success',
  );
  await expect(invites).toHaveCount(2);
  await expect(invites.nth(1).locator('.invitation-email')).toHaveText('gordon.freeman@blackmesa.nm');
});

msTest('Email invitations start greet', async ({ invitationsPage }) => {
  const invites = invitationsPage.locator('.invitations-container-list').locator('.invitation-list-item');
  await expect(invites).toHaveCount(1);
  const actions = invites.nth(0).locator('.invitation-actions').locator('ion-button');
  await expect(invitationsPage.locator('.greet-organization-modal')).toBeHidden();
  await actions.nth(0).click();
  await expect(invitationsPage.locator('.greet-organization-modal')).toBeVisible();
});

for (const hasPerms in [true, false]) {
  msTest(`Email invitations copy link ${['without', 'with'][Number(hasPerms)]} permissions`, async ({ invitationsPage }) => {
    const invites = invitationsPage.locator('.invitations-container-list').locator('.invitation-list-item');
    await expect(invites).toHaveCount(1);
    const actions = invites.nth(0).locator('.invitation-actions').locator('ion-button');
    if (hasPerms) {
      await setWriteClipboardPermission(invitationsPage.context(), true);
    }
    await actions.nth(1).click();
    if (hasPerms) {
      await expect(invitationsPage).toShowToast('Invitation link has been copied to clipboard.', 'Info');
      expect(await getClipboardText(invitationsPage)).toMatch(/^parsec3:\/\/.+$/);
    } else {
      await expect(invitationsPage).toShowToast(
        'Failed to copy the link. Your browser or device does not seem to support copy/paste.',
        'Error',
      );
    }
  });
}

msTest('Email invitations resend email', async ({ invitationsPage }) => {
  const invites = invitationsPage.locator('.invitations-container-list').locator('.invitation-list-item');
  await expect(invites).toHaveCount(1);
  const actions = invites.nth(0).locator('.invitation-actions').locator('ion-button');
  await actions.nth(2).click();
  await answerQuestion(invitationsPage, true, {
    expectedTitleText: 'Resend the link by email',
    expectedQuestionText: 'The user should already have received an email with the link. Are you sure you want to send another one?',
    expectedPositiveText: 'Resend email',
    expectedNegativeText: 'No',
  });
  await expect(invitationsPage).toShowToast('An invitation to join the organization has been sent to zack@example.invalid.', 'Success');
});

for (const answer of [true, false]) {
  msTest(`Email invitations delete invitation answer ${answer}`, async ({ invitationsPage }) => {
    const invites = invitationsPage.locator('.invitations-container-list').locator('.invitation-list-item');
    await expect(invites).toHaveCount(1);
    const actions = invites.nth(0).locator('.invitation-actions').locator('ion-button');
    await actions.nth(3).click();
    await answerQuestion(invitationsPage, answer);
    if (answer) {
      await expect(invitationsPage).toShowToast('Invitation has been cancelled.', 'Success');
      await expect(invites).toHaveCount(0);
    } else {
      await expect(invites).toHaveCount(1);
    }
  });
}
