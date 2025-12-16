// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { Locator } from '@playwright/test';
import { DisplaySize, expandSheetModal, MsPage } from '@tests/e2e/helpers';

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

async function openActionPopover(page: MsPage, index: number): Promise<Locator> {
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

export const DocumentsPage = {
  isInGridMode,
  toggleViewMode,
  openActionPopover,
  clickAction,
};
