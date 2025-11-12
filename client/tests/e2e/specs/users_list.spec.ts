// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { Locator, Page } from '@playwright/test';
import { addUser, answerQuestion, DisplaySize, expect, fillIonInput, inviteUsers, msTest, resizePage, sortBy } from '@tests/e2e/helpers';

const USERS = [
  {
    name: 'Alicey McAliceFace',
    email: 'alice@example.com',
    profile: 'Administrator',
    active: true,
    currentUser: true,
    frozen: false,
  },
  {
    // cspell:disable-next-line
    name: 'Boby McBobFace',
    email: 'bob@example.com',
    profile: 'Member',
    active: true,
    frozen: false,
  },
  {
    // cspell:disable-next-line
    name: 'Malloryy McMalloryFace',
    email: 'mallory@example.com',
    profile: 'External',
    active: true,
    frozen: false,
  },
];

msTest('User list default state', async ({ usersPage }) => {
  const actionBar = usersPage.locator('#activate-users-ms-action-bar');
  await expect(actionBar.getByText('Invite a user')).toBeVisible();
  await expect(actionBar.getByText('Invite a user')).toHaveText('Invite a user');
  await expect(actionBar.locator('.counter')).toHaveText(`${USERS.length} users`, { useInnerText: true });
  await expect(usersPage.locator('.user-list-header').locator('ion-checkbox')).toHaveState('unchecked');
  await expect(actionBar.locator('#select-popover-button')).toHaveText('Profile');
  await expect(actionBar.locator('.ms-grid-list-toggle').locator('#grid-view')).toNotHaveDisabledAttribute();
  await expect(actionBar.locator('.ms-grid-list-toggle').locator('#list-view')).toHaveDisabledAttribute();
  await expect(usersPage.locator('#users-page-user-list').getByRole('listitem')).toHaveCount(USERS.length);
});

function getStatusForUser(user: any): string {
  if (user.frozen) {
    return 'Suspended';
  }
  return user.active ? 'Active' : 'Revoked';
}

for (const displaySize of ['small', 'large']) {
  msTest(`Check user list items on ${displaySize} display`, async ({ usersPage }) => {
    if (displaySize === 'small') {
      await usersPage.setDisplaySize(DisplaySize.Small);
    }

    const usersList = usersPage.locator('#users-page-user-list');
    for (const [index, user] of USERS.entries()) {
      const item = usersList.getByRole('listitem').nth(index);

      if (displaySize === 'small') {
        await expect(item.locator('.user-mobile-text').locator('.user-mobile-text__name')).toHaveText(user.name);
        await expect(item.locator('.user-mobile-text').locator('.user-mobile-text__email')).toHaveText(user.email);
        await expect(item.locator('.user-mobile-text').locator('.user-mobile-text__profile')).toHaveText(user.profile);
        await expect(item.locator('.user-profile')).toBeHidden();
        await expect(item.locator('.user-email')).toBeHidden();
        await expect(item.locator('.user-status')).toBeHidden();
        await expect(item.locator('.user-options')).toBeVisible();
      } else {
        await expect(item.locator('.user-name').locator('.person-name')).toHaveText(user.name);
        await expect(item.locator('.user-profile')).toHaveText(user.profile);
        await expect(item.locator('.user-email')).toHaveText(user.email);
        await expect(item.locator('.user-status')).toHaveText(getStatusForUser(user));
        await expect(item.locator('.user-options').locator('.options-button')).toBeHidden();
        if (!user.active && !user.frozen) {
          await expect(item).toHaveTheClass('revoked');
        }
      }
    }
  });
}

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

msTest('Revoke one user with context menu and check context menu', async ({ usersPage }) => {
  const item = usersPage.locator('#users-page-user-list').getByRole('listitem').nth(1);
  await item.hover();
  const menu = usersPage.locator('#user-context-menu');
  await expect(menu).toBeHidden();
  // Opens context menu with button
  await item.locator('.options-button').click();
  await expect(menu).toBeVisible();
  // Full context menu
  await expect(menu.getByRole('listitem')).toHaveText([
    'Deletion',
    'Revoke this user',
    'Profile',
    'Change profile',
    'User details',
    'View details',
    'Copy roles',
    'Copy workspace roles',
  ]);

  // Revoke the user
  await menu.getByRole('listitem').nth(1).click();
  await answerQuestion(usersPage, true, {
    expectedTitleText: 'Revoke this user?',
    expectedQuestionText:
      'This will revoke Boby McBobFace, preventing them from accessing this organization. Are you sure you want to proceed?',
    expectedPositiveText: 'Revoke',
    expectedNegativeText: 'Cancel',
  });
  await expect(usersPage).toShowToast('Boby McBobFace has been revoked. They can no longer access this organization.', 'Success');

  // Opens context menu with right click
  await expect(menu).toBeHidden();
  await item.click({ button: 'right' });
  await expect(menu).toBeVisible();
  await expect(menu.getByRole('listitem')).toHaveText(['User details', 'View details', 'Copy roles', 'Copy workspace roles']);
});

msTest('Revoke one user with selection', async ({ usersPage }) => {
  const item = usersPage.locator('#users-page-user-list').getByRole('listitem').nth(1);
  await item.hover();
  await item.locator('ion-checkbox').click();
  await usersPage.locator('#activate-users-ms-action-bar').getByText('Revoke this user').click();
  await answerQuestion(usersPage, true, {
    expectedTitleText: 'Revoke this user?',
    expectedQuestionText:
      'This will revoke Boby McBobFace, preventing them from accessing this organization. Are you sure you want to proceed?',
    expectedPositiveText: 'Revoke',
    expectedNegativeText: 'Cancel',
  });
  await expect(usersPage).toShowToast('Boby McBobFace has been revoked. They can no longer access this organization.', 'Success');
});

msTest('Revoke two users with selection', async ({ usersPage }) => {
  const item1 = usersPage.locator('#users-page-user-list').getByRole('listitem').nth(1);
  await item1.hover();
  await item1.locator('ion-checkbox').click();

  const item2 = usersPage.locator('#users-page-user-list').getByRole('listitem').nth(2);
  await item2.hover();
  await item2.locator('ion-checkbox').click();

  await usersPage.locator('#activate-users-ms-action-bar').getByText('Revoke these users').click();
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
  await expect(actionBar.getByText('Invite a user')).toBeHidden();
  await expect(actionBar.getByText('Revoke this user')).toBeVisible();
  await expect(actionBar.getByText('Revoke this user')).toHaveText('Revoke this user');
  await expect(actionBar.getByText('Details')).toBeVisible();
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
  await expect(actionBar.getByText('Invite a user')).toBeHidden();
  await expect(actionBar.getByText('Revoke this user')).toBeVisible();
  await expect(actionBar.getByText('Revoke this user')).toHaveText('Revoke this user');
  await expect(actionBar.getByText('Details')).toBeVisible();
  await expect(actionBar.locator('.counter')).toHaveText('One user selected', { useInnerText: true });

  const headerCheckbox = usersPage.locator('.user-list-header').locator('ion-checkbox');
  // Header checkbox should be indeterminate since not all users are selected
  await expect(headerCheckbox).toHaveState('indeterminate');
  // Unselecting the user
  await item.locator('ion-checkbox').click();
  await expect(item.locator('ion-checkbox')).toHaveState('unchecked');
  // Header checkbox should be unchecked
  await expect(headerCheckbox).toHaveState('unchecked');

  // Select all with header checkbox
  await headerCheckbox.click();
  for (const [index, user] of USERS.entries()) {
    // Only active and not current user
    if (user.active && index > 0) {
      const checkbox = usersPage.locator('#users-page-user-list').getByRole('listitem').nth(index).locator('ion-checkbox');
      await expect(checkbox).toHaveState('checked');
    }
  }
  await expect(actionBar.locator('.counter')).toHaveText(`${USERS.length - 1} users selected`, { useInnerText: true });
  await expect(actionBar.getByText('Revoke these users')).toBeVisible();

  const expectedSelected = USERS.filter((u) => (u.active || u.frozen) && u.name !== 'Alicey McAliceFace');
  await expect(actionBar.locator('.counter')).toHaveText(`${expectedSelected.length} users selected`, { useInnerText: true });
  await expect(headerCheckbox).toHaveState('checked');
  // Unselect one
  await usersPage.locator('#users-page-user-list').getByRole('listitem').nth(2).locator('ion-checkbox').click();
  await expect(usersPage.locator('#users-page-user-list').getByRole('listitem').nth(2).locator('ion-checkbox')).toHaveState('unchecked');
  // Header checkbox goes to indeterminate
  await expect(headerCheckbox).toHaveState('indeterminate');
  // Reselect the user
  await usersPage.locator('#users-page-user-list').getByRole('listitem').nth(2).locator('ion-checkbox').click();
  await expect(usersPage.locator('#users-page-user-list').getByRole('listitem').nth(2).locator('ion-checkbox')).toHaveState('checked');
  // Header checkbox goes back to checked
  await expect(headerCheckbox).toHaveState('checked');

  // Unselect all
  await headerCheckbox.click();
  await expect(headerCheckbox).toHaveState('unchecked');
});

msTest('Maintain selection between modes', async ({ usersPage }) => {
  for (const index of [1, 2]) {
    const item = usersPage.locator('#users-page-user-list').getByRole('listitem').nth(index);
    await item.hover();
    await item.locator('ion-checkbox').click();
  }

  const actionBar = usersPage.locator('#activate-users-ms-action-bar');
  await expect(actionBar.locator('.counter')).toHaveText('2 users selected', { useInnerText: true });
  // Check the checkboxes in list mode
  for (const [index, user] of USERS.entries()) {
    // Revoked users do not have a checkbox
    if (user.active && !user.currentUser) {
      const item = usersPage.locator('#users-page-user-list').getByRole('listitem').nth(index);
      await expect(item.locator('ion-checkbox')).toHaveState([1, 2].includes(index) ? 'checked' : 'unchecked');
    }
  }

  // Switch to grid mode
  await usersPage.locator('#activate-users-ms-action-bar').locator('.ms-grid-list-toggle').locator('#grid-view').click();
  await expect(actionBar.locator('.counter')).toHaveText('2 users selected', { useInnerText: true });
  // Check the checkboxes in grid mode
  for (const [index, user] of USERS.entries()) {
    // Revoked users do not have a checkbox
    if (user.active && !user.currentUser) {
      const item = usersPage.locator('.users-container-grid').locator('.user-card-item').nth(index);
      await item.hover();
      await expect(item.locator('ion-checkbox')).toHaveState([1, 2].includes(index) ? 'checked' : 'unchecked');
    }
  }
  // Uncheck one
  await usersPage.locator('.users-container-grid').locator('.user-card-item').nth(2).locator('ion-checkbox').click();
  await expect(actionBar.locator('.counter')).toHaveText('One user selected', { useInnerText: true });
  for (const [index, user] of USERS.entries()) {
    if (user.active && !user.currentUser) {
      const item = usersPage.locator('.users-container-grid').locator('.user-card-item').nth(index);
      await item.hover();
      await expect(item.locator('ion-checkbox')).toHaveState([1].includes(index) ? 'checked' : 'unchecked');
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
  await expect(popover).toBeVisible();
  await popover.getByRole('listitem').filter({ hasText: name }).locator('ion-checkbox').click();
  // Click the backdrop to hide the popover
  await popover.locator('ion-backdrop').click();
  await expect(popover).toBeHidden();
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
  const item = usersPage.locator('#users-page-user-list').getByRole('listitem').nth(1);
  await item.hover();
  await item.locator('ion-checkbox').click();
  await expect(actionBar.locator('.counter')).toHaveText('One user selected', { useInnerText: true });
  await toggleFilter(usersPage, 'Member');
  await expect(actionBar.locator('.counter')).toHaveText('2 users', { useInnerText: true });
});

msTest('User sort popover default state', async ({ usersPage }) => {
  await expect(usersPage.locator('.sorter-popover')).toBeHidden();
  const sortButton = usersPage.locator('#activate-users-ms-action-bar').locator('#select-popover-button');
  await expect(sortButton).toHaveText('Profile');
  await sortButton.click();
  await expect(usersPage.locator('.sorter-popover')).toBeVisible();
  const popover = usersPage.locator('.sorter-popover');
  const items = popover.locator('.sorter-container').getByRole('listitem');
  await expect(items).toHaveCount(4);
  await expect(items).toHaveText(['Ascending', 'Name', 'Joining date', 'Profile']);
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

  await expect(actionBar.locator('.counter')).toHaveText('3 users', { useInnerText: true });
  await expect(items).toHaveCount(3);

  // No matches
  await fillIonInput(searchInput, 'abc');
  await expect(actionBar.locator('.counter')).toHaveText('No user', { useInnerText: true });
  await expect(usersList).toContainText('No users matching your filters');
  await expect(items).toHaveCount(0);

  // Search on email
  await fillIonInput(searchInput, 'example');
  await expect(actionBar.locator('.counter')).toHaveText('3 users', { useInnerText: true });
  await expect(items).toHaveCount(3);
  for (let i = 0; i < 3; i++) {
    await expect(items.nth(i).locator('.user-email')).toContainText('example');
  }

  // Search on name
  await fillIonInput(searchInput, 'al');
  await expect(actionBar.locator('.counter')).toHaveText('2 users', { useInnerText: true });
  // cspell:disable-next-line
  await expect(items.nth(0).locator('.user-name')).toContainText('Alicey McAliceFace');
  await expect(items.nth(1).locator('.user-name')).toContainText('Malloryy McMalloryFace');
  // cspell:disable-next-line
  await fillIonInput(searchInput, 'Bob');
  await expect(actionBar.locator('.counter')).toHaveText('One user', { useInnerText: true });
  // cspell:disable-next-line
  await expect(items.nth(0).locator('.user-name')).toContainText('Boby McBobFace');

  // Check that selection resets on filter
  await fillIonInput(searchInput, '');
  await usersPage.locator('.user-list-header').locator('ion-checkbox').click();
  await expect(actionBar.locator('.counter')).toHaveText('2 users selected', { useInnerText: true });
  await fillIonInput(searchInput, 'Bob');
  await expect(actionBar.locator('.counter')).toHaveText('One user selected', { useInnerText: true });
  await fillIonInput(searchInput, '');
  await expect(actionBar.locator('.counter')).toHaveText('One user selected', { useInnerText: true });
  await expect(items).toHaveCount(3);
  await expect(items.nth(1).locator('.user-name')).toContainText('Boby McBobFace');
  await expect(items.nth(1).locator('ion-checkbox')).toHaveState('checked');
  await expect(items.nth(2).locator('.user-name')).toContainText('Malloryy McMalloryFace');
  await expect(items.nth(2).locator('ion-checkbox')).toHaveState('unchecked');

  // Check that search persists on grid mode
  await fillIonInput(searchInput, 'al');
  await expect(items).toHaveCount(2);
  await actionBar.locator('.ms-grid-list-toggle').locator('#grid-view').click();
  await expect(gridItems).toHaveCount(2);
  // cspell:disable-next-line
  await fillIonInput(searchInput, 'allo');
  await expect(gridItems).toHaveCount(1);
  await expect(gridItems.nth(0).locator('.user-card-info__name')).toContainText('Malloryy McMalloryFace');
  await actionBar.locator('.ms-grid-list-toggle').locator('#list-view').click();
  await expect(items).toHaveCount(1);
  await expect(items.nth(0).locator('.user-name')).toContainText('Malloryy McMalloryFace');
});

msTest('Search user grid', async ({ usersPage }) => {
  const searchInput = usersPage.locator('#search-input-users').locator('ion-input');
  const actionBar = usersPage.locator('#activate-users-ms-action-bar');
  const usersList = usersPage.locator('.users-container-grid');
  const items = usersList.locator('.user-card-item');

  await usersPage.locator('#activate-users-ms-action-bar').locator('.ms-grid-list-toggle').locator('#grid-view').click();

  await expect(actionBar.locator('.counter')).toHaveText('3 users', { useInnerText: true });
  await expect(items).toHaveCount(3);

  // No matches
  await fillIonInput(searchInput, 'abc');
  await expect(actionBar.locator('.counter')).toHaveText('No user', { useInnerText: true });
  await expect(usersList).toContainText('No users matching your filters');
  await expect(items).toHaveCount(0);

  // Search on email
  await fillIonInput(searchInput, 'example');
  await expect(actionBar.locator('.counter')).toHaveText('3 users', { useInnerText: true });
  await expect(items).toHaveCount(3);
  for (let i = 0; i < 3; i++) {
    await expect(items.nth(i).locator('.user-card-info__email')).toContainText('example');
  }

  // Search on name
  await fillIonInput(searchInput, 'al');
  await expect(actionBar.locator('.counter')).toHaveText('2 users', { useInnerText: true });
  await expect(items.nth(0).locator('.user-card-info__name')).toContainText('Alicey McAliceFace');
  await expect(items.nth(1).locator('.user-card-info__name')).toContainText('Malloryy McMalloryFace');
  await fillIonInput(searchInput, 'Bob');
  await expect(actionBar.locator('.counter')).toHaveText('One user', { useInnerText: true });
  await expect(items.nth(0).locator('.user-card-info__name')).toContainText('Boby McBobFace');

  // Check that selection resets on filter
  await fillIonInput(searchInput, '');
  await expect(items).toHaveCount(3);
  await items.nth(1).hover();
  await items.nth(1).locator('ion-checkbox').click();
  await items.nth(2).hover();
  await items.nth(2).locator('ion-checkbox').click();
  await expect(actionBar.locator('.counter')).toHaveText('2 users selected', { useInnerText: true });
  await fillIonInput(searchInput, 'Bob');
  await expect(actionBar.locator('.counter')).toHaveText('One user selected', { useInnerText: true });
  await fillIonInput(searchInput, '');
  await expect(actionBar.locator('.counter')).toHaveText('One user selected', { useInnerText: true });

  await expect(items.nth(1).locator('.user-card-info__name')).toContainText('Boby McBobFace');
  await expect(items.nth(1).locator('ion-checkbox')).toHaveState('checked');
  await expect(items.nth(2).locator('.user-card-info__name')).toContainText('Malloryy McMalloryFace');
  await expect(items.nth(2).locator('ion-checkbox')).toHaveState('unchecked');
});

msTest('Invite new user', async ({ usersPage }) => {
  await usersPage.locator('#activate-users-ms-action-bar').getByText('Invite a user').click();
  // cspell:disable-next-line
  await inviteUsers(usersPage, 'zana@wraeclast');
  await expect(usersPage).toBeInvitationPage();
  // cspell:disable-next-line
  await expect(usersPage).toShowToast('An invitation to join the organization has been sent to zana@wraeclast.', 'Success');
});

msTest('Reassign workspace role', async ({ usersPage }) => {
  const sourceUser = usersPage.locator('.users-container').locator('#users-page-user-list').locator('.user-list-item').nth(1);
  await sourceUser.hover();
  await sourceUser.locator('.options-button').click();
  const menuButton = usersPage.locator('.user-context-menu').getByRole('group').nth(3).getByRole('listitem').nth(1);
  await expect(menuButton).toHaveText('Copy workspace roles');
  await menuButton.click();
  const modal = usersPage.locator('.role-assignment-modal');
  await expect(modal).toBeVisible();
  const nextButton = modal.locator('#next-button');
  await expect(nextButton).toBeHidden();
  const input = modal.locator('#select-user-input').locator('ion-input');
  await fillIonInput(input, 'example');
  const dropdown = usersPage.locator('.user-select-dropdown-overlay');
  await expect(dropdown.getByRole('listitem')).toHaveCount(2);
  await expect(dropdown.getByRole('listitem').nth(0).locator('.option-text__label')).toHaveText('Alicey McAliceFace');
  await expect(dropdown.getByRole('listitem').nth(0).locator('.option-warning')).toContainText('You cannot copy roles to yourself.');
  await expect(dropdown.getByRole('listitem').nth(1).locator('.option-text__label')).toHaveText('Malloryy McMalloryFace');
  await dropdown.getByRole('listitem').nth(1).click();
  const newRoles = modal.locator('.workspace-list').getByRole('listitem');
  await expect(newRoles).toHaveCount(1);
  await expect(newRoles.locator('.workspace-item__name')).toHaveText('wksp1');
  await expect(newRoles.locator('.workspace-item__role-old')).toHaveText('Not shared');
  await expect(newRoles.locator('.workspace-item__role-new')).toHaveText('Reader');
  await nextButton.click();
});

msTest('Update profile', async ({ usersPage }) => {
  const sourceUser = usersPage.locator('.users-container').locator('#users-page-user-list').locator('.user-list-item').nth(1);
  await sourceUser.hover();
  await sourceUser.locator('.options-button').click();
  const menuButton = usersPage.locator('.user-context-menu').getByRole('group').nth(1).getByRole('listitem').nth(1);
  await expect(menuButton).toHaveText('Change profile');
  await menuButton.click();
  const modal = usersPage.locator('.update-profile-modal');
  const modalContent = modal.locator('.ms-modal-content');
  await expect(modal).toBeVisible();
  const nextButton = modal.locator('#next-button');
  await expect(nextButton).toHaveText('Change');
  await expect(nextButton).toHaveDisabledAttribute();
  await expect(modalContent.locator('.update-profile-user__item').nth(0)).toHaveText('Boby McBobFace');
  const profileButton = modalContent.locator('#dropdown-popover-button');
  await expect(profileButton).toHaveText('Choose a profile');
  await profileButton.click();
  const profileDropdown = usersPage.locator('.dropdown-popover');
  await expect(profileDropdown.getByRole('listitem').locator('.option-text__label')).toHaveText(['Administrator', 'Member']);
  await expect(profileDropdown.getByRole('listitem').nth(1)).toHaveTheClass('item-disabled');
  await profileDropdown.getByRole('listitem').nth(0).click();
  await expect(profileButton).toHaveText('Administrator');
  await expect(nextButton).toBeTrulyEnabled();
  await nextButton.click();
  await expect(usersPage).toShowToast('The profile has been changed!', 'Success');
});

msTest('Update multiple profiles', async ({ usersPage }) => {
  msTest.setTimeout(120_000);

  const secondTab = await usersPage.openNewTab();
  await addUser(usersPage, secondTab, 'Gordon Freeman', 'gordon.freeman@blackmesa.nm', 'standard');
  if (secondTab.release) {
    // Liberating the second tab speeds up the test a lot
    await secondTab.release();
  }

  const users = usersPage.locator('#users-page-user-list').getByRole('listitem');
  await expect(users.locator('.user-profile')).toHaveText(['Administrator', 'Member', 'Member', 'External']);

  for (const index of [1, 2, 3]) {
    const item = usersPage.locator('#users-page-user-list').getByRole('listitem').nth(index);
    await item.hover();
    await item.locator('ion-checkbox').click();
  }
  await expect(usersPage.locator('#activate-users-ms-action-bar').getByText('Change profiles')).toHaveText('Change profiles');
  await usersPage.locator('#activate-users-ms-action-bar').getByText('Change profiles').click();
  const modal = usersPage.locator('.update-profile-modal');
  const modalContent = modal.locator('.ms-modal-content');
  await expect(modal).toBeVisible();
  const nextButton = modal.locator('#next-button');
  await expect(nextButton).toHaveText('Change');
  await expect(nextButton).toHaveDisabledAttribute();
  await expect(modalContent.locator('.update-profile-user__item')).toHaveText(['Boby McBobFace', 'Gordon Freeman']);
  await expect(modalContent.locator('.update-profile-user__more')).toBeHidden();
  await expect(modalContent.locator('.warn-outsiders')).toBeVisible();
  const profileButton = modalContent.locator('#dropdown-popover-button');
  await expect(profileButton).toHaveText('Choose a profile');
  await profileButton.click();
  const profileDropdown = usersPage.locator('.dropdown-popover');
  await expect(profileDropdown.getByRole('listitem').locator('.option-text__label')).toHaveText(['Administrator', 'Member']);
  await profileDropdown.getByRole('listitem').nth(0).click();
  await expect(profileButton).toHaveText('Administrator');
  await expect(nextButton).toBeTrulyEnabled();
  await nextButton.click();
  await expect(modal).toBeHidden();
  await expect(usersPage).toShowToast('The profiles have been changed!', 'Success');
  await expect(users.locator('.user-profile')).toHaveText(['Administrator', 'Administrator', 'Administrator', 'External']);
});

msTest('Small display selection', async ({ usersPage }) => {
  await usersPage.setDisplaySize(DisplaySize.Small);

  const headerTexts = usersPage.locator('.small-display-header-title').locator('ion-text');
  const headerOption = usersPage.locator('.small-display-header-title').locator('ion-icon');
  await expect(headerTexts).toHaveCount(1);
  await expect(headerTexts.nth(0)).toHaveText('Users');
  const user1 = usersPage.locator('#users-page-user-list').getByRole('listitem').nth(1);
  const user2 = usersPage.locator('#users-page-user-list').getByRole('listitem').nth(2);
  await expect(user1.locator('ion-checkbox')).not.toBeVisible();
  await headerOption.click();

  const modal = usersPage.locator('.user-context-sheet-modal');
  await expect(modal).toBeVisible();
  await expect(modal.locator('.button-left')).toHaveText('Selection');
  await expect(modal.locator('.button-right')).toHaveText('Select all');

  // Selection
  await modal.locator('.button-right').click();
  await expect(modal).not.toBeVisible();
  await expect(headerTexts).toHaveCount(3);
  await expect(headerTexts.nth(0)).toHaveText('Unselect');
  await expect(headerTexts.nth(1)).toHaveText('2 users selected');
  await expect(headerTexts.nth(2)).toHaveText('Cancel');
  await expect(user1.locator('ion-checkbox')).toBeVisible();
  await expect(user2.locator('ion-checkbox')).toBeVisible();
  await expect(user1.locator('ion-checkbox')).toHaveState('checked');
  await expect(user2.locator('ion-checkbox')).toHaveState('checked');
  await user1.locator('ion-checkbox').click();
  await expect(user1.locator('ion-checkbox')).toHaveState('unchecked');
  await expect(headerTexts.nth(1)).toHaveText('One user selected');
  await user1.locator('ion-checkbox').click();
  await expect(headerTexts.nth(1)).toHaveText('2 users selected');
  await headerTexts.nth(0).click();
  await expect(headerTexts.nth(1)).toHaveText('Users');
  await expect(user1.locator('ion-checkbox')).toHaveState('unchecked');
  await expect(user2.locator('ion-checkbox')).toHaveState('unchecked');
  await headerTexts.nth(2).click();
  await expect(user1.locator('ion-checkbox')).not.toBeVisible();
  await expect(user2.locator('ion-checkbox')).not.toBeVisible();
  await expect(headerTexts).toHaveCount(1);
});

msTest('Small display member context menu', async ({ usersPage }) => {
  await usersPage.setDisplaySize(DisplaySize.Small);
  const user1 = usersPage.locator('#users-page-user-list').getByRole('listitem').nth(1);
  const modal = usersPage.locator('.user-context-sheet-modal');

  await user1.locator('.user-options').click();
  await expect(modal).toBeVisible();
  await expect(modal.getByRole('group')).toHaveCount(2);
  await expect(modal.getByRole('listitem')).toHaveText(['Revoke this user', 'Copy workspace roles', 'Change profile', 'View details']);
});

msTest('Small display external context menu', async ({ usersPage }) => {
  await usersPage.setDisplaySize(DisplaySize.Small);
  const user2 = usersPage.locator('#users-page-user-list').getByRole('listitem').nth(2);
  const modal = usersPage.locator('.user-context-sheet-modal');

  await user2.locator('.user-options').click();
  await expect(modal).toBeVisible();
  await expect(modal.getByRole('group')).toHaveCount(2);
  await expect(modal.getByRole('listitem')).toHaveText(['Revoke this user', 'Copy workspace roles', 'View details']);
});

msTest('Small display multiple users context menu', async ({ usersPage }) => {
  await usersPage.setDisplaySize(DisplaySize.Small);
  const user1 = usersPage.locator('#users-page-user-list').getByRole('listitem').nth(1);
  const headerOption = usersPage.locator('.small-display-header-title').locator('ion-icon');
  const modal = usersPage.locator('.user-context-sheet-modal');
  await headerOption.click();

  await expect(modal).toBeVisible();
  await modal.locator('.button-right').click();

  await user1.locator('.user-options').click();
  await expect(modal.getByRole('group')).toHaveCount(1);
  await expect(modal.getByRole('listitem')).toHaveText(['Revoke these users', 'Change profiles']);
  await modal.getByRole('listitem').nth(1).click();
  await expect(usersPage.locator('.update-profile-modal').locator('.ms-modal-content').locator('.warn-outsiders')).toBeVisible();
});

msTest('No context menu with standard users and multiple selected', async ({ workspacesStandard }) => {
  await workspacesStandard.locator('.sidebar').locator('.sidebar-content-organization-button').nth(0).click();
  await expect(workspacesStandard).toHavePageTitle('Users');
  await expect(workspacesStandard).toBeUserPage();
  const usersPage = workspacesStandard;

  let item!: Locator;
  for (let i = 1; i < 3; i++) {
    item = usersPage.locator('#users-page-user-list').getByRole('listitem').nth(i);
    await item.hover();
    await item.locator('ion-checkbox').click();
  }
  const menu = usersPage.locator('#user-context-menu');
  await expect(menu).toBeHidden();
  await item.click({ button: 'right' });
  await expect(menu).toBeHidden();
});

msTest('Check in UsersPage if action bar updates after resized', async ({ usersPage }) => {
  const actionBar = usersPage.locator('#activate-users-ms-action-bar');
  const actionsBarButtons = actionBar.locator('.ms-action-bar-button--visible');
  const actionBarMoreButton = actionBar.locator('#action-bar-more-button');

  await resizePage(usersPage, 1600);
  await expect(actionBar).toBeVisible();
  await expect(actionsBarButtons).toHaveCount(2);
  await expect(actionsBarButtons).toHaveText(['Invite a user', 'Copy link (PKI)']);
  await expect(actionBarMoreButton).toBeHidden();

  await resizePage(usersPage, 1240);
  await expect(actionsBarButtons).toHaveText(['Invite a user']);
  await expect(actionBarMoreButton).toBeVisible();
  await actionBarMoreButton.click();
  await expect(usersPage.locator('.popover-viewport').getByRole('listitem')).toHaveText(['Copy link (PKI)']);
  await usersPage.keyboard.press('Escape');
  await expect(usersPage.locator('.popover-viewport')).toBeHidden();

  const firstUserEntry = usersPage.locator('.users-container').locator('.user-list-item').nth(1);
  await firstUserEntry.hover();
  await firstUserEntry.locator('ion-checkbox').isVisible();
  await firstUserEntry.locator('ion-checkbox').click();
  await expect(actionsBarButtons).toHaveText(['Revoke this user']);
  await actionBarMoreButton.click();
  await expect(usersPage.locator('.popover-viewport').getByRole('listitem')).toHaveText(['Change profile', 'View details']);
  await usersPage.keyboard.press('Escape');
  await expect(usersPage.locator('.popover-viewport')).toBeHidden();
  await expect(actionBarMoreButton).toBeVisible();

  // Finally, check if action bar buttons are updated after toggling the sidebar menu
  const topbarButton = usersPage.locator('#connected-header').locator('#trigger-toggle-menu-button');
  await expect(topbarButton).toBeVisible();
  await topbarButton.click();
  await expect(actionsBarButtons).toHaveText(['Revoke this user', 'Change profile', 'View details']);
  await expect(actionBarMoreButton).toBeHidden();
});
