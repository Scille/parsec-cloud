// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { Page } from '@playwright/test';
import { answerQuestion, expect, fillInputModal, msTest } from '@tests/e2e/helpers';

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
const TIME_MATCHER = /^(now|< 1 minute|(\d{1,2}|one) minutes? ago)$/;
const SIZE_MATCHER = /^[0-9.]+ (K|M|G)?B$/;

const NAME_MATCHER_ARRAY = new Array(2).fill(DIR_MATCHER).concat(new Array(8).fill(FILE_MATCHER));
const TIME_MATCHER_ARRAY = new Array(10).fill(TIME_MATCHER);
const SIZE_MATCHER_ARRAY = new Array(2).fill('').concat(new Array(8).fill(SIZE_MATCHER));

msTest('Documents page default state', async ({ documents }) => {
  const actionBar = documents.locator('#folders-ms-action-bar');
  await expect(actionBar.locator('.ms-action-bar-button:visible')).toHaveText(['New folder', 'Import']);
  await expect(actionBar.locator('.counter')).toHaveText('10 items', { useInnerText: true });
  await expect(actionBar.locator('#select-popover-button')).toHaveText('Name');
  await expect(actionBar.locator('#grid-view')).not.toHaveDisabledAttribute();
  await expect(actionBar.locator('#list-view')).toHaveDisabledAttribute();
  const entries = documents.locator('.folder-container').locator('.file-list-item');
  await expect(entries).toHaveCount(10);
  await expect(entries.locator('.file-name').locator('.file-name__label')).toHaveText(NAME_MATCHER_ARRAY);
  await expect(entries.locator('.file-lastUpdate')).toHaveText(TIME_MATCHER_ARRAY);
  await expect(entries.locator('.file-size')).toHaveText(SIZE_MATCHER_ARRAY);
});

msTest('Check documents in grid mode', async ({ documents }) => {
  await toggleViewMode(documents);
  const entries = documents.locator('.folder-container').locator('.file-card-item');
  await expect(entries).toHaveCount(10);
  await expect(entries.locator('.file-card__title')).toHaveText(NAME_MATCHER_ARRAY);
  await expect(entries.locator('.file-card-last-update')).toHaveText(TIME_MATCHER_ARRAY);
});

msTest('Documents page default state in a read only workspace', async ({ documentsReadOnly }) => {
  const actionBar = documentsReadOnly.locator('#folders-ms-action-bar');
  await expect(actionBar.locator('.ms-action-bar-button:visible')).toHaveCount(0);
  await expect(actionBar.locator('.right-side').locator('.label-role')).toHaveText('Reader');
  await expect(actionBar.locator('.counter')).toHaveText('10 items', { useInnerText: true });
  await expect(actionBar.locator('#select-popover-button')).toHaveText('Name');
  await expect(actionBar.locator('#grid-view')).not.toHaveDisabledAttribute();
  await expect(actionBar.locator('#list-view')).toHaveDisabledAttribute();
  const entries = documentsReadOnly.locator('.folder-container').locator('.file-list-item');
  await expect(entries).toHaveCount(10);
  await expect(entries.locator('.file-name').locator('.file-name__label')).toHaveText(NAME_MATCHER_ARRAY);
  await expect(entries.locator('.file-lastUpdate')).toHaveText(TIME_MATCHER_ARRAY);
  await expect(entries.locator('.file-size')).toHaveText(SIZE_MATCHER_ARRAY);
  // Useless click just to move the mouse
  await documentsReadOnly.locator('.folder-list-header__label').nth(1).click();
  for (const checkbox of await entries.locator('ion-checkbox').all()) {
    await expect(checkbox).toBeHidden();
  }
});

msTest('Select all documents', async ({ documents }) => {
  const globalCheckbox = documents.locator('.folder-container').locator('.folder-list-header').locator('ion-checkbox');
  await expect(globalCheckbox).toHaveState('unchecked');
  await globalCheckbox.click();
  await expect(globalCheckbox).toHaveState('checked');

  const entries = documents.locator('.folder-container').locator('.file-list-item');
  for (let i = 0; i < (await entries.all()).length; i++) {
    const checkbox = entries.nth(i).locator('ion-checkbox');
    await expect(checkbox).toBeVisible();
    await expect(checkbox).toHaveState('checked');
  }

  const actionBar = documents.locator('#folders-ms-action-bar');
  await expect(actionBar.locator('.ms-action-bar-button:visible')).toHaveCount(3);
  await expect(actionBar.locator('.ms-action-bar-button:visible')).toHaveText(['Move to', 'Make a copy', 'Delete']);
  await expect(actionBar.locator('.counter')).toHaveText(/\d+ selected items/, { useInnerText: true });

  await entries.nth(1).locator('ion-checkbox').click();
  await expect(entries.nth(1).locator('ion-checkbox')).toHaveState('unchecked');

  await expect(globalCheckbox).toHaveState('indeterminate');
  await expect(actionBar.locator('.counter')).toHaveText(/\d+ selected items/, { useInnerText: true });

  await globalCheckbox.click();
  await expect(globalCheckbox).toHaveState('checked');
  await expect(entries.nth(1).locator('ion-checkbox')).toHaveState('checked');

  await globalCheckbox.click();
  await expect(globalCheckbox).toHaveState('unchecked');
  for (const entry of await entries.all()) {
    const checkbox = entry.locator('ion-checkbox');
    await expect(checkbox).toBeHidden();
    await expect(checkbox).toHaveState('unchecked');
  }
});

msTest('Delete all documents', async ({ documents }) => {
  const globalCheckbox = documents.locator('.folder-container').locator('.folder-list-header').locator('ion-checkbox');
  await globalCheckbox.click();

  const actionBar = documents.locator('#folders-ms-action-bar');
  await actionBar.locator('.ms-action-bar-button:visible').nth(2).click();
  await answerQuestion(documents, true, {
    expectedTitleText: 'Delete multiple items',
    expectedQuestionText: /Are you sure you want to delete these \d+ items\?/,
    expectedPositiveText: /Delete \d+ items/,
    expectedNegativeText: 'Keep items',
  });
});

msTest('Create a folder', async ({ documents }) => {
  const actionBar = documents.locator('#folders-ms-action-bar');
  await actionBar.locator('.ms-action-bar-button:visible').nth(0).click();
  await fillInputModal(documents, 'My folder');
  await expect(documents).toShowToast('Failed to create folder `My folder` because an entry with the same name already exists.', 'Error');
});

msTest('Import context menu', async ({ documents }) => {
  await expect(documents.locator('.import-popover')).toBeHidden();
  const actionBar = documents.locator('#folders-ms-action-bar');
  await actionBar.locator('.ms-action-bar-button:visible').nth(1).click();

  const popover = documents.locator('.import-popover');
  await expect(popover).toBeVisible();

  await expect(popover.getByRole('listitem')).toHaveText(['Import files', 'Import a folder']);
});

msTest('Selection in grid mode', async ({ documents }) => {
  await documents.locator('.folder-container').locator('.folder-list-header').locator('ion-checkbox').click();
  await toggleViewMode(documents);
  const entries = documents.locator('.folder-container').locator('.file-card-item');

  for (const entry of await entries.all()) {
    await expect(entry.locator('ion-checkbox')).toHaveState('checked');
  }
  await entries.nth(1).locator('ion-checkbox').click();
  const actionBar = documents.locator('#folders-ms-action-bar');
  await expect(actionBar.locator('.counter')).toHaveText('9 selected items', { useInnerText: true });
  await entries.nth(3).locator('ion-checkbox').click();
  await expect(actionBar.locator('.counter')).toHaveText('8 selected items', { useInnerText: true });

  await expect(entries.nth(0).locator('ion-checkbox')).toHaveState('checked');
  await expect(entries.nth(1).locator('ion-checkbox')).toHaveState('unchecked');
  await expect(entries.nth(2).locator('ion-checkbox')).toHaveState('checked');
  await expect(entries.nth(3).locator('ion-checkbox')).toHaveState('unchecked');
});

for (const gridMode of [false, true]) {
  msTest(`Open file in ${gridMode ? 'grid' : 'list'} mode`, async ({ documents }) => {
    await expect(documents.locator('.information-modal')).toBeHidden();
    await expect(documents).toHaveHeader(['The Copper Coronet'], true, true);
    if (gridMode) {
      await toggleViewMode(documents);
      await documents.locator('.folder-container').locator('.file-card-item').nth(2).dblclick();
    } else {
      await documents.locator('.folder-container').getByRole('listitem').nth(2).dblclick();
    }

    await expect(documents).toHavePageTitle('File viewer');
  });

  msTest(`Navigation back and forth in ${gridMode ? 'grid' : 'list'} mode`, async ({ documents }) => {
    async function navigateDown(): Promise<void> {
      if (gridMode) {
        await documents.locator('.folder-container').locator('.file-card-item').nth(0).dblclick();
      } else {
        await documents.locator('.folder-container').getByRole('listitem').nth(0).dblclick();
      }
    }

    async function navigateUp(): Promise<void> {
      await documents.locator('#connected-header').locator('.topbar-left').locator('.back-button-container').locator('ion-button').click();
    }

    if (gridMode) {
      await toggleViewMode(documents);
    }

    await expect(documents).toHaveHeader(['The Copper Coronet'], true, true);
    await navigateDown();
    await expect(documents).toHaveHeader(['The Copper Coronet', DIR_MATCHER], true, true);
    await navigateDown();
    await expect(documents).toHaveHeader(['The Copper Coronet', DIR_MATCHER, DIR_MATCHER], true, true);
    await navigateUp();
    await expect(documents).toHaveHeader(['The Copper Coronet', DIR_MATCHER], true, true);
    await navigateUp();
    await expect(documents).toHaveHeader(['The Copper Coronet'], true, true);
    await navigateUp();
    await expect(documents).toBeWorkspacePage();
  });
}

msTest('Show recently opened files in sidebar', async ({ documents }) => {
  const sidebarRecentList = documents.locator('.sidebar').locator('.file-workspaces').locator('ion-list');
  await expect(sidebarRecentList.locator('.list-sidebar-header')).toHaveText('Recent documents');
  // Two recently opened files by default in dev mode
  await expect(sidebarRecentList.locator('.sidebar-item')).toHaveText(['File_Fake PDF document.pdf', 'File_Fake image.png']);

  await expect(documents.locator('.information-modal')).toBeHidden();
  await expect(documents).toHaveHeader(['The Copper Coronet'], true, true);
  const fileItem = documents.locator('.folder-container').getByRole('listitem').nth(2);
  const fileName = await fileItem.locator('.file-name').textContent();
  await fileItem.dblclick();
  await expect(documents.locator('.ms-spinner-modal')).toBeVisible();
  await expect(documents.locator('.ms-spinner-modal').locator('.spinner-label__text')).toHaveText('Opening file...');
  await expect(documents.locator('.ms-spinner-modal')).toBeHidden();
  await expect(documents).toHavePageTitle('File viewer');
  // One file added
  await expect(sidebarRecentList.locator('.sidebar-item')).toHaveText([
    fileName ?? '',
    'File_Fake PDF document.pdf',
    'File_Fake image.png',
  ]);
});
