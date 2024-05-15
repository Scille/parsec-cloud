// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { Page } from '@playwright/test';
import { expect } from '@tests/pw/helpers/assertions';
import { msTest } from '@tests/pw/helpers/fixtures';

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

const FILE_MATCHER = /^File_[a-z0-9_.]+$/;
const DIR_MATCHER = /^Dir_[a-z_]+$/;
const TIME_MATCHER = /^(now|\d{1,2} minutes ago)$/;
const SIZE_MATCHER = /^[0-9.]+ (K|M|G)?B$/;

msTest('Documents page default state', async ({ documents }) => {
  const actionBar = documents.locator('#folders-ms-action-bar');
  await expect(actionBar.locator('.ms-action-bar-button')).toHaveText(['New folder', 'Import']);
  await expect(actionBar.locator('.counter')).toHaveText('4 items', { useInnerText: true });
  await expect(actionBar.locator('#select-popover-button')).toHaveText('Name');
  await expect(actionBar.locator('#grid-view')).not.toHaveDisabledAttribute();
  await expect(actionBar.locator('#list-view')).toHaveDisabledAttribute();
  const entries = documents.locator('.folder-container').locator('.file-list-item');
  await expect(entries).toHaveCount(4);
  await expect(entries.locator('.file-name').locator('.file-name__label')).toHaveText([
    DIR_MATCHER,
    DIR_MATCHER,
    FILE_MATCHER,
    FILE_MATCHER,
  ]);
  await expect(entries.locator('.file-lastUpdate')).toHaveText(new Array(4).fill(TIME_MATCHER));
  await expect(entries.locator('.file-size')).toHaveText(['', '', SIZE_MATCHER, SIZE_MATCHER]);
});

msTest('Check documents in grid mode', async ({ documents }) => {
  await toggleViewMode(documents);
  const entries = documents.locator('.folder-container').locator('.file-card-item');
  await expect(entries).toHaveCount(4);
  await expect(entries.locator('.card-content__title')).toHaveText([DIR_MATCHER, DIR_MATCHER, FILE_MATCHER, FILE_MATCHER]);
  await expect(entries.locator('.card-content-last-update')).toHaveText(new Array(4).fill(TIME_MATCHER));
});
