// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { Page } from '@playwright/test';
import { DisplaySize, expect, msTest } from '@tests/e2e/helpers';

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

msTest('Files options tab menu display', async ({ documents }) => {
  await toggleViewMode(documents);
  await documents.setDisplaySize(DisplaySize.Small);
  const optionsTab = documents.locator('#tab-bar-options');
  const optionsTabModal = documents.locator('#tab-bar-options-modal');
  await expect(optionsTab).not.toBeVisible();
  await expect(optionsTabModal).not.toBeVisible();
  const entryFolder = documents.locator('.folder-container').locator('.folder-grid-item').nth(0);
  const entryFile = documents.locator('.folder-container').locator('.folder-grid-item').nth(1);

  // With 1 folder selected
  await entryFolder.hover();
  await entryFolder.locator('ion-checkbox').click();
  await expect(optionsTab).toBeVisible();
  await expect(optionsTab.locator('.tab-bar-menu-button')).toHaveCount(4);
  await expect(optionsTab.locator('.tab-bar-menu-button')).toHaveText(['Rename', 'Move', 'Delete', 'More']);
  await optionsTab.locator('.tab-bar-menu-button').nth(3).click();
  await expect(optionsTab.locator('.tab-bar-menu-button')).toHaveText(['Rename', 'Move', 'Delete', 'Less']);
  await expect(optionsTabModal).toBeVisible();
  await expect(optionsTabModal.locator('.tab-bar-menu-button')).toHaveCount(4);
  await expect(optionsTabModal.locator('.tab-bar-menu-button')).toHaveText(['Copy', 'Copy link', 'History', 'Details']);
  await optionsTab.locator('.tab-bar-menu-button').nth(3).click();
  await expect(optionsTabModal).not.toBeVisible();

  // With file + folder selected
  await entryFile.locator('ion-checkbox').click();
  await expect(optionsTab.locator('.tab-bar-menu-button')).toHaveCount(4);
  await expect(optionsTab.locator('.tab-bar-menu-button')).toHaveText(['Copy', 'Move', 'Delete', 'Download']);

  // With 1 file selected
  await entryFolder.locator('ion-checkbox').click();
  await expect(optionsTab.locator('.tab-bar-menu-button')).toHaveCount(4);
  await expect(optionsTab.locator('.tab-bar-menu-button')).toHaveText(['Rename', 'Move', 'Delete', 'More']);
  await optionsTab.locator('.tab-bar-menu-button').nth(3).click();
  await expect(optionsTab.locator('.tab-bar-menu-button')).toHaveText(['Rename', 'Move', 'Delete', 'Less']);
  await expect(optionsTabModal).toBeVisible();
  await expect(optionsTabModal.locator('.tab-bar-menu-button')).toHaveCount(6);
  await expect(optionsTabModal.locator('.tab-bar-menu-button')).toHaveText(['Open', 'Download', 'Copy', 'Copy link', 'History', 'Details']);

  // Nothing selected, menu goes away
  await optionsTab.locator('.tab-bar-menu-button').nth(3).click();
  await expect(optionsTabModal).not.toBeVisible();
});

msTest('Test files options tab menu', async ({ documents, context }) => {
  await toggleViewMode(documents);
  await documents.setDisplaySize(DisplaySize.Small);
  await context.grantPermissions(['clipboard-write']);

  const optionsTab = documents.locator('#tab-bar-options');
  const optionsTabModal = documents.locator('#tab-bar-options-modal');
  const entryFile = documents.locator('.folder-container').locator('.folder-grid-item').nth(1);
  await entryFile.hover();
  await entryFile.locator('ion-checkbox').click();
  await expect(optionsTab).toBeVisible();
  const tabItem = optionsTab.locator('.tab-bar-menu-button');
  await expect(tabItem).toHaveCount(4);
  await expect(tabItem).toHaveText(['Rename', 'Move', 'Delete', 'More']);

  // `Rename` button
  await tabItem.nth(0).click();
  await expect(documents.locator('.text-input-modal')).toBeVisible();
  await expect(documents.locator('.text-input-modal').locator('ion-text')).toHaveText('Rename a file');
  await documents.locator('.text-input-modal').locator('.closeBtn').click();

  // `Move` button
  await tabItem.nth(1).click();
  await expect(documents.locator('.folder-selection-modal')).toBeVisible();
  await expect(documents.locator('.folder-selection-modal').locator('.ms-modal-header__title')).toHaveText('Move one item');
  await documents.locator('.folder-selection-modal').locator('.closeBtn').click();

  // `Delete` button
  await tabItem.nth(2).click();
  await expect(documents.locator('.question-modal')).toBeVisible();
  await expect(documents.locator('.question-modal').locator('.ms-small-display-modal-header__title')).toHaveText('Delete one file');
  await documents.locator('.question-modal').locator('.ms-small-display-modal-footer-buttons-cancel').click();

  // `More` button
  await tabItem.nth(3).click();
  await expect(optionsTabModal).toBeVisible();
  const tabModalItem = optionsTabModal.locator('.tab-bar-menu-button');
  await expect(tabModalItem).toHaveCount(6);
  await expect(tabModalItem).toHaveText(['Open', 'Download', 'Copy', 'Copy link', 'History', 'Details']);

  // `Open` button
  await tabModalItem.nth(0).click();
  await documents.waitForTimeout(1000);
  await expect(documents.locator('.file-viewer-topbar')).toBeVisible();
  await expect(documents.locator('.file-viewer-topbar').locator('.file-viewer-topbar__title')).toHaveText('audio.mp3');

  // Ensure the main header is visible
  const isTopbarVisible = await documents.locator('#connected-header .topbar').isVisible();
  const fileViewerButton = documents.locator('.file-viewer-topbar-buttons__item.toggle-menu');
  if (!isTopbarVisible && fileViewerButton) {
    await fileViewerButton.click();
  }
  await documents.locator('#connected-header').locator('.topbar-left').locator('.back-button-container').click();

  // `Copy` button
  await entryFile.hover();
  await entryFile.locator('ion-checkbox').click();
  await tabItem.nth(3).click();
  await expect(optionsTabModal).toBeVisible();
  await tabModalItem.nth(2).click();
  await expect(documents.locator('.folder-selection-modal')).toBeVisible();
  await expect(documents.locator('.folder-selection-modal').locator('.ms-modal-header__title')).toHaveText('Copy one item');
  await documents.locator('.folder-selection-modal').locator('.closeBtn').click();

  // `Copy link` button
  await tabItem.nth(3).click();
  await expect(optionsTabModal).toBeVisible();
  await tabModalItem.nth(3).click();
  await expect(documents).toShowToast('Link has been copied to clipboard.', 'Info');
  const filePath = await documents.evaluate(() => navigator.clipboard.readText());
  expect(filePath).toMatch(/^parsec3:\/\/.+$/);

  // `History` button
  await tabItem.nth(3).click();
  await expect(optionsTabModal).toBeVisible();
  await tabModalItem.nth(4).click();
  await expect(documents.locator('.history-container')).toBeVisible();
  await expect(documents.locator('.history-container').locator('.current-folder__text')).toHaveText('wksp1');
  await documents.locator('#connected-header').locator('.topbar-left').locator('.back-button-container').click();

  // `Details` button
  await entryFile.hover();
  await entryFile.locator('ion-checkbox').click();
  await tabItem.nth(3).click();
  await expect(optionsTabModal).toBeVisible();
  await tabModalItem.nth(5).click();
  await expect(documents.locator('.file-details-modal')).toBeVisible();
  await expect(documents.locator('.file-details-modal').locator('.ms-modal-header__title')).toHaveText('Details on audio.mp3');
});

msTest('Users options tab menu display', async ({ usersPage }) => {
  await usersPage.setDisplaySize(DisplaySize.Small);
  const optionsTab = usersPage.locator('#tab-bar-options');
  await expect(optionsTab).not.toBeVisible();
  const user1 = usersPage.locator('#users-page-user-list').getByRole('listitem').nth(1);
  const user2 = usersPage.locator('#users-page-user-list').getByRole('listitem').nth(2);

  // With 1 External selected
  await user2.hover();
  await user2.locator('ion-checkbox').click();
  await expect(optionsTab).toBeVisible();
  await expect(optionsTab.locator('.tab-bar-menu-button')).toHaveCount(3);
  await expect(optionsTab.locator('.tab-bar-menu-button')).toHaveText(['Details', 'Roles', 'Revoke']);

  // With External + Standard selected
  await user1.locator('ion-checkbox').click();
  await expect(optionsTab.locator('.tab-bar-menu-button')).toHaveCount(2);
  await expect(optionsTab.locator('.tab-bar-menu-button')).toHaveText(['Profile', 'Revoke']);

  // With 1 Standard selected
  await user2.locator('ion-checkbox').click();
  await expect(optionsTab.locator('.tab-bar-menu-button')).toHaveCount(4);
  await expect(optionsTab.locator('.tab-bar-menu-button')).toHaveText(['Details', 'Roles', 'Profile', 'Revoke']);

  // Nothing selected, menu goes away
  await user1.locator('ion-checkbox').click();
  await expect(optionsTab).not.toBeVisible();
});

msTest('Test user options tab menu', async ({ usersPage }) => {
  await usersPage.setDisplaySize(DisplaySize.Small);
  const optionsTab = usersPage.locator('#tab-bar-options');
  const user1 = usersPage.locator('#users-page-user-list').getByRole('listitem').nth(1);
  await user1.hover();
  await user1.locator('ion-checkbox').click();
  await expect(optionsTab).toBeVisible();
  await expect(optionsTab.locator('.tab-bar-menu-button')).toHaveCount(4);
  const tabItem = optionsTab.locator('.tab-bar-menu-button');
  await expect(tabItem).toHaveText(['Details', 'Roles', 'Profile', 'Revoke']);

  // `Details` button
  await tabItem.nth(0).click();
  await expect(usersPage.locator('.user-details-modal')).toBeVisible();
  await usersPage.locator('.user-details-modal').locator('.closeBtn').click();

  // `Role assign` button
  await tabItem.nth(1).click();
  await expect(usersPage.locator('.role-assignment-modal')).toBeVisible();
  await usersPage.locator('.role-assignment-modal').locator('.closeBtn').nth(1).click();

  // `Profile` button
  await tabItem.nth(2).click();
  await expect(usersPage.locator('.update-profile-modal')).toBeVisible();
  await usersPage.locator('.update-profile-modal').locator('.closeBtn').click();

  // `Revoke` button
  await tabItem.nth(3).click();
  await expect(usersPage.locator('.question-modal')).toBeVisible();
  await expect(usersPage.locator('.question-modal').locator('ion-title')).toHaveText('Revoke this user?');
});
