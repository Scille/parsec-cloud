// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { Page } from '@playwright/test';
import { answerQuestion, expect, msTest } from '@tests/e2e/helpers';

async function isInGridMode(page: Page): Promise<boolean> {
  return (await page.locator('#workspaces-ms-action-bar').locator('#grid-view').getAttribute('disabled')) !== null;
}

async function toggleViewMode(page: Page): Promise<void> {
  if (await isInGridMode(page)) {
    await page.locator('#workspaces-ms-action-bar').locator('#list-view').click();
  } else {
    await page.locator('#workspaces-ms-action-bar').locator('#grid-view').click();
  }
}

for (const mode of ['grid', 'list']) {
  msTest(`Open workspace history in ${mode} mode`, async ({ documents }) => {
    await documents.locator('#connected-header').locator('.topbar-left').locator('.back-button-container').locator('ion-button').click();
    await expect(documents).toBeWorkspacePage();
    if (mode === 'list') {
      await toggleViewMode(documents);
      await expect(documents.locator('.workspace-list-item')).toHaveCount(1);
      await documents.locator('.workspace-list-item').nth(0).locator('.workspace-options').click();
    } else {
      await expect(documents.locator('.workspace-card-item')).toHaveCount(1);
      await documents.locator('.workspace-card-item').nth(0).locator('.icon-option-container').nth(0).click();
    }
    const contextMenu = documents.locator('.workspace-context-menu');
    await expect(contextMenu).toBeVisible();
    await contextMenu.locator('.menu-list').locator('ion-item-group').nth(1).locator('ion-item').nth(3).click();
    await expect(documents).toBeWorkspaceHistoryPage();
  });
}

msTest('Test workspace history page', async ({ documents }) => {
  await documents.locator('#connected-header').locator('.topbar-left').locator('.back-button-container').locator('ion-button').click();
  await expect(documents).toBeWorkspacePage();
  await documents.locator('.workspace-card-item').nth(0).locator('.icon-option-container').nth(0).click();
  const contextMenu = documents.locator('.workspace-context-menu');
  await contextMenu.locator('.menu-list').locator('ion-item-group').nth(1).locator('ion-item').nth(3).click();
  await expect(documents.locator('.topbar-left').locator('.topbar-left__title')).toHaveText('History');
  const container = documents.locator('.history-container');
  await expect(container.locator('.head-content__title')).toHaveText('Workspace: wksp1');
  const header = container.locator('.folder-container-header');
  const breadcrumbs = header.locator('ion-breadcrumb');
  await expect(breadcrumbs).toHaveCount(1);
  await expect(breadcrumbs).toHaveText(['wksp1']);
  const restoreButton = header.locator('.folder-container-header__actions').locator('ion-button');
  await expect(restoreButton).toHaveText('Restore');
  await expect(restoreButton).toBeTrulyDisabled();
  const folderList = container.locator('.folder-list-main');
  await expect(folderList.locator('.file-list-item')).toHaveCount(9);
  await folderList.locator('.file-list-item').nth(0).dblclick();
  await expect(breadcrumbs).toHaveText([' wksp1/', 'Dir_Folder']);
  await breadcrumbs.nth(0).click();
  await expect(breadcrumbs).toHaveText([' wksp1']);

  await folderList.locator('.file-list-item').nth(2).locator('ion-checkbox').click();
  await expect(restoreButton).toBeTrulyEnabled();
  await restoreButton.click();
  await answerQuestion(documents, true, {
    expectedTitleText: 'Confirm file restore?',
    expectedQuestionText: `Are you sure to restore the selected file or folder?
This will overwrite any current version but you can still retrieve it in the History.`,
    expectedPositiveText: 'Restore',
    expectedNegativeText: 'Cancel',
  });
  const uploadMenu = documents.locator('.upload-menu');
  await expect(uploadMenu).toBeVisible();
  const tabs = uploadMenu.locator('.upload-menu-tabs').getByRole('listitem');
  await expect(tabs.locator('.text-counter')).toHaveText(['0', '9', '0']);
  await expect(tabs.nth(0)).not.toHaveTheClass('active');
  await expect(tabs.nth(1)).toHaveTheClass('active');
  await expect(tabs.nth(2)).not.toHaveTheClass('active');

  const opContainer = uploadMenu.locator('.element-container');
  const elements = opContainer.locator('.element');
  await expect(elements).toHaveCount(9);
  await expect(elements.locator('.element-details__name').nth(0)).toHaveText(/^Restoration of [A-Za-z0-9_.]+$/);
});

msTest('Test viewer in history', async ({ documents }) => {
  await documents.locator('#connected-header').locator('.topbar-left').locator('.back-button-container').locator('ion-button').click();
  await expect(documents).toBeWorkspacePage();
  await documents.locator('.workspace-card-item').nth(0).locator('.icon-option-container').nth(0).click();
  const contextMenu = documents.locator('.workspace-context-menu');
  await contextMenu.locator('.menu-list').locator('ion-item-group').nth(1).locator('ion-item').nth(3).click();
  await expect(documents.locator('.topbar-left').locator('.topbar-left__title')).toHaveText('History');
  const container = documents.locator('.history-container');
  await expect(container.locator('.head-content__title')).toHaveText('Workspace: wksp1');
  const folderList = container.locator('.folder-list-main');
  await expect(folderList.locator('.file-list-item')).toHaveCount(9);

  for (const entry of await folderList.locator('.file-list-item').all()) {
    const entryName = (await entry.locator('.file-name').locator('.file-name__label').textContent()) ?? '';
    if (entryName.endsWith('.txt')) {
      await entry.dblclick();
      break;
    }
  }
  await expect(documents).toBeViewerPage();
  await expect(documents).toHavePageTitle('File viewer');
  await expect(documents.locator('.file-viewer').locator('.file-viewer-topbar').locator('ion-text').nth(0)).toHaveText(/^[a-z0-9_]+\.txt$/);
  const textContainer = documents.locator('.file-viewer').locator('.text-container');
  await expect(textContainer.locator('.margin').locator('.line-numbers')).toHaveCount(2);
  await expect(textContainer.locator('.editor-scrollable')).toHaveText('A simple text file');
});
