// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { Page } from '@playwright/test';
import { expect } from '@tests/pw/helpers/assertions';
import { msTest } from '@tests/pw/helpers/fixtures';
import { answerQuestion, fillInputModal } from '@tests/pw/helpers/utils';

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
const TIME_MATCHER = /^(now|(\d{1,2}|one) minutes? ago)$/;
const SIZE_MATCHER = /^[0-9.]+ (K|M|G)?B$/;

msTest('Documents page default state', async ({ documents }) => {
  const actionBar = documents.locator('#folders-ms-action-bar');
  await expect(actionBar.locator('.ms-action-bar-button:visible')).toHaveText(['New folder', 'Import']);
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

msTest('Documents page default state in a read only workspace', async ({ documentsReadOnly }) => {
  const actionBar = documentsReadOnly.locator('#folders-ms-action-bar');
  await expect(actionBar.locator('.ms-action-bar-button:visible')).toHaveCount(0);
  await expect(actionBar.locator('.counter')).toHaveText('4 items', { useInnerText: true });
  await expect(actionBar.locator('#select-popover-button')).toHaveText('Name');
  await expect(actionBar.locator('#grid-view')).not.toHaveDisabledAttribute();
  await expect(actionBar.locator('#list-view')).toHaveDisabledAttribute();
  const entries = documentsReadOnly.locator('.folder-container').locator('.file-list-item');
  await expect(entries).toHaveCount(4);
  await expect(entries.locator('.file-name').locator('.file-name__label')).toHaveText([
    DIR_MATCHER,
    DIR_MATCHER,
    FILE_MATCHER,
    FILE_MATCHER,
  ]);
  await expect(entries.locator('.file-lastUpdate')).toHaveText(new Array(4).fill(TIME_MATCHER));
  await expect(entries.locator('.file-size')).toHaveText(['', '', SIZE_MATCHER, SIZE_MATCHER]);
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
  await expect(actionBar.locator('.ms-action-bar-button:visible')).toHaveCount(1);
  await expect(actionBar.locator('.ms-action-bar-button:visible')).toHaveText('Delete');
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
  await actionBar.locator('.ms-action-bar-button:visible').click();
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
  await expect(actionBar.locator('.counter')).toHaveText('3 selected items', { useInnerText: true });
  await entries.nth(3).locator('ion-checkbox').click();
  await expect(actionBar.locator('.counter')).toHaveText('2 selected items', { useInnerText: true });

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
    const modal = documents.locator('.information-modal');
    await expect(modal).toBeVisible();
    await expect(modal.locator('.container-textinfo')).toHaveText('This feature is not yet available in web mode.');
    await modal.locator('#next-button').click();
    await expect(modal).toBeHidden();
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
