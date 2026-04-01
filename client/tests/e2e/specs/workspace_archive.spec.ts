// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { TestInfo } from '@playwright/test';
import { answerQuestion, expect, importDefaultFiles, ImportDocuments, msTest } from '@tests/e2e/helpers';

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

  await sidebarWorkspacesButton.click();
  const wk = workspaces.locator('.workspaces-container-grid').locator('.workspace-card-item').nth(0);
  await wk.click({ button: 'right' });
  const contextMenu = workspaces.locator('.workspace-context-menu');
  await expect(contextMenu).toBeVisible();
  const archiveButton = contextMenu.getByRole('listitem').nth(4);
  await expect(archiveButton).toHaveText('Archive this workspace');
  await archiveButton.click();
  await expect(workspaces.locator('.question-modal').locator('.ms-modal-header__title')).toHaveText(['Archive this workspace']);
  await answerQuestion(workspaces, false);
  await expect(wk).toBeVisible();
  await wk.click({ button: 'right' });
  await archiveButton.click();
  await answerQuestion(workspaces, true);
  await expect(workspaces).toShowToast('The workspace wksp1 has successfully been archived.', 'Success');

  await expect(wk).not.toBeVisible();
  await sidebarArchiveButton.click();
  await expect(wk).toBeVisible();
  await expect(wk.locator('.workspace-archive')).toBeVisible();
  await expect(wk.locator('.archived-label-text')).toBeVisible();
  await expect(wk.locator('.archived-label-text')).toHaveText('Read only');
  await wk.click({ button: 'right' });
  await expect(contextMenu.getByRole('group').getByRole('listitem')).toHaveText(['Restore this workspace', 'Move to bin']);
});

msTest('Restore archived workspace', async ({ workspaces }) => {
  const sidebarArchiveButton = workspaces.locator('.sidebar').locator('#sidebar-workspaces').locator('#sidebar-archived-workspaces');
  const sidebarWorkspacesButton = workspaces.locator('.sidebar').locator('#sidebar-workspaces').locator('#sidebar-all-workspaces');
  const wk = workspaces.locator('.workspaces-container-grid').locator('.workspace-card-item').nth(0);
  await wk.click({ button: 'right' });
  const contextMenu = workspaces.locator('.workspace-context-menu');
  await contextMenu.getByRole('listitem').nth(4).click();
  await answerQuestion(workspaces, true);
  await expect(workspaces).toShowToast('The workspace wksp1 has successfully been archived.', 'Success');

  await expect(wk).not.toBeVisible();
  await sidebarArchiveButton.click();
  await expect(wk).toBeVisible();
  await wk.click({ button: 'right' });
  await expect(contextMenu.getByRole('group').getByRole('listitem')).toHaveText(['Restore this workspace', 'Move to bin']);
  await contextMenu.getByRole('group').getByRole('listitem').nth(0).click();
  await expect(workspaces).toShowToast('The workspace wksp1 has successfully been restored.', 'Success');

  await expect(wk).not.toBeVisible();
  await sidebarWorkspacesButton.click();
  await expect(wk).toBeVisible();
});

msTest('Check archived workspace is read-only', async ({ workspaces }, testInfo: TestInfo) => {
  const sidebarArchiveButton = workspaces.locator('.sidebar').locator('#sidebar-workspaces').locator('#sidebar-archived-workspaces');
  const sidebarWorkspacesButton = workspaces.locator('.sidebar').locator('#sidebar-workspaces').locator('#sidebar-all-workspaces');
  const wk = workspaces.locator('.workspaces-container-grid').locator('.workspace-card-item').nth(0);
  await wk.click();
  await importDefaultFiles(workspaces, testInfo, ImportDocuments.Docx, false);
  await expect(workspaces.locator('.file-context-menu')).toBeHidden();
  const entry = workspaces.locator('.folder-container').locator('.file-list-item').nth(0);
  await entry.click({ button: 'right' });
  await expect(workspaces.locator('.file-context-menu')).toBeVisible();
  const popover = workspaces.locator('.file-context-menu');
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
  await wk.click({ button: 'right' });
  const contextMenu = workspaces.locator('.workspace-context-menu');
  await contextMenu.getByRole('listitem').nth(4).click();
  await answerQuestion(workspaces, true);
  await expect(workspaces).toShowToast('The workspace wksp1 has successfully been archived.', 'Success');
  await expect(wk).not.toBeVisible();
  await sidebarArchiveButton.click();
  await expect(wk).toBeVisible();

  await wk.click();
  await entry.click({ button: 'right' });
  await expect(workspaces.locator('.file-context-menu')).toBeVisible();
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
