// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { Locator, Page, TestInfo } from '@playwright/test';
import { expect } from '@tests/pw/helpers/assertions';
import { msTest } from '@tests/pw/helpers/fixtures';
import { dragAndDropFile } from '@tests/pw/helpers/utils';
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

const IMPORT_NAME_MATCHER = /^(hell_yeah|yo)\.png$/;

async function checkFilesUploaded(page: Page): Promise<void> {
  const uploadMenu = page.locator('.upload-menu');
  await expect(uploadMenu).toBeVisible();
  const tabs = uploadMenu.locator('.upload-menu-tabs').getByRole('listitem');
  await expect(tabs.locator('.text-counter')).toHaveText(['0', '2', '0']);
  await expect(tabs.nth(0)).not.toHaveTheClass('active');
  await expect(tabs.nth(1)).toHaveTheClass('active');
  await expect(tabs.nth(2)).not.toHaveTheClass('active');

  const container = uploadMenu.locator('.element-container');
  const elements = container.locator('.element');
  await expect(elements).toHaveCount(2);
  await expect(elements.locator('.element-details__name')).toHaveText([IMPORT_NAME_MATCHER, IMPORT_NAME_MATCHER]);
  await expect(elements.locator('.element-details-info__size')).toHaveText([/^23[78] KB$/, /^23[78] KB$/]);
}

for (const mode of ['list', 'grid']) {
  msTest(`Import by drag and drop in ${mode} mode`, async ({ documents }, testInfo: TestInfo) => {
    if (mode === 'grid') {
      await toggleViewMode(documents);
    }
    const dropZone = documents.locator('.folder-container').locator('.drop-zone').nth(0);
    await dragAndDropFile(documents, dropZone, [
      path.join(testInfo.config.rootDir, '..', 'data', 'imports', 'yo.png'),
      path.join(testInfo.config.rootDir, '..', 'data', 'imports', 'hell_yeah.png'),
    ]);
    await checkFilesUploaded(documents);
  });
}

for (const mode of ['list', 'grid']) {
  msTest(`Import by drag and drop on folder in ${mode} mode`, async ({ documents }, testInfo: TestInfo) => {
    let dropZone: Locator;
    if (mode === 'grid') {
      await toggleViewMode(documents);
      dropZone = documents.locator('.folder-container').locator('.file-card-item').nth(0).locator('.drop-zone');
    } else {
      dropZone = documents.locator('.folder-container').locator('.drop-zone').nth(1);
    }
    await dragAndDropFile(documents, dropZone, [
      path.join(testInfo.config.rootDir, '..', 'data', 'imports', 'yo.png'),
      path.join(testInfo.config.rootDir, '..', 'data', 'imports', 'hell_yeah.png'),
    ]);
    await checkFilesUploaded(documents);
  });
}

msTest('Import folder with button', async ({ documents }, testInfo: TestInfo) => {
  await documents.locator('#folders-ms-action-bar').locator('#button-import').click();
  const uploadMenu = documents.locator('.upload-menu');
  await expect(uploadMenu).toBeHidden();

  const fileChooserPromise = documents.waitForEvent('filechooser');
  await documents.locator('.import-popover').locator('.import-container').getByRole('listitem').nth(1).click();
  const fileChooser = await fileChooserPromise;
  expect(fileChooser.isMultiple()).toBe(false);
  await fileChooser.setFiles([path.join(testInfo.config.rootDir, '..', 'data', 'imports')]);
  await checkFilesUploaded(documents);
});

msTest('Import files with button', async ({ documents }, testInfo: TestInfo) => {
  await documents.locator('#folders-ms-action-bar').locator('#button-import').click();
  const uploadMenu = documents.locator('.upload-menu');
  await expect(uploadMenu).toBeHidden();

  const fileChooserPromise = documents.waitForEvent('filechooser');
  await documents.locator('.import-popover').locator('.import-container').getByRole('listitem').nth(0).click();
  const fileChooser = await fileChooserPromise;
  expect(fileChooser.isMultiple()).toBe(true);
  await fileChooser.setFiles([
    path.join(testInfo.config.rootDir, '..', 'data', 'imports', 'yo.png'),
    path.join(testInfo.config.rootDir, '..', 'data', 'imports', 'hell_yeah.png'),
  ]);
  await checkFilesUploaded(documents);
});
