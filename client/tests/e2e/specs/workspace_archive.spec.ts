// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { answerQuestion, DisplaySize, expect, msTest } from '@tests/e2e/helpers';

msTest('Archive workspace', async ({ workspaces }) => {
  const sidebarArchiveButton = workspaces.locator('.sidebar').locator('#sidebar-workspaces').locator('#sidebar-archived-workspaces');
  const sidebarWorkspacesButton = workspaces.locator('.sidebar').locator('#sidebar-workspaces').locator('#sidebar-all-workspaces');
  await expect(sidebarArchiveButton).toHaveText('Archived');
  await sidebarArchiveButton.click();
  await expect(workspaces).toHavePageTitle('Archived workspaces');
  await expect(workspaces.locator('.workspaces-container').locator('.no-archived-workspaces').locator('ion-text')).toHaveText(
    'Your archived workspaces can be consulted here in read-only. Sharing is disabled unless the workspace is restored. \
    Archived workspace owners can restore them.',
  );

  // Archive workspace
  await sidebarWorkspacesButton.click();
  const wk = workspaces.locator('.workspaces-container-grid').locator('.workspace-card-item').nth(0);
  await wk.click({ button: 'right' });
  const contextMenu = workspaces.locator('.workspace-context-menu');
  await expect(contextMenu).toBeVisible();
  const archiveButton = contextMenu.getByRole('listitem').nth(4);
  await expect(archiveButton).toHaveText('Archive this workspace');
  await archiveButton.click();
  await expect(workspaces.locator('.question-modal').locator('.ms-modal-header__title')).toHaveText('Archive this workspace');
  await answerQuestion(workspaces, false);
  await expect(wk).toBeVisible();
  await wk.click({ button: 'right' });
  await archiveButton.click();
  await answerQuestion(workspaces, true);
  await expect(workspaces).toShowToast('The workspace wksp1 has successfully been archived.', 'Success');

  await expect(wk).not.toBeVisible();
  await sidebarArchiveButton.click();
  await expect(wk).toBeVisible();
  await expect(wk.locator('.custom-icon')).toBeVisible();
  await expect(wk.locator('.archived-label')).toBeVisible();
  await expect(wk.locator('.archived-label')).toHaveText('Read only');
  await wk.click({ button: 'right' });
  await expect(contextMenu.getByRole('group').getByRole('listitem')).toHaveText([
    'Workspace management',
    'History',
    'Restore this workspace',
    'Delete this workspace',
  ]);
});

msTest('Restore an archived workspace', async ({ workspaces }) => {
  const sidebarArchiveButton = workspaces.locator('.sidebar').locator('#sidebar-workspaces').locator('#sidebar-archived-workspaces');
  const sidebarWorkspacesButton = workspaces.locator('.sidebar').locator('#sidebar-workspaces').locator('#sidebar-all-workspaces');
  const wk = workspaces.locator('.workspaces-container-grid').locator('.workspace-card-item').nth(0);

  // Archive workspace
  await wk.click({ button: 'right' });
  const contextMenu = workspaces.locator('.workspace-context-menu');
  await contextMenu.getByRole('listitem').nth(4).click();
  await answerQuestion(workspaces, true);
  await expect(workspaces).toShowToast('The workspace wksp1 has successfully been archived.', 'Success');

  // Restore workspace
  await expect(wk).not.toBeVisible();
  await sidebarArchiveButton.click();
  await expect(wk).toBeVisible();
  await wk.click({ button: 'right' });
  const restoreButton = contextMenu.getByRole('listitem').nth(2);
  await expect(restoreButton).toHaveText('Restore this workspace');
  await restoreButton.click();
  await answerQuestion(workspaces, false);
  await expect(wk).toBeVisible();
  await wk.click({ button: 'right' });
  await restoreButton.click();
  await answerQuestion(workspaces, true);
  await expect(workspaces).toShowToast('The workspace wksp1 has successfully been restored.', 'Success');

  await expect(wk).not.toBeVisible();
  await sidebarWorkspacesButton.click();
  await expect(wk).toBeVisible();
});

msTest('Check archived workspace is read-only', async ({ parsecEditics }) => {
  const sidebarArchiveButton = parsecEditics.locator('.sidebar').locator('#sidebar-workspaces').locator('#sidebar-archived-workspaces');
  const sidebarWorkspacesButton = parsecEditics.locator('.sidebar').locator('#sidebar-workspaces').locator('#sidebar-all-workspaces');
  await sidebarWorkspacesButton.click();
  const wk = parsecEditics.locator('.workspaces-container-grid').locator('.workspace-card-item').nth(0);

  // Check file context menu
  await wk.click();
  await expect(parsecEditics.locator('.file-context-menu')).toBeHidden();
  const entry = parsecEditics.locator('.folder-container').locator('.file-list-item').nth(2);
  await entry.click({ button: 'right' });
  await expect(parsecEditics.locator('.file-context-menu')).toBeVisible();
  const popover = parsecEditics.locator('.file-context-menu');
  await expect(popover.getByRole('group')).toHaveCount(2);
  await expect(popover.getByRole('listitem')).toHaveText([
    'File management',
    'Preview',
    'Edit',
    'Rename',
    'Move to',
    'Make a copy',
    'History',
    'Download',
    'Details',
    'Delete',
    'Collaboration',
    'Copy link',
  ]);
  await popover.locator('ion-backdrop').click();
  await sidebarWorkspacesButton.click();

  // Archive workspace
  await wk.click({ button: 'right' });
  const contextMenu = parsecEditics.locator('.workspace-context-menu');
  await contextMenu.getByRole('listitem').nth(4).click();
  await answerQuestion(parsecEditics, true);
  await expect(parsecEditics).toShowToast('The workspace wksp1 has successfully been archived.', 'Success');
  await expect(wk).not.toBeVisible();
  await sidebarArchiveButton.click();
  await expect(wk).toBeVisible();

  // Check file context menu
  await wk.click();
  await entry.click({ button: 'right' });
  await expect(parsecEditics.locator('.file-context-menu')).toBeVisible();
  await expect(popover.getByRole('group')).toHaveCount(2);
  await expect(popover.getByRole('listitem')).toHaveText([
    'File management',
    'Preview',
    'Download',
    'Details',
    'Collaboration',
    'Copy link',
  ]);
});

msTest('Archive workspace in small display', async ({ workspaces }) => {
  await workspaces.setDisplaySize(DisplaySize.Small);

  const workspaceCard = workspaces.locator('.workspaces-container-grid').locator('.workspace-card-item').nth(0);
  await workspaceCard.locator('.icon-option-container').click();

  const popover = workspaces.locator('.workspace-context-sheet-modal');
  await expect(popover).toBeVisible();
  await expect(popover.getByRole('listitem').nth(3)).toHaveText('Archive this workspace');
  await popover.getByRole('listitem').nth(3).click();

  await expect(workspaces.locator('.question-modal').locator('.ms-small-display-modal-header__title')).toHaveText('Archive this workspace');
  await answerQuestion(workspaces, true);
  await expect(workspaces).toShowToast('The workspace wksp1 has successfully been archived.', 'Success');
  await expect(workspaceCard).not.toBeVisible();

  const switchRouteButton = workspaces.locator('.topbar-left-workspaces-mobile-dropdown');
  await expect(switchRouteButton).toBeVisible();
  await switchRouteButton.click();

  const switchModal = workspaces.locator('.workspace-switch-modal');
  await expect(switchModal).toBeVisible();
  await expect(switchModal.locator('.switch-item')).toHaveText(['My workspaces', 'Archived workspaces', 'Workspaces bin']);
  await switchModal.locator('.switch-item').nth(1).click();
  await switchModal.locator('.workspace-switch-button').click();

  await expect(workspaces.locator('.topbar-left-workspaces-mobile-dropdown__title')).toHaveText('Archived workspaces');
  const archivedCard = workspaces.locator('.workspaces-container-grid').locator('.workspace-card-item').nth(0);
  await expect(archivedCard).toBeVisible();
  await expect(archivedCard).toHaveClass(/workspace-card-item--archived/);
  await expect(archivedCard.locator('.archived-label')).toHaveText('Read only');

  await archivedCard.click();
  await expect(workspaces.locator('.header-archived')).toBeVisible();
  await expect(workspaces.locator('.header-archived')).toHaveText('Archived workspace (read-only)');
});
