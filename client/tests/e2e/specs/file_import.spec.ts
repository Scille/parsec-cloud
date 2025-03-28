// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { Locator, Page, TestInfo } from '@playwright/test';
import { createFolder, createWorkspace, dragAndDropFile, expect, msTest } from '@tests/e2e/helpers';
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

async function checkFilesUploaded(page: Page, mode: string, expectedCount: number): Promise<void> {
  const uploadMenu = page.locator('.upload-menu');
  await expect(uploadMenu).toBeVisible();
  const tabs = uploadMenu.locator('.upload-menu-tabs').getByRole('listitem');
  await expect(tabs.locator('.text-counter')).toHaveText(['0', `${expectedCount}`, '0']);
  await expect(tabs.nth(0)).not.toHaveTheClass('active');
  await expect(tabs.nth(1)).toHaveTheClass('active');
  await expect(tabs.nth(2)).not.toHaveTheClass('active');

  const container = uploadMenu.locator('.element-container');
  const elements = container.locator('.element');
  await expect(elements).toHaveCount(expectedCount);
  await expect(elements.locator('.element-details-info__size')).toHaveText(Array(expectedCount).fill(/^[0-9.]+ (G|M|K)?B$/));
}

for (const mode of ['list', 'grid']) {
  msTest(`Import by drag and drop in ${mode} mode`, async ({ workspaces }, testInfo: TestInfo) => {
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
    await checkFilesUploaded(documents, mode, 2);
    const actionBar = documents.locator('#folders-ms-action-bar');
    await expect(actionBar.locator('.counter')).toHaveText('2 items', { useInnerText: true });
    if (mode === 'list') {
      const entries = documents.locator('.folder-container').locator('.file-list-item');
      await expect(entries).toHaveCount(2);
    } else {
      const entries = documents.locator('.folder-container').locator('.file-card-item');
      await expect(entries).toHaveCount(2);
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
    await checkFilesUploaded(documents, mode, 2);
    const actionBar = documents.locator('#folders-ms-action-bar');
    await expect(actionBar.locator('.counter')).toHaveText('2 items', { useInnerText: true });
    if (mode === 'list') {
      const entries = documents.locator('.folder-container').locator('.file-list-item');
      await expect(entries).toHaveCount(2);
    } else {
      const entries = documents.locator('.folder-container').locator('.file-card-item');
      await expect(entries).toHaveCount(2);
    }
  });
}

msTest('Import folder with button', async ({ workspaces }, testInfo: TestInfo) => {
  // Start with an empty workspace
  await createWorkspace(workspaces, 'New_Workspace');
  await workspaces.locator('.workspaces-container-grid').locator('.workspace-card-item').nth(0).click();
  await expect(workspaces).toHaveHeader(['New_Workspace'], true, true);
  const documents = workspaces;

  await documents.locator('#folders-ms-action-bar').locator('#button-import').click();
  const uploadMenu = documents.locator('.upload-menu');
  await expect(uploadMenu).toBeHidden();

  const fileChooserPromise = documents.waitForEvent('filechooser');
  await documents.locator('.import-popover').locator('.import-container').getByRole('listitem').nth(1).click();
  const fileChooser = await fileChooserPromise;
  expect(fileChooser.isMultiple()).toBe(false);
  const importPath = path.join(testInfo.config.rootDir, 'data', 'imports');
  await fileChooser.setFiles([importPath]);
  await checkFilesUploaded(documents, 'list', 10);
  await documents.locator('.folder-container').locator('.file-list-item').nth(0).dblclick();
  await expect(workspaces).toHaveHeader(['New_Workspace', 'imports'], true, true);
  await expect(documents.locator('.folder-container').locator('.file-list-item')).toHaveCount(fs.readdirSync(importPath).length);
});

msTest('Import files with button', async ({ workspaces }, testInfo: TestInfo) => {
  // Start with an empty workspace
  await createWorkspace(workspaces, 'New_Workspace');
  await workspaces.locator('.workspaces-container-grid').locator('.workspace-card-item').nth(0).click();
  await expect(workspaces).toHaveHeader(['New_Workspace'], true, true);
  const documents = workspaces;

  await documents.locator('#folders-ms-action-bar').locator('#button-import').click();
  const uploadMenu = documents.locator('.upload-menu');
  await expect(uploadMenu).toBeHidden();

  const fileChooserPromise = documents.waitForEvent('filechooser');
  await documents.locator('.import-popover').locator('.import-container').getByRole('listitem').nth(0).click();
  const fileChooser = await fileChooserPromise;
  expect(fileChooser.isMultiple()).toBe(true);
  await fileChooser.setFiles([
    path.join(testInfo.config.rootDir, 'data', 'imports', 'yo.png'),
    path.join(testInfo.config.rootDir, 'data', 'imports', 'hell_yeah.png'),
  ]);
  await checkFilesUploaded(documents, 'list', 2);
  const actionBar = documents.locator('#folders-ms-action-bar');
  await expect(actionBar.locator('.counter')).toHaveText('2 items', { useInnerText: true });
  const entries = documents.locator('.folder-container').locator('.file-list-item');
  await expect(entries).toHaveCount(2);
});
