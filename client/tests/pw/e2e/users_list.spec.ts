// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { Page } from '@playwright/test';
import { expect } from '@tests/pw/helpers/assertions';
import { msTest } from '@tests/pw/helpers/fixtures';
import { answerQuestion } from '@tests/pw/helpers/utils';

const USERS = [
  {
    name: 'Gordon Freeman',
    email: 'user@host.com',
    profile: 'Administrator',
    active: true,
  },
  {
    // cspell:disable-next-line
    name: 'Jaheira',
    email: 'jaheira@gmail.com',
    profile: 'Administrator',
    active: true,
  },
  {
    // cspell:disable-next-line
    name: 'Arthas Menethil',
    email: 'arthasmenethil@gmail.com',
    profile: 'Administrator',
    active: false,
  },
  {
    // cspell:disable-next-line
    name: 'Cernd',
    email: 'cernd@gmail.com',
    profile: 'Standard',
    active: true,
  },
  {
    name: 'Patches',
    email: 'patches@yahoo.fr',
    profile: 'Standard',
    active: true,
  },
  {
    // cspell:disable-next-line
    name: 'Valygar Corthala',
    email: 'val@gmail.com',
    profile: 'Standard',
    active: false,
  },
  {
    // cspell:disable-next-line
    name: 'Karl Hungus',
    email: 'karlhungus@gmail.com',
    profile: 'Outsider',
    active: true,
  },
  {
    name: 'Gaia',
    email: 'gaia@gmail.com',
    profile: 'Outsider',
    active: false,
  },
];

const usersTest = msTest.extend<{ usersPage: Page }>({
  usersPage: async ({ connected }, use) => {
    await connected.locator('.sidebar').locator('.organization-card-manageBtn').click();
    await expect(connected).toHavePageTitle('Users');
    use(connected);
  },
});

usersTest('User list default state', async ({ usersPage }) => {
  const actionBar = usersPage.locator('#activate-users-ms-action-bar');
  await expect(actionBar.locator('#button-invite-user')).toBeVisible();
  await expect(actionBar.locator('#button-invite-user')).toHaveText('Invite a user');
  await expect(actionBar.locator('.counter')).toHaveText(`${USERS.length} users`, { useInnerText: true });
  await expect(usersPage.locator('.user-list-header').locator('ion-checkbox')).toHaveState('unchecked');
  await expect(actionBar.locator('#select-popover-button')).toHaveText('Profile');
  await expect(actionBar.locator('.ms-grid-list-toggle').locator('#grid-view')).not.toHaveDisabledAttribute();
  await expect(actionBar.locator('.ms-grid-list-toggle').locator('#list-view')).toHaveDisabledAttribute();
  await expect(usersPage.locator('#users-page-user-list').getByRole('listitem')).toHaveCount(USERS.length);
});

for (const [index, user] of USERS.entries()) {
  usersTest(`Check user list item of ${user.name}`, async ({ usersPage }) => {
    const usersList = usersPage.locator('#users-page-user-list');
    const item = usersList.getByRole('listitem').nth(index);
    await expect(item.locator('.user-name').locator('.person-name')).toHaveText(user.name);
    await expect(item.locator('.user-profile')).toHaveText(user.profile);
    await expect(item.locator('.user-email')).toHaveText(user.email);
    await expect(item.locator('.user-status')).toHaveText(user.active ? 'Active' : 'Revoked');
    if (!user.active) {
      await expect(item).toHaveTheClass('revoked');
    }
  });
}

for (const [index, user] of USERS.entries()) {
  usersTest(`Check user grid item of ${user.name}`, async ({ usersPage }) => {
    await usersPage.locator('#activate-users-ms-action-bar').locator('.ms-grid-list-toggle').locator('#grid-view').click();
    const usersGrid = usersPage.locator('.users-container-grid');
    const card = usersGrid.locator('.user-card-item').nth(index);
    await expect(card.locator('.user-card-info').locator('.user-card-info__name').locator('span').nth(0)).toHaveText(user.name);
    await expect(card.locator('.user-card-info').locator('.user-card-info__email')).toHaveText(user.email);
    await expect(card.locator('.user-card-profile')).toHaveText(user.profile);
    if (user.active) {
      await expect(card.locator('.user-revoked')).toBeHidden();
    } else {
      await expect(card.locator('.user-revoked')).toBeVisible();
      await expect(card).toHaveTheClass('revoked');
    }
  });
}

for (const revokedUser of [false, true]) {
  usersTest(`Check user context menu for ${revokedUser ? 'revoked' : 'active'} user`, async ({ usersPage }) => {
    const item = usersPage
      .locator('#users-page-user-list')
      .getByRole('listitem')
      .nth(revokedUser ? 2 : 1);
    await expect(usersPage.locator('#user-context-menu')).toBeHidden();
    await item.hover();
    await item.locator('.options-button').click();
    await expect(usersPage.locator('#user-context-menu')).toBeVisible();
    const menu = usersPage.locator('#user-context-menu');
    const expectedActions = ['User details', 'View details'];
    if (!revokedUser) {
      expectedActions.unshift(...['Deletion', 'Revoke this user']);
    }
    await expect(menu.getByRole('listitem')).toHaveText(expectedActions);
  });
}

usersTest('Revoke one user with context menu', async ({ usersPage }) => {
  const item = usersPage.locator('#users-page-user-list').getByRole('listitem').nth(1);
  await item.hover();
  await item.locator('.options-button').click();
  await usersPage.locator('#user-context-menu').getByRole('listitem').nth(1).click();
  await answerQuestion(usersPage, true, {
    expectedTitleText: 'Revoke this user?',
    // cspell:disable-next-line
    expectedQuestionText: 'This will revoke Jaheira, preventing them from accessing this organization. Are you sure you want to proceed?',
    expectedPositiveText: 'Revoke',
    expectedNegativeText: 'Cancel',
  });
  // cspell:disable-next-line
  await expect(usersPage).toShowToast('Jaheira has been revoked. They can no longer access this organization.', 'Success');
});

usersTest('Revoke one user with selection', async ({ usersPage }) => {
  const item = usersPage.locator('#users-page-user-list').getByRole('listitem').nth(1);
  await item.hover();
  await item.locator('ion-checkbox').click();
  await usersPage.locator('#activate-users-ms-action-bar').locator('#button-revoke-user').click();
  await answerQuestion(usersPage, true, {
    expectedTitleText: 'Revoke this user?',
    // cspell:disable-next-line
    expectedQuestionText: 'This will revoke Jaheira, preventing them from accessing this organization. Are you sure you want to proceed?',
    expectedPositiveText: 'Revoke',
    expectedNegativeText: 'Cancel',
  });
  // cspell:disable-next-line
  await expect(usersPage).toShowToast('Jaheira has been revoked. They can no longer access this organization.', 'Success');
});

usersTest('Revoke two users with selection', async ({ usersPage }) => {
  const item1 = usersPage.locator('#users-page-user-list').getByRole('listitem').nth(1);
  await item1.hover();
  await item1.locator('ion-checkbox').click();

  const item2 = usersPage.locator('#users-page-user-list').getByRole('listitem').nth(3);
  await item2.hover();
  await item2.locator('ion-checkbox').click();

  await usersPage.locator('#activate-users-ms-action-bar').locator('#button-revoke-user').click();
  await answerQuestion(usersPage, true, {
    expectedTitleText: 'Revoke these users?',
    expectedQuestionText:
      'This will revoke these 2 users, preventing them from accessing this organization. Are you sure you want to proceed?',
    expectedPositiveText: 'Revoke',
    expectedNegativeText: 'Cancel',
  });
  await expect(usersPage).toShowToast('2 users have been revoked, they can no longer access this organization.', 'Success');
});

usersTest('Test users selection', async ({ usersPage }) => {
  const item = usersPage.locator('#users-page-user-list').getByRole('listitem').nth(1);
  await item.hover();
  await expect(item.locator('ion-checkbox')).toHaveState('unchecked');
  // Selecting one user
  await item.locator('ion-checkbox').click();
  await expect(item.locator('ion-checkbox')).toHaveState('checked');
  const actionBar = usersPage.locator('#activate-users-ms-action-bar');
  await expect(actionBar.locator('#button-invite-user')).toBeHidden();
  await expect(actionBar.locator('#button-revoke-user')).toBeVisible();
  await expect(actionBar.locator('#button-revoke-user')).toHaveText('Revoke this user');
  await expect(actionBar.locator('#button-common-workspaces')).toBeVisible();
  await expect(actionBar.locator('.counter')).toHaveText('One user selected', { useInnerText: true });
  const headerCheckbox = usersPage.locator('.user-list-header').locator('ion-checkbox');
  // Header checkbox should be indeterminate since not all users are selected
  await expect(headerCheckbox).toHaveState('indeterminate');
  // Unselecting the user
  await item.locator('ion-checkbox').click();
  await expect(item.locator('ion-checkbox')).toHaveState('unchecked');
  // Header checkbox should be unchecked
  await expect(headerCheckbox).toHaveState('unchecked');
  await expect(actionBar.locator('.counter')).toHaveText(`${USERS.length} users`, { useInnerText: true });
  await expect(actionBar.locator('#button-revoke-user')).toHaveText('Revoke these users');

  // Select all with header checkbox
  await headerCheckbox.click();
  for (const [index, user] of USERS.entries()) {
    // Only active and not current user
    if (user.active && index > 0) {
      const checkbox = usersPage.locator('#users-page-user-list').getByRole('listitem').nth(index).locator('ion-checkbox');
      await expect(checkbox).toHaveState('checked');
    }
  }
  const expectedSelected = USERS.filter((u) => u.active && u.name !== 'Gordon Freeman');
  await expect(actionBar.locator('.counter')).toHaveText(`${expectedSelected.length} users selected`, { useInnerText: true });
  await expect(headerCheckbox).toHaveState('checked');
  // Unselect one
  await usersPage.locator('#users-page-user-list').getByRole('listitem').nth(3).locator('ion-checkbox').click();
  await expect(usersPage.locator('#users-page-user-list').getByRole('listitem').nth(3).locator('ion-checkbox')).toHaveState('unchecked');
  // Header checkbox goes to indeterminate
  await expect(headerCheckbox).toHaveState('indeterminate');
  // Reselect the user
  await usersPage.locator('#users-page-user-list').getByRole('listitem').nth(3).locator('ion-checkbox').click();
  await expect(usersPage.locator('#users-page-user-list').getByRole('listitem').nth(3).locator('ion-checkbox')).toHaveState('checked');
  // Header checkbox goes back to checked
  await expect(headerCheckbox).toHaveState('checked');

  // Unselect all
  await headerCheckbox.click();
  await expect(headerCheckbox).toHaveState('unchecked');
});
