// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { Locator, Page } from '@playwright/test';
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

async function openPopover(page: Page): Promise<Locator> {
  if (await isInGridMode(page)) {
    const entry = page.locator('.folder-container').locator('.file-card-item').nth(2);
    await entry.hover();
    await entry.locator('.card-option').click();
  } else {
    const entry = page.locator('.folder-container').locator('.file-list-item').nth(2);
    await entry.hover();
    await entry.locator('.options-button').click();
  }
  return page.locator('.file-context-menu');
}

async function clickAction(popover: Locator, action: string): Promise<void> {
  await popover.getByRole('listitem').filter({ hasText: action }).click();
}

for (const gridMode of [false, true]) {
  msTest(`Document actions default state in ${gridMode ? 'grid' : 'list'} mode`, async ({ documents }) => {
    await expect(documents.locator('.file-context-menu')).toBeHidden();
    if (!gridMode) {
      const entry = documents.locator('.folder-container').locator('.file-list-item').nth(2);
      await entry.hover();
      await entry.locator('.options-button').click();
    } else {
      await toggleViewMode(documents);
      const entry = documents.locator('.folder-container').locator('.file-card-item').nth(2);
      await entry.hover();
      await entry.locator('.card-option').click();
    }
    await expect(documents.locator('.file-context-menu')).toBeVisible();
    const popover = documents.locator('.file-context-menu');
    await expect(popover.getByRole('group')).toHaveCount(2);
    await expect(popover.getByRole('listitem')).toHaveCount(7);
    await expect(popover.getByRole('listitem')).toHaveText([
      'Manage file',
      'Rename',
      'Move to',
      'Delete',
      'Details',
      'Collaboration',
      'Copy link',
    ]);
  });

  msTest(`Get document link in ${gridMode ? 'grid' : 'list'} mode`, async ({ documents, context }) => {
    if (gridMode) {
      await toggleViewMode(documents);
    }
    await clickAction(await openPopover(documents), 'Copy link');

    // Fail to copy because no permission
    await expect(documents).toShowToast('Failed to copy link. Your browser or device does not seem to support copy/paste.', 'Error');

    // Grant the permissions
    await context.grantPermissions(['clipboard-write']);

    await clickAction(await openPopover(documents), 'Copy link');
    await expect(documents).toShowToast('Link has been copied to clipboard.', 'Info');
    const filePath = await documents.evaluate(() => navigator.clipboard.readText());
    expect(filePath).toMatch(/parsec3:\/\/[a-z.]+(:\d+)?\/[a-zA-Z0-9_]+\?.+/);
  });

  msTest(`Rename document in ${gridMode ? 'grid' : 'list'} mode`, async ({ documents }) => {
    if (gridMode) {
      await toggleViewMode(documents);
    }
    await clickAction(await openPopover(documents), 'Rename');
    await fillInputModal(documents, 'My file', true);
    if (!gridMode) {
      const entry = documents.locator('.folder-container').locator('.file-list-item').nth(2);
      await expect(entry.locator('.file-name').locator('.file-name__label')).toHaveText('My file');
    } else {
      const entry = documents.locator('.folder-container').locator('.file-card-item').nth(2);
      await expect(entry.locator('.card-content__title')).toHaveText('My file');
    }
  });

  msTest(`Delete document in ${gridMode ? 'grid' : 'list'} mode`, async ({ documents }) => {
    if (gridMode) {
      await toggleViewMode(documents);
    }
    let fileName;
    if (gridMode) {
      fileName = await documents
        .locator('.folder-container')
        .locator('.file-card-item')
        .nth(2)
        .locator('.card-content__title')
        .textContent();
    } else {
      fileName = await documents
        .locator('.folder-container')
        .locator('.file-list-item')
        .nth(2)
        .locator('.file-name')
        .locator('.file-name__label')
        .textContent();
    }

    await clickAction(await openPopover(documents), 'Delete');

    await answerQuestion(documents, true, {
      expectedTitleText: 'Delete one file',
      expectedQuestionText: `Are you sure you want to delete file \`${fileName}\`?`,
      expectedNegativeText: 'Keep file',
      expectedPositiveText: 'Delete file',
    });
  });

  msTest(`Document actions default state in a read only workspace in ${gridMode ? 'grid' : 'list'} mode`, async ({ documentsReadOnly }) => {
    await expect(documentsReadOnly.locator('.file-context-menu')).toBeHidden();
    if (!gridMode) {
      const entry = documentsReadOnly.locator('.folder-container').locator('.file-list-item').nth(2);
      await entry.hover();
      await entry.locator('.options-button').click();
    } else {
      await toggleViewMode(documentsReadOnly);
      const entry = documentsReadOnly.locator('.folder-container').locator('.file-card-item').nth(2);
      await entry.hover();
      await entry.locator('.card-option').click();
    }
    await expect(documentsReadOnly.locator('.file-context-menu')).toBeVisible();
    const popover = documentsReadOnly.locator('.file-context-menu');
    await expect(popover.getByRole('group')).toHaveCount(2);
    await expect(popover.getByRole('listitem')).toHaveCount(4);
    await expect(popover.getByRole('listitem')).toHaveText(['Manage file', 'Details', 'Collaboration', 'Copy link']);
  });

  msTest(`Move document in ${gridMode ? 'grid' : 'list'} mode`, async ({ documents }) => {
    if (gridMode) {
      await toggleViewMode(documents);
    }
    await expect(documents.locator('.folder-selection-modal')).toBeHidden();
    await clickAction(await openPopover(documents), 'Move to');
    const modal = documents.locator('.folder-selection-modal');
    await expect(modal).toBeVisible();
    await expect(modal.locator('.ms-modal-header__title')).toHaveText('Move one item');
    const okButton = modal.locator('#next-button');
    await expect(okButton).toHaveText('Move here');
    await expect(okButton).toHaveDisabledAttribute();
    await modal.locator('.folder-container').getByRole('listitem').nth(0).click();
    await expect(okButton).not.toHaveDisabledAttribute();
    await okButton.click();
    await expect(modal).toBeHidden();
  });
}
