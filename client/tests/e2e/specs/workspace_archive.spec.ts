// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import {
  answerQuestion,
  checkEntryContextMenu,
  checkWorkspaceContextMenu,
  DisplaySize,
  expect,
  login,
  mockLibParsec,
  msTest,
} from '@tests/e2e/helpers';
import { DateTime } from 'luxon';

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
  await checkWorkspaceContextMenu(workspaces, 'owner', 'Archive this workspace');
  await expect(workspaces.locator('.question-modal').locator('.ms-modal-header__title')).toHaveText('Archive this workspace');
  await answerQuestion(workspaces, false);
  await expect(wk).toBeVisible();
  await wk.click({ button: 'right' });
  await checkWorkspaceContextMenu(workspaces, 'owner', 'Archive this workspace');
  await answerQuestion(workspaces, true);
  await expect(workspaces).toShowToast('The workspace wksp1 has successfully been archived.', 'Success');

  await expect(wk).not.toBeVisible();
  await sidebarArchiveButton.click();
  await expect(wk).toBeVisible();
  await expect(wk.locator('.custom-icon')).toBeVisible();
  await expect(wk.locator('.archived-label')).toBeVisible();
  await expect(wk.locator('.archived-label')).toHaveText('Read only');
  await wk.click({ button: 'right' });
  await checkWorkspaceContextMenu(workspaces, 'archived', 'dismiss');
});

msTest('Restore an archived workspace', async ({ workspaces }) => {
  const sidebarArchiveButton = workspaces.locator('.sidebar').locator('#sidebar-workspaces').locator('#sidebar-archived-workspaces');
  const sidebarWorkspacesButton = workspaces.locator('.sidebar').locator('#sidebar-workspaces').locator('#sidebar-all-workspaces');
  const wk = workspaces.locator('.workspaces-container-grid').locator('.workspace-card-item').nth(0);

  // Archive workspace
  await wk.click({ button: 'right' });
  await checkWorkspaceContextMenu(workspaces, 'owner', 'Archive this workspace');
  await answerQuestion(workspaces, true);
  await expect(workspaces).toShowToast('The workspace wksp1 has successfully been archived.', 'Success');

  // Restore workspace
  await expect(wk).not.toBeVisible();
  await sidebarArchiveButton.click();
  await expect(wk).toBeVisible();
  await wk.click({ button: 'right' });
  await checkWorkspaceContextMenu(workspaces, 'archived', 'Restore this workspace');
  await answerQuestion(workspaces, false);
  await expect(wk).toBeVisible();
  await wk.click({ button: 'right' });
  await checkWorkspaceContextMenu(workspaces, 'archived', 'Restore this workspace');
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
  await checkEntryContextMenu(parsecEditics, 'file-full', 'dismiss', { canEdit: true });
  await sidebarWorkspacesButton.click();

  // Archive workspace
  await wk.click({ button: 'right' });
  await checkWorkspaceContextMenu(parsecEditics, 'owner', 'Archive this workspace');
  await answerQuestion(parsecEditics, true);
  await expect(parsecEditics).toShowToast('The workspace wksp1 has successfully been archived.', 'Success');
  await expect(wk).not.toBeVisible();
  await sidebarArchiveButton.click();
  await expect(wk).toBeVisible();

  // Check file context menu
  await wk.click();
  await entry.click({ button: 'right' });

  await checkEntryContextMenu(parsecEditics, 'file-readonly', 'dismiss');
});

msTest('Archive workspace in small display', async ({ workspaces }) => {
  await workspaces.setDisplaySize(DisplaySize.Small);

  const workspaceCard = workspaces.locator('.workspaces-container-grid').locator('.workspace-card-item').nth(0);
  await workspaceCard.locator('.icon-option-container').click();

  await checkWorkspaceContextMenu(workspaces, 'owner', 'Archive this workspace');

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

msTest('Check workspace archiving sync on second client', async ({ workspaces }) => {
  const secondTab = await workspaces.openNewTab();
  await login(secondTab, 'Boby McBobFace');

  const aliceSidebarArchiveButton = workspaces.locator('.sidebar').locator('#sidebar-workspaces').locator('#sidebar-archived-workspaces');
  const bobSidebarArchiveButton = secondTab.locator('.sidebar').locator('#sidebar-workspaces').locator('#sidebar-archived-workspaces');

  const bobWk = secondTab.locator('.workspaces-container-grid').locator('.workspace-card-item').nth(0);
  await expect(bobWk).toBeVisible();
  await bobSidebarArchiveButton.click();
  await expect(bobWk).not.toBeVisible();

  // Archive workspace
  const aliceWk = workspaces.locator('.workspaces-container-grid').locator('.workspace-card-item').nth(0);
  await aliceWk.click({ button: 'right' });
  await checkWorkspaceContextMenu(workspaces, 'owner', 'Archive this workspace');
  await answerQuestion(workspaces, true);
  await expect(workspaces).toShowToast('The workspace wksp1 has successfully been archived.', 'Success');

  // Check on first tab
  await expect(aliceWk).not.toBeVisible();
  await aliceSidebarArchiveButton.click();
  await expect(aliceWk).toBeVisible();

  // Check on second tab
  await expect(bobWk).toBeVisible();

  // Restore workspace
  await aliceWk.click({ button: 'right' });
  await checkWorkspaceContextMenu(workspaces, 'archived', 'Restore this workspace');
  await answerQuestion(workspaces, true);
  await expect(workspaces).toShowToast('The workspace wksp1 has successfully been restored.', 'Success');

  // Check both tabs
  await expect(aliceWk).not.toBeVisible();
  await expect(bobWk).not.toBeVisible();
});

msTest('Check archived workspace displayed timestamp', async ({ workspaces }) => {
  const sidebarArchiveButton = workspaces.locator('.sidebar').locator('#sidebar-workspaces').locator('#sidebar-archived-workspaces');
  const wk = workspaces.locator('.workspaces-container-grid').locator('.workspace-card-item').nth(0);

  // Archive workspace
  await wk.click({ button: 'right' });
  await checkWorkspaceContextMenu(workspaces, 'owner', 'Archive this workspace');
  const archivingTimestamp = DateTime.now();
  await answerQuestion(workspaces, true);
  await expect(workspaces).toShowToast('The workspace wksp1 has successfully been archived.', 'Success');

  // Check timestamp
  await sidebarArchiveButton.click();
  const wkTimestamp = wk.locator('.workspace-card-content__update');
  await expect(wkTimestamp).toContainText('Archived on: ');
  await expect(wkTimestamp).toContainText(`${archivingTimestamp.day}`);
  await expect(wkTimestamp).toContainText(`${archivingTimestamp.year}`);
});

msTest('Archived workspaces list error', async ({ workspaces }) => {
  const sidebarArchiveButton = workspaces.locator('.sidebar').locator('#sidebar-workspaces').locator('#sidebar-archived-workspaces');
  const wk = workspaces.locator('.workspaces-container-grid').locator('.workspace-card-item').nth(0);

  // Archive workspace
  await wk.click({ button: 'right' });
  const contextMenu = workspaces.locator('.workspace-context-menu');
  await contextMenu.getByRole('listitem').nth(4).click();
  await answerQuestion(workspaces, true);
  await expect(workspaces).toShowToast('The workspace wksp1 has successfully been archived.', 'Success');

  await mockLibParsec(workspaces, [
    {
      name: 'clientListWorkspaces',
      result: { ok: false, error: { tag: 'ClientListWorkspacesErrorInternal', error: 'failed' } },
    },
  ]);
  await sidebarArchiveButton.click();
  await expect(workspaces).toShowToast('Failed to list the workspaces.', 'Error');
});
