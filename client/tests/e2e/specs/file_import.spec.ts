// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { Locator, Page, TestInfo } from '@playwright/test';
import { createFolder, createWorkspace, DisplaySize, dragAndDropFile, expect, msTest } from '@tests/e2e/helpers';
import * as fs from 'fs';
import path from 'path';

async function isInGridMode(page: Page): Promise<boolean> {
  return (await page.locator('#folders-ms-action-bar').locator('#grid-view').getAttribute('disabled')) !== null;
}

async function toggleViewMode(page: Page): Promise<void> {
  if (await isInGridMode(page)) {
    await page.locator('#folders-ms-action-bar').locator('#list-view').click();
  } else {
    await page.locator('#folders-ms-action-bar').locator('#grid-view').click();
  }
}

async function checkFilesUploaded(page: Page, expectedCount: number, lastOneFilesCount = 1): Promise<void> {
  const uploadMenu = page.locator('.upload-menu');
  await expect(uploadMenu).toBeVisible();
  const opItems = uploadMenu.locator('.upload-menu-list').locator('.file-operation-item');
  await expect(opItems).toHaveCount(expectedCount);

  await expect(uploadMenu.locator('.upload-menu-tabs__item').nth(0)).toHaveText('In progress');
  await uploadMenu.locator('.upload-menu-tabs__item').nth(0).click();
  await expect(opItems).toHaveCount(0);

  await expect(uploadMenu.locator('.upload-menu-tabs__item').nth(1)).toHaveText('Done');
  await uploadMenu.locator('.upload-menu-tabs__item').nth(1).click();
  await expect(opItems).toHaveCount(expectedCount);

  await expect(uploadMenu.locator('.upload-menu-tabs__item').nth(2)).toHaveText('Failed');
  await uploadMenu.locator('.upload-menu-tabs__item').nth(2).click();
  await expect(opItems).toHaveCount(0);

  await uploadMenu.locator('.upload-menu-tabs__item').nth(2).click();
  await expect(opItems).toHaveCount(expectedCount);

  const lastElement = opItems.first();
  await expect(lastElement.locator('.element-details')).toBeVisible();

  if (lastOneFilesCount > 1) {
    await expect(lastElement).toHaveTheClass('multiple_elements');
    await expect(lastElement.locator('.element-details-title__name')).toHaveText(`Importing ${lastOneFilesCount} files`);
  } else {
    await expect(lastElement).not.toHaveTheClass('multiple_elements');
    await expect(lastElement.locator('.element-details-title__name')).toHaveText(/^[A-Za-z0-9-_.]+$/);
    await expect(lastElement.locator('.element-details-info')).toHaveText(/^[0-9.]+ (G|M|K)?B • [a-zA-Z0-9-_ ]+$/);
  }
}

for (const mode of ['list', 'grid']) {
  msTest(`Import by drag and drop in ${mode} mode`, { tag: '@important' }, async ({ workspaces }, testInfo: TestInfo) => {
    // Start with an empty workspace
    await createWorkspace(workspaces, 'New_Workspace');
    await workspaces.locator('.workspaces-container-grid').locator('.workspace-card-item').nth(0).click();
    await expect(workspaces).toHaveHeader(['New_Workspace'], true, true);

    const documents = workspaces;
    if (mode === 'grid') {
      await toggleViewMode(documents);
    }
    const dropZone = documents.locator('.folder-container').locator('.drop-zone').nth(0);
    await dragAndDropFile(documents, dropZone, [
      path.join(testInfo.config.rootDir, 'data', 'imports', 'yo.png'),
      path.join(testInfo.config.rootDir, 'data', 'imports', 'hell_yeah.png'),
    ]);
    await checkFilesUploaded(documents, 1, 2);
    const actionBar = documents.locator('#folders-ms-action-bar');
    await expect(actionBar.locator('.counter')).toHaveText('2 items', { useInnerText: true });
    if (mode === 'list') {
      const entries = documents.locator('.folder-container').locator('.file-list-item');
      await expect(entries).toHaveCount(2);
      await expect(entries.locator('.label-name')).toHaveText(['hell_yeah.png', 'yo.png']);
    } else {
      const entries = documents.locator('.folder-container').locator('.file-card-item');
      await expect(entries).toHaveCount(2);
      await expect(entries.locator('.file-card__title')).toHaveText(['hell_yeah.png', 'yo.png']);
    }
  });
}

for (const mode of ['list', 'grid']) {
  msTest(`Import by drag and drop on folder in ${mode} mode`, async ({ workspaces }, testInfo: TestInfo) => {
    // Start with an empty workspace
    await createWorkspace(workspaces, 'New_Workspace');
    await workspaces.locator('.workspaces-container-grid').locator('.workspace-card-item').nth(0).click();
    await expect(workspaces).toHaveHeader(['New_Workspace'], true, true);
    await createFolder(workspaces, 'Folder');

    const documents = workspaces;
    let dropZone: Locator;
    if (mode === 'grid') {
      await toggleViewMode(documents);
      dropZone = documents.locator('.folder-container').locator('.file-card-item').nth(0).locator('.drop-zone');
    } else {
      dropZone = documents.locator('.folder-container').locator('.drop-zone').nth(1);
    }

    await dragAndDropFile(documents, dropZone, [
      path.join(testInfo.config.rootDir, 'data', 'imports', 'yo.png'),
      path.join(testInfo.config.rootDir, 'data', 'imports', 'hell_yeah.png'),
    ]);
    if (mode === 'grid') {
      documents.locator('.folder-container').locator('.file-card-item').nth(0).dblclick();
    } else {
      documents.locator('.folder-container').locator('.file-list-item').nth(0).dblclick();
    }
    await expect(workspaces).toHaveHeader(['New_Workspace', 'Folder'], true, true);
    await checkFilesUploaded(documents, 1, 2);
    const actionBar = documents.locator('#folders-ms-action-bar');
    await expect(actionBar.locator('.counter')).toHaveText('2 items', { useInnerText: true });
    if (mode === 'list') {
      const entries = documents.locator('.folder-container').locator('.file-list-item');
      await expect(entries).toHaveCount(2);
      await expect(entries.locator('.label-name')).toHaveText(['hell_yeah.png', 'yo.png']);
    } else {
      const entries = documents.locator('.folder-container').locator('.file-card-item');
      await expect(entries).toHaveCount(2);
      await expect(entries.locator('.file-card__title')).toHaveText(['hell_yeah.png', 'yo.png']);
    }

    await documents.locator('#connected-header').locator('.topbar-left').locator('.back-button').click();
    await expect(workspaces).toHaveHeader(['New_Workspace'], true, true);

    const uploadMenu = workspaces.locator('.upload-menu');
    await expect(uploadMenu).toBeVisible();
    const opItems = uploadMenu.locator('.upload-menu-list').locator('.file-operation-item');
    await expect(opItems).toHaveCount(1);
    await expect(opItems.nth(0).locator('.folder-icon ')).toBeVisible();
    await opItems.nth(0).locator('.folder-icon').hover();
    await expect(opItems.nth(0).locator('.hover-state')).toBeVisible();
    await expect(opItems.nth(0).locator('.hover-state')).toHaveText('Show destination folder');
    await opItems.nth(0).locator('.folder-icon').click();
    await expect(workspaces).toHaveHeader(['New_Workspace', 'Folder'], true, true);
  });
}

for (const displaySize of [DisplaySize.Small, DisplaySize.Large]) {
  msTest(`Import folder with button ${displaySize} display`, async ({ workspaces }, testInfo: TestInfo) => {
    if (displaySize === DisplaySize.Small) {
      await workspaces.setDisplaySize(DisplaySize.Small);
    }

    // Start with an empty workspace
    await createWorkspace(workspaces, 'New_Workspace');
    await workspaces.locator('.workspaces-container-grid').locator('.workspace-card-item').nth(0).click();
    if (displaySize === DisplaySize.Large) {
      await expect(workspaces).toHaveHeader(['New_Workspace'], true, true);
    } else {
      await expect(workspaces.locator('.breadcrumb-file-mobile__title')).toHaveText('New_Workspace');
    }
    const documents = workspaces;

    if (displaySize === DisplaySize.Small) {
      const addButton = workspaces.locator('.tab-bar-menu').locator('#add-menu-fab-button');
      await expect(addButton).toBeVisible();
      await addButton.click();
      const modal = workspaces.locator('.tab-menu-modal');
      await expect(modal).toBeVisible();
      await modal.locator('.list-group-item').filter({ hasText: 'Import a folder' }).click();
    } else {
      await documents.locator('#folders-ms-action-bar').getByText('Import').click();
    }
    const uploadMenu = documents.locator('.upload-menu');
    await expect(uploadMenu).toBeHidden();

    const fileChooserPromise = documents.waitForEvent('filechooser');
    if (displaySize === DisplaySize.Large) {
      await documents.locator('.import-popover').locator('.import-container').getByRole('listitem').nth(1).click();
    }
    const fileChooser = await fileChooserPromise;
    expect(fileChooser.isMultiple()).toBe(false);
    const importPath = path.join(testInfo.config.rootDir, 'data', 'imports');
    await fileChooser.setFiles([importPath]);
    await checkFilesUploaded(documents, 1, 11);
    await documents.locator('.upload-menu').locator('.menu-header-icons').locator('ion-icon').nth(1).click();
    await expect(documents.locator('.upload-menu')).toBeHidden();
    await documents.locator('.folder-container').locator('.file-list-item').locator('.label-name').nth(0).click();
    if (displaySize === DisplaySize.Large) {
      await expect(workspaces).toHaveHeader(['New_Workspace', 'imports'], true, true);
    }
    await expect(documents.locator('.folder-container').locator('.file-list-item')).toHaveCount(fs.readdirSync(importPath).length);
  });
}

for (const displaySize of [DisplaySize.Small, DisplaySize.Large]) {
  msTest(`Import files with button ${displaySize} display`, async ({ workspaces }, testInfo: TestInfo) => {
    if (displaySize === DisplaySize.Small) {
      await workspaces.setDisplaySize(DisplaySize.Small);
    }

    // Start with an empty workspace
    await createWorkspace(workspaces, 'New_Workspace');
    await workspaces.locator('.workspaces-container-grid').locator('.workspace-card-item').nth(0).click();
    if (displaySize === DisplaySize.Large) {
      await expect(workspaces).toHaveHeader(['New_Workspace'], true, true);
    }
    const documents = workspaces;

    if (displaySize === DisplaySize.Small) {
      const addButton = workspaces.locator('.tab-bar-menu').locator('#add-menu-fab-button');
      await expect(addButton).toBeVisible();
      await addButton.click();
      const modal = workspaces.locator('.tab-menu-modal');
      await expect(modal).toBeVisible();
      await modal.locator('.list-group-item').filter({ hasText: 'Import files' }).click();
    } else {
      await expect(documents.locator('.import-popover')).toBeHidden();
      await documents.locator('#folders-ms-action-bar').getByText('Import').click();
    }

    const uploadMenu = documents.locator('.upload-menu');
    await expect(uploadMenu).toBeHidden();

    const fileChooserPromise = documents.waitForEvent('filechooser');
    if (displaySize === DisplaySize.Large) {
      const importFilesButton = documents.locator('.import-popover').locator('.import-container').getByRole('listitem').nth(0);
      await expect(importFilesButton).toHaveText('Import files');
      await importFilesButton.click();
      await expect(documents.locator('.import-popover')).toBeHidden();
    }
    const fileChooser = await fileChooserPromise;
    expect(fileChooser.isMultiple()).toBe(true);
    await fileChooser.setFiles([
      path.join(testInfo.config.rootDir, 'data', 'imports', 'yo.png'),
      path.join(testInfo.config.rootDir, 'data', 'imports', 'hell_yeah.png'),
    ]);
    await checkFilesUploaded(documents, 1, 2);
    if (displaySize === DisplaySize.Large) {
      const actionBar = documents.locator('#folders-ms-action-bar');
      await expect(actionBar.locator('.counter')).toHaveText('2 items', { useInnerText: true });
    }
    const entries = documents.locator('.folder-container').locator('.file-list-item');
    await expect(entries).toHaveCount(2);
    await expect(entries.locator('.label-name')).toHaveText(['hell_yeah.png', 'yo.png']);
  });
}

for (const dupPolicy of ['replace', 'ignore', 'addCount']) {
  msTest(`Import existing file and ${dupPolicy}`, async ({ workspaces }) => {
    // Start with an empty workspace
    await createWorkspace(workspaces, 'New_Workspace');
    await workspaces.locator('.workspaces-container-grid').locator('.workspace-card-item').nth(0).click();
    await expect(workspaces).toHaveHeader(['New_Workspace'], true, true);

    const documents = workspaces;
    const dropZone = documents.locator('.folder-container').locator('.drop-zone').nth(0);
    await dragAndDropFile(documents, dropZone, [], [{ name: 'file.txt', content: 'MQ==' }]);
    await checkFilesUploaded(documents, 1, 1);
    const actionBar = documents.locator('#folders-ms-action-bar');
    await expect(actionBar.locator('.counter')).toHaveText('1 item', { useInnerText: true });
    const entries = documents.locator('.folder-container').locator('.file-list-item');
    await expect(entries).toHaveCount(1);
    await expect(entries.nth(0).locator('.label-name')).toHaveText('file.txt');
    await expect(entries.nth(0).locator('.label-size')).toHaveText('1 B');

    const dupModal = documents.locator('.file-operation-conflicts-modal');
    await expect(dupModal).toBeHidden();

    await dragAndDropFile(documents, dropZone, [], [{ name: 'file.txt', content: 'MTIzNDU=' }]);

    await expect(dupModal).toBeVisible();
    const buttons = dupModal.locator('.modal-footer-buttons__item');
    await expect(buttons).toHaveText(['Ignore', 'Replace', 'Keep both']);

    if (dupPolicy === 'replace') {
      await buttons.nth(1).click();
      await expect(entries).toHaveCount(1);
      await expect(entries.nth(0).locator('.label-name')).toHaveText('file.txt');
      await expect(entries.nth(0).locator('.label-size')).toHaveText('5 B');
    } else if (dupPolicy === 'ignore') {
      await buttons.nth(0).click();
      await expect(entries).toHaveCount(1);
      await expect(entries.nth(0).locator('.label-name')).toHaveText('file.txt');
      await expect(entries.nth(0).locator('.label-size')).toHaveText('1 B');
    } else {
      await buttons.nth(2).click();
      await expect(entries).toHaveCount(2);
      await expect(entries.locator('.label-name')).toHaveText(['file (2).txt', 'file.txt']);
      await expect(entries.locator('.label-size')).toHaveText(['5 B', '1 B']);
    }
    await checkFilesUploaded(documents, 2, 1);
  });
}
