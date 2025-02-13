// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { expect, fillIonInput, msTest } from '@tests/e2e/helpers';

msTest('Workspace sharing modal default state', async ({ workspaceSharingModal }) => {
  await expect(workspaceSharingModal.locator('.ms-modal-header__title')).toHaveText('Trademeet');
  const content = workspaceSharingModal.locator('.ms-modal-content');
  await expect(content.locator('.only-owner-warning')).toBeVisible();
  const users = content.locator('.user-list').locator('.content');
  await expect(users).toHaveCount(3);
  // cspell:disable-next-line
  await expect(users.locator('.person-name')).toHaveText(['Gordon Freeman', 'Korgan Bloodaxe', 'Jaheira']);
  await expect(users.locator('.filter-button')).toHaveText(['Owner', 'Reader', 'Not shared']);
  await expect(users.nth(2).locator('.label-profile')).toBeVisible();
  await expect(users.nth(2).locator('.label-profile')).toHaveText('External');
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
  // User is outsider, owner and manager should be disabled
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
  // cspell:disable
  await expect(content.locator('.user-list').locator('.content').locator('.person-name')).toHaveText([
    'Gordon Freeman',
    'Korgan Bloodaxe',
    'Jaheira',
  ]);
  // cspell:enable
  await fillIonInput(searchInput, 'or');
  // cspell:disable-next-line
  await expect(content.locator('.user-list').locator('.content').locator('.person-name')).toHaveText(['Gordon Freeman', 'Korgan Bloodaxe']);
  await searchInput.locator('.input-clear-icon').click();
  // cspell:disable
  await expect(content.locator('.user-list').locator('.content').locator('.person-name')).toHaveText([
    'Gordon Freeman',
    'Korgan Bloodaxe',
    'Jaheira',
  ]);
  // cspell:enable
});

msTest('Filter users no match', async ({ workspaceSharingModal }) => {
  const content = workspaceSharingModal.locator('.ms-modal-content');
  const searchInput = content.locator('.ms-search-input');
  // cspell:disable
  await expect(content.locator('.user-list').locator('.content').locator('.person-name')).toHaveText([
    'Gordon Freeman',
    'Korgan Bloodaxe',
    'Jaheira',
  ]);
  // cspell:enable
  await fillIonInput(searchInput, 'nomatch');
  // cspell:disable-next-line
  await expect(content.locator('.user-list').locator('.content')).toBeHidden();
  await expect(content.locator('.no-match-result')).toBeVisible();
  await expect(content.locator('.no-match-result')).toHaveText("No user found that matches 'nomatch'.");
});

msTest('Batch workspace sharing', async ({ workspaceSharingModal }) => {
  const content = workspaceSharingModal.locator('.ms-modal-content');
  const batchDropdown = content.locator('.modal-head-content').locator('.dropdown-container').locator('#dropdown-popover-button');
  const activateBatchButton = content.locator('#batch-activate-button');
  const membersCheckbox = content.locator('#all-members-checkbox');

  await expect(content.locator('#profile-assign-info')).not.toBeVisible();
  await expect(batchDropdown).not.toBeVisible();
  await expect(activateBatchButton).toContainText('Multiple selection');
  await expect(membersCheckbox).not.toBeVisible();

  // Share with non-external users
  await activateBatchButton.click();
  await expect(content.locator('#profile-assign-info')).toBeVisible();
  await expect(content.locator('#profile-assign-info')).toContainText('External profiles can only have Contributor or Reader roles.');
  await expect(batchDropdown).toBeVisible();
  await expect(batchDropdown).toBeTrulyDisabled();
  await expect(activateBatchButton).toContainText('Finish');
  await expect(membersCheckbox).toBeVisible();

  await membersCheckbox.click();
  await batchDropdown.click();

  const roleDropdown = workspaceSharingModal.page().locator('.dropdown-popover');
  const roles = roleDropdown.getByRole('listitem');
  await expect(roles.locator('.option-text__label')).toHaveText(['Owner', 'Manager', 'Contributor', 'Reader', 'Not shared']);

  await roles.nth(4).click();
  await expect(workspaceSharingModal.page()).toShowToast('The workspace is no longer shared with selected members.', 'Success');

  await expect(content.locator('#profile-assign-info')).not.toBeVisible();
  await expect(batchDropdown).not.toBeVisible();
  await expect(activateBatchButton).toContainText('Multiple selection');
  await expect(membersCheckbox).not.toBeVisible();

  // Check external user restriction
  await activateBatchButton.click();
  await content.locator('#member-checkbox').nth(0).click();
  await content.locator('#suggested-checkbox').nth(0).click();
  await expect(membersCheckbox).toHaveState('checked');
  await batchDropdown.click();
  await expect(roles.locator('.option-text__label')).toHaveText(['Owner', 'Manager', 'Contributor', 'Reader', 'Not shared']);

  for (const [index, role] of (await roles.all()).entries()) {
    if (index === 0 || index === 1) {
      await expect(role).toHaveTheClass('item-disabled');
    } else {
      await expect(role).not.toHaveTheClass('item-disabled');
    }
  }
  await roles.nth(2).click();
  await expect(workspaceSharingModal.page()).toShowToast("Selected members' roles have been updated to Contributor.", 'Success');
  await expect(batchDropdown).not.toBeVisible();
  await expect(activateBatchButton).toContainText('Multiple selection');
  await expect(membersCheckbox).not.toBeVisible();
});

msTest('Batch workspace sharing hidden when reader', async ({ connected }) => {
  await connected.locator('.workspaces-container-grid').locator('.workspace-card-item').nth(2).locator('.not-shared-label').click();
  const modal = connected.locator('.workspace-sharing-modal');
  await expect(modal).toBeVisible();
  const content = modal.locator('.ms-modal-content');

  await expect(content.locator('.modal-head-content').locator('.dropdown-container').locator('#dropdown-popover-button')).toBeHidden();
  await expect(content.locator('#batch-activate-button')).toBeHidden();
});
