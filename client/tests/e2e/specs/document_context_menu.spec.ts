// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { Locator, Page } from '@playwright/test';
import { answerQuestion, expandSheetModal, expect, fillInputModal, msTest, setSmallDisplay } from '@tests/e2e/helpers';

async function isInGridMode(page: Page, smallDisplay?: boolean): Promise<boolean> {
  return (
    (await page
      .locator(smallDisplay ? '.action-bar-mobile-filter' : '#folders-ms-action-bar')
      .locator('#grid-view')
      .getAttribute('disabled')) !== null
  );
}

async function toggleViewMode(page: Page, smallDisplay?: boolean): Promise<void> {
  const locator = smallDisplay ? '.action-bar-mobile-filter' : '#folders-ms-action-bar';
  if (await isInGridMode(page, smallDisplay)) {
    await page.locator(locator).locator('#list-view').click();
  } else {
    await page.locator(locator).locator('#grid-view').click();
  }
}

async function openPopover(page: Page, index: number, smallDisplay?: boolean): Promise<Locator> {
  if (await isInGridMode(page, smallDisplay)) {
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

for (const gridMode of [false, true]) {
  msTest(`Document actions default state in ${gridMode ? 'grid' : 'list'} mode for file`, async ({ documents }) => {
    await expect(documents.locator('.file-context-menu')).toBeHidden();
    if (!gridMode) {
      const entry = documents.locator('.folder-container').locator('.file-list-item').nth(1);
      await entry.hover();
      await entry.locator('.options-button').click();
    } else {
      await toggleViewMode(documents);
      const entry = documents.locator('.folder-container').locator('.file-card-item').nth(1);
      await entry.hover();
      await entry.locator('.card-option').click();
    }
    await expect(documents.locator('.file-context-menu')).toBeVisible();
    const popover = documents.locator('.file-context-menu');
    await expect(popover.getByRole('group')).toHaveCount(2);
    await expect(popover.getByRole('listitem')).toHaveText([
      'Manage file',
      'Rename',
      'Move to',
      'Make a copy',
      'Delete',
      'History',
      'Download',
      'Details',
      'Collaboration',
      'Copy link',
    ]);
  });

  msTest(`Document actions default state in ${gridMode ? 'grid' : 'list'} mode for folder`, async ({ documents }) => {
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
      'Manage file',
      'Rename',
      'Move to',
      'Make a copy',
      'Delete',
      'History',
      'Details',
      'Collaboration',
      'Copy link',
    ]);
  });

  msTest(`Document popover on right click in ${gridMode ? 'grid' : 'list'} mode for file`, async ({ documents }) => {
    await expect(documents.locator('.file-context-menu')).toBeHidden();
    if (!gridMode) {
      const entry = documents.locator('.folder-container').locator('.file-list-item').nth(1);
      await entry.click({ button: 'right' });
    } else {
      await toggleViewMode(documents);
      const entry = documents.locator('.folder-container').locator('.file-card-item').nth(1);
      await entry.click({ button: 'right' });
    }
    await expect(documents.locator('.file-context-menu')).toBeVisible();
    const popover = documents.locator('.file-context-menu');
    await expect(popover.getByRole('group')).toHaveCount(2);
    await expect(popover.getByRole('listitem')).toHaveText([
      'Manage file',
      'Rename',
      'Move to',
      'Make a copy',
      'Delete',
      'History',
      'Download',
      'Details',
      'Collaboration',
      'Copy link',
    ]);
  });

  msTest(`Document popover on right click in ${gridMode ? 'grid' : 'list'} mode for folder`, async ({ documents }) => {
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
      'Manage file',
      'Rename',
      'Move to',
      'Make a copy',
      'Delete',
      'History',
      'Details',
      'Collaboration',
      'Copy link',
    ]);
  });

  msTest(`Document popover on right click on multiple files in ${gridMode ? 'grid' : 'list'} only files`, async ({ documents }) => {
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
      await entry.locator('ion-checkbox').click();
      await expect(entry.locator('ion-checkbox')).toHaveState('checked');
    }
    // Unselect the folder
    entries.nth(0).locator('ion-checkbox').click();
    await entries.nth(1).click({ button: 'right' });

    await expect(documents.locator('.file-context-menu')).toBeVisible();
    const popover = documents.locator('.file-context-menu');
    await expect(popover.getByRole('group')).toHaveCount(1);
    await expect(popover.getByRole('listitem')).toHaveText(['Manage file', 'Move to', 'Make a copy', 'Delete', 'Download']);
  });

  msTest(`Document popover on right click on multiple files in ${gridMode ? 'grid' : 'list'} with a folder`, async ({ documents }) => {
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
      await entry.locator('ion-checkbox').click();
      await expect(entry.locator('ion-checkbox')).toHaveState('checked');
    }
    await entries.nth(0).click({ button: 'right' });

    await expect(documents.locator('.file-context-menu')).toBeVisible();
    const popover = documents.locator('.file-context-menu');
    await expect(popover.getByRole('group')).toHaveCount(1);
    await expect(popover.getByRole('listitem')).toHaveText(['Manage file', 'Move to', 'Make a copy', 'Delete']);
  });

  msTest(`Popover with right click on empty space in ${gridMode ? 'grid' : 'list'} mode`, async ({ documents }) => {
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

  msTest(`Get document link in ${gridMode ? 'grid' : 'list'} mode`, async ({ documents, context }) => {
    if (gridMode) {
      await toggleViewMode(documents);
    }
    await clickAction(await openPopover(documents, 2), 'Copy link');

    // Fail to copy because no permission
    await expect(documents).toShowToast('Failed to copy link. Your browser or device does not seem to support copy/paste.', 'Error');

    // Grant the permissions
    await context.grantPermissions(['clipboard-write']);

    await clickAction(await openPopover(documents, 2), 'Copy link');
    await expect(documents).toShowToast('Link has been copied to clipboard.', 'Info');
    const filePath = await documents.evaluate(() => navigator.clipboard.readText());
    expect(filePath).toMatch(/^parsec3:\/\/.+$/);
  });

  msTest(`Rename document in ${gridMode ? 'grid' : 'list'} mode`, async ({ documents }) => {
    let entry;
    if (gridMode) {
      await toggleViewMode(documents);
      entry = documents.locator('.folder-container').locator('.file-card-item').nth(1);
    } else {
      entry = documents.locator('.folder-container').locator('.file-list-item').nth(1);
    }

    await clickAction(await openPopover(documents, 1), 'Rename');
    await fillInputModal(documents, 'New Name', true);
    if (!gridMode) {
      await expect(entry.locator('.file-name').locator('.file-name__label')).toHaveText('New Name');
    } else {
      await expect(entry.locator('.file-card__title')).toHaveText('New Name');
    }
  });

  msTest(`Delete document in ${gridMode ? 'grid' : 'list'} mode`, async ({ documents }) => {
    if (gridMode) {
      await toggleViewMode(documents);
    }
    let fileName;
    let count = 0;
    if (gridMode) {
      fileName = await documents.locator('.folder-container').locator('.file-card-item').nth(2).locator('.file-card__title').textContent();
      count = await documents.locator('.folder-container').locator('.file-card-item').count();
    } else {
      fileName = await documents
        .locator('.folder-container')
        .locator('.file-list-item')
        .nth(2)
        .locator('.file-name')
        .locator('.file-name__label')
        .textContent();
      count = await documents.locator('.folder-container').locator('.file-list-item').count();
    }

    await clickAction(await openPopover(documents, 2), 'Delete');

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
    await expect(popover.getByRole('listitem')).toHaveText(['Manage file', 'Details', 'Collaboration', 'Copy link']);
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
    await expect(popover.getByRole('listitem')).toHaveText(['Manage file', 'Download', 'Details', 'Collaboration', 'Copy link']);
  });

  msTest(`Move document in ${gridMode ? 'grid' : 'list'} mode`, async ({ documents }) => {
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
    await expect(okButton).toNotHaveDisabledAttribute();
    await okButton.click();
    await expect(modal).toBeHidden();
  });
}

for (const gridMode of [false, true]) {
  msTest(`Small display document actions default state in ${gridMode ? 'grid' : 'list'} mode for file`, async ({ documents }) => {
    await setSmallDisplay(documents);
    await expect(documents.locator('.file-context-menu')).toBeHidden();
    if (!gridMode) {
      const entry = documents.locator('.folder-container').locator('.file-list-item').nth(1);
      await entry.hover();
      await entry.locator('.options-button').click();
    } else {
      await toggleViewMode(documents, true);
      const entry = documents.locator('.folder-container').locator('.file-card-item').nth(1);
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
      'Download',
      'Copy link',
      'Details',
      'Delete',
    ]);
  });

  msTest(`Small display document actions default state in ${gridMode ? 'grid' : 'list'} mode for folder`, async ({ documents }) => {
    await setSmallDisplay(documents);
    await expect(documents.locator('.file-context-menu')).toBeHidden();
    if (!gridMode) {
      const entry = documents.locator('.folder-container').locator('.file-list-item').nth(0);
      await entry.hover();
      await entry.locator('.options-button').click();
    } else {
      await toggleViewMode(documents, true);
      const entry = documents.locator('.folder-container').locator('.file-card-item').nth(0);
      await entry.hover();
      await entry.locator('.card-option').click();
    }
    await expect(documents.locator('.file-context-sheet-modal')).toBeVisible();
    const modal = documents.locator('.file-context-sheet-modal');
    await expect(modal.getByRole('group')).toHaveCount(2);
    await expect(modal.getByRole('listitem')).toHaveText(['Rename', 'Move to', 'Make a copy', 'History', 'Copy link', 'Details', 'Delete']);
  });

  msTest(`Small display document popover on right click in ${gridMode ? 'grid' : 'list'} mode for file`, async ({ documents }) => {
    await setSmallDisplay(documents);
    await expect(documents.locator('.file-context-menu')).toBeHidden();
    if (!gridMode) {
      const entry = documents.locator('.folder-container').locator('.file-list-item').nth(1);
      await entry.click({ button: 'right' });
    } else {
      await toggleViewMode(documents, true);
      const entry = documents.locator('.folder-container').locator('.file-card-item').nth(1);
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
      'Download',
      'Copy link',
      'Details',
      'Delete',
    ]);
  });

  msTest(`Small display document popover on right click in ${gridMode ? 'grid' : 'list'} mode for folder`, async ({ documents }) => {
    await setSmallDisplay(documents);
    await expect(documents.locator('.file-context-menu')).toBeHidden();
    if (!gridMode) {
      const entry = documents.locator('.folder-container').locator('.file-list-item').nth(0);
      await entry.click({ button: 'right' });
    } else {
      await toggleViewMode(documents, true);
      const entry = documents.locator('.folder-container').locator('.file-card-item').nth(0);
      await entry.click({ button: 'right' });
    }
    await expect(documents.locator('.file-context-sheet-modal')).toBeVisible();
    const modal = documents.locator('.file-context-sheet-modal');
    await expect(modal.getByRole('group')).toHaveCount(2);
    await expect(modal.getByRole('listitem')).toHaveText(['Rename', 'Move to', 'Make a copy', 'History', 'Copy link', 'Details', 'Delete']);
  });

  msTest(`Small display popover with right click on empty space in ${gridMode ? 'grid' : 'list'} mode`, async ({ documents }) => {
    await setSmallDisplay(documents);
    await expect(documents.locator('.folder-global-context-menu')).toBeHidden();

    if (gridMode) {
      await toggleViewMode(documents, true);
    }
    await documents.locator('.folder-container').click({ button: 'right', position: { x: 100, y: 10 } });
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
    await expect(headerElements.nth(1)).toHaveText('9 selected items');
    await expect(headerElements.nth(2)).toHaveText('Cancel');
    await headerElements.nth(0).click();
    await expect(headerElements.nth(1)).toHaveText('wksp1');
    await expect(headerElements.nth(0)).toBeVisible();
    await expect(headerElements.nth(2)).toBeVisible();
    await headerElements.nth(2).click();
    await expect(headerElements).toHaveCount(1);
    await expect(headerElements.nth(0)).toHaveText('wksp1');
  });

  msTest(
    `Small display document popover on right click on multiple files in ${gridMode ? 'grid' : 'list'} only files`,
    async ({ documents }) => {
      await setSmallDisplay(documents);
      await expect(documents.locator('.file-context-menu')).toBeHidden();
      let entries: Locator;

      if (!gridMode) {
        entries = documents.locator('.folder-container').locator('.file-list-item');
      } else {
        await toggleViewMode(documents, true);
        entries = documents.locator('.folder-container').locator('.file-card-item');
      }
      await documents.locator('.folder-container').click({ button: 'right', position: { x: 100, y: 10 } });
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
    async ({ documents }) => {
      await setSmallDisplay(documents);
      await expect(documents.locator('.file-context-menu')).toBeHidden();
      let entries: Locator;

      if (!gridMode) {
        entries = documents.locator('.folder-container').locator('.file-list-item');
      } else {
        await toggleViewMode(documents, true);
        entries = documents.locator('.folder-container').locator('.file-card-item');
      }
      await documents.locator('.folder-container').click({ button: 'right', position: { x: 100, y: 10 } });
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

  msTest(`Small display get document link in ${gridMode ? 'grid' : 'list'} mode`, async ({ documents, context }) => {
    await setSmallDisplay(documents);
    if (gridMode) {
      await toggleViewMode(documents, true);
    }
    await clickAction(await openPopover(documents, 1, true), 'Copy link');

    // Fail to copy because no permission
    await expect(documents).toShowToast('Failed to copy link. Your browser or device does not seem to support copy/paste.', 'Error');

    // Grant the permissions
    await context.grantPermissions(['clipboard-write']);

    await clickAction(await openPopover(documents, 1, true), 'Copy link');
    await expect(documents).toShowToast('Link has been copied to clipboard.', 'Info');
    const filePath = await documents.evaluate(() => navigator.clipboard.readText());
    expect(filePath).toMatch(/^parsec3:\/\/.+$/);
  });

  msTest(`Small display rename document in ${gridMode ? 'grid' : 'list'} mode`, async ({ documents }) => {
    await setSmallDisplay(documents);
    let entry;
    if (gridMode) {
      await toggleViewMode(documents, true);
      entry = documents.locator('.folder-container').locator('.file-card-item').nth(1);
    } else {
      entry = documents.locator('.folder-container').locator('.file-list-item').nth(1);
    }

    await clickAction(await openPopover(documents, 1, true), 'Rename');
    await fillInputModal(documents, 'New Name', true);
    if (!gridMode) {
      await expect(entry.locator('.file-name').locator('.file-name__label')).toHaveText('New Name');
    } else {
      await expect(entry.locator('.file-card__title')).toHaveText('New Name');
    }
  });

  msTest(`Small display delete document in ${gridMode ? 'grid' : 'list'} mode`, async ({ documents }) => {
    await setSmallDisplay(documents);
    if (gridMode) {
      await toggleViewMode(documents, true);
    }
    let fileName;
    let count = 0;
    if (gridMode) {
      fileName = await documents.locator('.folder-container').locator('.file-card-item').nth(2).locator('.file-card__title').textContent();
      count = await documents.locator('.folder-container').locator('.file-card-item').count();
    } else {
      fileName = await documents
        .locator('.folder-container')
        .locator('.file-list-item')
        .nth(2)
        .locator('.file-name')
        .locator('.file-name__label')
        .textContent();
      count = await documents.locator('.folder-container').locator('.file-list-item').count();
    }

    await clickAction(await openPopover(documents, 2, true), 'Delete');

    await answerQuestion(
      documents,
      true,
      {
        expectedTitleText: 'Delete one file',
        expectedQuestionText: `Are you sure you want to delete file \`${fileName}\`?`,
        expectedNegativeText: 'Keep file',
        expectedPositiveText: 'Delete file',
      },
      true,
    );
    if (gridMode) {
      await expect(documents.locator('.folder-container').locator('.file-card-item')).toHaveCount(count - 1);
    } else {
      await expect(documents.locator('.folder-container').locator('.file-list-item')).toHaveCount(count - 1);
    }
  });

  msTest(
    `Small display folder actions default state in a read only workspace in ${gridMode ? 'grid' : 'list'} mode`,
    async ({ documentsReadOnly }) => {
      await setSmallDisplay(documentsReadOnly);
      await expect(documentsReadOnly.locator('.file-context-menu')).toBeHidden();
      if (!gridMode) {
        const entry = documentsReadOnly.locator('.folder-container').locator('.file-list-item').nth(0);
        await entry.hover();
        await entry.locator('.options-button').click();
      } else {
        await toggleViewMode(documentsReadOnly, true);
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
      await setSmallDisplay(documentsReadOnly);
      await expect(documentsReadOnly.locator('.file-context-menu')).toBeHidden();
      if (!gridMode) {
        const entry = documentsReadOnly.locator('.folder-container').locator('.file-list-item').nth(1);
        await entry.hover();
        await entry.locator('.options-button').click();
      } else {
        await toggleViewMode(documentsReadOnly, true);
        const entry = documentsReadOnly.locator('.folder-container').locator('.file-card-item').nth(1);
        await entry.hover();
        await entry.locator('.card-option').click();
      }
      await expect(documentsReadOnly.locator('.file-context-sheet-modal')).toBeVisible();
      const modal = documentsReadOnly.locator('.file-context-sheet-modal');
      await expect(modal.getByRole('group')).toHaveCount(1);
      await expect(modal.getByRole('listitem')).toHaveText(['Download', 'Copy link', 'Details']);
    },
  );

  msTest(`Small display move document in ${gridMode ? 'grid' : 'list'} mode`, async ({ documents }) => {
    await setSmallDisplay(documents);
    if (gridMode) {
      await toggleViewMode(documents, true);
    }
    await expect(documents.locator('.folder-selection-modal')).toBeHidden();
    await clickAction(await openPopover(documents, 1, true), 'Move to');
    const modal = documents.locator('.folder-selection-modal');
    await expect(modal).toBeVisible();
    await expect(modal.locator('.ms-modal-header__title')).toHaveText('Move one item');
    const okButton = modal.locator('#next-button');
    await expect(okButton).toHaveText('Move here');
    await expect(okButton).toHaveDisabledAttribute();
    await modal.locator('.folder-container').getByRole('listitem').nth(0).click();
    await expect(okButton).toNotHaveDisabledAttribute();
    await okButton.click();
    await expect(modal).toBeHidden();
  });
}
