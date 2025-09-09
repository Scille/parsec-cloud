// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { Locator } from '@playwright/test';
import { DisplaySize, expect, fillInputModal, login, msTest } from '@tests/e2e/helpers';

msTest('Sidebar in organization management', async ({ organizationPage }) => {
  const sidebar = organizationPage.locator('.sidebar');

  const mainButtons = sidebar.locator('.list-sidebar-header-text');
  await expect(mainButtons).toHaveText(['Organization', 'My workspaces', 'Recent documents']);

  const items = sidebar.locator('.sidebar-content-organization').locator('.sidebar-content-organization-button__text');
  await expect(items).toHaveText(['Users', 'Information']);
});

msTest('Sidebar in workspaces page', async ({ workspaces }) => {
  const sidebar = workspaces.locator('.sidebar');

  await workspaces.locator('.workspaces-container-grid').locator('.workspace-card-item').nth(0).click();
  await workspaces.locator('#connected-header').locator('.topbar-left').locator('ion-breadcrumb').nth(0).click();

  const mainButtons = sidebar.locator('.list-sidebar-header-text');
  await expect(mainButtons).toHaveText(['Organization', 'My workspaces', 'Recent documents']);

  const recentWorkspaces = sidebar.locator('.sidebar-content-workspaces').nth(1);
  await expect(recentWorkspaces.locator('.sidebar-content-workspaces--no-recent')).toBeHidden();
  await expect(recentWorkspaces.locator('.sidebar-content-workspaces__title')).toHaveText('Recent');
  await expect(recentWorkspaces.getByRole('listitem').nth(0)).toHaveText('wksp1');
});

msTest('Sidebar in connected page', async ({ workspaces }) => {
  const sidebar = workspaces.locator('.sidebar');

  const mainButtons = sidebar.locator('.list-sidebar-header-text');
  await expect(mainButtons).toHaveText(['Organization', 'My workspaces', 'Recent documents']);

  const organizationContent = sidebar.locator('#sidebar-organization');
  const workspacesContent = sidebar.locator('#sidebar-workspaces');
  const filesContent = sidebar.locator('#sidebar-files');
  const recentWorkspaces = workspacesContent.locator('.sidebar-content-workspaces').nth(1);

  async function checkSidebarToggleVisibility(content: Locator, title: string): Promise<void> {
    await expect(content.locator('.list-sidebar-header-text')).toHaveText(title);
    await expect(content.locator('.list-sidebar-content')).toBeVisible();
    await content.locator('.list-sidebar-header__toggle').click();
    await expect(content.locator('.list-sidebar-content')).toBeHidden();
    await content.locator('.list-sidebar-header__toggle').click();
    await expect(content.locator('.list-sidebar-content')).toBeVisible();
  }

  await checkSidebarToggleVisibility(organizationContent, 'Organization');
  await checkSidebarToggleVisibility(workspacesContent, 'My workspaces');
  await checkSidebarToggleVisibility(filesContent, 'Recent documents');

  await expect(recentWorkspaces.locator('.sidebar-content-workspaces--no-recent')).toBeVisible();

  await workspaces.locator('.workspaces-container-grid').locator('.workspace-card-item').nth(0).click();
  await workspaces.locator('#connected-header').locator('.topbar-left').locator('ion-breadcrumb').nth(0).click();

  await expect(recentWorkspaces.locator('.sidebar-content-workspaces--no-recent')).toBeHidden();
});

msTest('Sidebar recommendations checklist in large display', async ({ workspaces }) => {
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
  await expect(checklist.locator('.checklist-text__description')).toHaveText('2 remaining tasks');
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
  await expect(checklist.locator('.checklist-text__description')).toHaveText('1 remaining task');
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

msTest('Sidebar recommendations checklist in small display', async ({ workspaces }) => {
  async function checkChecklist(modal: Locator, states: Array<{ text: string; checked: boolean }>): Promise<void> {
    const items = modal.locator('.checklist-list-item');

    for (const [index, state] of states.entries()) {
      await expect(items.nth(index).locator('.checklist-list-item__text')).toHaveText(state.text);
      if (state.checked) {
        await expect(items.nth(index)).toHaveTheClass('done');
      } else {
        await expect(items.nth(index)).not.toHaveTheClass('done');
      }
    }
  }

  await workspaces.setDisplaySize(DisplaySize.Small);

  const checklistModal = workspaces.locator('.small-display-recommendation-checklist');
  const checklistButton = workspaces.locator('#trigger-checklist-button');

  await expect(checklistModal).toBeHidden();
  await expect(checklistButton).toBeVisible();

  await checklistButton.click();
  await expect(checklistModal).toBeVisible();
  // Order may differ from what is seen on the page because the CSS property
  // `order` only re-orders the items visually, not in the DOM.
  await checkChecklist(checklistModal, [
    { text: 'Add an Owner to the workspace wksp1', checked: false },
    { text: 'Add second device', checked: true },
    { text: 'Create a recovery file', checked: false },
  ]);
  await checklistModal.locator('ion-backdrop').click();
  await expect(checklistModal).toBeHidden();

  // Add an owner on the workspace
  await workspaces.locator('.workspace-card-item').nth(0).locator('.icon-share-container').nth(0).click();
  const workspaceSharingModal = workspaces.locator('.workspace-sharing-modal');
  await expect(workspaceSharingModal).toBeVisible();
  const roleDropdown = workspaceSharingModal.page().locator('.sheet-modal');
  const roles = roleDropdown.getByRole('listitem');
  const users = workspaceSharingModal.locator('.ms-modal-content').locator('.user-member-item');

  await users.nth(1).locator('.dropdown-button').click();
  await expect(roles.locator('.option-text__label')).toHaveText(['Owner', 'Manager', 'Contributor', 'Reader', 'Not shared']);
  // Set contributor
  await roles.nth(0).click();
  await workspaces.locator('.sheet-modal').locator('.button-solid').nth(1).click();
  await expect(users.nth(1).locator('#dropdown-popover-button').locator('.input-text')).toHaveText('Owner');
  await workspaceSharingModal.locator('.closeBtn').click();

  await expect(checklistButton).toHaveTheClass('unread');

  await checklistButton.click();
  await expect(checklistModal).toBeVisible();
  // `order` only re-orders the items visually, not in the DOM.
  await checkChecklist(checklistModal, [
    { text: 'Add a second Owner to the workspaces you own', checked: true },
    { text: 'Add second device', checked: true },
    { text: 'Create a recovery file', checked: false },
  ]);
  await checklistModal.locator('ion-backdrop').click();
  await expect(checklistModal).toBeHidden();
  await workspaces.locator('.toast-container').locator('.toast-button').click();

  // Create a recovery file
  await workspaces.locator('#tab-bar').locator('.tab-bar-menu-button').nth(3).click();
  workspaces.locator('.menu-list__item').nth(3).click();
  const profilePage = workspaces;
  await expect(profilePage.locator('.menu-list__item').nth(3)).toHaveText('Recovery files');
  await profilePage.locator('.restore-password-button').click();

  await workspaces.locator('.recovery-item-download').locator('ion-button').nth(0).click();
  await workspaces.locator('.recovery-item-download').locator('ion-button').nth(1).click();
  await workspaces.locator('#tab-bar').locator('.tab-bar-menu-button').nth(1).click();
  await expect(checklistModal).toBeHidden();
});

msTest('Show recently opened files in sidebar', async ({ documents }) => {
  const sidebarRecentFiles = documents.locator('#sidebar-files');
  await expect(sidebarRecentFiles.locator('.list-sidebar-header-text')).toHaveText('Recent documents');
  await expect(sidebarRecentFiles.locator('.sidebar-item')).toHaveCount(0);

  await expect(documents.locator('.information-modal')).toBeHidden();
  await expect(documents).toHaveHeader(['wksp1'], true, true);
  const fileItem = documents.locator('.folder-container').getByRole('listitem').nth(2);
  const fileName = await fileItem.locator('.file-name').textContent();
  await fileItem.dblclick();
  await expect(documents.locator('.ms-spinner-modal')).toBeVisible();
  await expect(documents.locator('.ms-spinner-modal').locator('.spinner-label__text')).toHaveText('Opening file...');
  await expect(documents.locator('.ms-spinner-modal')).toBeHidden();
  await expect(documents).toBeViewerPage();
  // One file added
  await expect(sidebarRecentFiles.locator('.sidebar-item')).toHaveText([fileName ?? '']);

  await documents.locator('.file-handler-topbar').locator('#trigger-toggle-menu-button').click();
  await sidebarRecentFiles.locator('.list-sidebar-header__toggle').click();
  await expect(sidebarRecentFiles.locator('.list-sidebar-content')).toBeHidden();
  await sidebarRecentFiles.locator('.list-sidebar-header__toggle').click();
  await expect(sidebarRecentFiles.locator('.list-sidebar-content')).toBeVisible();
});

msTest('Recent document updates when file is renamed', async ({ documents }) => {
  const sidebarRecentFiles = documents.locator('#sidebar-files');
  await expect(sidebarRecentFiles.locator('.list-sidebar-header-text')).toHaveText('Recent documents');
  await expect(sidebarRecentFiles.locator('.sidebar-item')).toHaveCount(0);

  await expect(documents.locator('.information-modal')).toBeHidden();
  await expect(documents).toHaveHeader(['wksp1'], true, true);
  const fileItem = documents.locator('.folder-container').getByRole('listitem').nth(2);
  const fileName = await fileItem.locator('.file-name').textContent();
  await fileItem.dblclick();
  await expect(documents.locator('.ms-spinner-modal')).toBeVisible();
  await expect(documents.locator('.ms-spinner-modal').locator('.spinner-label__text')).toHaveText('Opening file...');
  await expect(documents.locator('.ms-spinner-modal')).toBeHidden();
  await expect(documents).toBeViewerPage();
  // One file added
  await expect(sidebarRecentFiles.locator('.sidebar-item')).toHaveText([fileName!]);
  await documents.locator('.file-handler-topbar').locator('.back-button').click();
  await expect(documents.locator('#connected-header .topbar')).toBeVisible();
  await expect(documents).toBeDocumentPage();

  await fileItem.click({ button: 'right' });
  const popover = documents.locator('.file-context-menu');
  await popover.getByRole('listitem').filter({ hasText: 'Rename' }).click();
  await fillInputModal(documents, `New-${fileName}`, true);
  await expect(fileItem.locator('.file-name').locator('.file-name__label')).toHaveText(`New-${fileName}`);
  await expect(sidebarRecentFiles.locator('.sidebar-item')).toHaveText([`New-${fileName}`]);
});

msTest('Recent workspaces displayed in sidebar', async ({ workspaces }) => {
  const sidebarRecentWorkspaces = workspaces.locator('#sidebar-workspaces-recent');
  await expect(sidebarRecentWorkspaces.locator('.sidebar-content-workspaces__title ')).toHaveText('Recent');
  await expect(sidebarRecentWorkspaces.locator('.sidebar-item')).toHaveCount(0);
  await workspaces.locator('.workspaces-container-grid').locator('.workspace-card-item').nth(0).click();
  await expect(workspaces).toBeDocumentPage();
  await expect(sidebarRecentWorkspaces.locator('.sidebar-item')).toHaveCount(1);
  await expect(sidebarRecentWorkspaces.locator('.sidebar-item').locator('.sidebar-item-workspace').nth(0)).toHaveText('wksp1');
});

msTest('Recent and pinned workspaces are updated when workspace is renamed', async ({ workspaces }) => {
  const sidebarRecentWorkspaces = workspaces.locator('#sidebar-workspaces-recent');
  const sidebarFavoriteWorkspaces = workspaces.locator('#sidebar-workspaces-favorites');
  await expect(sidebarRecentWorkspaces.locator('.sidebar-content-workspaces__title ')).toHaveText('Recent');
  await expect(sidebarRecentWorkspaces.locator('.sidebar-item')).toHaveCount(0);
  await expect(sidebarFavoriteWorkspaces.locator('.sidebar-item')).toHaveCount(0);

  const card = workspaces.locator('.workspaces-container-grid').locator('.workspace-card-item').nth(0);
  await card.locator('.workspace-favorite-icon').click();
  await card.click();

  await expect(workspaces).toBeDocumentPage();
  await expect(sidebarRecentWorkspaces.locator('.sidebar-item')).toHaveCount(1);
  await expect(sidebarFavoriteWorkspaces.locator('.sidebar-item')).toHaveCount(1);
  await expect(sidebarRecentWorkspaces.locator('.sidebar-item').locator('.sidebar-item-workspace').nth(0)).toHaveText('wksp1');
  await expect(sidebarFavoriteWorkspaces.locator('.sidebar-item').locator('.sidebar-item-workspace').nth(0)).toHaveText('wksp1');
  await workspaces.locator('#connected-header').locator('.back-button').click();
  await expect(workspaces).toBeWorkspacePage();
  await card.click({ button: 'right' });
  const popover = workspaces.locator('.workspace-context-menu');
  await popover.getByRole('listitem').nth(1).click();
  await fillInputModal(workspaces, 'New-wksp1', true);
  await expect(workspaces).toShowToast('Workspace has been successfully renamed to New-wksp1.', 'Success');

  await expect(sidebarRecentWorkspaces.locator('.sidebar-item').locator('.sidebar-item-workspace').nth(0)).toHaveText('New-wksp1');
  await expect(sidebarFavoriteWorkspaces.locator('.sidebar-item').locator('.sidebar-item-workspace').nth(0)).toHaveText('New-wksp1');

  // Check update from other user point of view
  const bobTab = await workspaces.openNewTab();
  await login(bobTab, 'Boby McBobFace');
  const bobSidebarRecentWorkspaces = bobTab.locator('#sidebar-workspaces-recent');
  const bobSidebarFavoriteWorkspaces = bobTab.locator('#sidebar-workspaces-favorites');
  const bobCard = bobTab.locator('.workspaces-container-grid').locator('.workspace-card-item').nth(0);
  await bobCard.click();
  await expect(bobSidebarRecentWorkspaces.locator('.sidebar-item')).toHaveCount(1);
  await expect(bobSidebarFavoriteWorkspaces.locator('.sidebar-item')).toHaveCount(1);
  await expect(bobSidebarRecentWorkspaces.locator('.sidebar-item').locator('.sidebar-item-workspace').nth(0)).toHaveText('New-wksp1');
  await expect(bobSidebarFavoriteWorkspaces.locator('.sidebar-item').locator('.sidebar-item-workspace').nth(0)).toHaveText('New-wksp1');

  await card.click({ button: 'right' });
  await popover.getByRole('listitem').nth(1).click();
  await fillInputModal(workspaces, 'Newer-wksp1', true);
  await expect(workspaces).toShowToast('Workspace has been successfully renamed to Newer-wksp1.', 'Success');
  await expect(sidebarRecentWorkspaces.locator('.sidebar-item').locator('.sidebar-item-workspace').nth(0)).toHaveText('Newer-wksp1');
  await expect(sidebarFavoriteWorkspaces.locator('.sidebar-item').locator('.sidebar-item-workspace').nth(0)).toHaveText('Newer-wksp1');
  await expect(bobSidebarRecentWorkspaces.locator('.sidebar-item').locator('.sidebar-item-workspace').nth(0)).toHaveText('Newer-wksp1');
  await expect(bobSidebarFavoriteWorkspaces.locator('.sidebar-item').locator('.sidebar-item-workspace').nth(0)).toHaveText('Newer-wksp1');
});
