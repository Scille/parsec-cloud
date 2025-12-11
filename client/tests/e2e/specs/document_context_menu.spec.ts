// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { Locator, TestInfo } from '@playwright/test';
import {
  answerQuestion,
  DisplaySize,
  expandSheetModal,
  expect,
  fillInputModal,
  importDefaultFiles,
  ImportDocuments,
  MsPage,
  msTest,
} from '@tests/e2e/helpers';

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
    await entry.hover();
    await entry.locator('.options-button').click();
  }
  if (smallDisplay) {
    await expandSheetModal(page, page.locator('.file-context-sheet-modal'));
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
      await expect(documents.locator('.file-context-menu')).toBeVisible();
      const popover = documents.locator('.file-context-menu');
      await expect(popover.getByRole('group')).toHaveCount(2);
      await expect(popover.getByRole('listitem')).toHaveText([
        'File management',
        'Preview',
        'Rename',
        'Move to',
        'Make a copy',
        'History',
        'Download',
        'Details',
        'Delete',
        'Collaboration',
        'Copy link',
      ]);
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
      await expect(documents.locator('.file-context-menu')).toBeVisible();
      const popover = documents.locator('.file-context-menu');
      await expect(popover.getByRole('group')).toHaveCount(2);
      await expect(popover.getByRole('listitem')).toHaveText([
        'Folder management',
        'Rename',
        'Move to',
        'Make a copy',
        'History',
        'Details',
        'Delete',
        'Collaboration',
        'Copy link',
      ]);
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
      await expect(documents.locator('.file-context-menu')).toBeVisible();
      const popover = documents.locator('.file-context-menu');
      await expect(popover.getByRole('group')).toHaveCount(2);
      await expect(popover.getByRole('listitem')).toHaveText([
        'File management',
        'Preview',
        'Rename',
        'Move to',
        'Make a copy',
        'History',
        'Download',
        'Details',
        'Delete',
        'Collaboration',
        'Copy link',
      ]);
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
        await expect(documents.locator('.file-context-menu')).toBeVisible();
        const popover = documents.locator('.file-context-menu');
        await expect(popover.getByRole('group')).toHaveCount(2);
        await expect(popover.getByRole('listitem')).toHaveText([
          'Folder management',
          'Rename',
          'Move to',
          'Make a copy',
          'History',
          'Details',
          'Delete',
          'Collaboration',
          'Copy link',
        ]);
      },
    );

    msTest(
      `Document popover on right click in ${gridMode ? 'grid' : 'list'} mode for file with editics`,
      async ({ parsecEditics }, testInfo: TestInfo) => {
        await importDefaultFiles(parsecEditics, testInfo, ImportDocuments.Docx, false);
        await expect(parsecEditics.locator('.file-context-menu')).toBeHidden();
        if (!gridMode) {
          const entry = parsecEditics.locator('.folder-container').locator('.file-list-item').nth(0);
          await entry.click({ button: 'right' });
        } else {
          await toggleViewMode(parsecEditics);
          const entry = parsecEditics.locator('.folder-container').locator('.file-card-item').nth(0);
          await entry.click({ button: 'right' });
        }
        await expect(parsecEditics.locator('.file-context-menu')).toBeVisible();
        const popover = parsecEditics.locator('.file-context-menu');
        await expect(popover.getByRole('group')).toHaveCount(2);
        await expect(popover.getByRole('listitem')).toHaveText([
          'File management',
          'Preview',
          'Edit',
          'Rename',
          'Move to',
          'Make a copy',
          'History',
          'Download',
          'Details',
          'Delete',
          'Collaboration',
          'Copy link',
        ]);
      },
    );

    msTest(
      `Document popover on right click in ${gridMode ? 'grid' : 'list'} mode for file with editics on non-editable file`,
      async ({ parsecEditics }, testInfo: TestInfo) => {
        await importDefaultFiles(parsecEditics, testInfo, ImportDocuments.Png, false);
        await expect(parsecEditics.locator('.file-context-menu')).toBeHidden();
        if (!gridMode) {
          const entry = parsecEditics.locator('.folder-container').locator('.file-list-item').nth(0);
          await entry.click({ button: 'right' });
        } else {
          await toggleViewMode(parsecEditics);
          const entry = parsecEditics.locator('.folder-container').locator('.file-card-item').nth(0);
          await entry.click({ button: 'right' });
        }
        await expect(parsecEditics.locator('.file-context-menu')).toBeVisible();
        const popover = parsecEditics.locator('.file-context-menu');
        await expect(popover.getByRole('group')).toHaveCount(2);
        await expect(popover.getByRole('listitem')).toHaveText([
          'File management',
          'Preview',
          'Rename',
          'Move to',
          'Make a copy',
          'History',
          'Download',
          'Details',
          'Delete',
          'Collaboration',
          'Copy link',
        ]);
      },
    );

    msTest(
      `Document popover on right click on multiple files in ${gridMode ? 'grid' : 'list'} only files`,
      async ({ documents }, testInfo: TestInfo) => {
        await importDefaultFiles(documents, testInfo, ImportDocuments.Pdf | ImportDocuments.Png, false);
        await expect(documents.locator('.file-context-menu')).toBeHidden();
        let entries: Locator;

        if (!gridMode) {
          entries = documents.locator('.folder-container').locator('.file-list-item');
        } else {
          await toggleViewMode(documents);
          entries = documents.locator('.folder-container').locator('.file-card-item');
        }

        for (const entry of await entries.all()) {
          await entry.hover();
          await new Promise((resolve) => setTimeout(resolve, 600));
          await entry.locator('.checkbox').click();
          await expect(entry.locator('.checkbox')).toHaveState('checked');
        }
        await entries.nth(0).click({ button: 'right' });

        await expect(documents.locator('.file-context-menu')).toBeVisible();
        const popover = documents.locator('.file-context-menu');
        await expect(popover.getByRole('group')).toHaveCount(1);
        await expect(popover.getByRole('listitem')).toHaveText(['File management', 'Move to', 'Make a copy', 'Download', 'Delete']);
      },
    );

    msTest(
      `Document popover on right click on multiple files in ${gridMode ? 'grid' : 'list'} with a folder`,
      async ({ documents }, testInfo: TestInfo) => {
        await importDefaultFiles(documents, testInfo, ImportDocuments.Png, true);
        await expect(documents.locator('.file-context-menu')).toBeHidden();
        let entries: Locator;

        if (!gridMode) {
          entries = documents.locator('.folder-container').locator('.file-list-item');
        } else {
          await toggleViewMode(documents);
          entries = documents.locator('.folder-container').locator('.file-card-item');
        }
        await expect(entries).toHaveCount(2);

        for (const entry of await entries.all()) {
          await entry.hover();
          await entry.locator('ion-checkbox').click();
          await expect(entry.locator('ion-checkbox')).toHaveState('checked');
        }
        await entries.nth(0).click({ button: 'right' });

        await expect(documents.locator('.file-context-menu')).toBeVisible();
        const popover = documents.locator('.file-context-menu');
        await expect(popover.getByRole('group')).toHaveCount(1);
        await expect(popover.getByRole('listitem')).toHaveText(['Folder management', 'Move to', 'Make a copy', 'Delete']);
      },
    );

    msTest(`Popover with right click on empty space in ${gridMode ? 'grid' : 'list'} mode`, async ({ documents }, testInfo: TestInfo) => {
      await importDefaultFiles(documents, testInfo, ImportDocuments.Png, false);
      await expect(documents.locator('.folder-global-context-menu')).toBeHidden();

      if (gridMode) {
        await toggleViewMode(documents);
      }
      await documents.locator('.folder-container').click({ button: 'right', position: { x: 100, y: 10 } });
      await expect(documents.locator('.folder-global-context-menu')).toBeVisible();
      const popover = documents.locator('.folder-global-context-menu');
      await expect(popover.getByRole('group')).toHaveCount(1);
      await expect(popover.getByRole('listitem')).toHaveCount(3);
      await expect(popover.getByRole('listitem')).toHaveText(['New folder', 'Import files', 'Import a folder']);
    });

    msTest(`Get document link in ${gridMode ? 'grid' : 'list'} mode`, async ({ documents, context }, testInfo: TestInfo) => {
      await importDefaultFiles(documents, testInfo, ImportDocuments.Png, false);
      if (gridMode) {
        await toggleViewMode(documents);
      }
      await clickAction(await openPopover(documents, 0), 'Copy link');

      // Fail to copy because no permission
      await expect(documents).toShowToast('Failed to copy link. Your browser or device does not seem to support copy/paste.', 'Error');

      // Grant the permissions
      await context.grantPermissions(['clipboard-write']);

      await clickAction(await openPopover(documents, 0), 'Copy link');
      await expect(documents).toShowToast('Link has been copied to clipboard.', 'Info');
      const filePath = await documents.evaluate(() => navigator.clipboard.readText());
      expect(filePath).toMatch(/^parsec3:\/\/.+$/);
    });

    msTest(`Rename document in ${gridMode ? 'grid' : 'list'} mode`, async ({ documents }, testInfo: TestInfo) => {
      await importDefaultFiles(documents, testInfo, ImportDocuments.Png, false);
      let entry;
      if (gridMode) {
        await toggleViewMode(documents);
        entry = documents.locator('.folder-container').locator('.file-card-item').nth(0);
      } else {
        entry = documents.locator('.folder-container').locator('.file-list-item').nth(0);
      }

      await clickAction(await openPopover(documents, 0), 'Rename');
      await fillInputModal(documents, 'New Name', true);
      if (!gridMode) {
        await expect(entry.locator('.file-name').locator('.label-name')).toHaveText('New Name');
      } else {
        await expect(entry.locator('.file-card__title')).toHaveText('New Name');
      }
    });

    msTest(`Delete document in ${gridMode ? 'grid' : 'list'} mode`, async ({ documents }, testInfo: TestInfo) => {
      await importDefaultFiles(documents, testInfo, ImportDocuments.Png | ImportDocuments.Pdf, false);
      if (gridMode) {
        await toggleViewMode(documents);
      }
      let fileName;
      let count = 0;
      if (gridMode) {
        fileName = await documents
          .locator('.folder-container')
          .locator('.file-card-item')
          .nth(0)
          .locator('.file-card__title')
          .textContent();
        count = await documents.locator('.folder-container').locator('.file-card-item').count();
      } else {
        fileName = await documents
          .locator('.folder-container')
          .locator('.file-list-item')
          .nth(0)
          .locator('.file-name')
          .locator('.label-name')
          .textContent();
        count = await documents.locator('.folder-container').locator('.file-list-item').count();
      }

      await clickAction(await openPopover(documents, 0), 'Delete');

      await answerQuestion(documents, true, {
        expectedTitleText: 'Delete one file',
        expectedQuestionText: `Are you sure you want to delete file \`${fileName}\`?`,
        expectedNegativeText: 'Keep file',
        expectedPositiveText: 'Delete file',
      });
      if (gridMode) {
        await expect(documents.locator('.folder-container').locator('.file-card-item')).toHaveCount(count - 1);
      } else {
        await expect(documents.locator('.folder-container').locator('.file-list-item')).toHaveCount(count - 1);
      }
    });

    msTest(`Folder actions default state in a read only workspace in ${gridMode ? 'grid' : 'list'} mode`, async ({ documentsReadOnly }) => {
      await expect(documentsReadOnly.locator('.file-context-menu')).toBeHidden();
      if (!gridMode) {
        const entry = documentsReadOnly.locator('.folder-container').locator('.file-list-item').nth(0);
        await entry.hover();
        await entry.locator('.options-button').click();
      } else {
        await toggleViewMode(documentsReadOnly);
        const entry = documentsReadOnly.locator('.folder-container').locator('.file-card-item').nth(0);
        await entry.hover();
        await entry.locator('.card-option').click();
      }
      await expect(documentsReadOnly.locator('.file-context-menu')).toBeVisible();
      const popover = documentsReadOnly.locator('.file-context-menu');
      await expect(popover.getByRole('group')).toHaveCount(2);
      await expect(popover.getByRole('listitem')).toHaveText(['Folder management', 'Details', 'Collaboration', 'Copy link']);
    });

    msTest(`File actions default state in a read only workspace in ${gridMode ? 'grid' : 'list'} mode`, async ({ documentsReadOnly }) => {
      await expect(documentsReadOnly.locator('.file-context-menu')).toBeHidden();
      if (!gridMode) {
        const entry = documentsReadOnly.locator('.folder-container').locator('.file-list-item').nth(1);
        await entry.hover();
        await entry.locator('.options-button').click();
      } else {
        await toggleViewMode(documentsReadOnly);
        const entry = documentsReadOnly.locator('.folder-container').locator('.file-card-item').nth(1);
        await entry.hover();
        await entry.locator('.card-option').click();
      }
      await expect(documentsReadOnly.locator('.file-context-menu')).toBeVisible();
      const popover = documentsReadOnly.locator('.file-context-menu');
      await expect(popover.getByRole('group')).toHaveCount(2);
      await expect(popover.getByRole('listitem')).toHaveText([
        'File management',
        'Preview',
        'Download',
        'Details',
        'Collaboration',
        'Copy link',
      ]);
    });

    msTest(`Move document in ${gridMode ? 'grid' : 'list'} mode`, async ({ documents }, testInfo: TestInfo) => {
      await importDefaultFiles(documents, testInfo, ImportDocuments.Png, true);
      if (gridMode) {
        await toggleViewMode(documents);
      }
      await expect(documents.locator('.folder-selection-modal')).toBeHidden();
      await clickAction(await openPopover(documents, 1), 'Move to');
      const modal = documents.locator('.folder-selection-modal');
      await expect(modal).toBeVisible();
      await expect(modal.locator('.ms-modal-header__title')).toHaveText('Move one item');
      const okButton = modal.locator('#next-button');
      await expect(okButton).toHaveText('Move here');
      await expect(okButton).toHaveDisabledAttribute();
      await modal.locator('.folder-container').getByRole('listitem').nth(0).click();
      await expect(modal.locator('.current-folder__text')).toHaveText(/^Dir_[A-Za-z_]+$/);
      await expect(okButton).toNotHaveDisabledAttribute();
      await okButton.click();
      await expect(modal).toBeHidden();
    });
  }

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
        await expect(modal.getByRole('listitem')).toHaveText([
          'Preview',
          'Rename',
          'Move to',
          'Make a copy',
          'History',
          'Download',
          'Copy link',
          'Details',
          'Delete',
        ]);
      },
    );

    msTest(
      `Small display document actions default state in ${gridMode ? 'grid' : 'list'} mode for folder`,
      async ({ documents }, testInfo: TestInfo) => {
        await importDefaultFiles(documents, testInfo, 0, true);
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
        await expect(modal.getByRole('listitem')).toHaveText([
          'Rename',
          'Move to',
          'Make a copy',
          'History',
          'Copy link',
          'Details',
          'Delete',
        ]);
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
        await expect(modal.getByRole('listitem')).toHaveText([
          'Preview',
          'Rename',
          'Move to',
          'Make a copy',
          'History',
          'Download',
          'Copy link',
          'Details',
          'Delete',
        ]);
      },
    );

    msTest(
      `Small display document popover on right click in ${gridMode ? 'grid' : 'list'} mode for folder`,
      async ({ documents }, testInfo: TestInfo) => {
        await importDefaultFiles(documents, testInfo, 0, true);
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
        await expect(modal.getByRole('listitem')).toHaveText([
          'Rename',
          'Move to',
          'Make a copy',
          'History',
          'Copy link',
          'Details',
          'Delete',
        ]);
      },
    );

    msTest(
      `Small display popover on right click in ${gridMode ? 'grid' : 'list'} mode for file with editics`,
      async ({ parsecEditics }, testInfo: TestInfo) => {
        await importDefaultFiles(parsecEditics, testInfo, ImportDocuments.Docx, false);
        await parsecEditics.setDisplaySize(DisplaySize.Small);
        await expect(parsecEditics.locator('.file-context-menu')).toBeHidden();
        if (!gridMode) {
          const entry = parsecEditics.locator('.folder-container').locator('.file-list-item').nth(0);
          await entry.click({ button: 'right' });
        } else {
          await toggleViewMode(parsecEditics);
          const entry = parsecEditics.locator('.folder-container').locator('.file-card-item').nth(0);
          await entry.click({ button: 'right' });
        }
        await expect(parsecEditics.locator('.file-context-sheet-modal')).toBeVisible();
        const modal = parsecEditics.locator('.file-context-sheet-modal');
        await expect(modal.getByRole('group')).toHaveCount(2);
        await expect(modal.getByRole('listitem')).toHaveText([
          'Preview',
          'Edit',
          'Rename',
          'Move to',
          'Make a copy',
          'History',
          'Download',
          'Copy link',
          'Details',
          'Delete',
        ]);
      },
    );

    msTest(
      `Small display popover on right click in ${gridMode ? 'grid' : 'list'} mode for file with editics on non-editable file`,
      async ({ parsecEditics }, testInfo: TestInfo) => {
        await importDefaultFiles(parsecEditics, testInfo, ImportDocuments.Png, false);
        await parsecEditics.setDisplaySize(DisplaySize.Small);
        await expect(parsecEditics.locator('.file-context-menu')).toBeHidden();
        if (!gridMode) {
          const entry = parsecEditics.locator('.folder-container').locator('.file-list-item').nth(0);
          await entry.click({ button: 'right' });
        } else {
          await toggleViewMode(parsecEditics);
          const entry = parsecEditics.locator('.folder-container').locator('.file-card-item').nth(0);
          await entry.click({ button: 'right' });
        }
        await expect(parsecEditics.locator('.file-context-sheet-modal')).toBeVisible();
        const modal = parsecEditics.locator('.file-context-sheet-modal');
        await expect(modal.getByRole('group')).toHaveCount(2);
        await expect(modal.getByRole('listitem')).toHaveText([
          'Preview',
          'Rename',
          'Move to',
          'Make a copy',
          'History',
          'Download',
          'Copy link',
          'Details',
          'Delete',
        ]);
      },
    );

    msTest(
      `Small display popover with right click on empty space in ${gridMode ? 'grid' : 'list'} mode`,
      async ({ documents }, testInfo: TestInfo) => {
        await importDefaultFiles(documents, testInfo, ImportDocuments.Png | ImportDocuments.Pdf, false);
        await documents.setDisplaySize(DisplaySize.Small);
        await expect(documents.locator('.folder-global-context-menu')).toBeHidden();

        if (gridMode) {
          await toggleViewMode(documents);
        }
        await documents.locator('.folder-content').locator('.small-display-header-title .title__icon').click();
        await expect(documents.locator('.file-context-sheet-modal')).toBeVisible();
        const modal = documents.locator('.file-context-sheet-modal');
        await expandSheetModal(documents, modal);
        await expect(modal.locator('.button-left')).toHaveText('Selection');
        await expect(modal.locator('.button-right')).toHaveText('Share');
        await expect(modal.locator('.list-group-item')).toHaveCount(1);
        await expect(modal.locator('.list-group-item').nth(0)).toHaveText('Select all');
        await modal.locator('.list-group-item').click();

        const headerElements = documents.locator('.small-display-header-title').locator('ion-text');
        await expect(headerElements).toHaveCount(3);
        await expect(headerElements.nth(0)).toHaveText('Unselect');
        await expect(headerElements.nth(1)).toHaveText('2 selected items');
        await expect(headerElements.nth(2)).toHaveText('Cancel');
        await headerElements.nth(0).click();
        await expect(headerElements.nth(1)).toHaveText('wksp1');
        await expect(headerElements.nth(0)).toBeVisible();
        await expect(headerElements.nth(2)).toBeVisible();
        await headerElements.nth(2).click();
        await expect(headerElements).toHaveCount(1);
        await expect(headerElements.nth(0)).toHaveText('wksp1');
      },
    );

    msTest(
      `Small display document popover on right click on multiple files in ${gridMode ? 'grid' : 'list'} only files`,
      async ({ documents }, testInfo: TestInfo) => {
        await importDefaultFiles(documents, testInfo, ImportDocuments.Png | ImportDocuments.Pdf, true);
        await documents.setDisplaySize(DisplaySize.Small);
        await expect(documents.locator('.file-context-menu')).toBeHidden();
        let entries: Locator;

        if (!gridMode) {
          entries = documents.locator('.folder-container').locator('.file-list-item');
        } else {
          await toggleViewMode(documents);
          entries = documents.locator('.folder-container').locator('.file-card-item');
        }
        await documents.locator('.folder-content').locator('.small-display-header-title .title__icon').click();
        await expandSheetModal(documents, documents.locator('.file-context-sheet-modal'));
        await documents.locator('.file-context-sheet-modal').locator('.list-group-item').nth(0).click();
        await documents.waitForTimeout(500);
        for (const entry of await entries.all()) {
          await expect(entry.locator('ion-checkbox')).toHaveState('checked');
        }

        // Unselect the folder
        entries.nth(0).locator('ion-checkbox').click();
        await entries.nth(1).click({ button: 'right' });

        await expect(documents.locator('.file-context-sheet-modal')).toBeVisible();
        const modal = documents.locator('.file-context-sheet-modal');
        await expect(modal.getByRole('group')).toHaveCount(2);
        await expect(modal.getByRole('listitem')).toHaveText(['Move to', 'Make a copy', 'Download', 'Delete']);
      },
    );

    msTest(
      `Small display document popover on right click on multiple files in ${gridMode ? 'grid' : 'list'} with a folder`,
      async ({ documents }, testInfo: TestInfo) => {
        await importDefaultFiles(documents, testInfo, ImportDocuments.Png | ImportDocuments.Pdf, true);
        await documents.setDisplaySize(DisplaySize.Small);
        await expect(documents.locator('.file-context-menu')).toBeHidden();
        let entries: Locator;

        if (!gridMode) {
          entries = documents.locator('.folder-container').locator('.file-list-item');
        } else {
          await toggleViewMode(documents);
          entries = documents.locator('.folder-container').locator('.file-card-item');
        }
        await documents.locator('.folder-content').locator('.small-display-header-title .title__icon').click();
        await expandSheetModal(documents, documents.locator('.file-context-sheet-modal'));
        await documents.locator('.file-context-sheet-modal').locator('.list-group-item').nth(0).click();
        await documents.waitForTimeout(500);
        for (const entry of await entries.all()) {
          await expect(entry.locator('ion-checkbox')).toHaveState('checked');
        }

        await entries.nth(0).click({ button: 'right' });

        await expect(documents.locator('.file-context-sheet-modal')).toBeVisible();
        const modal = documents.locator('.file-context-sheet-modal');
        await expect(modal.getByRole('group')).toHaveCount(2);
        await expect(modal.getByRole('listitem')).toHaveText(['Move to', 'Make a copy', 'Delete']);
      },
    );

    msTest(`Small display get document link in ${gridMode ? 'grid' : 'list'} mode`, async ({ documents, context }, testInfo: TestInfo) => {
      await importDefaultFiles(documents, testInfo, ImportDocuments.Png, false);
      await documents.setDisplaySize(DisplaySize.Small);
      if (gridMode) {
        await toggleViewMode(documents);
      }
      await clickAction(await openPopover(documents, 0), 'Copy link');

      // Fail to copy because no permission
      await expect(documents).toShowToast('Failed to copy link. Your browser or device does not seem to support copy/paste.', 'Error');

      // Grant the permissions
      await context.grantPermissions(['clipboard-write']);

      await clickAction(await openPopover(documents, 0), 'Copy link');
      await expect(documents).toShowToast('Link has been copied to clipboard.', 'Info');
      const filePath = await documents.evaluate(() => navigator.clipboard.readText());
      expect(filePath).toMatch(/^parsec3:\/\/.+$/);
    });

    msTest(`Small display rename document in ${gridMode ? 'grid' : 'list'} mode`, async ({ documents }, testInfo: TestInfo) => {
      await importDefaultFiles(documents, testInfo, ImportDocuments.Png, false);
      await documents.setDisplaySize(DisplaySize.Small);
      let entry;
      if (gridMode) {
        await toggleViewMode(documents);
        entry = documents.locator('.folder-container').locator('.file-card-item').nth(0);
      } else {
        entry = documents.locator('.folder-container').locator('.file-list-item').nth(0);
      }

      await clickAction(await openPopover(documents, 0), 'Rename');
      await fillInputModal(documents, 'New Name', true);
      if (!gridMode) {
        await expect(entry.locator('.file-name').locator('.label-name')).toHaveText('New Name');
      } else {
        await expect(entry.locator('.file-card__title')).toHaveText('New Name');
      }
    });

    msTest(`Small display delete document in ${gridMode ? 'grid' : 'list'} mode`, async ({ documents }, testInfo: TestInfo) => {
      await importDefaultFiles(documents, testInfo, ImportDocuments.Png | ImportDocuments.Pdf, false);
      await documents.setDisplaySize(DisplaySize.Small);
      if (gridMode) {
        await toggleViewMode(documents);
      }
      let fileName;
      let count = 0;
      if (gridMode) {
        fileName = await documents
          .locator('.folder-container')
          .locator('.file-card-item')
          .nth(0)
          .locator('.file-card__title')
          .textContent();
        count = await documents.locator('.folder-container').locator('.file-card-item').count();
      } else {
        fileName = await documents
          .locator('.folder-container')
          .locator('.file-list-item')
          .nth(0)
          .locator('.file-name')
          .locator('.label-name')
          .textContent();
        count = await documents.locator('.folder-container').locator('.file-list-item').count();
      }

      await clickAction(await openPopover(documents, 0), 'Delete');

      await answerQuestion(documents, true, {
        expectedTitleText: 'Delete one file',
        expectedQuestionText: `Are you sure you want to delete file \`${fileName}\`?`,
        expectedNegativeText: 'Keep file',
        expectedPositiveText: 'Delete file',
      });
      if (gridMode) {
        await expect(documents.locator('.folder-container').locator('.file-card-item')).toHaveCount(count - 1);
      } else {
        await expect(documents.locator('.folder-container').locator('.file-list-item')).toHaveCount(count - 1);
      }
    });

    msTest(
      `Small display folder actions default state in a read only workspace in ${gridMode ? 'grid' : 'list'} mode`,
      async ({ documentsReadOnly }) => {
        await documentsReadOnly.setDisplaySize(DisplaySize.Small);
        await expect(documentsReadOnly.locator('.file-context-menu')).toBeHidden();
        if (!gridMode) {
          const entry = documentsReadOnly.locator('.folder-container').locator('.file-list-item').nth(0);
          await entry.hover();
          await entry.locator('.options-button').click();
        } else {
          await toggleViewMode(documentsReadOnly);
          const entry = documentsReadOnly.locator('.folder-container').locator('.file-card-item').nth(0);
          await entry.hover();
          await entry.locator('.card-option').click();
        }
        await expect(documentsReadOnly.locator('.file-context-sheet-modal')).toBeVisible();
        const modal = documentsReadOnly.locator('.file-context-sheet-modal');
        await expect(modal.getByRole('group')).toHaveCount(1);
        await expect(modal.getByRole('listitem')).toHaveText(['Copy link', 'Details']);
      },
    );

    msTest(
      `Small display file actions default state in a read only workspace in ${gridMode ? 'grid' : 'list'} mode`,
      async ({ documentsReadOnly }) => {
        await documentsReadOnly.setDisplaySize(DisplaySize.Small);
        await expect(documentsReadOnly.locator('.file-context-menu')).toBeHidden();
        if (!gridMode) {
          const entry = documentsReadOnly.locator('.folder-container').locator('.file-list-item').nth(1);
          await entry.hover();
          await entry.locator('.options-button').click();
        } else {
          await toggleViewMode(documentsReadOnly);
          const entry = documentsReadOnly.locator('.folder-container').locator('.file-card-item').nth(1);
          await entry.hover();
          await entry.locator('.card-option').click();
        }
        await expect(documentsReadOnly.locator('.file-context-sheet-modal')).toBeVisible();
        const modal = documentsReadOnly.locator('.file-context-sheet-modal');
        await expect(modal.getByRole('group')).toHaveCount(1);
        await expect(modal.getByRole('listitem')).toHaveText(['Preview', 'Download', 'Copy link', 'Details']);
      },
    );

    msTest(`Small display move document in ${gridMode ? 'grid' : 'list'} mode`, async ({ documents }, testInfo: TestInfo) => {
      await importDefaultFiles(documents, testInfo, ImportDocuments.Png, true);
      await documents.setDisplaySize(DisplaySize.Small);
      if (gridMode) {
        await toggleViewMode(documents);
      }
      await expect(documents.locator('.folder-selection-modal')).toBeHidden();
      await clickAction(await openPopover(documents, 1), 'Move to');
      const modal = documents.locator('.folder-selection-modal');
      await expect(modal).toBeVisible();
      await expect(modal.locator('.ms-modal-header__title')).toHaveText('Move one item');
      const okButton = modal.locator('#next-button');
      await expect(okButton).toHaveText('Move here');
      await expect(okButton).toHaveDisabledAttribute();
      await modal.locator('.folder-container').getByRole('listitem').nth(0).click();
      await expect(modal.locator('.current-folder__text')).toHaveText(/^Dir_[A-Za-z_]+$/);
      await expect(okButton).toNotHaveDisabledAttribute();
      await okButton.click();
      await expect(modal).toBeHidden();
    });
  }
});
