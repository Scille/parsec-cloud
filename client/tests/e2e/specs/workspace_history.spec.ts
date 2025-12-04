// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { Page, TestInfo } from '@playwright/test';
import { answerQuestion, createFolder, DisplaySize, expect, importDefaultFiles, ImportDocuments, msTest } from '@tests/e2e/helpers';

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

msTest.describe(() => {
  msTest.use({
    documentsOptions: {
      empty: true,
    },
  });

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

  msTest('Test workspace history page', async ({ documents }, testInfo: TestInfo) => {
    await importDefaultFiles(documents, testInfo, ImportDocuments.Png, true);

    await documents.locator('#connected-header').locator('.topbar-left').locator('.back-button-container').locator('ion-button').click();
    await expect(documents).toBeWorkspacePage();
    await documents.locator('.workspace-card-item').nth(0).locator('.icon-option-container').nth(0).click();
    const contextMenu = documents.locator('.workspace-context-menu');
    await contextMenu.locator('.menu-list').locator('ion-item-group').nth(1).locator('ion-item').nth(3).click();
    await expect(documents.locator('.topbar-left').locator('.topbar-left-text__title')).toHaveText('History');
    const container = documents.locator('.history-container');
    await expect(container.locator('.head-content__title')).toHaveText('Workspace: wksp1');
    const header = container.locator('.folder-header');
    const breadcrumbs = header.locator('ion-breadcrumb');
    await expect(breadcrumbs).toHaveCount(1);
    await expect(breadcrumbs).toHaveText(['wksp1']);
    const restoreButton = header.locator('.folder-header__actions').locator('#restore-button');
    await expect(restoreButton).toHaveText('Restore');
    await expect(restoreButton).toBeTrulyDisabled();
    const folderList = container.locator('.folder-list-main');
    await expect(folderList.locator('.file-list-item')).toHaveCount(2);
    await folderList.locator('.file-list-item').nth(0).dblclick();
    await expect(breadcrumbs).toHaveText([' wksp1/', 'Dir_Folder']);
    await breadcrumbs.nth(0).click();
    await expect(breadcrumbs).toHaveText([' wksp1']);

    await folderList.locator('.file-list-item').nth(1).locator('ion-checkbox').click();
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
    await expect(tabs.locator('.text-counter')).toHaveText(['0', '2', '0']);
    await expect(tabs.nth(0)).not.toHaveTheClass('active');
    await expect(tabs.nth(1)).toHaveTheClass('active');
    await expect(tabs.nth(2)).not.toHaveTheClass('active');

    const opContainer = uploadMenu.locator('.element-container');
    const elements = opContainer.locator('.element');
    await expect(elements).toHaveCount(2);
    await expect(elements.locator('.element-details__name').nth(0)).toHaveText(/^Restoration of [A-Za-z0-9_.]+$/);
  });

  msTest('Test viewer in history', async ({ documents }, testInfo: TestInfo) => {
    await importDefaultFiles(documents, testInfo, ImportDocuments.Png, true);

    await documents.locator('#connected-header').locator('.topbar-left').locator('.back-button-container').locator('ion-button').click();
    await expect(documents).toBeWorkspacePage();
    await documents.locator('.workspace-card-item').nth(0).locator('.icon-option-container').nth(0).click();
    const contextMenu = documents.locator('.workspace-context-menu');
    await contextMenu.locator('.menu-list').locator('ion-item-group').nth(1).locator('ion-item').nth(3).click();
    await expect(documents.locator('.topbar-left').locator('.topbar-left-text__title')).toHaveText('History');
    const container = documents.locator('.history-container');
    await expect(container.locator('.head-content__title')).toHaveText('Workspace: wksp1');
    const folderList = container.locator('.folder-list-main');
    await expect(folderList.locator('.file-list-item')).toHaveCount(2);

    await folderList.locator('.file-list-item').nth(1).dblclick();

    await expect(documents).toBeViewerPage();
    await expect(documents.locator('.file-handler').locator('.file-handler-topbar').locator('ion-text').nth(0)).toHaveText('image.png');
  });

  msTest('Workspace history breadcrumbs', async ({ documents }, testInfo: TestInfo) => {
    msTest.setTimeout(45_000);
    await importDefaultFiles(documents, testInfo, 0, true);

    async function clickOnBreadcrumb(i: number): Promise<void> {
      await documents.locator('.history-container').locator('.navigation-breadcrumb').locator('ion-breadcrumb').nth(i).click();
    }

    async function headerContentMatch(breadcrumbs: Array<string | RegExp>): Promise<void> {
      const bcs = documents.locator('.history-container').locator('.navigation-breadcrumb').locator('ion-breadcrumb');
      expect(bcs).toHaveCount(breadcrumbs.length);
      await expect(bcs).toHaveText(breadcrumbs, { useInnerText: true });
    }

    async function navigateDown(): Promise<void> {
      await documents.locator('.folder-container').getByRole('listitem').nth(0).dblclick();
    }

    const entries = documents.locator('.folder-container').locator('.file-list-item');
    await navigateDown();
    await createFolder(documents, 'Subdir 1');
    await expect(entries.locator('.file-name').locator('.label-name')).toHaveText(['Subdir 1']);
    await navigateDown();
    await createFolder(documents, 'Subdir 2');
    await expect(entries.locator('.file-name').locator('.label-name')).toHaveText(['Subdir 2']);
    await navigateDown();
    await createFolder(documents, 'Subdir 3');
    await expect(entries.locator('.file-name').locator('.label-name')).toHaveText(['Subdir 3']);
    await documents.locator('.sidebar').locator('#sidebar-workspaces').locator('.list-sidebar-header-text').click();
    await expect(documents).toBeWorkspacePage();
    await expect(documents.locator('.workspace-card-item')).toHaveCount(1);
    await documents.locator('.workspace-card-item').nth(0).locator('.icon-option-container').nth(0).click();
    const contextMenu = documents.locator('.workspace-context-menu');
    await expect(contextMenu).toBeVisible();
    await contextMenu.locator('.menu-list').locator('ion-item-group').nth(1).locator('ion-item').nth(3).click();
    await expect(documents).toBeWorkspaceHistoryPage();
    await headerContentMatch(['wksp1']);
    await navigateDown();
    await headerContentMatch(['wksp1', 'Dir_Folder']);
    await navigateDown();
    await headerContentMatch(['wksp1', '', 'Subdir 1']);
    await navigateDown();
    await headerContentMatch(['wksp1', '', '', 'Subdir 2']);
    await navigateDown();
    await headerContentMatch(['wksp1', '', '', '', 'Subdir 3']);

    const popoverItems = documents.locator('.breadcrumbs-popover').locator('.popover-item');
    await clickOnBreadcrumb(1);
    await expect(popoverItems).toHaveCount(3);
    await popoverItems.nth(2).click();
    await headerContentMatch(['wksp1', '', '', 'Subdir 2']);
    await clickOnBreadcrumb(0);
    await headerContentMatch(['wksp1']);

    await documents.setDisplaySize(DisplaySize.Small);
    const smallBreadcrumbsButton = documents
      .locator('.history-container')
      .locator('.breadcrumb-small-container')
      .locator('.breadcrumb-popover-button');

    await expect(documents.locator('.history-container').locator('.navigation-breadcrumb').locator('ion-breadcrumbs')).not.toBeVisible();
    await expect(smallBreadcrumbsButton).toBeHidden();
    await expect(documents.locator('.topbar-left-text__workspace')).toHaveText('wksp1');
    const currentFolder = documents.locator('.history-container').locator('.current-folder__text');
    await navigateDown();
    await expect(currentFolder).toHaveText('Dir_Folder');
    await expect(currentFolder).toBeVisible();
    await expect(smallBreadcrumbsButton).toBeVisible();
    await navigateDown();
    await expect(currentFolder).toHaveText('Subdir 1');
    await navigateDown();
    await expect(currentFolder).toHaveText('Subdir 2');
    await navigateDown();
    await expect(currentFolder).toHaveText('Subdir 3');
    await smallBreadcrumbsButton.click();

    await expect(popoverItems).toHaveCount(4);
    await expect(popoverItems.nth(0)).toHaveText('wksp1');
    await expect(popoverItems.nth(1)).toHaveText('Dir_Folder');
    await expect(popoverItems.nth(2)).toHaveText('Subdir 1');
    await expect(popoverItems.nth(3)).toHaveText('Subdir 2');
    await popoverItems.nth(1).click();
    await expect(currentFolder).toHaveText('Dir_Folder');
    await expect(smallBreadcrumbsButton).toBeVisible();
    await smallBreadcrumbsButton.click();
    await expect(popoverItems).toHaveCount(1);
    await expect(popoverItems).toHaveText('wksp1');
    await popoverItems.click();
    await expect(smallBreadcrumbsButton).toBeHidden();
  });

  msTest('Workspace history select all', async ({ documents }, testInfo: TestInfo) => {
    await importDefaultFiles(documents, testInfo, ImportDocuments.Png | ImportDocuments.Pdf, true);

    const entries = documents.locator('.folder-list-main').locator('.file-list-item');
    await expect(documents.locator('.workspace-card-item')).toHaveCount(1);
    await documents.locator('.sidebar').locator('#sidebar-workspaces').locator('.list-sidebar-header-text').click();
    await expect(documents).toBeWorkspacePage();
    await documents.locator('.workspace-card-item').nth(0).locator('.icon-option-container').nth(0).click();
    const contextMenu = documents.locator('.workspace-context-menu');
    await expect(contextMenu).toBeVisible();
    await contextMenu.locator('.menu-list').locator('ion-item-group').nth(1).locator('ion-item').nth(3).click();
    await expect(documents).toBeWorkspaceHistoryPage();
    const selectAllButton = documents.locator('.folder-header').locator('.select-button');
    await expect(selectAllButton).toHaveText('Select all');
    await selectAllButton.click();
    await expect(entries).toHaveCount(3);
    await expect(selectAllButton).toHaveText('Deselect all');
    for (const entry of await entries.all()) {
      await expect(entry.locator('ion-checkbox')).toHaveState('checked');
    }
    await selectAllButton.click();
    for (const checkbox of await entries.locator('ion-checkbox').all()) {
      await expect(checkbox).toHaveState('unchecked');
    }
  });
});
