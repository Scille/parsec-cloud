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
  msTest(`Open workspace history in ${mode} mode`, async ({ connected }) => {
    if (mode === 'list') {
      await toggleViewMode(connected);
      await expect(connected.locator('.workspace-list-item')).toHaveCount(3);
      await connected.locator('.workspace-list-item').nth(1).locator('.workspace-options').click();
    } else {
      await expect(connected.locator('.workspace-card-item')).toHaveCount(3);
      await connected.locator('.workspace-card-item').nth(1).locator('.icon-option-container').nth(1).click();
    }
    const contextMenu = connected.locator('.workspace-context-menu');
    await expect(contextMenu).toBeVisible();
    await contextMenu.locator('.menu-list').locator('ion-item-group').nth(1).locator('ion-item').nth(3).click();
    await expect(connected).toBeWorkspaceHistoryPage();
  });
}

msTest('Test workspace history page', async ({ connected }) => {
  await connected.locator('.workspace-card-item').nth(1).locator('.icon-option-container').nth(1).click();
  const contextMenu = connected.locator('.workspace-context-menu');
  await contextMenu.locator('.menu-list').locator('ion-item-group').nth(1).locator('ion-item').nth(3).click();
  await expect(connected.locator('.topbar-left').locator('.topbar-left__title')).toHaveText('History');
  const container = connected.locator('.history-container');
  await expect(container.locator('.head-content__title')).toHaveText('Workspace: Trademeet');
  const header = container.locator('.folder-container-header');
  const breadcrumbs = header.locator('ion-breadcrumb');
  await expect(breadcrumbs).toHaveCount(1);
  await expect(breadcrumbs).toHaveText(['Trademeet']);
  const restoreButton = header.locator('.folder-container-header__actions').locator('ion-button');
  await expect(restoreButton).toHaveText('Restore');
  await expect(restoreButton).toBeTrulyDisabled();
  const folderList = container.locator('.folder-list-main');
  await expect(folderList.locator('.file-list-item')).toHaveCount(11);
  await folderList.locator('.file-list-item').nth(0).dblclick();
  await expect(breadcrumbs).toHaveText([' Trademeet/', /^ Dir_.+$/]);

  await folderList.locator('.file-list-item').nth(2).locator('ion-checkbox').click();
  await expect(restoreButton).toBeTrulyEnabled();
  await restoreButton.click();
  await answerQuestion(connected, true, {
    expectedTitleText: 'Confirm file restore?',
    expectedQuestionText: `Are you sure to restore the selected file or folder?
This will overwrite any current version but you can still retrieve it in the History.`,
    expectedPositiveText: 'Restore',
    expectedNegativeText: 'Cancel',
  });
  const uploadMenu = connected.locator('.upload-menu');
  await expect(uploadMenu).toBeVisible();
  const tabs = uploadMenu.locator('.upload-menu-tabs').getByRole('listitem');
  await expect(tabs.locator('.text-counter')).toHaveText(['0', '1', '0']);
  await expect(tabs.nth(0)).not.toHaveTheClass('active');
  await expect(tabs.nth(1)).toHaveTheClass('active');
  await expect(tabs.nth(2)).not.toHaveTheClass('active');

  const opContainer = uploadMenu.locator('.element-container');
  const elements = opContainer.locator('.element');
  await expect(elements).toHaveCount(1);
  await expect(elements.locator('.element-details__name')).toHaveText(/^Restoration of File_[a-z0-9_.]+$/);
});

msTest('Test viewer in history', async ({ connected }) => {
  await connected.locator('.workspace-card-item').nth(1).locator('.icon-option-container').nth(1).click();
  const contextMenu = connected.locator('.workspace-context-menu');
  await contextMenu.locator('.menu-list').locator('ion-item-group').nth(1).locator('ion-item').nth(3).click();
  await expect(connected.locator('.topbar-left').locator('.topbar-left__title')).toHaveText('History');
  const container = connected.locator('.history-container');
  await expect(container.locator('.head-content__title')).toHaveText('Workspace: Trademeet');
  const folderList = container.locator('.folder-list-main');
  await expect(folderList.locator('.file-list-item')).toHaveCount(11);

  for (const entry of await folderList.locator('.file-list-item').all()) {
    const entryName = (await entry.locator('.file-name').locator('.file-name__label').textContent()) ?? '';
    if (entryName.endsWith('.txt')) {
      await entry.dblclick();
      break;
    }
  }
  await expect(connected).toBeViewerPage();
  await expect(connected).toHavePageTitle('File viewer');
  await expect(connected.locator('.file-viewer').locator('.file-viewer-topbar').locator('ion-text').nth(0)).toHaveText(
    /^File_[a-z0-9_]+\.txt$/,
  );
  const textContainer = connected.locator('.file-viewer').locator('.text-container');
  await expect(textContainer.locator('.margin').locator('.line-numbers')).toHaveCount(2);
  await expect(textContainer.locator('.editor-scrollable')).toHaveText('A simple text file');
});
