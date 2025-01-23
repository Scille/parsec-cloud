// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { Page } from '@playwright/test';
import { answerQuestion, expect, fillInputModal, fillIonInput, msTest, sortBy } from '@tests/e2e/helpers';

const USERS = [
  {
    name: 'Gordon Freeman',
    email: 'user@host.com',
    profile: 'Administrator',
    active: true,
    currentUser: true,
    frozen: false,
  },
  {
    // cspell:disable-next-line
    name: 'Jaheira',
    email: 'jaheira@gmail.com',
    profile: 'Administrator',
    active: true,
    frozen: false,
  },
  {
    // cspell:disable-next-line
    name: 'Arthas Menethil',
    email: 'arthasmenethil@gmail.com',
    profile: 'Administrator',
    active: false,
    frozen: false,
  },
  {
    // cspell:disable-next-line
    name: 'Cernd',
    email: 'cernd@gmail.com',
    profile: 'Member',
    active: true,
    frozen: false,
  },
  {
    name: 'Patches',
    email: 'patches@yahoo.fr',
    profile: 'Member',
    active: true,
    frozen: false,
  },
  {
    // cspell:disable-next-line
    name: 'Valygar Corthala',
    email: 'val@gmail.com',
    profile: 'Member',
    active: false,
    frozen: false,
  },
  {
    name: 'Gaia',
    email: 'gaia@gmail.com',
    profile: 'External',
    active: false,
    frozen: false,
  },
  {
    // cspell:disable-next-line
    name: 'Karl Hungus',
    email: 'karlhungus@gmail.com',
    profile: 'External',
    active: false,
    frozen: true,
  },
];

msTest('User list default state', async ({ usersPage }) => {
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

function getStatusForUser(user: any): string {
  if (user.frozen) {
    return 'Suspended';
  }
  return user.active ? 'Active' : 'Revoked';
}

msTest('Check user list items', async ({ usersPage }) => {
  const usersList = usersPage.locator('#users-page-user-list');
  for (const [index, user] of USERS.entries()) {
    const item = usersList.getByRole('listitem').nth(index);
    await expect(item.locator('.user-name').locator('.person-name')).toHaveText(user.name);
    await expect(item.locator('.user-profile')).toHaveText(user.profile);
    await expect(item.locator('.user-email')).toHaveText(user.email);
    await expect(item.locator('.user-status')).toHaveText(getStatusForUser(user));
    if (!user.active && !user.frozen) {
      await expect(item).toHaveTheClass('revoked');
    }
  }
});

msTest('Check user grid items', async ({ usersPage }) => {
  await usersPage.locator('#activate-users-ms-action-bar').locator('.ms-grid-list-toggle').locator('#grid-view').click();
  const usersGrid = usersPage.locator('.users-container-grid');

  for (const [index, user] of USERS.entries()) {
    const card = usersGrid.locator('.user-card-item').nth(index);
    await expect(card.locator('.user-card-info').locator('.user-card-info__name').locator('span').nth(0)).toHaveText(user.name);
    await expect(card.locator('.user-card-info').locator('.user-card-info__email')).toHaveText(user.email);
    await expect(card.locator('.user-card-profile').locator('.label-profile')).toHaveText(user.profile);
    if (user.frozen || !user.active) {
      await expect(card.locator('.user-card-profile').locator('.label-status')).toBeVisible();
      await expect(card.locator('.user-card-profile').locator('.label-status')).toHaveText(getStatusForUser(user));
    } else {
      await expect(card.locator('.user-card-profile').locator('.label-status')).toBeHidden();
    }
  }
});

for (const revokedUser of [false, true]) {
  msTest(`Check user context menu for ${revokedUser ? 'revoked' : 'active'} user`, async ({ usersPage }) => {
    const item = usersPage
      .locator('#users-page-user-list')
      .getByRole('listitem')
      .nth(revokedUser ? 2 : 1);
    await expect(usersPage.locator('#user-context-menu')).toBeHidden();
    await item.hover();
    await item.locator('.options-button').click();
    await expect(usersPage.locator('#user-context-menu')).toBeVisible();
    const menu = usersPage.locator('#user-context-menu');
    const expectedActions = ['User details', 'View details', 'Copy roles', 'Copy workspace roles to...'];
    if (!revokedUser) {
      expectedActions.unshift(...['Deletion', 'Revoke this user']);
    }
    await expect(menu.getByRole('listitem')).toHaveText(expectedActions);
  });
}

for (const revokedUser of [false, true]) {
  msTest(`Check user context menu on right click for ${revokedUser ? 'revoked' : 'active'} user`, async ({ usersPage }) => {
    const item = usersPage
      .locator('#users-page-user-list')
      .getByRole('listitem')
      .nth(revokedUser ? 2 : 1);
    await expect(usersPage.locator('#user-context-menu')).toBeHidden();
    await item.click({ button: 'right' });
    await expect(usersPage.locator('#user-context-menu')).toBeVisible();
    const menu = usersPage.locator('#user-context-menu');
    const expectedActions = ['User details', 'View details', 'Copy roles', 'Copy workspace roles to...'];
    if (!revokedUser) {
      expectedActions.unshift(...['Deletion', 'Revoke this user']);
    }
    await expect(menu.getByRole('listitem')).toHaveText(expectedActions);
  });
}

msTest('Revoke one user with context menu', async ({ usersPage }) => {
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

msTest('Revoke one user with selection', async ({ usersPage }) => {
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

msTest('Revoke two users with selection', async ({ usersPage }) => {
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

msTest('Selection in grid mode', async ({ usersPage }) => {
  await usersPage.locator('#activate-users-ms-action-bar').locator('.ms-grid-list-toggle').locator('#grid-view').click();
  const item = usersPage.locator('.users-container-grid').locator('.user-card-item').nth(1);
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
});

msTest('Test users selection in list mode', async ({ usersPage }) => {
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
  const expectedSelected = USERS.filter((u) => (u.active || u.frozen) && u.name !== 'Gordon Freeman');
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

msTest('Maintain selection between modes', async ({ usersPage }) => {
  for (const index of [1, 3, 4]) {
    const item = usersPage.locator('#users-page-user-list').getByRole('listitem').nth(index);
    await item.hover();
    await item.locator('ion-checkbox').click();
  }

  const actionBar = usersPage.locator('#activate-users-ms-action-bar');
  await expect(actionBar.locator('.counter')).toHaveText('3 users selected', { useInnerText: true });
  // Check the checkboxes in list mode
  for (const [index, user] of USERS.entries()) {
    // Revoked users do not have a checkbox
    if (user.active && !user.currentUser) {
      const item = usersPage.locator('#users-page-user-list').getByRole('listitem').nth(index);
      await expect(item.locator('ion-checkbox')).toHaveState([1, 3, 4].includes(index) ? 'checked' : 'unchecked');
    }
  }

  // Switch to grid mode
  await usersPage.locator('#activate-users-ms-action-bar').locator('.ms-grid-list-toggle').locator('#grid-view').click();
  await expect(actionBar.locator('.counter')).toHaveText('3 users selected', { useInnerText: true });
  // Check the checkboxes in grid mode
  for (const [index, user] of USERS.entries()) {
    // Revoked users do not have a checkbox
    if (user.active && !user.currentUser) {
      const item = usersPage.locator('.users-container-grid').locator('.user-card-item').nth(index);
      await item.hover();
      await expect(item.locator('ion-checkbox')).toHaveState([1, 3, 4].includes(index) ? 'checked' : 'unchecked');
    }
  }
  // Uncheck one
  await usersPage.locator('.users-container-grid').locator('.user-card-item').nth(3).locator('ion-checkbox').click();
  await expect(actionBar.locator('.counter')).toHaveText('2 users selected', { useInnerText: true });
  for (const [index, user] of USERS.entries()) {
    if (user.active && !user.currentUser) {
      const item = usersPage.locator('.users-container-grid').locator('.user-card-item').nth(index);
      await item.hover();
      await expect(item.locator('ion-checkbox')).toHaveState([1, 4].includes(index) ? 'checked' : 'unchecked');
    }
  }

  // Back to list mode
  await usersPage.locator('#activate-users-ms-action-bar').locator('.ms-grid-list-toggle').locator('#list-view').click();

  // Check the checkboxes in list mode
  for (const [index, user] of USERS.entries()) {
    // Revoked users do not have a checkbox
    if (user.active && !user.currentUser) {
      const item = usersPage.locator('#users-page-user-list').getByRole('listitem').nth(index);
      await expect(item.locator('ion-checkbox')).toHaveState([1, 4].includes(index) ? 'checked' : 'unchecked');
    }
  }
});

msTest('User filter popover default state', async ({ usersPage }) => {
  await expect(usersPage.locator('.filter-popover')).toBeHidden();
  await usersPage.locator('#activate-users-ms-action-bar').locator('#select-filter-popover-button').click();
  await expect(usersPage.locator('.filter-popover')).toBeVisible();
  const popover = usersPage.locator('.filter-popover');
  await expect(popover.locator('#user-filter-list').getByRole('group')).toHaveCount(2);
  const statusGroup = popover.locator('#user-filter-list').getByRole('group').nth(0);
  await expect(statusGroup.locator('.list-group-title')).toHaveText('Status');
  await expect(statusGroup.getByRole('listitem')).toHaveCount(3);
  await expect(statusGroup.getByRole('listitem')).toHaveText(['Active', 'Revoked', 'Suspended']);
  for (const checkbox of await statusGroup.locator('ion-checkbox').all()) {
    await expect(checkbox).toHaveState('checked');
  }
  const profileGroup = popover.locator('#user-filter-list').getByRole('group').nth(1);
  await expect(profileGroup.locator('.list-group-title')).toHaveText('Profile');
  await expect(profileGroup.getByRole('listitem')).toHaveCount(3);
  await expect(profileGroup.getByRole('listitem')).toHaveText(['Administrator', 'Member', 'External']);
  for (const checkbox of await profileGroup.locator('ion-checkbox').all()) {
    await expect(checkbox).toHaveState('checked');
  }
});

async function toggleFilter(page: Page, name: string): Promise<void> {
  await page.locator('#activate-users-ms-action-bar').locator('#select-filter-popover-button').click();
  const popover = page.locator('.filter-popover');
  await popover.getByRole('listitem').filter({ hasText: name }).locator('ion-checkbox').click();
  // Click the backdrop to hide the popover
  await page.locator('.filter-popover').locator('ion-backdrop').click();
}

msTest('Filter users list', async ({ usersPage }) => {
  const usersList = usersPage.locator('#users-page-user-list');
  await expect(usersList.getByRole('listitem').locator('.user-name').locator('.person-name')).toHaveText(USERS.map((u) => u.name));
  // Hide admins
  await toggleFilter(usersPage, 'Administrator');
  await expect(usersList.getByRole('listitem').locator('.user-name').locator('.person-name')).toHaveText(
    USERS.filter((u) => u.profile !== 'Administrator').map((u) => u.name),
  );
  // Also hides revoked
  await toggleFilter(usersPage, 'Revoked');
  await expect(usersList.getByRole('listitem').locator('.user-name').locator('.person-name')).toHaveText(
    USERS.filter((u) => u.profile !== 'Administrator' && (u.active === true || u.frozen)).map((u) => u.name),
  );
  // Also hides external
  await toggleFilter(usersPage, 'External');
  await expect(usersList.getByRole('listitem').locator('.user-name').locator('.person-name')).toHaveText(
    USERS.filter((u) => u.profile !== 'Administrator' && u.profile !== 'External' && (u.active === true || u.frozen)).map((u) => u.name),
  );
  // Show admins again
  await toggleFilter(usersPage, 'Administrator');
  await expect(usersList.getByRole('listitem').locator('.user-name').locator('.person-name')).toHaveText(
    USERS.filter((u) => u.profile !== 'External' && u.active === true && !u.frozen).map((u) => u.name),
  );
  await expect(usersPage.locator('.no-match-result')).toBeHidden();
  // Also hide active users
  await toggleFilter(usersPage, 'Active');
  await expect(usersList.getByRole('listitem').locator('.user-name').locator('.person-name')).toHaveText([]);
  await expect(usersPage.locator('.no-match-result')).toBeVisible();
  await expect(usersPage.locator('.no-match-result')).toHaveText('No users matching your filters.');
});

msTest('Remove selection on filtering', async ({ usersPage }) => {
  const actionBar = usersPage.locator('#activate-users-ms-action-bar');
  await expect(actionBar.locator('.counter')).toHaveText(`${USERS.length} users`, { useInnerText: true });
  const item = usersPage.locator('#users-page-user-list').getByRole('listitem').nth(3);
  await item.hover();
  await item.locator('ion-checkbox').click();
  await expect(actionBar.locator('.counter')).toHaveText('One user selected', { useInnerText: true });
  await toggleFilter(usersPage, 'Member');
  const expectedUsers = USERS.filter((u) => u.currentUser || u.profile !== 'Member');
  await expect(actionBar.locator('.counter')).toHaveText(`${expectedUsers.length} users`, { useInnerText: true });
});

msTest('User sort popover default state', async ({ usersPage }) => {
  await expect(usersPage.locator('.sorter-popover')).toBeHidden();
  const sortButton = usersPage.locator('#activate-users-ms-action-bar').locator('#select-popover-button');
  await expect(sortButton).toHaveText('Profile');
  await sortButton.click();
  await expect(usersPage.locator('.sorter-popover')).toBeVisible();
  const popover = usersPage.locator('.sorter-popover');
  const items = popover.locator('.sorter-container').getByRole('listitem');
  await expect(items).toHaveCount(5);
  await expect(items).toHaveText(['Ascending', 'Name', 'Joining date', 'Profile', 'Status']);
  for (const [index, item] of (await items.all()).entries()) {
    if (index === 3) {
      await expect(item).toHaveTheClass('selected');
    } else {
      await expect(item).not.toHaveTheClass('selected');
    }
  }
});

msTest('Sort users list', async ({ usersPage }) => {
  const usersList = usersPage.locator('#users-page-user-list');
  const sortButton = usersPage.locator('#activate-users-ms-action-bar').locator('#select-popover-button');

  await sortBy(sortButton, 'Name');
  await expect(sortButton).toHaveText('Name');
  await expect(usersList.getByRole('listitem').locator('.user-name').locator('.person-name')).toHaveText(
    USERS.sort((u1, u2) => {
      if (u1.currentUser) {
        return -1;
      } else if (u2.currentUser) {
        return 1;
      } else {
        return u1.name.localeCompare(u2.name);
      }
    }).map((u) => u.name),
  );

  await sortBy(sortButton, 'Ascending');
  await expect(usersList.getByRole('listitem').locator('.user-name').locator('.person-name')).toHaveText(
    USERS.sort((u1, u2) => {
      if (u1.currentUser) {
        return -1;
      } else if (u2.currentUser) {
        return 1;
      } else {
        return u2.name.localeCompare(u1.name);
      }
    }).map((u) => u.name),
  );
});

msTest('Search user list', async ({ usersPage }) => {
  const searchInput = usersPage.locator('#search-input-users').locator('ion-input');
  const actionBar = usersPage.locator('#activate-users-ms-action-bar');
  const usersList = usersPage.locator('#users-page-user-list');
  const items = usersList.getByRole('listitem');
  const gridItems = usersPage.locator('.users-container-grid').locator('.user-card-item');

  await expect(actionBar.locator('.counter')).toHaveText('8 users', { useInnerText: true });
  await expect(items).toHaveCount(8);

  // No matches
  await fillIonInput(searchInput, 'abc');
  await expect(actionBar.locator('.counter')).toHaveText('No user', { useInnerText: true });
  await expect(usersList).toContainText('No users matching your filters');
  await expect(items).toHaveCount(0);

  // Search on email
  await fillIonInput(searchInput, 'gmail');
  await expect(actionBar.locator('.counter')).toHaveText('6 users', { useInnerText: true });
  await expect(items).toHaveCount(6);
  for (let i = 0; i < 6; i++) {
    await expect(items.nth(i).locator('.user-email')).toContainText('gmail');
  }

  // Search on name
  await fillIonInput(searchInput, 'he');
  await expect(actionBar.locator('.counter')).toHaveText('2 users', { useInnerText: true });
  // cspell:disable-next-line
  await expect(items.nth(0).locator('.user-name')).toContainText('Jaheira');
  await expect(items.nth(1).locator('.user-name')).toContainText('Patches');
  // cspell:disable-next-line
  await fillIonInput(searchInput, 'Valygar');
  await expect(actionBar.locator('.counter')).toHaveText('One user', { useInnerText: true });
  // cspell:disable-next-line
  await expect(items.nth(0).locator('.user-name')).toContainText('Valygar');

  // Check that selection resets on filter
  await fillIonInput(searchInput, '');
  await usersPage.locator('.user-list-header').locator('ion-checkbox').click();
  await expect(actionBar.locator('.counter')).toHaveText('4 users selected', { useInnerText: true });
  await fillIonInput(searchInput, 'he');
  await expect(actionBar.locator('.counter')).toHaveText('2 users selected', { useInnerText: true });
  await fillIonInput(searchInput, '');
  await expect(actionBar.locator('.counter')).toHaveText('2 users selected', { useInnerText: true });
  await expect(items).toHaveCount(8);
  // cspell:disable-next-line
  await expect(items.nth(1).locator('.user-name')).toContainText('Jaheira');
  await expect(items.nth(1).locator('ion-checkbox')).toHaveState('checked');
  await expect(items.nth(4).locator('.user-name')).toContainText('Patches');
  await expect(items.nth(4).locator('ion-checkbox')).toHaveState('checked');

  // Check that search persists on grid mode
  await fillIonInput(searchInput, 'he');
  await expect(items).toHaveCount(2);
  await actionBar.locator('.ms-grid-list-toggle').locator('#grid-view').click();
  await expect(gridItems).toHaveCount(2);
  await fillIonInput(searchInput, 'hei');
  await expect(gridItems).toHaveCount(1);
  // cspell:disable-next-line
  await expect(gridItems.nth(0).locator('.user-card-info__name')).toContainText('Jaheira');
  await actionBar.locator('.ms-grid-list-toggle').locator('#list-view').click();
  await expect(items).toHaveCount(1);
  // cspell:disable-next-line
  await expect(items.nth(0).locator('.user-name')).toContainText('Jaheira');
});

msTest('Search user grid', async ({ usersPage }) => {
  const searchInput = usersPage.locator('#search-input-users').locator('ion-input');
  const actionBar = usersPage.locator('#activate-users-ms-action-bar');
  const usersList = usersPage.locator('.users-container-grid');
  const items = usersList.locator('.user-card-item');

  await usersPage.locator('#activate-users-ms-action-bar').locator('.ms-grid-list-toggle').locator('#grid-view').click();

  await expect(actionBar.locator('.counter')).toHaveText('8 users', { useInnerText: true });
  await expect(items).toHaveCount(8);

  // No matches
  await fillIonInput(searchInput, 'abc');
  await expect(actionBar.locator('.counter')).toHaveText('No user', { useInnerText: true });
  await expect(usersList).toContainText('No users matching your filters');
  await expect(items).toHaveCount(0);

  // Search on email
  await fillIonInput(searchInput, 'gmail');
  await expect(actionBar.locator('.counter')).toHaveText('6 users', { useInnerText: true });
  await expect(items).toHaveCount(6);
  for (let i = 0; i < 6; i++) {
    await expect(items.nth(i).locator('.user-card-info__email')).toContainText('gmail');
  }

  // Search on name
  await fillIonInput(searchInput, 'he');
  await expect(actionBar.locator('.counter')).toHaveText('2 users', { useInnerText: true });
  // cspell:disable-next-line
  await expect(items.nth(0).locator('.user-card-info__name')).toContainText('Jaheira');
  await expect(items.nth(1).locator('.user-card-info__name')).toContainText('Patches');
  // cspell:disable-next-line
  await fillIonInput(searchInput, 'Valygar');
  await expect(actionBar.locator('.counter')).toHaveText('One user', { useInnerText: true });
  // cspell:disable-next-line
  await expect(items.nth(0).locator('.user-card-info__name')).toContainText('Valygar');

  // Check that selection resets on filter
  await fillIonInput(searchInput, '');
  await expect(items).toHaveCount(8);
  await items.nth(1).hover();
  await items.nth(1).locator('ion-checkbox').click();
  await items.nth(3).hover();
  await items.nth(3).locator('ion-checkbox').click();
  await items.nth(4).hover();
  await items.nth(4).locator('ion-checkbox').click();
  await items.nth(7).hover();
  await items.nth(7).locator('ion-checkbox').click();
  await expect(actionBar.locator('.counter')).toHaveText('4 users selected', { useInnerText: true });
  await fillIonInput(searchInput, 'he');
  await expect(actionBar.locator('.counter')).toHaveText('2 users selected', { useInnerText: true });
  await fillIonInput(searchInput, '');
  await expect(actionBar.locator('.counter')).toHaveText('2 users selected', { useInnerText: true });
  // cspell:disable-next-line
  await expect(items.nth(1).locator('.user-card-info__name')).toContainText('Jaheira');
  await expect(items.nth(1).locator('ion-checkbox')).toHaveState('checked');
  await expect(items.nth(4).locator('.user-card-info__name')).toContainText('Patches');
  await expect(items.nth(4).locator('ion-checkbox')).toHaveState('checked');
});

msTest('Invite new user', async ({ usersPage }) => {
  await usersPage.locator('#activate-users-ms-action-bar').locator('#button-invite-user').click();
  // cspell:disable-next-line
  await fillInputModal(usersPage, 'zana@wraeclast');
  // cspell:disable-next-line
  await expect(usersPage).toShowToast('An invitation to join the organization has been sent to zana@wraeclast.', 'Success');
});

msTest('Invite user with already existing email', async ({ usersPage }) => {
  await usersPage.locator('#activate-users-ms-action-bar').locator('#button-invite-user').click();
  await fillInputModal(usersPage, 'jaheira@gmail.com');
  await expect(usersPage).toShowToast('The email jaheira@gmail.com is already used by someone in this organization.', 'Error');
});

msTest('Reassign workspace role', async ({ usersPage }) => {
  const sourceUser = usersPage.locator('.users-container').locator('#users-page-user-list').locator('.user-list-item').nth(3);
  await sourceUser.hover();
  await sourceUser.locator('.options-button').click();
  const menuButton = usersPage.locator('.user-context-menu').getByRole('group').nth(2).getByRole('listitem').nth(1);
  await expect(menuButton).toHaveText('Copy workspace roles to...');
  await menuButton.click();
  const modal = usersPage.locator('.role-assignment-modal');
  await expect(modal).toBeVisible();
  const nextButton = modal.locator('#next-button');
  await expect(nextButton).toHaveText('Select');
  await expect(nextButton).toHaveDisabledAttribute();
  const input = modal.locator('#select-user-input').locator('ion-input');
  await fillIonInput(input, 'gmail');
  const dropdown = usersPage.locator('.user-select-dropdown-popover');
  await expect(dropdown.getByRole('listitem')).toHaveCount(2);
  // cspell:disable-next-line
  await expect(dropdown.getByRole('listitem').nth(0).locator('.option-text__label')).toHaveText('Jaheira');
  // cspell:disable-next-line
  await expect(dropdown.getByRole('listitem').nth(1).locator('.option-text__label')).toHaveText('Karl Hungus');
  await dropdown.getByRole('listitem').nth(1).click();
  // cspell:disable-next-line
  await expect(input.locator('input')).toHaveValue('Karl Hungus (karlhungus@gmail.com)');
  await expect(nextButton).not.toHaveDisabledAttribute();
  await nextButton.click();
  await usersPage.waitForTimeout(1000);
  const newRoles = modal.locator('.workspace-list').getByRole('listitem');
  await expect(newRoles).toHaveCount(1);
  await expect(newRoles.locator('.workspace-item__name')).toHaveText('Trademeet');
  await expect(newRoles.locator('.workspace-item__role-old')).toHaveText('Not shared');
  await expect(newRoles.locator('.workspace-item__role-new')).toHaveText('Reader');
  await nextButton.click();
});
