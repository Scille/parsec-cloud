// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { Locator, TestInfo } from '@playwright/test';
import {
  answerQuestion,
  checkEntryContextMenu,
  checkFolderGlobalContextMenu,
  DisplaySize,
  expect,
  fillInputModal,
  fillIonInput,
  importDefaultFiles,
  ImportDocuments,
  mockLibParsec,
  MsPage,
  msTest,
} from '@tests/e2e/helpers';
import { mockDesktop } from '@tests/e2e/helpers/mock';

async function isInGridMode(page: MsPage): Promise<boolean> {
  const smallDisplay = (await page.getDisplaySize()) === DisplaySize.Small;
  return (
    (await page
      .locator(smallDisplay ? '.mobile-filters' : '#folders-ms-action-bar')
      .locator('#grid-view')
      .getAttribute('disabled')) !== null
  );
}

async function toggleViewMode(page: MsPage): Promise<void> {
  const smallDisplay = (await page.getDisplaySize()) === DisplaySize.Small;
  const locator = smallDisplay ? '.mobile-filters' : '#folders-ms-action-bar';
  if (await isInGridMode(page)) {
    await page.locator(locator).locator('#list-view').click();
  } else {
    await page.locator(locator).locator('#grid-view').click();
  }
}

async function openPopover(page: MsPage, index: number): Promise<Locator> {
  const smallDisplay = (await page.getDisplaySize()) === DisplaySize.Small;
  if (await isInGridMode(page)) {
    const entry = page.locator('.folder-container').locator('.file-card-item').nth(index);
    await entry.hover();
    await entry.locator('.card-option').click();
  } else {
    const entry = page.locator('.folder-container').locator('.file-list-item').nth(index);
    await expect(entry).toBeVisible();
    await entry.hover();
    await entry.locator('.options-button').click();
  }
  if (smallDisplay) {
    return page.locator('.file-context-sheet-modal');
  }
  return page.locator('.file-context-menu');
}

async function clickAction(popover: Locator, action: string): Promise<void> {
  await popover.getByRole('listitem').filter({ hasText: action }).click();
}

msTest.describe(() => {
  msTest.use({
    documentsOptions: {
      empty: true,
    },
  });
  for (const gridMode of [false, true]) {
    msTest(`Document actions default state in ${gridMode ? 'grid' : 'list'} mode for file`, async ({ documents }, testInfo: TestInfo) => {
      await importDefaultFiles(documents, testInfo, ImportDocuments.Png, false);
      await expect(documents.locator('.file-context-menu')).toBeHidden();
      if (!gridMode) {
        const entry = documents.locator('.folder-container').locator('.file-list-item').nth(0);
        await entry.hover();
        await entry.locator('.options-button').click();
      } else {
        await toggleViewMode(documents);
        const entry = documents.locator('.folder-container').locator('.file-card-item').nth(0);
        await entry.hover();
        await entry.locator('.card-option').click();
      }
      await checkEntryContextMenu(documents, 'file-full', 'dismiss');
    });

    msTest(`Document actions default state in ${gridMode ? 'grid' : 'list'} mode for folder`, async ({ documents }, testInfo: TestInfo) => {
      await importDefaultFiles(documents, testInfo, 0, true);
      await expect(documents.locator('.file-context-menu')).toBeHidden();
      if (!gridMode) {
        const entry = documents.locator('.folder-container').locator('.file-list-item').nth(0);
        await entry.hover();
        await entry.locator('.options-button').click();
      } else {
        await toggleViewMode(documents);
        const entry = documents.locator('.folder-container').locator('.file-card-item').nth(0);
        await entry.hover();
        await entry.locator('.card-option').click();
      }
      await checkEntryContextMenu(documents, 'folder-full', 'dismiss');
    });

    msTest(`Document popover on right click in ${gridMode ? 'grid' : 'list'} mode for file`, async ({ documents }, testInfo: TestInfo) => {
      await importDefaultFiles(documents, testInfo, ImportDocuments.Png, false);
      await expect(documents.locator('.file-context-menu')).toBeHidden();
      if (!gridMode) {
        const entry = documents.locator('.folder-container').locator('.file-list-item').nth(0);
        await entry.click({ button: 'right' });
      } else {
        await toggleViewMode(documents);
        const entry = documents.locator('.folder-container').locator('.file-card-item').nth(0);
        await entry.click({ button: 'right' });
      }
      await checkEntryContextMenu(documents, 'file-full', 'dismiss');
    });

    msTest(
      `Document popover on right click in ${gridMode ? 'grid' : 'list'} mode for folder`,
      async ({ documents }, testInfo: TestInfo) => {
        await importDefaultFiles(documents, testInfo, 0, true);
        await expect(documents.locator('.file-context-menu')).toBeHidden();
        if (!gridMode) {
          const entry = documents.locator('.folder-container').locator('.file-list-item').nth(0);
          await entry.click({ button: 'right' });
        } else {
          await toggleViewMode(documents);
          const entry = documents.locator('.folder-container').locator('.file-card-item').nth(0);
          await entry.click({ button: 'right' });
        }
        await checkEntryContextMenu(documents, 'folder-full', 'dismiss');
      },
    );
  }

  msTest('Document actions default state on desktop mode for file and folder', async ({ documents }, testInfo: TestInfo) => {
    await mockDesktop(documents);
    await importDefaultFiles(documents, testInfo, ImportDocuments.Png, true);
    await expect(documents.locator('.file-context-menu')).toBeHidden();
    const entry = documents.locator('.folder-container').locator('.file-list-item').nth(1);
    await entry.hover();
    await entry.locator('.options-button').click();
    await checkEntryContextMenu(documents, 'file-full', 'dismiss', { onDesktop: true });
    const folder = documents.locator('.folder-container').locator('.file-list-item').nth(0);
    await folder.hover();
    await folder.locator('.options-button').click();
    await checkEntryContextMenu(documents, 'folder-full', 'dismiss', { onDesktop: true });
  });

  msTest('Document actions default state on desktop mode for file and folder in readonly', async ({ documentsReadOnly }) => {
    await mockDesktop(documentsReadOnly);
    await expect(documentsReadOnly.locator('.file-context-menu')).toBeHidden();
    const entry = documentsReadOnly.locator('.folder-container').locator('.file-list-item').nth(1);
    await entry.hover();
    await entry.locator('.options-button').click();
    await checkEntryContextMenu(documentsReadOnly, 'file-readonly', 'dismiss', { onDesktop: true });
    const folder = documentsReadOnly.locator('.folder-container').locator('.file-list-item').nth(0);
    await folder.hover();
    await folder.locator('.options-button').click();
    await checkEntryContextMenu(documentsReadOnly, 'folder-readonly', 'dismiss', { onDesktop: true });
  });

  msTest('Document popover on right click for file with editics', async ({ parsecEditics }, testInfo: TestInfo) => {
    await importDefaultFiles(parsecEditics, testInfo, ImportDocuments.Docx, false);
    const entry = parsecEditics.locator('.folder-container').locator('.file-list-item').nth(0);
    await entry.click({ button: 'right' });
    await checkEntryContextMenu(parsecEditics, 'file-full', 'dismiss', { canEdit: true });
  });

  msTest('Document popover on right click for file with editics on non-editable file', async ({ parsecEditics }, testInfo: TestInfo) => {
    await importDefaultFiles(parsecEditics, testInfo, ImportDocuments.Png, false);
    const entry = parsecEditics.locator('.folder-container').locator('.file-list-item').nth(0);
    await entry.click({ button: 'right' });
    await checkEntryContextMenu(parsecEditics, 'file-full', 'dismiss');
  });

  msTest('Document popover on right click on multiple files only files', async ({ documents }, testInfo: TestInfo) => {
    await importDefaultFiles(documents, testInfo, ImportDocuments.Pdf | ImportDocuments.Png, false);
    await documents.waitForTimeout(500);
    await documents.locator('.folder-container').locator('.files-list-header').locator('.ms-checkbox').check();
    await expect(documents.locator('.action-bar').locator('.counter')).toHaveText('2 selected items');

    const entries = documents.locator('.folder-container').locator('.file-list-item');
    await entries.nth(0).click({ button: 'right' });
    await checkEntryContextMenu(documents, 'multiple-entries-full', 'dismiss');
  });

  msTest('Document popover on right click on multiple files in readonly', async ({ documentsReadOnly }) => {
    await documentsReadOnly.locator('.folder-container').locator('.files-list-header').locator('.ms-checkbox').check();
    const entries = documentsReadOnly.locator('.folder-container').locator('.file-list-item');

    await entries.nth(0).click({ button: 'right' });
    await checkEntryContextMenu(documentsReadOnly, 'multiple-entries-readonly', 'dismiss');
  });

  msTest('Document popover on right click on multiple files with a folder', async ({ documents }, testInfo: TestInfo) => {
    await importDefaultFiles(documents, testInfo, ImportDocuments.Png, true);
    await documents.waitForTimeout(300);
    await documents.locator('.folder-container').locator('.files-list-header').locator('.ms-checkbox').check();
    const entries = documents.locator('.folder-container').locator('.file-list-item');
    await expect(entries).toHaveCount(2);
    await expect(documents.locator('#folders-ms-action-bar').locator('.counter')).toHaveText('2 selected items');

    await entries.nth(0).locator('.file-last-update').click({ button: 'right' });
    await checkEntryContextMenu(documents, 'multiple-entries-full', 'dismiss');
  });

  msTest('Popover with right click on empty space', async ({ documents }, testInfo: TestInfo) => {
    await importDefaultFiles(documents, testInfo, ImportDocuments.Png, false);

    await documents.locator('.folder-container').click({ button: 'right', position: { x: 100, y: 10 } });
    await checkFolderGlobalContextMenu(documents, 'full', 'dismiss');
  });

  msTest('Popover with right click on empty space readonly', async ({ documentsReadOnly }) => {
    await documentsReadOnly.locator('.folder-container').click({ button: 'right', position: { x: 100, y: 10 } });
    await checkFolderGlobalContextMenu(documentsReadOnly, 'readonly', 'dismiss');
  });

  msTest('Get document link', async ({ documents, context }, testInfo: TestInfo) => {
    await importDefaultFiles(documents, testInfo, ImportDocuments.Png, false);
    await clickAction(await openPopover(documents, 0), 'Copy link');

    // Fail to copy because no permission
    await expect(documents).toShowToast('Failed to copy link. Your browser or device does not seem to support copy/paste.', 'Error');

    // Grant the permissions
    await context.grantPermissions(['clipboard-write']);

    await clickAction(await openPopover(documents, 0), 'Copy link');
    await expect(documents).toShowToast('Link has been copied to clipboard.', 'Info');
    const filePath = await documents.evaluate(() => navigator.clipboard.readText());
    expect(filePath).toMatch(/^https?:\/\/.+\/redirect\/.+a=path&p=.+$/);
  });

  msTest('Rename document', async ({ documents }, testInfo: TestInfo) => {
    await importDefaultFiles(documents, testInfo, ImportDocuments.Png, false);
    const entry = documents.locator('.folder-container').locator('.file-list-item').nth(0);

    await clickAction(await openPopover(documents, 0), 'Rename');
    await fillInputModal(documents, 'New Name', true);
    await expect(entry.locator('.file-name').locator('.label-name')).toHaveText('New Name');
  });

  msTest('Rename document name already exists', async ({ documents }, testInfo: TestInfo) => {
    await importDefaultFiles(documents, testInfo, ImportDocuments.Png | ImportDocuments.Txt, false);
    await clickAction(await openPopover(documents, 0), 'Rename');

    const modal = documents.locator('.text-input-modal');
    await expect(modal).toBeVisible();
    await fillIonInput(modal.locator('ion-input'), '');
    const okButton = modal.locator('.ms-modal-footer-buttons').locator('#next-button');
    await fillIonInput(modal.locator('ion-input'), 'text.txt');
    await expect(okButton).toBeTrulyDisabled();
    await expect(modal.locator('.form-error')).toBeVisible();
    await expect(modal.locator('.form-error')).toHaveText('A file with this name already exists.');
  });

  msTest('Open history on file', async ({ documents }, testInfo: TestInfo) => {
    await importDefaultFiles(documents, testInfo, ImportDocuments.Png, false);
    await documents.waitForTimeout(300);
    await clickAction(await openPopover(documents, 0), 'History');
    await expect(documents).toBeWorkspaceHistoryPage();
  });

  msTest('Delete document', async ({ documents }, testInfo: TestInfo) => {
    await importDefaultFiles(documents, testInfo, ImportDocuments.Png | ImportDocuments.Pdf, false);
    await clickAction(await openPopover(documents, 0), 'Delete');
    await answerQuestion(documents, true, {
      expectedTitleText: 'Delete one file',
      expectedQuestionText: 'Are you sure you want to delete file image.png?',
      expectedNegativeText: 'Keep file',
      expectedPositiveText: 'Delete file',
    });
    await expect(documents.locator('.folder-container').locator('.file-list-item')).toHaveCount(1);
  });

  msTest('Delete folder', async ({ documents }, testInfo: TestInfo) => {
    await importDefaultFiles(documents, testInfo, ImportDocuments.Png, true);
    await clickAction(await openPopover(documents, 0), 'Delete');

    await answerQuestion(documents, true, {
      expectedTitleText: 'Delete one folder',
      expectedQuestionText: 'Are you sure you want to delete folder Dir_Folder?',
      expectedNegativeText: 'Keep folder',
      expectedPositiveText: 'Delete folder',
    });
    await expect(documents.locator('.folder-container').locator('.file-list-item')).toHaveCount(1);
  });

  msTest('Folder actions default state in a read only workspace', async ({ documentsReadOnly }) => {
    await expect(documentsReadOnly.locator('.file-context-menu')).toBeHidden();
    const entry = documentsReadOnly.locator('.folder-container').locator('.file-list-item').nth(0);
    await entry.hover();
    await entry.locator('.options-button').click();
    await checkEntryContextMenu(documentsReadOnly, 'folder-readonly', 'dismiss');
  });

  msTest('File actions default state in a read only workspace', async ({ documentsReadOnly }) => {
    await expect(documentsReadOnly.locator('.file-context-menu')).toBeHidden();
    const entry = documentsReadOnly.locator('.folder-container').locator('.file-list-item').nth(1);
    await entry.hover();
    await entry.locator('.options-button').click();
    await checkEntryContextMenu(documentsReadOnly, 'file-readonly', 'dismiss');
  });

  for (const gridMode of [false, true]) {
    msTest(
      `Small display document actions default state in ${gridMode ? 'grid' : 'list'} mode for file`,
      async ({ documents }, testInfo: TestInfo) => {
        await importDefaultFiles(documents, testInfo, ImportDocuments.Png, false);
        await documents.setDisplaySize(DisplaySize.Small);
        await expect(documents.locator('.file-context-menu')).toBeHidden();
        if (!gridMode) {
          const entry = documents.locator('.folder-container').locator('.file-list-item').nth(0);
          await entry.hover();
          await entry.locator('.options-button').click();
        } else {
          await toggleViewMode(documents);
          const entry = documents.locator('.folder-container').locator('.file-card-item').nth(0);
          await entry.hover();
          await entry.locator('.card-option').click();
        }
        await expect(documents.locator('.file-context-sheet-modal')).toBeVisible();
        const modal = documents.locator('.file-context-sheet-modal');
        await expect(modal.getByRole('group')).toHaveCount(2);
        await checkEntryContextMenu(documents, 'file-full', 'dismiss');
      },
    );

    msTest(
      `Small display document actions default state in ${gridMode ? 'grid' : 'list'} mode for folder`,
      async ({ documents }, testInfo: TestInfo) => {
        await importDefaultFiles(documents, testInfo, ImportDocuments.Png, true);
        await documents.setDisplaySize(DisplaySize.Small);
        if (!gridMode) {
          const entry = documents.locator('.folder-container').locator('.file-list-item').nth(0);
          await entry.hover();
          await entry.locator('.options-button').click();
        } else {
          await toggleViewMode(documents);
          const entry = documents.locator('.folder-container').locator('.file-card-item').nth(0);
          await entry.hover();
          await entry.locator('.card-option').click();
        }
        await checkEntryContextMenu(documents, 'folder-full', 'dismiss');
      },
    );

    msTest(
      `Small display document popover on right click in ${gridMode ? 'grid' : 'list'} mode for file`,
      async ({ documents }, testInfo: TestInfo) => {
        await importDefaultFiles(documents, testInfo, ImportDocuments.Png, false);
        await documents.setDisplaySize(DisplaySize.Small);
        await expect(documents.locator('.file-context-menu')).toBeHidden();
        if (!gridMode) {
          const entry = documents.locator('.folder-container').locator('.file-list-item').nth(0);
          await entry.click({ button: 'right' });
        } else {
          await toggleViewMode(documents);
          const entry = documents.locator('.folder-container').locator('.file-card-item').nth(0);
          await entry.click({ button: 'right' });
        }
        await expect(documents.locator('.file-context-sheet-modal')).toBeVisible();
        const modal = documents.locator('.file-context-sheet-modal');
        await expect(modal.getByRole('group')).toHaveCount(2);
        await checkEntryContextMenu(documents, 'file-full', 'dismiss');
      },
    );

    msTest(
      `Small display document popover on right click in ${gridMode ? 'grid' : 'list'} mode for folder`,
      async ({ documents }, testInfo: TestInfo) => {
        await importDefaultFiles(documents, testInfo, 0, true);
        await documents.setDisplaySize(DisplaySize.Small);
        await documents.waitForTimeout(300);

        if (!gridMode) {
          const entry = documents.locator('.folder-container').locator('.file-list-item').nth(0);
          await entry.click({ button: 'right' });
        } else {
          await toggleViewMode(documents);
          const entry = documents.locator('.folder-container').locator('.file-card-item').nth(0);
          await entry.click({ button: 'right' });
        }
        await checkEntryContextMenu(documents, 'folder-full', 'dismiss');
      },
    );
  }

  msTest('Small display popover on right click for file with editics', async ({ parsecEditics }, testInfo: TestInfo) => {
    await importDefaultFiles(parsecEditics, testInfo, ImportDocuments.Docx, false);
    await parsecEditics.setDisplaySize(DisplaySize.Small);
    await parsecEditics.waitForTimeout(300);
    const entry = parsecEditics.locator('.folder-container').locator('.file-list-item').nth(0);
    await entry.click({ button: 'right' });
    await checkEntryContextMenu(parsecEditics, 'file-full', 'dismiss', { canEdit: true });
  });

  msTest(
    'Small display popover on right click for file with editics on non-editable file',
    async ({ parsecEditics }, testInfo: TestInfo) => {
      await importDefaultFiles(parsecEditics, testInfo, ImportDocuments.Png, false);
      await parsecEditics.setDisplaySize(DisplaySize.Small);
      await parsecEditics.waitForTimeout(300);
      const entry = parsecEditics.locator('.folder-container').locator('.file-list-item').nth(0);
      await entry.click({ button: 'right' });
      await checkEntryContextMenu(parsecEditics, 'file-full', 'dismiss');
    },
  );

  msTest('Small display popover with right click on empty space', async ({ documents }, testInfo: TestInfo) => {
    await importDefaultFiles(documents, testInfo, ImportDocuments.Png | ImportDocuments.Txt, false);
    await documents.setDisplaySize(DisplaySize.Small);
    await documents.waitForTimeout(300);

    await documents.locator('.topbar-right-buttons').locator('.topbar-right-buttons__item').nth(0).click();
    await expect(documents.locator('.file-context-sheet-modal')).toBeVisible();
    const modal = documents.locator('.file-context-sheet-modal');
    await expect(modal.locator('.button-left')).toHaveText('Selection');
    await expect(modal.locator('.button-right')).toHaveText('Share');
    await expect(modal.locator('.list-group-item')).toHaveCount(1);
    await expect(modal.locator('.list-group-item').nth(0)).toHaveText('Select all');
    await modal.locator('.list-group-item').nth(0).click();
    await documents.waitForTimeout(300);

    const headerElements = documents.locator('.small-display-selection-header').locator('ion-text');
    await expect(headerElements).toHaveCount(3);
    await expect(headerElements.nth(0)).toHaveText('Unselect');
    await expect(headerElements.nth(1)).toHaveText('2 selected items');
    await expect(headerElements.nth(2)).toHaveText('Cancel');
    await headerElements.nth(0).click();
    await expect(headerElements.nth(1)).toHaveText('wksp1');
    await expect(headerElements.nth(0)).toBeVisible();
    await expect(headerElements.nth(2)).toBeVisible();
    await headerElements.nth(2).click();
    await expect(headerElements).toBeHidden();
  });

  msTest('Small display document popover on right click on multiple files only files', async ({ documents }, testInfo: TestInfo) => {
    await importDefaultFiles(documents, testInfo, ImportDocuments.Png | ImportDocuments.Pdf, true);
    await documents.setDisplaySize(DisplaySize.Small);
    await documents.waitForTimeout(300);
    const entries = documents.locator('.folder-container').locator('.file-list-item');
    await documents.locator('.topbar-right-buttons').locator('.topbar-right-buttons__item').nth(0).click();

    await documents.locator('.file-context-sheet-modal').locator('.list-group-item').nth(0).click();

    await expect(documents.locator('.folder-content').locator('.title__text')).toContainText('3 selected items');

    await expect(entries.nth(0).locator('.ms-checkbox')).toBeChecked();
    await expect(entries.nth(1).locator('.ms-checkbox')).toBeChecked();
    await expect(entries.nth(2).locator('.ms-checkbox')).toBeChecked();

    // Unselect the folder
    entries.nth(0).locator('.ms-checkbox').click();
    await entries.nth(1).click({ button: 'right' });

    await checkEntryContextMenu(documents, 'multiple-entries-full', 'dismiss');
  });

  msTest('Small display document popover on right click on multiple files with a folder', async ({ documents }, testInfo: TestInfo) => {
    await importDefaultFiles(documents, testInfo, ImportDocuments.Png | ImportDocuments.Pdf, true);
    await documents.setDisplaySize(DisplaySize.Small);
    await documents.waitForTimeout(300);

    const entries = documents.locator('.folder-container').locator('.file-list-item');
    await documents.locator('.topbar-right-buttons').locator('.topbar-right-buttons__item').nth(0).click();
    await documents.locator('.file-context-sheet-modal').locator('.list-group-item').nth(0).click();
    await expect(entries.nth(0).locator('.ms-checkbox')).toBeChecked();
    await expect(entries.nth(1).locator('.ms-checkbox')).toBeChecked();
    await expect(entries.nth(2).locator('.ms-checkbox')).toBeChecked();

    await entries.nth(0).click({ button: 'right' });

    await checkEntryContextMenu(documents, 'multiple-entries-full', 'dismiss');
  });

  msTest('Small display get document link', async ({ documents, context }, testInfo: TestInfo) => {
    await importDefaultFiles(documents, testInfo, ImportDocuments.Png, false);
    await documents.setDisplaySize(DisplaySize.Small);
    await clickAction(await openPopover(documents, 0), 'Copy link');

    // Fail to copy because no permission
    await expect(documents).toShowToast('Failed to copy link. Your browser or device does not seem to support copy/paste.', 'Error');

    // Grant the permissions
    await context.grantPermissions(['clipboard-write']);

    await clickAction(await openPopover(documents, 0), 'Copy link');
    await expect(documents).toShowToast('Link has been copied to clipboard.', 'Info');
    const filePath = await documents.evaluate(() => navigator.clipboard.readText());
    expect(filePath).toMatch(/^https?:\/\/.+\/redirect\/.+a=path&p=.+$/);
  });

  msTest('Small display rename document', async ({ documents }, testInfo: TestInfo) => {
    await importDefaultFiles(documents, testInfo, ImportDocuments.Png, false);
    await documents.setDisplaySize(DisplaySize.Small);
    const entry = documents.locator('.folder-container').locator('.file-list-item').nth(0);

    await clickAction(await openPopover(documents, 0), 'Rename');
    await fillInputModal(documents, 'New Name', true);
    await expect(entry.locator('.file-name').locator('.label-name')).toHaveText('New Name');
  });

  msTest('Small display delete document', async ({ documents }, testInfo: TestInfo) => {
    await importDefaultFiles(documents, testInfo, ImportDocuments.Png, false);
    await documents.setDisplaySize(DisplaySize.Small);

    await clickAction(await openPopover(documents, 0), 'Delete');

    await answerQuestion(documents, true, {
      expectedTitleText: 'Delete one file',
      expectedQuestionText: 'Are you sure you want to delete file image.png?',
      expectedNegativeText: 'Keep file',
      expectedPositiveText: 'Delete file',
    });
    await expect(documents.locator('.folder-container').locator('.file-list-item')).toHaveCount(0);
  });

  msTest('Small display folder actions default state in a read only workspace', async ({ documentsReadOnly }) => {
    await documentsReadOnly.setDisplaySize(DisplaySize.Small);
    await expect(documentsReadOnly.locator('.file-context-menu')).toBeHidden();
    const entry = documentsReadOnly.locator('.folder-container').locator('.file-list-item').nth(0);
    await entry.hover();
    await entry.locator('.options-button').click();
    await checkEntryContextMenu(documentsReadOnly, 'folder-readonly', 'dismiss');
  });

  msTest('Small display file actions default state in a read only workspace', async ({ documentsReadOnly }) => {
    await documentsReadOnly.setDisplaySize(DisplaySize.Small);
    await expect(documentsReadOnly.locator('.file-context-menu')).toBeHidden();
    const entry = documentsReadOnly.locator('.folder-container').locator('.file-list-item').nth(1);
    await entry.hover();
    await entry.locator('.options-button').click();
    await checkEntryContextMenu(documentsReadOnly, 'file-readonly', 'dismiss');
  });

  msTest('Rename document error', async ({ documents }, testInfo: TestInfo) => {
    await importDefaultFiles(documents, testInfo, ImportDocuments.Png, false);
    await mockLibParsec(documents, [
      {
        name: 'workspaceMoveEntry',
        result: { ok: false, error: { tag: 'WorkspaceMoveEntryErrorOffline', error: 'failed' } },
      },
    ]);
    const entry = documents.locator('.folder-container').locator('.file-list-item').nth(0);

    await clickAction(await openPopover(documents, 0), 'Rename');
    await fillInputModal(documents, 'New Name', true);
    await expect(documents).toShowToast('Failed to rename file `image.png`, please try again.', 'Error');
    await expect(entry.locator('.label-name')).toHaveText('image.png');
  });

  msTest('Delete document error', async ({ documents }, testInfo: TestInfo) => {
    await importDefaultFiles(documents, testInfo, ImportDocuments.Png, false);
    await mockLibParsec(documents, [
      {
        name: 'workspaceRemoveFile',
        result: { ok: false, error: { tag: 'WorkspaceRemoveEntryErrorOffline', error: 'failed' } },
      },
    ]);
    const entry = documents.locator('.folder-container').locator('.file-list-item').nth(0);

    await clickAction(await openPopover(documents, 0), 'Delete');
    await answerQuestion(documents, true, {
      expectedTitleText: 'Delete one file',
      expectedQuestionText: 'Are you sure you want to delete file image.png?',
      expectedNegativeText: 'Keep file',
      expectedPositiveText: 'Delete file',
    });
    await expect(documents).toShowToast('Failed to delete file `image.png`, please try again.', 'Error');
    await expect(entry.locator('.label-name')).toHaveText('image.png');
  });
});
