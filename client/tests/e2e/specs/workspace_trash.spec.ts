// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { answerQuestion, DisplaySize, expect, msTest } from '@tests/e2e/helpers';

msTest('Trash an archived workspace', async ({ workspaces }) => {
  const sidebarArchiveButton = workspaces.locator('.sidebar').locator('#sidebar-workspaces').locator('#sidebar-archived-workspaces');
  const sidebarTrashButton = workspaces.locator('.sidebar').locator('#sidebar-workspaces').locator('#sidebar-trashed-workspaces');
  const sidebarWorkspacesButton = workspaces.locator('.sidebar').locator('#sidebar-workspaces').locator('#sidebar-all-workspaces');
  await expect(sidebarTrashButton).toHaveText('Bin');
  await sidebarTrashButton.click();
  await expect(workspaces).toHavePageTitle('Workspaces bin');
  await expect(workspaces.locator('.workspaces-container').locator('.no-trashed-workspaces').locator('ion-text')).toHaveText(
    'Your deleted workspaces can be consulted here in read-only. They will be permanently deleted after a set period. \
    Deleted workspace owners can restore them before this date.',
  );
  await sidebarWorkspacesButton.click();

  // Archive workspace
  const wk = workspaces.locator('.workspaces-container-grid').locator('.workspace-card-item').nth(0);
  await wk.click({ button: 'right' });
  const contextMenu = workspaces.locator('.workspace-context-menu');
  await expect(contextMenu).toBeVisible();
  const archiveButton = contextMenu.getByRole('listitem').nth(4);
  await expect(archiveButton).toHaveText('Archive this workspace');
  await archiveButton.click();
  await expect(workspaces.locator('.question-modal').locator('.ms-modal-header__title')).toHaveText(['Archive this workspace']);
  await answerQuestion(workspaces, true);
  await expect(workspaces).toShowToast('The workspace wksp1 has successfully been archived.', 'Success');

  // Move workspace to bin
  await expect(wk).not.toBeVisible();
  await sidebarArchiveButton.click();
  await expect(wk).toBeVisible();
  await wk.click({ button: 'right' });
  await expect(contextMenu.getByRole('group').getByRole('listitem')).toHaveText(['Restore this workspace', 'Delete this workspace']);
  const binButton = contextMenu.getByRole('listitem').nth(1);
  await binButton.click();
  await answerQuestion(workspaces, false);
  await expect(wk).toBeVisible();
  await wk.click({ button: 'right' });
  await binButton.click();
  await answerQuestion(workspaces, true);
  await expect(workspaces).toShowToast('The workspace wksp1 has successfully been moved to the bin.', 'Success');

  await expect(wk).not.toBeVisible();
  await sidebarTrashButton.click();
  await expect(wk).toBeVisible();
  await expect(wk.locator('.custom-icon')).toBeVisible();
  await wk.click({ button: 'right' });
  await expect(contextMenu.getByRole('group').getByRole('listitem')).toHaveText('Restore this workspace');
});

msTest('Restore a trashed workspace', async ({ workspaces }) => {
  const sidebarArchiveButton = workspaces.locator('.sidebar').locator('#sidebar-workspaces').locator('#sidebar-archived-workspaces');
  const sidebarTrashButton = workspaces.locator('.sidebar').locator('#sidebar-workspaces').locator('#sidebar-trashed-workspaces');
  const sidebarWorkspacesButton = workspaces.locator('.sidebar').locator('#sidebar-workspaces').locator('#sidebar-all-workspaces');
  const wk = workspaces.locator('.workspaces-container-grid').locator('.workspace-card-item').nth(0);

  // Archive workspace
  await wk.click({ button: 'right' });
  const contextMenu = workspaces.locator('.workspace-context-menu');
  await contextMenu.getByRole('listitem').nth(4).click();
  await answerQuestion(workspaces, true);
  await expect(workspaces).toShowToast('The workspace wksp1 has successfully been archived.', 'Success');

  // Move workspace to bin
  await expect(wk).not.toBeVisible();
  await sidebarArchiveButton.click();
  await expect(wk).toBeVisible();
  await wk.click({ button: 'right' });
  await contextMenu.getByRole('listitem').nth(1).click();
  await answerQuestion(workspaces, true);
  await expect(workspaces).toShowToast('The workspace wksp1 has successfully been moved to the bin.', 'Success');

  // Restore workspace
  await expect(wk).not.toBeVisible();
  await sidebarTrashButton.click();
  await expect(wk).toBeVisible();
  await wk.click({ button: 'right' });
  const restoreButton = contextMenu.getByRole('listitem');
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

msTest('Check trashed workspace is read-only', async ({ parsecEditics }) => {
  const sidebarArchiveButton = parsecEditics.locator('.sidebar').locator('#sidebar-workspaces').locator('#sidebar-archived-workspaces');
  const sidebarTrashButton = parsecEditics.locator('.sidebar').locator('#sidebar-workspaces').locator('#sidebar-trashed-workspaces');
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

  // Archive workspace
  await sidebarWorkspacesButton.click();
  await wk.click({ button: 'right' });
  const contextMenu = parsecEditics.locator('.workspace-context-menu');
  await contextMenu.getByRole('listitem').nth(4).click();
  await answerQuestion(parsecEditics, true);
  await expect(parsecEditics).toShowToast('The workspace wksp1 has successfully been archived.', 'Success');
  await expect(wk).not.toBeVisible();
  await sidebarArchiveButton.click();
  await expect(wk).toBeVisible();

  // Move workspace to bin
  await wk.click({ button: 'right' });
  await contextMenu.getByRole('listitem').nth(1).click();
  await answerQuestion(parsecEditics, true);
  await expect(parsecEditics).toShowToast('The workspace wksp1 has successfully been moved to the bin.', 'Success');
  await expect(wk).not.toBeVisible();
  await sidebarTrashButton.click();
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

msTest('Trash archived workspace in small display', async ({ workspaces }) => {
  await workspaces.setDisplaySize(DisplaySize.Small);

  // Archive workspace
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

  // Trash workspace
  await workspaceCard.locator('.icon-option-container').click();

  await expect(popover).toBeVisible();
  await expect(popover.getByRole('listitem').nth(1)).toHaveText('Delete this workspace');
  await popover.getByRole('listitem').nth(1).click();

  await expect(workspaces.locator('.question-modal').locator('.ms-small-display-modal-header__title')).toHaveText('Delete this workspace');
  await answerQuestion(workspaces, true);
  await expect(workspaces).toShowToast('The workspace wksp1 has successfully been moved to the bin.', 'Success');
  await expect(workspaceCard).not.toBeVisible();

  await expect(switchRouteButton).toBeVisible();
  await switchRouteButton.click();

  await expect(switchModal).toBeVisible();
  await expect(switchModal.locator('.switch-item')).toHaveText(['My workspaces', 'Archived workspaces', 'Workspaces bin']);
  await switchModal.locator('.switch-item').nth(2).click();
  await switchModal.locator('.workspace-switch-button').click();

  await expect(workspaces.locator('.topbar-left-workspaces-mobile-dropdown__title')).toHaveText('Workspaces bin');
  const trashedCard = workspaces.locator('.workspaces-container-grid').locator('.workspace-card-item').nth(0);
  await expect(trashedCard).toBeVisible();
  await expect(trashedCard).toHaveClass(/workspace-card-item--trashed/);

  await trashedCard.click();
  await expect(workspaces.locator('.header-archived')).toBeVisible();
  await expect(workspaces.locator('.header-archived')).toHaveText('Deleted workspace (read-only)');
});
