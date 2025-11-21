// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { Page } from '@playwright/test';
import { answerQuestion, createFolder, DisplaySize, expect, fillInputModal, msTest, resizePage } from '@tests/e2e/helpers';

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

const FILE_MATCHER = /^[A-Za-z0-9_.]+$/;
const DIR_MATCHER = /^Dir_[A-Za-z_]+$/;
const TIME_MATCHER = /^(now|< 1 minute|(\d{1,2}|one) minutes? ago)$/;
const SIZE_MATCHER = /^[0-9.]+ (K|M|G)?B$/;

const NAME_MATCHER_ARRAY = new Array(1).fill(DIR_MATCHER).concat(new Array(8).fill(FILE_MATCHER));
const TIME_MATCHER_ARRAY = new Array(9).fill(TIME_MATCHER);
const SIZE_MATCHER_ARRAY = new Array(1).fill('').concat(new Array(8).fill(SIZE_MATCHER));

for (const displaySize of ['small', 'large']) {
  msTest(`Documents page default state on ${displaySize} display`, { tag: '@important' }, async ({ documents }) => {
    const entries = documents.locator('.folder-container').locator('.file-list-item');
    if (displaySize === DisplaySize.Small) {
      await documents.setDisplaySize(DisplaySize.Small);
      await expect(entries.locator('.file-name').locator('.label-name')).toHaveText(NAME_MATCHER_ARRAY);
      await expect(entries.locator('.data-date')).toHaveText(TIME_MATCHER_ARRAY);
      await expect(entries.locator('.data-size')).toHaveText(SIZE_MATCHER_ARRAY.slice(1));
      for (let i = 0; i < (await entries.count()); i++) {
        const entry = entries.nth(i);
        await expect(entry.locator('.options-button')).toBeVisible();
      }
    } else {
      const actionBar = documents.locator('#folders-ms-action-bar');
      await expect(actionBar.locator('.ms-action-bar-button:visible')).toHaveText(['New folder', 'Import']);
      await expect(actionBar.locator('.counter')).toHaveText('9 items', { useInnerText: true });
      await expect(actionBar.locator('#select-popover-button')).toHaveText('Name');
      await expect(actionBar.locator('#grid-view')).toNotHaveDisabledAttribute();
      await expect(actionBar.locator('#list-view')).toHaveDisabledAttribute();
      await expect(entries).toHaveCount(9);
      await expect(entries.locator('.file-name').locator('.label-name')).toHaveText(NAME_MATCHER_ARRAY);
      await expect(entries.locator('.file-last-update')).toHaveText(TIME_MATCHER_ARRAY);
      await expect(entries.locator('.file-size')).toHaveText(SIZE_MATCHER_ARRAY);
    }
  });
}

msTest('Check documents in grid mode', async ({ documents }) => {
  await toggleViewMode(documents);
  const entries = documents.locator('.folder-container').locator('.file-card-item');
  await expect(entries).toHaveCount(9);
  await expect(entries.locator('.file-card__title')).toHaveText(NAME_MATCHER_ARRAY);
  await expect(entries.locator('.file-card-last-update')).toHaveText(TIME_MATCHER_ARRAY);
});

msTest('Documents page default state in a read only workspace', async ({ documentsReadOnly }) => {
  const actionBar = documentsReadOnly.locator('#folders-ms-action-bar');
  await expect(actionBar.locator('.ms-action-bar-button:visible')).toHaveCount(0);
  await expect(actionBar.locator('.right-side').locator('.label-role')).toHaveText('Reader');
  await expect(actionBar.locator('.counter')).toHaveText('9 items', { useInnerText: true });
  await expect(actionBar.locator('#select-popover-button')).toHaveText('Name');
  await expect(actionBar.locator('#grid-view')).toNotHaveDisabledAttribute();
  await expect(actionBar.locator('#list-view')).toHaveDisabledAttribute();
  const entries = documentsReadOnly.locator('.folder-container').locator('.file-list-item');
  await expect(entries).toHaveCount(9);
  await expect(entries.locator('.file-name').locator('.label-name')).toHaveText(NAME_MATCHER_ARRAY);
  await expect(entries.locator('.file-last-update')).toHaveText(TIME_MATCHER_ARRAY);
  await expect(entries.locator('.file-creation-date')).toHaveText(TIME_MATCHER_ARRAY);
  await expect(entries.locator('.file-size')).toHaveText(SIZE_MATCHER_ARRAY);
  // Useless click just to move the mouse
  await documentsReadOnly.locator('.folder-list-header__label').nth(1).click();
  for (const checkbox of await entries.locator('ion-checkbox').all()) {
    await expect(checkbox).toBeHidden();
  }
});

msTest('Sort document by creation date', async ({ documents }) => {
  const actionBar = documents.locator('#folders-ms-action-bar');
  await expect(actionBar.locator('.ms-action-bar-button:visible')).toHaveText(['New folder', 'Import']);
  await expect(actionBar.locator('.counter')).toHaveText('9 items', { useInnerText: true });
  const entries = documents.locator('.folder-container').locator('.file-list-item');

  const sorterPopoverButton = actionBar.locator('#select-popover-button');
  await expect(sorterPopoverButton).toHaveText('Name');
  await sorterPopoverButton.click();
  const sorterPopover = documents.locator('.sorter-popover');
  await expect(sorterPopover).toBeVisible();
  await expect(sorterPopover.getByRole('listitem').nth(3)).toHaveText('Creation date');
  await sorterPopover.getByRole('listitem').nth(3).click();
  await expect(sorterPopover).toBeHidden();

  const firstEntryBefore = await entries.nth(1).locator('.label-name').textContent();
  const lastEntryBefore = await entries.nth(8).locator('.label-name').textContent();

  await sorterPopoverButton.click();
  await expect(sorterPopover).toBeVisible();
  await sorterPopover.locator('.order-button').click();
  await expect(sorterPopover).toBeHidden();

  const firstEntryAfter = await entries.nth(1).locator('.label-name').textContent();
  const lastEntryAfter = await entries.nth(8).locator('.label-name').textContent();

  expect(firstEntryAfter).toBe(lastEntryBefore);
  expect(lastEntryAfter).toBe(firstEntryBefore);
});

msTest('Sort document with header title', async ({ documents }) => {
  const actionBar = documents.locator('#folders-ms-action-bar');
  const entries = documents.locator('.folder-container').locator('.file-list-item');
  const sorterPopoverButton = actionBar.locator('#select-popover-button');
  const HEADERS = new Map<number, string>([
    [1, 'Name'],
    [5, 'Size'],
  ]);

  await expect(entries).toHaveCount(9);

  let sortOrder = false;

  for (const [headerIndex, headerName] of HEADERS.entries()) {
    const headerLabel = documents.locator('.folder-list-header__label').nth(headerIndex);
    await expect(headerLabel).toBeVisible();
    await expect(headerLabel).toHaveText(headerName);
    await headerLabel.click();
    await expect(sorterPopoverButton).toHaveText(headerName);
    await expect(sorterPopoverButton).toHaveAttribute('sort-ascending', ['false', 'true'][Number(sortOrder)]);

    // Folder should stay at the top
    await expect(entries.nth(0).locator('.label-name')).toHaveText('Dir_Folder');

    const firstEntryName = await entries.nth(1).locator('.label-name').textContent();
    const lastEntryName = await entries.nth(8).locator('.label-name').textContent();

    expect(firstEntryName).not.toBeNull();
    expect(lastEntryName).not.toBeNull();

    // Revert the order
    await headerLabel.click();
    sortOrder = !sortOrder;
    await expect(sorterPopoverButton).toHaveAttribute('sort-ascending', ['false', 'true'][Number(sortOrder)]);
    // Folder should stay at the top
    await expect(entries.nth(0).locator('.label-name')).toHaveText('Dir_Folder');
    await expect(entries.nth(1).locator('.label-name')).toHaveText(lastEntryName as string);
    await expect(entries.nth(8).locator('.label-name')).toHaveText(firstEntryName as string);
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
  await expect(actionBar.locator('.ms-action-bar-button:visible')).toHaveCount(4);
  await expect(actionBar.locator('.ms-action-bar-button:visible')).toHaveText(['Move to', 'Make a copy', 'Delete', 'Download']);
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
  await expect(globalCheckbox).toHaveState('checked');

  const actionBar = documents.locator('#folders-ms-action-bar');
  await expect(actionBar.locator('.counter')).toHaveText('9 selected items');
  const deleteButton = actionBar.locator('.ms-action-bar-button:visible').nth(2);
  await expect(deleteButton).toHaveText('Delete');
  await deleteButton.click();
  await answerQuestion(documents, true, {
    expectedTitleText: 'Delete multiple items',
    expectedQuestionText: /Are you sure you want to delete these \d+ items\?/,
    expectedPositiveText: /Delete \d+ items/,
    expectedNegativeText: 'Keep items',
  });
  const entries = documents.locator('.folder-container').locator('.file-list-item');
  await expect(entries).toHaveCount(0);
});

for (const displaySize of [DisplaySize.Small, DisplaySize.Large]) {
  msTest(`Create a folder ${displaySize} display`, async ({ documents }) => {
    if (displaySize === DisplaySize.Small) {
      await documents.setDisplaySize(DisplaySize.Small);
    }

    const entries = documents.locator('.folder-container').locator('.file-list-item');
    await expect(entries).toHaveCount(9);
    await expect(entries.locator('.file-name').locator('.label-name')).toHaveText(NAME_MATCHER_ARRAY);

    if (displaySize === DisplaySize.Small) {
      const addButton = documents.locator('.tab-bar-menu').locator('#add-menu-fab-button');
      await expect(addButton).toBeVisible();
      await addButton.click();
      const modal = documents.locator('.tab-menu-modal');
      await expect(modal).toBeVisible();
      await modal.locator('.list-group-item').filter({ hasText: 'New folder' }).click();
    } else {
      const actionBar = documents.locator('#folders-ms-action-bar');
      await actionBar.getByText('New folder').click();
    }

    await fillInputModal(documents, 'My folder');

    await expect(entries).toHaveCount(10);
    // Don't ask.
    await expect(entries.locator('.file-name').locator('.label-name')).toHaveText([
      'My folder',
      ...NAME_MATCHER_ARRAY.slice(0, 1),
      ...NAME_MATCHER_ARRAY.slice(1),
    ]);
  });

  msTest(`Header breadcrumbs ${displaySize} display`, async ({ documents }) => {
    async function navigateDown(): Promise<void> {
      await documents.locator('.folder-container').getByRole('listitem').nth(0).dblclick();
    }

    async function clickOnBreadcrumb(i: number): Promise<void> {
      await documents.locator('#connected-header').locator('.topbar-left__breadcrumb').locator('ion-breadcrumb').nth(i).click();
    }

    if (displaySize === DisplaySize.Small) {
      await documents.setDisplaySize(DisplaySize.Small);
      const smallBreadcrumbs = documents.locator('#connected-header').locator('.topbar-left').locator('.breadcrumb-small-container');

      await expect(documents.locator('#connected-header').locator('.topbar-left').locator('ion-breadcrumbs')).not.toBeVisible();
      await expect(smallBreadcrumbs).not.toBeVisible();
      await navigateDown();
      await expect(smallBreadcrumbs).toBeVisible();
      await expect(smallBreadcrumbs.locator('ion-text')).toHaveCount(2);
      await expect(smallBreadcrumbs.locator('ion-text').nth(0)).toHaveText('wksp1');
      await expect(smallBreadcrumbs.locator('ion-text').nth(1)).toHaveText('/');
      await expect(smallBreadcrumbs.locator('.breadcrumb-popover-button')).toBeVisible();
      await createFolder(documents, 'Subdir');
      await navigateDown();
      await expect(smallBreadcrumbs.locator('ion-text')).toHaveCount(4);
      await expect(smallBreadcrumbs.locator('ion-text').nth(0)).toHaveText('...');
      await expect(smallBreadcrumbs.locator('ion-text').nth(1)).toHaveText('/');
      await expect(smallBreadcrumbs.locator('ion-text').nth(2)).toHaveText('Dir_Folder');
      await expect(smallBreadcrumbs.locator('ion-text').nth(3)).toHaveText('/');
      await smallBreadcrumbs.locator('.breadcrumb-popover-button').click();

      const popoverItems = documents.locator('ion-popover').locator('.popover-item');
      await expect(popoverItems).toHaveCount(2);
      await expect(popoverItems.nth(0)).toHaveText('wksp1');
      await expect(popoverItems.nth(1)).toHaveText('Dir_Folder');
      await popoverItems.nth(1).click();
      await expect(smallBreadcrumbs.locator('ion-text')).toHaveCount(2);
      await expect(smallBreadcrumbs.locator('ion-text').nth(0)).toHaveText('wksp1');
      await expect(smallBreadcrumbs.locator('ion-text').nth(1)).toHaveText('/');
      await smallBreadcrumbs.locator('.breadcrumb-popover-button').click();
      await expect(popoverItems).toHaveCount(1);
      await expect(popoverItems).toHaveText('wksp1');
      await popoverItems.click();
      await expect(smallBreadcrumbs).not.toBeVisible();
    } else {
      await expect(documents).toHaveHeader(['wksp1'], true, true);
      await navigateDown();
      await expect(documents).toHaveHeader(['wksp1', 'Dir_Folder'], true, true);
      await createFolder(documents, 'Subdir');
      await navigateDown();
      await expect(documents).toHaveHeader(['wksp1', '', 'Subdir'], true, true);
      await createFolder(documents, 'Subdir 2');
      await navigateDown();
      await expect(documents).toHaveHeader(['wksp1', '', '', 'Subdir 2'], true, true);
      await createFolder(documents, 'Subdir 3');
      await navigateDown();
      await expect(documents).toHaveHeader(['wksp1', '', '', '', 'Subdir 3'], true, true);
      await clickOnBreadcrumb(2);
      const popoverItems = documents.locator('ion-popover').locator('.popover-item');
      await expect(popoverItems).toHaveCount(3);
      await popoverItems.nth(2).click();
      await expect(documents).toHaveHeader(['wksp1', '', '', 'Subdir 2'], true, true);
      await clickOnBreadcrumb(2);
      await expect(popoverItems).toHaveCount(2);
      await popoverItems.nth(1).click();
      await expect(documents).toHaveHeader(['wksp1', '', 'Subdir'], true, true);
      await clickOnBreadcrumb(1);
      await expect(documents).toHaveHeader(['wksp1'], true, true);
      await clickOnBreadcrumb(0);
      await expect(documents).toBeWorkspacePage();
    }
  });
}

msTest('Import context menu', async ({ documents }) => {
  await expect(documents.locator('.import-popover')).toBeHidden();
  const actionBar = documents.locator('#folders-ms-action-bar');
  await actionBar.locator('.ms-action-bar-button:visible').nth(1).click();

  const popover = documents.locator('.import-popover');
  await expect(popover).toBeVisible();

  await expect(popover.getByRole('listitem')).toHaveText(['Import files', 'Import a folder']);
});

msTest('Selection in grid mode by by clicking on the checkbox', async ({ documents }) => {
  await documents.locator('.folder-container').locator('.folder-list-header').locator('ion-checkbox').click();
  await toggleViewMode(documents);
  const entries = documents.locator('.folder-container').locator('.file-card-item');

  for (const entry of await entries.all()) {
    await expect(entry.locator('ion-checkbox')).toHaveState('checked');
  }
  await entries.nth(1).locator('ion-checkbox').click();
  const actionBar = documents.locator('#folders-ms-action-bar');
  await expect(actionBar.locator('.counter')).toHaveText('8 selected items', { useInnerText: true });
  await entries.nth(3).locator('ion-checkbox').click();
  await expect(actionBar.locator('.counter')).toHaveText('7 selected items', { useInnerText: true });

  await expect(entries.nth(0).locator('ion-checkbox')).toHaveState('checked');
  await expect(entries.nth(1).locator('ion-checkbox')).toHaveState('unchecked');
  await expect(entries.nth(2).locator('ion-checkbox')).toHaveState('checked');
  await expect(entries.nth(3).locator('ion-checkbox')).toHaveState('unchecked');
});

msTest('Check in FoldersPage if action bar updates after resized', async ({ documents }) => {
  const actionBar = documents.locator('#folders-ms-action-bar');
  const actionsBarButtons = actionBar.locator('.ms-action-bar-button--visible');
  const actionBarMoreButton = actionBar.locator('#action-bar-more-button');
  const popover = documents.locator('.action-bar-more-popover');

  await resizePage(documents, 1600);
  await expect(actionBar).toBeVisible();
  await expect(actionsBarButtons).toHaveCount(2);
  await expect(actionsBarButtons).toHaveText(['New folder', 'Import']);
  await expect(actionBarMoreButton).toBeHidden();

  await resizePage(documents, 850);
  await expect(actionsBarButtons).toHaveText(['New folder']);
  await expect(actionBarMoreButton).toBeVisible();
  await actionBarMoreButton.click();
  await expect(documents.locator('.popover-viewport').getByRole('listitem').nth(0)).toHaveText('Import');
  await documents.keyboard.press('Escape');
  await expect(documents.locator('.popover-viewport')).toBeHidden();

  await resizePage(documents, 2000);
  const firstFileEntry = documents.locator('.folder-container').locator('.file-list-item').nth(1);
  await firstFileEntry.hover();
  await firstFileEntry.locator('ion-checkbox').click();
  // All buttons should be visible
  await expect(actionsBarButtons).toHaveText(['Preview', 'Rename', 'Move to', 'Make a copy', 'Delete', 'Download', 'Details', 'Copy link']);
  await expect(actionBarMoreButton).toBeHidden();

  // Resize to 1500px
  await resizePage(documents, 1500);
  await expect(actionsBarButtons).toHaveText(['Preview', 'Rename', 'Move to', 'Make a copy', 'Delete', 'Download']);
  await expect(actionBarMoreButton).toBeVisible();
  await actionBarMoreButton.click();
  await expect(documents.locator('.popover-viewport').getByRole('listitem')).toHaveText(['Details', 'Copy link']);
  await documents.keyboard.press('Escape');

  // Resize to 1200px (to be sure the "more" button works)
  await resizePage(documents, 1200);
  await expect(actionsBarButtons).toHaveText(['Preview', 'Rename', 'Move to']);
  await expect(actionBarMoreButton).toBeVisible();
  await actionBarMoreButton.click();
  await expect(popover.getByRole('listitem')).toHaveText(['Make a copy', 'Delete', 'Download', 'Details', 'Copy link']);
  await documents.keyboard.press('Escape');

  // Finally, check if action bar is checkable after toggling the sidebar menu
  const topbarButton = documents.locator('#connected-header').locator('#trigger-toggle-menu-button');
  await expect(topbarButton).toBeVisible();
  await topbarButton.click();
  await expect(actionsBarButtons).toHaveText(['Preview', 'Rename', 'Move to', 'Make a copy', 'Delete', 'Download']);
  await expect(actionBarMoreButton).toBeVisible();
  await actionBarMoreButton.click();
  await expect(popover.getByRole('listitem')).toHaveText(['Details', 'Copy link']);
});

for (const gridMode of [false, true]) {
  msTest(`Selection in ${gridMode ? 'grid' : 'list'} mode by clicking on the item`, async ({ documents }) => {
    if (gridMode) {
      await toggleViewMode(documents);
    }

    const entries = documents.locator('.folder-container').locator(gridMode ? '.file-card-item' : '.file-list-item');
    const actionBar = documents.locator('#folders-ms-action-bar');

    for (const entry of await entries.all()) {
      await entry.click();
      await expect(entry.locator('ion-checkbox')).toHaveState('checked');
      await expect(actionBar.locator('.counter')).toHaveText(/^\d+ selected items?$/, { useInnerText: true });
    }
    await expect(actionBar.locator('.counter')).toHaveText(`${await entries.count()} selected items`, { useInnerText: true });
  });
}

for (const gridMode of [false, true]) {
  msTest(`Open file in ${gridMode ? 'grid' : 'list'} mode`, async ({ documents }) => {
    await expect(documents.locator('.information-modal')).toBeHidden();
    await expect(documents).toHaveHeader(['wksp1'], true, true);
    if (gridMode) {
      await toggleViewMode(documents);
      await documents.locator('.folder-container').locator('.file-card-item').nth(2).dblclick();
    } else {
      await documents.locator('.folder-container').getByRole('listitem').nth(2).dblclick();
    }

    await expect(documents).toBeViewerPage();
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

    await expect(documents).toHaveHeader(['wksp1'], true, true);
    await navigateDown();
    await expect(documents).toHaveHeader(['wksp1', 'Dir_Folder'], true, true);
    await createFolder(documents, 'Subdir');
    await navigateDown();
    await expect(documents).toHaveHeader(['wksp1', '', 'Subdir'], true, true);
    await navigateUp();
    await expect(documents).toHaveHeader(['wksp1', 'Dir_Folder'], true, true);
    await navigateUp();
    await expect(documents).toHaveHeader(['wksp1'], true, true);
    await navigateUp();
    await expect(documents).toBeWorkspacePage();
  });
}
