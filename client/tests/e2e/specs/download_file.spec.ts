// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { expect, getDownloadedFile, MsPage, msTest } from '@tests/e2e/helpers';

async function confirmDownload(page: MsPage, noReminder = false): Promise<void> {
  const modal = page.locator('.download-warning-modal');

  await expect(modal).toBeVisible();
  if (noReminder) {
    await modal.locator('.download-warning-checkbox').click();
  }
  await modal.locator('#next-button').click();
  await expect(modal).toBeHidden();
}

msTest('Download file', async ({ documents }) => {
  // showSaveFilePicker is not yet supported by Playwright: https://github.com/microsoft/playwright/issues/31162
  const entries = documents.locator('.folder-container').locator('.file-list-item');
  let entryName = '';

  for (const entry of await entries.all()) {
    entryName = (await entry.locator('.file-name').locator('.file-name__label').textContent()) ?? '';
    if (entryName.endsWith('.mp3')) {
      await entry.hover();
      await entry.locator('.options-button').click();
      const popover = documents.locator('.file-context-menu');
      await popover.getByRole('listitem').filter({ hasText: 'Download' }).click();
      break;
    }
  }
  await confirmDownload(documents, true);
  await documents.waitForTimeout(1000);

  const uploadMenu = documents.locator('.upload-menu');
  await expect(uploadMenu).toBeVisible();
  const tabs = uploadMenu.locator('.upload-menu-tabs').getByRole('listitem');
  await expect(tabs.locator('.text-counter')).toHaveText(['0', '9', '0']);
  await expect(tabs.nth(0)).not.toHaveTheClass('active');
  await expect(tabs.nth(1)).toHaveTheClass('active');
  await expect(tabs.nth(2)).not.toHaveTheClass('active');

  const container = uploadMenu.locator('.element-container');
  const elements = container.locator('.element');
  await expect(elements).toHaveCount(9);
  await expect(elements.nth(0).locator('.element-details__name')).toHaveText(entryName);
  await expect(elements.nth(0).locator('.element-details-info__size')).toHaveText('40.9 KB');

  const content = await getDownloadedFile(documents);
  expect(content).toBeTruthy();
  if (content) {
    expect(content.length).toEqual(41866);
  }
});

msTest('Download multiple files', async ({ documents }) => {
  // showDirectoryPicker is not yet supported by Playwright: https://github.com/microsoft/playwright/issues/31162
  const entries = documents.locator('.folder-container').locator('.file-list-item');
  let mp3EntryName = '';
  let xlsxEntryName = '';
  let pdfEntryName = '';

  for (const entry of await entries.all()) {
    const entryName = (await entry.locator('.file-name').locator('.file-name__label').textContent()) ?? '';
    if (entryName.endsWith('.mp3') || entryName.endsWith('.xlsx') || entryName.endsWith('.pdf')) {
      if (entryName.endsWith('.mp3')) {
        mp3EntryName = entryName;
      } else if (entryName.endsWith('.xlsx')) {
        xlsxEntryName = entryName;
      } else if (entryName.endsWith('.pdf')) {
        pdfEntryName = entryName;
      }
      await entry.hover();
      await entry.locator('ion-checkbox').click();
    }
  }
  const actionBar = documents.locator('#folders-ms-action-bar');
  await expect(actionBar.locator('.counter')).toHaveText('3 selected items');
  await expect(actionBar.locator('.ms-action-bar-button:visible').nth(3)).toHaveText('Download');
  await actionBar.locator('.ms-action-bar-button:visible').nth(3).click();
  await confirmDownload(documents, true);

  await documents.waitForTimeout(1000);

  const uploadMenu = documents.locator('.upload-menu');
  await expect(uploadMenu).toBeVisible();
  const tabs = uploadMenu.locator('.upload-menu-tabs').getByRole('listitem');
  await expect(tabs.locator('.text-counter')).toHaveText(['0', '11', '0']);
  await expect(tabs.nth(0)).not.toHaveTheClass('active');
  await expect(tabs.nth(1)).toHaveTheClass('active');
  await expect(tabs.nth(2)).not.toHaveTheClass('active');

  const container = uploadMenu.locator('.element-container');
  const elements = container.locator('.element');
  await expect(elements).toHaveCount(11);

  for (let i = 0; i < 3; i++) {
    const fileRe = new RegExp(`^(${mp3EntryName}|${xlsxEntryName}|${pdfEntryName})$`);
    await expect(elements.nth(i).locator('.element-details__name')).toHaveText(fileRe);
    const sizeRe = /^(76\.9 KB|5\.99 KB|40\.9 KB)$/;
    await expect(elements.nth(i).locator('.element-details-info__size')).toHaveText(sizeRe);
  }

  const mp3Content = await getDownloadedFile(documents, mp3EntryName);
  const pdfContent = await getDownloadedFile(documents, pdfEntryName);
  const xlsxContent = await getDownloadedFile(documents, xlsxEntryName);

  expect(mp3Content).toBeTruthy();
  expect(pdfContent).toBeTruthy();
  expect(xlsxContent).toBeTruthy();

  if (mp3Content && pdfContent && xlsxContent) {
    expect(mp3Content.length).toEqual(41866);
    expect(pdfContent.length).toEqual(78731);
    expect(xlsxContent.length).toEqual(6139);
  }
});

msTest('Download folder', async ({ documents }) => {
  const entry = documents.locator('.folder-container').locator('.file-list-item').nth(0);
  await entry.hover();
  await entry.locator('ion-checkbox').click();

  const actionBar = documents.locator('#folders-ms-action-bar');
  await expect(actionBar.locator('.ms-action-bar-button:visible').nth(4)).toHaveText('Download');
  await actionBar.locator('.ms-action-bar-button:visible').nth(4).click();

  await expect(documents).toShowToast('Cannot download folders.', 'Warning');
  const uploadMenu = documents.locator('.upload-menu');
  await expect(uploadMenu).toBeHidden();
});

msTest('Download file and folder', async ({ documents }) => {
  const entries = documents.locator('.folder-container').locator('.file-list-item');
  let entryName = '';

  const folderEntry = documents.locator('.folder-container').locator('.file-list-item').nth(0);
  await folderEntry.hover();
  await folderEntry.locator('ion-checkbox').click();

  for (const entry of await entries.all()) {
    entryName = (await entry.locator('.file-name').locator('.file-name__label').textContent()) ?? '';
    if (entryName.endsWith('.mp3')) {
      await entry.hover();
      await entry.locator('ion-checkbox').click();
      break;
    }
  }

  const actionBar = documents.locator('#folders-ms-action-bar');
  await expect(actionBar.locator('.counter')).toHaveText('2 selected items');
  await expect(actionBar.locator('.ms-action-bar-button:visible').nth(3)).toHaveText('Download');
  await actionBar.locator('.ms-action-bar-button:visible').nth(3).click();

  await expect(documents).toShowToast('Folders will not be downloaded.', 'Warning');
  await confirmDownload(documents, true);
  await documents.waitForTimeout(1000);

  const uploadMenu = documents.locator('.upload-menu');
  await expect(uploadMenu).toBeVisible();
  const tabs = uploadMenu.locator('.upload-menu-tabs').getByRole('listitem');
  await expect(tabs.locator('.text-counter')).toHaveText(['0', '9', '0']);
  await expect(tabs.nth(0)).not.toHaveTheClass('active');
  await expect(tabs.nth(1)).toHaveTheClass('active');
  await expect(tabs.nth(2)).not.toHaveTheClass('active');

  const container = uploadMenu.locator('.element-container');
  const elements = container.locator('.element');
  await expect(elements).toHaveCount(9);
  await expect(elements.nth(0).locator('.element-details__name')).toHaveText(entryName);
  await expect(elements.nth(0).locator('.element-details-info__size')).toHaveText('40.9 KB');

  const content = await getDownloadedFile(documents);
  expect(content).toBeTruthy();
  if (content) {
    expect(content.length).toEqual(41866);
  }
});

msTest('Download warning', async ({ documents }) => {
  // showSaveFilePicker is not yet supported by Playwright: https://github.com/microsoft/playwright/issues/31162
  const entries = documents.locator('.folder-container').locator('.file-list-item');

  for (const entry of await entries.all()) {
    const entryName = (await entry.locator('.file-name').locator('.file-name__label').textContent()) ?? '';
    if (entryName.endsWith('.mp3')) {
      await entry.hover();
      await entry.locator('.options-button').click();
      const popover = documents.locator('.file-context-menu');
      await popover.getByRole('listitem').filter({ hasText: 'Download' }).click();
      break;
    }
  }

  // Download warning modal is visible
  const warningModal = documents.locator('.download-warning-modal');
  await expect(warningModal).toBeVisible();
  await expect(warningModal.locator('.ms-modal-header__title')).toHaveText('Are you sure you want to download this file?');
  const docLink = warningModal.locator('.download-warning-documentation').locator('a');

  // Check the link to the doc
  await expect(docLink).toHaveText('documentation');
  const newTabPromise = documents.waitForEvent('popup');
  await docLink.click();
  const newTab = await newTabPromise;
  await expect(newTab).toHaveURL(new RegExp('^https://docs.parsec.cloud/.+$'));
  await newTab.close();

  // Check the do not remind me
  await warningModal.locator('.download-warning-checkbox').click();

  await warningModal.locator('#next-button').click();
  await expect(warningModal).toBeHidden();

  await documents.waitForTimeout(1000);

  // File was downloaded
  const uploadMenu = documents.locator('.upload-menu');
  await expect(uploadMenu).toBeVisible();
  const tabs = uploadMenu.locator('.upload-menu-tabs').getByRole('listitem');
  await expect(tabs.locator('.text-counter')).toHaveText(['0', '9', '0']);
  const elements = uploadMenu.locator('.element-container').locator('.element');
  await expect(elements).toHaveCount(9);

  for (const entry of await entries.all()) {
    const entryName = (await entry.locator('.file-name').locator('.file-name__label').textContent()) ?? '';
    if (entryName.endsWith('.py')) {
      await entry.hover();
      await entry.locator('.options-button').click();
      const popover = documents.locator('.file-context-menu');
      await popover.getByRole('listitem').filter({ hasText: 'Download' }).click();
      break;
    }
  }
  // This time the warning doesn't show up
  await documents.waitForTimeout(1000);
  await expect(tabs.locator('.text-counter')).toHaveText(['0', '10', '0']);
  await expect(elements).toHaveCount(10);
});
