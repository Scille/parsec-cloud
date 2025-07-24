// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { TestInfo } from '@playwright/test';
import { answerQuestion, dragAndDropFile, expect, getDownloadedFile, MsPage, msTest, openExternalLink } from '@tests/e2e/helpers';
import AdmZip from 'adm-zip';
import path from 'path';

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

msTest('Download multiple files and folder', async ({ documents }, testInfo: TestInfo) => {
  const entries = documents.locator('.folder-container').locator('.file-list-item');
  await entries.nth(0).dblclick();
  const dropZone = documents.locator('.folder-container').locator('.drop-zone').nth(0);
  await dragAndDropFile(documents, dropZone, [path.join(testInfo.config.rootDir, 'data', 'imports', 'image.png')]);
  await documents.waitForTimeout(1000);
  const uploadMenu = documents.locator('.upload-menu');
  await expect(uploadMenu).toBeVisible();
  const tabs = uploadMenu.locator('.upload-menu-tabs').getByRole('listitem');
  await expect(tabs.locator('.text-counter')).toHaveText(['0', '9', '0']);
  await expect(tabs.nth(0)).not.toHaveTheClass('active');
  await expect(tabs.nth(1)).toHaveTheClass('active');
  await expect(tabs.nth(2)).not.toHaveTheClass('active');
  await uploadMenu.locator('.menu-header-icons').locator('ion-icon').nth(1).click();
  await expect(documents.locator('.folder-container').locator('.no-files-content')).toBeHidden();

  await documents.locator('#connected-header').locator('.topbar-left__breadcrumb').locator('ion-breadcrumb').nth(1).click();
  await expect(documents).toHaveHeader(['wksp1'], true, true);
  await expect(entries).toHaveCount(9);
  await documents.waitForTimeout(1000);

  for (const entry of await entries.all()) {
    const entryName = (await entry.locator('.file-name').locator('.file-name__label').textContent()) ?? '';
    if (entryName.endsWith('.mp3') || entryName.endsWith('.xlsx') || entryName.endsWith('.pdf') || entryName === 'Dir_Folder') {
      await entry.hover();
      await entry.locator('ion-checkbox').click();
    }
  }
  const actionBar = documents.locator('#folders-ms-action-bar');
  await expect(actionBar.locator('.counter')).toHaveText('4 selected items');
  await expect(actionBar.locator('.ms-action-bar-button:visible').nth(3)).toHaveText('Download');
  await actionBar.locator('.ms-action-bar-button:visible').nth(3).click();
  await confirmDownload(documents, true);
  await answerQuestion(documents, true, {
    expectedNegativeText: 'Cancel',
    expectedPositiveText: 'Download archive',
    expectedQuestionText: 'The selected files will be downloaded as an archive, with a total size of 130 KB. Do you want to continue?',
    expectedTitleText: 'Downloading multiple files',
  });

  await documents.waitForTimeout(1000);

  await expect(uploadMenu).toBeVisible();
  await expect(tabs.locator('.text-counter')).toHaveText(['0', '10', '0']);
  await expect(tabs.nth(0)).not.toHaveTheClass('active');
  await expect(tabs.nth(1)).toHaveTheClass('active');
  await expect(tabs.nth(2)).not.toHaveTheClass('active');

  const container = uploadMenu.locator('.element-container');
  const elements = container.locator('.element');
  await expect(elements).toHaveCount(10);

  await expect(elements.nth(0).locator('.element-details__name')).toHaveText('wksp1_ROOT.zip');
  await expect(elements.nth(0).locator('.element-details-info__size')).toHaveText('130 KB');

  const zipContent = await getDownloadedFile(documents);
  expect(zipContent).toBeTruthy();
  if (!zipContent) {
    throw new Error('No downloaded file');
  }
  expect(zipContent.length).toEqual(120415);
  const zip = new AdmZip(Buffer.from(zipContent));
  const zipEntries = zip.getEntries().sort((a, b) => a.entryName.localeCompare(b.entryName));
  expect(zipEntries.length).toBe(4);
  expect(zipEntries.at(0)?.entryName).toBe('audio.mp3');
  expect(zipEntries.at(0)?.header.size).toBe(41866);
  expect(zipEntries.at(1)?.entryName).toBe('Dir_Folder/image.png');
  expect(zipEntries.at(1)?.header.size).toBe(6335);
  expect(zipEntries.at(2)?.entryName).toBe('pdfDocument.pdf');
  expect(zipEntries.at(2)?.header.size).toBe(78731);
  expect(zipEntries.at(3)?.entryName).toBe('spreadsheet.xlsx');
  expect(zipEntries.at(3)?.header.size).toBe(6139);
});

msTest('Download warning', async ({ documents }) => {
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
  await openExternalLink(documents, docLink, new RegExp('^https://docs.parsec.cloud/.+$'));

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
