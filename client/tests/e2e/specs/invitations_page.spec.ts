// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { answerQuestion, expect, fillIonInput, getClipboardText, msTest, setWriteClipboardPermission } from '@tests/e2e/helpers';

msTest('Email invitations default state', async ({ invitationsPage }) => {
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
      expect(await getClipboardText(invitationsPage)).toMatch(/^https?:\/\/.+\/redirect\/.+$/);
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

interface InvitationParam {
  description: string;
  emails: Array<string>;
  used: Array<string>;
  expectedToast: {
    level: 'Info' | 'Success' | 'Error' | 'Warning';
    message: string;
  };
  expectedInvitations: Array<string>;
}

const INVITATION_PARAMS: Array<InvitationParam> = [
  {
    description: 'Normal invitation',
    emails: ['gordon.freeman@blackmesa.nm'],
    used: ['gordon.freeman@blackmesa.nm'],
    expectedToast: {
      level: 'Success',
      message: 'An invitation to join the organization has been sent to gordon.freeman@blackmesa.nm.',
    },
    expectedInvitations: ['gordon.freeman@blackmesa.nm'],
  },
  {
    description: 'Already used email',
    emails: ['bob@example.com'],
    used: ['bob@example.com'],
    expectedToast: {
      level: 'Warning',
      message: 'The email bob@example.com is already used by someone in this organization.',
    },
    expectedInvitations: [],
  },
  {
    description: 'Multiple invitations with one invalid',
    emails: ['a@b.c', '  d@e.f    ', 'invalid'],
    used: ['a@b.c', 'd@e.f'],
    expectedToast: {
      level: 'Success',
      message: 'Invitations to join the organization have been sent to 2 emails.',
    },
    expectedInvitations: ['a@b.c', 'd@e.f'],
  },
  {
    description: 'Multiple invitations with one already used email',
    emails: ['bob@example.com', 'a@b.c'],
    used: ['bob@example.com', 'a@b.c'],
    expectedToast: {
      level: 'Warning',
      message: 'The invitations have successfully been sent, however one or more addresses already belong to members in this organization.',
    },
    expectedInvitations: ['a@b.c'],
  },
  {
    description: 'Same email multiple times',
    emails: ['a@b.c', 'd@e.f', 'a@b.c', 'g@h.i', 'a@b.c'],
    used: ['a@b.c', 'd@e.f', 'g@h.i'],
    expectedToast: {
      level: 'Success',
      message: 'Invitations to join the organization have been sent to 3 emails.',
    },
    expectedInvitations: ['a@b.c', 'd@e.f', 'g@h.i'],
  },
];

for (const params of INVITATION_PARAMS) {
  msTest(`Invite new email: ${params.description}`, async ({ invitationsPage }) => {
    const viewToggle = invitationsPage.locator('.toggle-view-container');
    const invites = invitationsPage.locator('.invitations-container-list').locator('.invitation-list-item');
    await expect(invites).toHaveCount(1);
    await viewToggle.locator('#invite-user-button').click();
    const modal = invitationsPage.locator('.invite-modal');
    await expect(modal).toBeVisible();
    const okButton = modal.locator('#next-button');
    await expect(okButton).toBeTrulyDisabled();
    await fillIonInput(modal.locator('ion-input'), params.emails.join(';'));
    if (params.emails.length <= 1) {
      await expect(okButton).toHaveText('Send invitation');
      await expect(modal.locator('.email-list-container')).toBeHidden();
    } else {
      await expect(okButton).toHaveText(`Send ${params.used.length} invitations`);
      await expect(modal.locator('.email-list-container')).toBeVisible();
      await expect(modal.locator('.email-list-container').locator('.email-list__title')).toHaveText(
        `List of email addresses (${params.used.length})`,
      );
      await expect(modal.locator('.email-list-container').locator('.email-list__item')).toHaveText(params.used);
    }
    await expect(okButton).toBeTrulyEnabled();
    await okButton.click();
    await expect(modal).toBeHidden();
    await expect(invitationsPage).toShowToast(params.expectedToast.message, params.expectedToast.level);
    await expect(invites).toHaveCount(params.expectedInvitations.length + 1);
    await expect(invites.locator('.invitation-email')).toHaveText(['zack@example.invalid', ...params.expectedInvitations]);
  });
}
