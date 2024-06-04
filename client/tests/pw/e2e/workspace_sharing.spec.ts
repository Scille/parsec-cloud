// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { expect } from '@tests/pw/helpers/assertions';
import { msTest } from '@tests/pw/helpers/fixtures';
import { fillIonInput } from '@tests/pw/helpers/utils';

msTest('Workspace sharing modal default state', async ({ workspaceSharingModal }) => {
  await expect(workspaceSharingModal.locator('.ms-modal-header__title')).toHaveText('Share this workspace');
  const content = workspaceSharingModal.locator('.ms-modal-content');
  await expect(content.locator('.modal-title')).toHaveText('Trademeet');
  await expect(content.locator('.only-owner-warning')).toBeVisible();
  const users = content.locator('.user-list').locator('.content');
  await expect(users).toHaveCount(3);
  // cspell:disable-next-line
  await expect(users.locator('.person-name')).toHaveText(['Gordon Freeman', 'Korgan Bloodaxe', 'Jaheira']);
  await expect(users.locator('.filter-button')).toHaveText(['Owner', 'Reader', 'Not shared']);
  await expect(users.nth(0).locator('.filter-button')).toHaveDisabledAttribute();
});

msTest('Update user role', async ({ workspaceSharingModal }) => {
  const content = workspaceSharingModal.locator('.ms-modal-content');
  const user3 = content.locator('.user-list').locator('.content').nth(2);
  await expect(user3.locator('.filter-button')).toHaveText('Not shared');
  await user3.locator('.filter-button').click();
  const roleDropdown = workspaceSharingModal.page().locator('.dropdown-popover');
  const roles = roleDropdown.getByRole('listitem');
  await expect(roles.locator('.option-text__label')).toHaveText(['Owner', 'Manager', 'Contributor', 'Reader', 'Not shared']);
  // Use is outsider, owner and manager should be disabled
  for (const [index, role] of (await roles.all()).entries()) {
    if (index === 0 || index === 1) {
      await expect(role).toHaveTheClass('item-disabled');
    } else {
      await expect(role).not.toHaveTheClass('item-disabled');
    }
  }
  // Set contributor
  await roles.nth(2).click();
  // cspell:disable-next-line
  await expect(workspaceSharingModal.page()).toShowToast("Jaheira's role has been updated to Contributor.", 'Success');
  await expect(user3.locator('.filter-button')).toHaveText('Contributor');
});

msTest('Unshare workspace', async ({ workspaceSharingModal }) => {
  const content = workspaceSharingModal.locator('.ms-modal-content');
  const user2 = content.locator('.user-list').locator('.content').nth(1);
  await expect(user2.locator('.filter-button')).toHaveText('Reader');
  await user2.locator('.filter-button').click();
  const roleDropdown = workspaceSharingModal.page().locator('.dropdown-popover');
  const roles = roleDropdown.getByRole('listitem');
  // Unshare
  await roles.nth(4).click();
  // cspell:disable-next-line
  await expect(workspaceSharingModal.page()).toShowToast('The workspace is no longer shared with Korgan Bloodaxe.', 'Success');
  await expect(user2.locator('.filter-button')).toHaveText('Not shared');
});

msTest('Filter users', async ({ workspaceSharingModal }) => {
  const content = workspaceSharingModal.locator('.ms-modal-content');
  const searchInput = content.locator('.ms-search-input');
  // cspell:disable-next-line
  await expect(content.locator('.user-list').locator('.content').locator('.person-name')).toHaveText([
    'Gordon Freeman',
    'Korgan Bloodaxe',
    'Jaheira',
  ]);
  await fillIonInput(searchInput, 'or');
  // cspell:disable-next-line
  await expect(content.locator('.user-list').locator('.content').locator('.person-name')).toHaveText(['Gordon Freeman', 'Korgan Bloodaxe']);
  await searchInput.locator('.input-clear-icon').click();
  // cspell:disable-next-line
  await expect(content.locator('.user-list').locator('.content').locator('.person-name')).toHaveText([
    'Gordon Freeman',
    'Korgan Bloodaxe',
    'Jaheira',
  ]);
});

msTest('Filter users no match', async ({ workspaceSharingModal }) => {
  const content = workspaceSharingModal.locator('.ms-modal-content');
  const searchInput = content.locator('.ms-search-input');
  // cspell:disable-next-line
  await expect(content.locator('.user-list').locator('.content').locator('.person-name')).toHaveText([
    'Gordon Freeman',
    'Korgan Bloodaxe',
    'Jaheira',
  ]);
  await fillIonInput(searchInput, 'nomatch');
  // cspell:disable-next-line
  await expect(content.locator('.user-list').locator('.content')).toBeHidden();
  await expect(content.locator('.no-match-result')).toBeVisible();
  await expect(content.locator('.no-match-result')).toHaveText("No user found that matches 'nomatch'.");
});
