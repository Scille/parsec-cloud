// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { Page, TestInfo } from '@playwright/test';
import {
  answerQuestion,
  clearLibParsecMocks,
  createFolder,
  DisplaySize,
  expect,
  fillInputModal,
  fillIonInput,
  importDefaultFiles,
  ImportDocuments,
  mockLibParsec,
  msTest,
  resizePage,
} from '@tests/e2e/helpers';

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

msTest.describe(() => {
  msTest.use({
    documentsOptions: {
      empty: true,
    },
  });

  for (const displaySize of ['small', 'large']) {
    msTest(`Documents page default state on ${displaySize} display`, { tag: '@important' }, async ({ documents }, testInfo: TestInfo) => {
      await importDefaultFiles(documents, testInfo, ImportDocuments.Mp3 | ImportDocuments.Pdf, true);

      const entries = documents.locator('.folder-container').locator('.file-list-item');
      if (displaySize === DisplaySize.Small) {
        await documents.setDisplaySize(DisplaySize.Small);
        await expect(entries.locator('.file-name').locator('.label-name')).toHaveText(['Dir_Folder', 'audio.mp3', 'pdfDocument.pdf']);
        await expect(entries.locator('.data-date')).toHaveText(['now', 'now', 'now']);
        await expect(entries.locator('.data-size')).toHaveText(['40.9 KB', '76.9 KB']);
        await expect(entries.nth(0).locator('.options-button')).toBeVisible();
        await expect(entries.nth(1).locator('.options-button')).toBeVisible();
        await expect(entries.nth(2).locator('.options-button')).toBeVisible();
      } else {
        const actionBar = documents.locator('#folders-ms-action-bar');
        await expect(actionBar.locator('.ms-action-bar-button:visible')).toHaveText(['New folder', 'Import']);
        await expect(actionBar.locator('.counter')).toHaveText('3 items', { useInnerText: true });
        await expect(actionBar.locator('#select-popover-button')).toHaveText('Name');
        await expect(actionBar.locator('#grid-view')).toNotHaveDisabledAttribute();
        await expect(actionBar.locator('#list-view')).toHaveDisabledAttribute();
        await expect(entries).toHaveCount(3);
        await expect(entries.locator('.file-name').locator('.label-name')).toHaveText(['Dir_Folder', 'audio.mp3', 'pdfDocument.pdf']);
        await expect(entries.locator('.file-last-update')).toHaveText(['now', 'now', 'now']);
        await expect(entries.locator('.file-creation-date')).toHaveText(['now', 'now', 'now']);
        await expect(entries.locator('.file-size')).toHaveText(['', '40.9 KB', '76.9 KB']);
      }
    });
  }

  msTest('Check documents in grid mode', async ({ documents }, testInfo: TestInfo) => {
    await importDefaultFiles(documents, testInfo, ImportDocuments.Mp3 | ImportDocuments.Pdf, true);
    await toggleViewMode(documents);
    const entries = documents.locator('.folder-container').locator('.file-card-item');
    await expect(entries).toHaveCount(3);
    await expect(entries.locator('.file-card__title')).toHaveText(['Dir_Folder', 'audio.mp3', 'pdfDocument.pdf']);
    await expect(entries.locator('.file-card-last-update')).toHaveText(['now', 'now', 'now']);
  });

  msTest('Sort document by creation date', async ({ documents }, testInfo: TestInfo) => {
    await importDefaultFiles(documents, testInfo, ImportDocuments.Mp3 | ImportDocuments.Pdf | ImportDocuments.Png, true);
    const actionBar = documents.locator('#folders-ms-action-bar');
    await expect(actionBar.locator('.counter')).toHaveText('4 items', { useInnerText: true });
    const entries = documents.locator('.folder-container').locator('.file-list-item');

    await expect(entries.locator('.file-name').locator('.label-name')).toHaveText([
      'Dir_Folder',
      'audio.mp3',
      'image.png',
      'pdfDocument.pdf',
    ]);

    const sorterPopoverButton = actionBar.locator('#select-popover-button');
    await expect(sorterPopoverButton).toHaveText('Name');
    await sorterPopoverButton.click();
    const sorterPopover = documents.locator('.sorter-popover');
    await expect(sorterPopover).toBeVisible();
    await expect(sorterPopover.getByRole('listitem').nth(3)).toHaveText('Creation date');
    await sorterPopover.getByRole('listitem').nth(3).click();
    await expect(sorterPopover).toBeHidden();

    await expect(entries.locator('.file-name').locator('.label-name')).toHaveText([
      'Dir_Folder',
      'audio.mp3',
      'pdfDocument.pdf',
      'image.png',
    ]);

    await sorterPopoverButton.click();
    await expect(sorterPopover).toBeVisible();
    await sorterPopover.locator('.order-button').click();
    await expect(sorterPopover).toBeHidden();

    await expect(entries.locator('.file-name').locator('.label-name')).toHaveText([
      'Dir_Folder',
      'image.png',
      'pdfDocument.pdf',
      'audio.mp3',
    ]);
  });

  msTest('Sort document with header title', async ({ documents }, testInfo: TestInfo) => {
    await importDefaultFiles(documents, testInfo, ImportDocuments.Mp3 | ImportDocuments.Pdf | ImportDocuments.Png, true);
    const actionBar = documents.locator('#folders-ms-action-bar');
    const entries = documents.locator('.folder-container').locator('.file-list-item');
    const sorterPopoverButton = actionBar.locator('#select-popover-button');
    await expect(entries).toHaveCount(4);

    let sortOrder = false;

    const headerNameLabel = documents.locator('.folder-list-header__label').nth(1);
    await expect(headerNameLabel).toBeVisible();
    await expect(headerNameLabel).toHaveText('Name');
    await expect(sorterPopoverButton).toHaveText('Name');
    await expect(sorterPopoverButton).toHaveAttribute('sort-ascending', ['false', 'true'][Number(sortOrder)]);

    // Folder should stay at the top
    await expect(entries.locator('.label-name')).toHaveText(['Dir_Folder', 'audio.mp3', 'image.png', 'pdfDocument.pdf']);

    // Revert the order
    await headerNameLabel.click();
    sortOrder = !sortOrder;
    await expect(sorterPopoverButton).toHaveAttribute('sort-ascending', ['false', 'true'][Number(sortOrder)]);
    // Folder should stay at the top
    await expect(entries.locator('.label-name')).toHaveText(['Dir_Folder', 'pdfDocument.pdf', 'image.png', 'audio.mp3']);
    await expect(entries.locator('.file-size')).toHaveText(['', '76.9 KB', '6.18 KB', '40.9 KB']);

    const headerSizeLabel = documents.locator('.folder-list-header__label').nth(5);
    await expect(headerSizeLabel).toBeVisible();
    await expect(headerSizeLabel).toHaveText('Size');
    await headerSizeLabel.click();
    await expect(sorterPopoverButton).toHaveText('Size');
    await expect(sorterPopoverButton).toHaveAttribute('sort-ascending', ['false', 'true'][Number(sortOrder)]);

    await expect(entries.locator('.label-name')).toHaveText(['Dir_Folder', 'image.png', 'audio.mp3', 'pdfDocument.pdf']);
    await expect(entries.locator('.file-size')).toHaveText(['', '6.18 KB', '40.9 KB', '76.9 KB']);

    // Revert the order
    await headerSizeLabel.click();
    sortOrder = !sortOrder;
    await expect(sorterPopoverButton).toHaveAttribute('sort-ascending', ['false', 'true'][Number(sortOrder)]);
    // Folder should stay at the top
    await expect(entries.locator('.label-name')).toHaveText(['Dir_Folder', 'pdfDocument.pdf', 'audio.mp3', 'image.png']);
    await expect(entries.locator('.file-size')).toHaveText(['', '76.9 KB', '40.9 KB', '6.18 KB']);
  });

  msTest.skip('Select all documents', async ({ documents }, testInfo: TestInfo) => {
    await importDefaultFiles(documents, testInfo, ImportDocuments.Mp3 | ImportDocuments.Pdf | ImportDocuments.Png, true);
    const globalCheckbox = documents.locator('.folder-container').locator('.folder-list-header').locator('.ms-checkbox');
    await expect(globalCheckbox).not.toBeChecked();
    await globalCheckbox.check();
    await expect(globalCheckbox).toBeChecked();
    const actionBar = documents.locator('#folders-ms-action-bar');
    await expect(actionBar.locator('.counter')).toHaveText('4 selected items');

    const entries = documents.locator('.folder-container').locator('.file-list-item');
    for (let i = 0; i < (await entries.all()).length; i++) {
      const checkbox = entries.nth(i).locator('.ms-checkbox');
      await expect(checkbox).toBeVisible();
      await expect(checkbox).toBeChecked();
    }

    await expect(actionBar.locator('.ms-action-bar-button:visible')).toHaveCount(4);
    await expect(actionBar.locator('.ms-action-bar-button:visible')).toHaveText(['Move to', 'Make a copy', 'Delete', 'Download']);
    await expect(actionBar.locator('.counter')).toHaveText('4 selected items');

    await entries.nth(1).locator('.ms-checkbox').click();
    await expect(entries.nth(1).locator('.ms-checkbox')).not.toBeChecked();

    await expect(globalCheckbox).toHaveAttribute('ms-indeterminate', 'true');
    await expect(actionBar.locator('.counter')).toHaveText('3 selected items');

    await globalCheckbox.check();
    await expect(globalCheckbox).toBeChecked();
    await expect(entries.nth(1).locator('.ms-checkbox')).toBeChecked();

    await globalCheckbox.uncheck();
    await expect(globalCheckbox).not.toBeChecked();
    for (const entry of await entries.all()) {
      const checkbox = entry.locator('.ms-checkbox');
      await expect(checkbox).toBeHidden();
      await expect(checkbox).not.toBeChecked();
    }
    await expect(actionBar.locator('.counter')).toHaveText('4 items');
  });

  msTest('Delete all documents', async ({ documents }, testInfo: TestInfo) => {
    await importDefaultFiles(documents, testInfo, ImportDocuments.Mp3 | ImportDocuments.Pdf | ImportDocuments.Png, true);
    const globalCheckbox = documents.locator('.folder-container').locator('.folder-list-header').locator('.ms-checkbox');
    await expect(globalCheckbox).not.toBeChecked();
    await globalCheckbox.check();
    await expect(globalCheckbox).toBeChecked();

    const entries = documents.locator('.folder-container').locator('.file-list-item');
    for (const entry of await entries.all()) {
      const checkbox = entry.locator('.ms-checkbox');
      await expect(checkbox).toBeChecked();
    }

    const actionBar = documents.locator('#folders-ms-action-bar');
    await expect(actionBar.locator('.counter')).toHaveText('4 selected items');

    const deleteButton = actionBar.locator('.ms-action-bar-button:visible').nth(2);
    await expect(deleteButton).toHaveText('Delete');
    await deleteButton.click();
    await answerQuestion(documents, true, {
      expectedTitleText: 'Delete multiple items',
      expectedQuestionText: /Are you sure you want to delete these \d+ items\?/,
      expectedPositiveText: /Delete \d+ items/,
      expectedNegativeText: 'Keep items',
    });
    await expect(entries).toHaveCount(0);
    await expect(documents.locator('.folder-container').locator('.no-files')).toBeVisible();
    await expect(actionBar.locator('.counter')).toHaveText('No items');
  });

  for (const displaySize of [DisplaySize.Small, DisplaySize.Large]) {
    msTest(`Create a folder ${displaySize} display`, async ({ documents }, testInfo: TestInfo) => {
      await importDefaultFiles(documents, testInfo, ImportDocuments.Png, false);

      const entries = documents.locator('.folder-container').locator('.file-list-item');

      if (displaySize === DisplaySize.Small) {
        await documents.setDisplaySize(DisplaySize.Small);
      } else {
        const actionBar = documents.locator('#folders-ms-action-bar');
        await expect(entries).toHaveCount(1);
        await expect(actionBar.locator('.counter')).toHaveText('1 item');
      }

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

      await expect(entries).toHaveCount(2);
      await expect(entries.locator('.file-name').locator('.label-name')).toHaveText(['My folder', 'image.png']);
    });

    msTest(`Header breadcrumbs ${displaySize} display`, async ({ documents }, testInfo: TestInfo) => {
      async function navigateDown(): Promise<void> {
        await documents.locator('.folder-container').getByRole('listitem').nth(0).dblclick();
      }

      async function clickOnBreadcrumb(i: number): Promise<void> {
        await documents.locator('#connected-header').locator('.topbar-left__breadcrumb').locator('ion-breadcrumb').nth(i).click();
      }

      await importDefaultFiles(documents, testInfo, ImportDocuments.Png, true);

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

  msTest('Create a folder with a name too long', async ({ documents }) => {
    const entries = documents.locator('.folder-container').locator('.file-list-item');
    await expect(entries).toHaveCount(0);

    const actionBar = documents.locator('#folders-ms-action-bar');
    await actionBar.getByText('New folder').click();

    const modal = documents.locator('.text-input-modal');
    await expect(modal).toBeVisible();
    const okButton = modal.locator('.ms-modal-footer-buttons').locator('#next-button');
    await fillIonInput(modal.locator('ion-input'), 'A'.repeat(132));
    await expect(modal.locator('.form-error')).toBeVisible();
    await expect(modal.locator('.form-error')).toHaveText('File name is too long, limit is 128 characters.');
    await fillIonInput(modal.locator('ion-input'), 'A'.repeat(64));
    await expect(modal.locator('.form-error')).toBeHidden();
    await expect(okButton).toBeTrulyEnabled();
    await okButton.click();

    await expect(entries).toHaveCount(1);
    await expect(entries.locator('.file-name').locator('.label-name').nth(0)).toHaveText('A'.repeat(64));
  });

  msTest('Import context menu', async ({ documents }) => {
    await expect(documents.locator('.import-popover')).toBeHidden();
    const actionBar = documents.locator('#folders-ms-action-bar');
    await actionBar.locator('.ms-action-bar-button:visible').nth(1).click();

    const popover = documents.locator('.import-popover');
    await expect(popover).toBeVisible();

    await expect(popover.getByRole('listitem')).toHaveText(['Import files', 'Import a folder']);
  });

  msTest.skip('Selection in grid mode by clicking on the checkbox', async ({ documents }, testInfo: TestInfo) => {
    await importDefaultFiles(documents, testInfo, ImportDocuments.Png | ImportDocuments.Pdf, true);

    const actionBar = documents.locator('#folders-ms-action-bar');

    await expect(actionBar.locator('.counter')).toHaveText('3 items');
    const globalCheckbox = documents.locator('.folder-container').locator('.folder-list-header').locator('.ms-checkbox');
    await expect(globalCheckbox).not.toBeChecked();
    await globalCheckbox.check();
    await expect(globalCheckbox).toBeChecked();
    await expect(actionBar.locator('.counter')).toHaveText('3 selected items');

    const listEntries = documents.locator('.folder-container').locator('.file-list-item');

    await expect(listEntries.nth(0).locator('.ms-checkbox')).toBeChecked();
    await expect(listEntries.nth(1).locator('.ms-checkbox')).toBeChecked();
    await expect(listEntries.nth(2).locator('.ms-checkbox')).toBeChecked();

    await expect(actionBar.locator('.counter')).toHaveText('3 selected items');

    await toggleViewMode(documents);
    await expect(actionBar.locator('.counter')).toHaveText('3 selected items');
    const entries = documents.locator('.folder-container').locator('.file-card-item');

    await expect(entries.nth(0).locator('.ms-checkbox')).toBeChecked();
    await expect(entries.nth(1).locator('.ms-checkbox')).toBeChecked();
    await expect(entries.nth(2).locator('.ms-checkbox')).toBeChecked();

    await entries.nth(1).locator('.ms-checkbox').click();
    await expect(actionBar.locator('.counter')).toHaveText('2 selected items');
    await entries.nth(2).locator('.ms-checkbox').click();
    await expect(actionBar.locator('.counter')).toHaveText('1 selected item');

    await expect(entries.nth(0).locator('.ms-checkbox')).toBeChecked();
    await expect(entries.nth(1).locator('.ms-checkbox')).not.toBeChecked();
    await expect(entries.nth(2).locator('.ms-checkbox')).not.toBeChecked();
  });

  msTest('Check in FoldersPage if action bar updates after resized', async ({ documents }, testInfo: TestInfo) => {
    await importDefaultFiles(documents, testInfo, ImportDocuments.Png, true);

    const actionBar = documents.locator('#folders-ms-action-bar');
    const actionsBarButtons = actionBar.locator('.ms-action-bar-button--visible');
    const actionBarMoreButton = actionBar.locator('#action-bar-more-button');
    const popover = documents.locator('.action-bar-more-popover');

    await resizePage(documents, 1600);
    await expect(actionBar).toBeVisible();
    await expect(actionsBarButtons).toHaveCount(2);
    await expect(actionsBarButtons).toHaveText(['New folder', 'Import']);
    await expect(actionBarMoreButton).toBeHidden();

    await resizePage(documents, 820);
    await expect(actionsBarButtons).toContainText(['New folder']);
    await expect(actionBarMoreButton).toBeVisible();
    await actionBarMoreButton.click();
    await expect(documents.locator('.popover-viewport').getByRole('listitem').nth(0)).toHaveText('Import');
    await documents.keyboard.press('Escape');
    await expect(documents.locator('.popover-viewport')).toBeHidden();

    await resizePage(documents, 2000);
    const firstFileEntry = documents.locator('.folder-container').locator('.file-list-item').nth(1);
    await firstFileEntry.hover();
    await firstFileEntry.locator('.ms-checkbox').check();
    // All buttons should be visible
    await expect(actionsBarButtons).toHaveText([
      'Preview',
      'Rename',
      'Move to',
      'Make a copy',
      'Delete',
      'Download',
      'Details',
      'Copy link',
    ]);
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
    msTest.skip(`Selection in ${gridMode ? 'grid' : 'list'} mode by clicking on the item`, async ({ documents }, testInfo: TestInfo) => {
      await importDefaultFiles(documents, testInfo, ImportDocuments.Png | ImportDocuments.Pdf | ImportDocuments.Txt, true);

      if (gridMode) {
        await toggleViewMode(documents);
      }
      const entries = documents.locator('.folder-container').locator(gridMode ? '.file-card-item' : '.file-list-item');
      const actionBar = documents.locator('#folders-ms-action-bar');

      await expect(actionBar.locator('.counter')).toHaveText('4 items');

      await expect(entries).toHaveCount(4);
      if (gridMode) {
        await entries.nth(1).click();
        await expect(actionBar.locator('.counter')).toHaveText('1 selected item');
        await expect(entries.nth(1).locator('.ms-checkbox')).toBeChecked();
        await entries.nth(3).click();
        await expect(entries.nth(3).locator('.ms-checkbox')).toBeChecked();
      } else {
        await entries.nth(1).click();
        await expect(actionBar.locator('.counter')).toHaveText('1 selected item');
        await expect(entries.nth(1).locator('.ms-checkbox')).toBeChecked();
        await entries.nth(3).click();
        await expect(entries.nth(3).locator('.ms-checkbox')).toBeChecked();
      }

      await expect(actionBar.locator('.counter')).toHaveText('2 selected items');
    });
  }

  for (const gridMode of [false, true]) {
    msTest(`Open file in ${gridMode ? 'grid' : 'list'} mode`, async ({ documents }, testInfo) => {
      await importDefaultFiles(documents, testInfo, ImportDocuments.Png, true);

      await expect(documents.locator('.information-modal')).toBeHidden();
      await expect(documents).toHaveHeader(['wksp1'], true, true);
      if (gridMode) {
        await toggleViewMode(documents);
        await documents.locator('.folder-container').locator('.file-card-item').nth(1).dblclick();
      } else {
        await documents.locator('.folder-container').getByRole('listitem').nth(1).dblclick();
      }

      await expect(documents).toBeViewerPage();
    });

    msTest(`Navigation back and forth in ${gridMode ? 'grid' : 'list'} mode`, async ({ documents }, testInfo: TestInfo) => {
      async function navigateDown(): Promise<void> {
        if (gridMode) {
          await documents.locator('.folder-container').locator('.file-card-item').nth(0).dblclick();
        } else {
          await documents.locator('.folder-container').getByRole('listitem').nth(0).dblclick();
        }
      }

      async function navigateUp(): Promise<void> {
        await documents
          .locator('#connected-header')
          .locator('.topbar-left')
          .locator('.back-button-container')
          .locator('ion-button')
          .click();
      }

      await importDefaultFiles(documents, testInfo, ImportDocuments.Png, true);

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

  msTest('List folder with offline mode', async ({ documents }, testInfo: TestInfo) => {
    await importDefaultFiles(documents, testInfo, ImportDocuments.Png, true);

    const errorListFolder = documents.locator('.error-list-folder');
    const noFilesContent = documents.locator('.no-files-content');

    await expect(errorListFolder).toBeHidden();
    await expect(noFilesContent).toBeHidden();

    await mockLibParsec(documents, [
      {
        name: 'workspaceStatFolderChildren',
        result: { ok: false, error: { tag: 'WorkspaceStatEntryErrorEntryNotFound', error: "Path doesn't exist" } },
      },
      {
        name: 'workspaceStatFolderChildrenById',
        result: { ok: false, error: { tag: 'WorkspaceStatEntryErrorEntryNotFound', error: "Path doesn't exist" } },
      },
    ]);

    await documents.locator('.folder-container').locator('.file-list-item').nth(0).dblclick();

    await expect(errorListFolder.locator('.container-textinfo__text')).toHaveText('Failed to list the content of this folder.');
    await expect(noFilesContent).toBeHidden();
    await expect(documents).toHaveHeader(['wksp1', 'Dir_Folder'], true, true);

    await clearLibParsecMocks(documents);
    await documents.locator('.topbar-left').locator('.back-button').click();
    await expect(documents).toHaveHeader(['wksp1'], true, true);
    const entries = documents.locator('.folder-container').locator('.file-list-item');
    await expect(entries).toHaveCount(2);
    await expect(noFilesContent).toBeHidden();
    await expect(errorListFolder).toBeHidden();
  });
});

msTest('Documents page default state in a read only workspace', async ({ documentsReadOnly }) => {
  const actionBar = documentsReadOnly.locator('#folders-ms-action-bar');
  await expect(actionBar.locator('.ms-action-bar-button:visible')).toHaveCount(0);
  await expect(actionBar.locator('.right-side').locator('.label-role')).toHaveText('Reader');
  await expect(actionBar.locator('.counter')).toHaveText('9 items');
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
  for (const checkbox of await entries.locator('.checkbox').all()) {
    await expect(checkbox).toBeHidden();
  }
});
