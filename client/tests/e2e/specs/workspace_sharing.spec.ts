// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { DisplaySize, expect, fillIonInput, login, MsPage, msTest } from '@tests/e2e/helpers';

for (const displaySize of ['small', 'large']) {
  msTest(`Workspace sharing modal default state on ${displaySize} display`, { tag: '@important' }, async ({ workspaceSharingModal }) => {
    if (displaySize === 'small') {
      await (workspaceSharingModal.page() as MsPage).setDisplaySize(DisplaySize.Small);
    }

    await expect(workspaceSharingModal.locator('.ms-modal-header__title')).toHaveText('Share the workspace');
    await expect(workspaceSharingModal.locator('.sharing-modal__title')).toHaveText('wksp1');
    const content = workspaceSharingModal.locator('.ms-modal-content');
    await expect(content.locator('#only-owner-warning')).toBeVisible();
    const users = content.locator('.user-member-item');
    await expect(users).toHaveCount(2);
    await expect(users.locator('.person-name')).toHaveText(['Alicey McAliceFace', 'Boby McBobFace']);
    await expect(users.locator('.dropdown-button')).toHaveText(['Owner', 'Reader']);
    await expect(users.nth(0).locator('.dropdown-button')).toHaveDisabledAttribute();

    const suggestions = workspaceSharingModal.locator('.user-list-suggestions-item');
    await expect(suggestions).toHaveCount(1);
    await expect(suggestions.locator('.person-name')).toHaveText(['Malloryy McMalloryFace']);
    await expect(suggestions.locator('.dropdown-container')).toHaveText(['Not shared']);
    await expect(suggestions.nth(0).locator('.label-profile')).toHaveText('External');
    await expect(suggestions.nth(0).locator('.label-profile')).toBeVisible();
  });
}

for (const displaySize of ['small', 'large']) {
  msTest(`Update user role on ${displaySize} display`, async ({ workspaceSharingModal }) => {
    if (displaySize === 'small') {
      await (workspaceSharingModal.page() as MsPage).setDisplaySize(DisplaySize.Small);
    }
    const content = workspaceSharingModal.locator('.ms-modal-content');
    const users = content.locator('.user-member-item');

    await expect(users.nth(1).locator('.dropdown-button')).toHaveText('Reader');
    await users.nth(1).locator('.dropdown-button').click();

    if (displaySize === 'small') {
      const roleDropdown = workspaceSharingModal.page().locator('.sheet-modal');
      const roles = roleDropdown.getByRole('listitem');
      await expect(roles.locator('.option-text__label')).toHaveText(['Owner', 'Manager', 'Contributor', 'Reader', 'Not shared']);
      // Set contributor
      await roles.nth(2).click();
      await workspaceSharingModal.page().locator('.sheet-modal').locator('.button').nth(2).click();
    } else {
      const roleDropdown = workspaceSharingModal.page().locator('.dropdown-popover');
      const roles = roleDropdown.getByRole('listitem');
      await expect(roles.locator('.option-text__label')).toHaveText(['Owner', 'Manager', 'Contributor', 'Reader', 'Not shared']);
      // Set contributor
      await roles.nth(2).click();
    }

    await expect(workspaceSharingModal.page()).toShowToast("Boby McBobFace's role has been updated to Contributor.", 'Success');
    await expect(users.nth(1).locator('.dropdown-button')).toHaveText('Contributor');
  });
}

msTest('Share with external', async ({ workspaceSharingModal }) => {
  const content = workspaceSharingModal.locator('.ms-modal-content');
  const users = content.locator('.user-member-item');
  const suggestions = content.locator('.user-list-suggestions-item');

  await expect(users).toHaveCount(2);
  await expect(suggestions).toHaveCount(1);

  const user = suggestions.nth(0);
  await expect(user.locator('.dropdown-button')).toHaveText('Not shared');
  await user.locator('.dropdown-button').click();
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
  await expect(workspaceSharingModal.page()).toShowToast("Malloryy McMalloryFace's role has been updated to Contributor.", 'Success');
  await expect(users).toHaveCount(3);
  await expect(suggestions).toHaveCount(0);
  await expect(users.nth(1).locator('.dropdown-button')).toHaveText('Contributor');

  // Set reader, should be the last user now
  await users.nth(1).locator('.dropdown-button').click();
  await roles.nth(3).click();
  await expect(workspaceSharingModal.page()).toShowToast("Malloryy McMalloryFace's role has been updated to Reader.", 'Success');
  await expect(users).toHaveCount(3);
  await expect(suggestions).toHaveCount(0);
  await expect(users.nth(2).locator('.dropdown-button')).toHaveText('Reader');
});

msTest('Unshare workspace', async ({ workspaceSharingModal }) => {
  msTest.setTimeout(45_000);
  const page = workspaceSharingModal.page();
  const secondTab = await (workspaceSharingModal.page() as MsPage).openNewTab();
  // Login on the second tab with Bob, should have one workspace shared by default
  await login(secondTab, 'Boby McBobFace');
  const workspaces = secondTab.locator('.workspaces-container-grid').locator('.workspace-card-item');
  await expect(workspaces).toHaveCount(1);
  await expect(workspaces.locator('.workspace-card-content__title')).toHaveText(['wksp1']);
  await expect(secondTab.locator('.workspaces-container').locator('.no-workspaces')).toBeHidden();

  // On the first tab, unshare the workspace with Bob
  const content = workspaceSharingModal.locator('.ms-modal-content');
  await expect(content.locator('.user-list-members').locator('.workspace-user-role')).toHaveCount(2);
  await expect(content.locator('.user-list-suggestions').locator('.workspace-user-role')).toHaveCount(1);
  const user2 = content.locator('.user-list-members').locator('.workspace-user-role').nth(1);
  await expect(user2.locator('.dropdown-button')).toHaveText('Reader');
  await user2.locator('.dropdown-button').click();
  const roleDropdown = page.locator('.dropdown-popover');
  const roles = roleDropdown.getByRole('list').getByRole('listitem');
  await expect(roles.locator('.option-text__label')).toHaveText(['Owner', 'Manager', 'Contributor', 'Reader', 'Not shared']);
  await roles.nth(4).click();
  await expect(page).toShowToast('The workspace is no longer shared with Boby McBobFace.', 'Success');
  await expect(content.locator('.user-list-members').locator('.workspace-user-role')).toHaveCount(1);
  await expect(content.locator('.user-list-suggestions').locator('.workspace-user-role')).toHaveCount(2);

  // Check that Bob doesn't have any workspaces anymore
  await expect(workspaces).toHaveCount(0);
  await expect(secondTab.locator('.workspaces-container').locator('.no-workspaces')).toBeVisible();
});

msTest('Filter users', async ({ workspaceSharingModal }) => {
  const content = workspaceSharingModal.locator('.ms-modal-content');
  const searchInput = content.locator('.ms-search-input');
  const members = content.locator('.user-member-item');
  const suggestions = workspaceSharingModal.locator('.user-list-suggestions-item');
  await expect(members.locator('.person-name')).toHaveText(['Alicey McAliceFace', 'Boby McBobFace']);
  await expect(suggestions.locator('.person-name')).toHaveText(['Malloryy McMalloryFace']);

  await fillIonInput(searchInput, 'al');

  await expect(members.locator('.person-name')).toHaveText(['Alicey McAliceFace']);
  await expect(suggestions.locator('.person-name')).toHaveText(['Malloryy McMalloryFace']);

  await searchInput.locator('.input-clear-icon').click();
  await expect(members.locator('.person-name')).toHaveText(['Alicey McAliceFace', 'Boby McBobFace']);
  await expect(suggestions.locator('.person-name')).toHaveText(['Malloryy McMalloryFace']);

  await fillIonInput(searchInput, 'bo');
  await expect(members.locator('.person-name')).toHaveText(['Boby McBobFace']);
  await expect(suggestions).toBeHidden();
});

msTest('Filter users no match', async ({ workspaceSharingModal }) => {
  const content = workspaceSharingModal.locator('.ms-modal-content');
  const searchInput = content.locator('.ms-search-input');
  const members = content.locator('.user-member-item');
  const suggestions = workspaceSharingModal.locator('.user-list-suggestions-item');
  await expect(members.locator('.person-name')).toHaveText(['Alicey McAliceFace', 'Boby McBobFace']);
  await expect(suggestions.locator('.person-name')).toHaveText(['Malloryy McMalloryFace']);
  await fillIonInput(searchInput, 'nomatch');

  await expect(members).toBeHidden();
  await expect(suggestions).toBeHidden();
  await expect(content.locator('.no-match-result')).toBeVisible();
  await expect(content.locator('.no-match-result')).toHaveText("No user found that matches 'nomatch'.");
});

msTest('Batch workspace sharing', async ({ workspaceSharingModal }) => {
  const content = workspaceSharingModal.locator('.ms-modal-content');
  const batchDropdown = content.locator('.modal-head-content').locator('.dropdown-container').locator('#dropdown-popover-button');
  const activateBatchButton = content.locator('#batch-activate-button');
  const membersCheckbox = content.locator('#all-members-checkbox');

  await expect(content.locator('#profile-assign-info')).toBeVisible();
  await expect(batchDropdown).toBeHidden();
  await expect(activateBatchButton).toContainText('Multiple selection');
  await expect(membersCheckbox).toBeHidden();

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

  await expect(content.locator('#profile-assign-info')).toBeVisible();
  await expect(batchDropdown).toBeHidden();
  await expect(activateBatchButton).toContainText('Multiple selection');
  await expect(membersCheckbox).toBeHidden();

  // Check external user restriction
  await activateBatchButton.click();
  await content.locator('.suggested-checkbox').nth(0).click();
  await content.locator('.suggested-checkbox').nth(1).click();
  await expect(membersCheckbox).toBeHidden();
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
  await expect(batchDropdown).toBeHidden();
  await expect(activateBatchButton).toContainText('Multiple selection');
  await expect(membersCheckbox).toBeHidden();
});

msTest('Batch workspace sharing hidden when reader', async ({ home }) => {
  await login(home, 'Boby McBobFace');
  await home
    .locator('.workspaces-container-grid')
    .locator('.workspace-card-item')
    .nth(0)
    .locator('.workspace-card-bottom__icons')
    .locator('.icon-share-container')
    .nth(0)
    .click();
  const modal = home.locator('.workspace-sharing-modal');
  await expect(modal).toBeVisible();
  const content = modal.locator('.ms-modal-content');

  await expect(content.locator('.modal-head-content').locator('.dropdown-container').locator('#dropdown-popover-button')).toBeHidden();
  await expect(content.locator('#batch-activate-button')).toBeHidden();
});
