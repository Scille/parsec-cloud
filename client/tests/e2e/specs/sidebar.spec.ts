// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { Locator } from '@playwright/test';
import { expect, msTest } from '@tests/e2e/helpers';

msTest('Sidebar in organization management', async ({ organizationPage }) => {
  const sidebar = organizationPage.locator('.sidebar');

  await expect(sidebar.locator('.back-button')).toBeVisible();

  const mainButtons = sidebar.locator('.organization-card-buttons').locator('.organization-card-buttons__item');
  await expect(mainButtons).toHaveText(['Manage my organization', 'My workspaces']);
  await expect(mainButtons.nth(0)).toHaveTheClass('active');
  await expect(mainButtons.nth(1)).not.toHaveTheClass('active');

  await expect(sidebar.locator('.file-workspaces')).toBeHidden();
  await expect(sidebar.locator('.favorites')).toBeHidden();
  await expect(sidebar.locator('.workspaces')).toBeHidden();

  await expect(sidebar.locator('.manage-organization')).toBeVisible();
  await expect(sidebar.locator('.manage-organization').locator('.list-sidebar-header')).toHaveText('Manage my organization');
  const items = sidebar.locator('.manage-organization').locator('.organization-card-buttons').getByRole('listitem');
  await expect(items).toHaveText(['Users', 'Information']);
});

msTest('Sidebar in workspaces page', async ({ workspaces }) => {
  const sidebar = workspaces.locator('.sidebar');

  await workspaces.locator('.workspaces-container-grid').locator('.workspace-card-item').nth(0).click();
  await workspaces.locator('#connected-header').locator('.topbar-left').locator('ion-breadcrumb').nth(0).click();

  await expect(sidebar.locator('.back-button')).toBeHidden();

  const mainButtons = sidebar.locator('.organization-card-buttons').locator('.organization-card-buttons__item');
  await expect(mainButtons).toHaveText(['Manage my organization', 'My workspaces']);
  await expect(mainButtons.nth(0)).not.toHaveTheClass('active');
  await expect(mainButtons.nth(1)).toHaveTheClass('active');

  await expect(sidebar.locator('.file-workspaces')).toBeHidden();
  await expect(sidebar.locator('.favorites')).toBeHidden();
  await expect(sidebar.locator('.workspaces')).toBeVisible();
  await expect(sidebar.locator('.workspaces').locator('.list-sidebar-header')).toHaveText('Recent workspaces');
  await expect(sidebar.locator('.workspaces').getByRole('listitem').nth(0)).toHaveText('wksp1');

  await expect(sidebar.locator('.manage-organization')).toBeHidden();
});

msTest('Sidebar recommendations checklist', async ({ workspaces }) => {
  async function checkChecklist(popover: Locator, states: Array<{ text: string; checked: boolean }>): Promise<void> {
    const items = popover.locator('.checklist-list-item');

    for (const [index, state] of states.entries()) {
      await expect(items.nth(index).locator('.checklist-list-item__text')).toHaveText(state.text);
      if (state.checked) {
        await expect(items.nth(index)).toHaveTheClass('done');
      } else {
        await expect(items.nth(index)).not.toHaveTheClass('done');
      }
    }
  }

  const sidebar = workspaces.locator('.sidebar');
  const checklist = sidebar.locator('.organization-checklist');
  const checklistPopover = workspaces.locator('.recommendation-checklist');

  await expect(checklistPopover).toBeHidden();
  await expect(checklist).toBeVisible();
  await expect(checklist.locator('.checklist-text__title')).toHaveText('Security checklist');
  await expect(checklist.locator('.checklist-text__description')).toHaveText('2 remaining');
  await checklist.click();
  await expect(checklistPopover).toBeVisible();
  // Order may differ from what is seen on the page because the CSS property
  // `order` only re-orders the items visually, not in the DOM.
  await checkChecklist(checklistPopover, [
    { text: 'Add an Owner to the workspace wksp1', checked: false },
    { text: 'Add second device', checked: true },
    { text: 'Create a recovery file', checked: false },
  ]);
  await checklistPopover.locator('ion-backdrop').click();
  await expect(checklistPopover).toBeHidden();

  // Add an owner on the workspace
  await workspaces.locator('.workspace-card-item').nth(0).locator('.icon-share-container').nth(0).click();
  const workspaceSharingModal = workspaces.locator('.workspace-sharing-modal');
  await expect(workspaceSharingModal).toBeVisible();
  await workspaceSharingModal.locator('.user-member-item').nth(1).locator('.dropdown-button').click();
  workspaces.locator('.dropdown-popover').getByRole('listitem').nth(0).click();
  await workspaceSharingModal.locator('.closeBtn').click();

  await expect(checklist.locator('.checklist-text__title')).toHaveText('Security checklist');
  await expect(checklist.locator('.checklist-text__description')).toHaveText('1 remaining');
  await checklist.click();
  await expect(checklistPopover).toBeVisible();
  // Order may differ from what is seen on the page because the CSS property
  // `order` only re-orders the items visually, not in the DOM.
  await checkChecklist(checklistPopover, [
    { text: 'Add a second Owner to the workspaces you own', checked: true },
    { text: 'Add second device', checked: true },
    { text: 'Create a recovery file', checked: false },
  ]);
  await checklistPopover.locator('ion-backdrop').click();
  await expect(checklistPopover).toBeHidden();

  // Create a recovery file
  await workspaces.locator('.topbar').locator('.profile-header').click();
  workspaces.locator('.profile-header-organization-popover').locator('.main-list').getByRole('listitem').nth(0).click();
  const profilePage = workspaces;
  await expect(profilePage.locator('.menu-list__item').nth(3)).toHaveText('Recovery files');
  await profilePage.locator('.menu-list__item').nth(3).click();
  const recovery = profilePage.locator('.recovery');
  const recoveryFiles = recovery.locator('.recovery-list');
  await recovery.locator('.restore-password-button').click();
  await expect(recoveryFiles).toBeVisible();
  const recoveryItems = recoveryFiles.locator('.recovery-item');
  await expect(recoveryItems).toHaveCount(2);

  await recoveryItems.nth(0).locator('.recovery-item-download').locator('ion-button').click();
  await recoveryItems.nth(1).locator('.recovery-item-download').locator('ion-button').click();
  await expect(checklist).toBeHidden();
});
