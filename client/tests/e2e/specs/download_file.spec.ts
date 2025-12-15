// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { TestInfo } from '@playwright/test';
import {
  answerQuestion,
  createFolder,
  dragAndDropFile,
  expect,
  getDownloadedFile,
  importDefaultFiles,
  ImportDocuments,
  MsPage,
  msTest,
  openExternalLink,
  renameDocument,
} from '@tests/e2e/helpers';
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

msTest.describe(() => {
  msTest.use({
    documentsOptions: {
      empty: true,
    },
  });

  msTest('Download file', async ({ documents }, testInfo: TestInfo) => {
    await importDefaultFiles(documents, testInfo, ImportDocuments.Mp3, false);

    // showSaveFilePicker is not yet supported by Playwright: https://github.com/microsoft/playwright/issues/31162
    const entry = documents.locator('.folder-container').locator('.file-list-item').first();
    await entry.hover();
    await entry.locator('.options-button').click();
    const popover = documents.locator('.file-context-menu');
    await popover.getByRole('listitem').filter({ hasText: 'Download' }).click();
    await confirmDownload(documents, true);
    await documents.waitForTimeout(1000);

    const uploadMenu = documents.locator('.upload-menu');
    await expect(uploadMenu).toBeVisible();
    const tabs = uploadMenu.locator('.upload-menu-tabs').getByRole('listitem');
    await expect(tabs.locator('.text-counter')).toHaveText(['0', '2', '0']);
    await expect(tabs.nth(0)).not.toHaveTheClass('active');
    await expect(tabs.nth(1)).toHaveTheClass('active');
    await expect(tabs.nth(2)).not.toHaveTheClass('active');

    const container = uploadMenu.locator('.element-container');
    const elements = container.locator('.element');
    await expect(elements).toHaveCount(2);
    await expect(elements.nth(0).locator('.element-details__name')).toHaveText('audio.mp3');
    await expect(elements.nth(0).locator('.element-details-info__size')).toHaveText('40.9 KB');

    const content = await getDownloadedFile(documents);
    expect(content).toBeTruthy();
    if (content) {
      expect(content.length).toEqual(41866);
    }
  });

  msTest('Download multiple files and folder', async ({ documents }, testInfo: TestInfo) => {
    await importDefaultFiles(documents, testInfo, ImportDocuments.Mp3 | ImportDocuments.Pdf | ImportDocuments.Xlsx, true);

    msTest.setTimeout(45_000);
    const entries = documents.locator('.folder-container').locator('.file-list-item');
    await entries.nth(0).dblclick();
    const dropZone = documents.locator('.folder-container').locator('.drop-zone').nth(0);
    await dragAndDropFile(documents, dropZone, [path.join(testInfo.config.rootDir, 'data', 'imports', 'image.png')]);
    await documents.waitForTimeout(1000);
    const uploadMenu = documents.locator('.upload-menu');
    await expect(uploadMenu).toBeVisible();
    const tabs = uploadMenu.locator('.upload-menu-tabs').getByRole('listitem');
    await expect(tabs.locator('.text-counter')).toHaveText(['0', '4', '0']);
    await expect(tabs.nth(0)).not.toHaveTheClass('active');
    await expect(tabs.nth(1)).toHaveTheClass('active');
    await expect(tabs.nth(2)).not.toHaveTheClass('active');
    await uploadMenu.locator('.menu-header-icons').locator('ion-icon').nth(1).click();
    await expect(documents.locator('.folder-container').locator('.no-files-content')).toBeHidden();

    await documents.locator('#connected-header').locator('.topbar-left__breadcrumb').locator('ion-breadcrumb').nth(1).click();
    await expect(documents).toHaveHeader(['wksp1'], true, true);
    await expect(entries).toHaveCount(4);
    await documents.waitForTimeout(1000);

    for (const entry of await entries.all()) {
      await entry.hover();
      await entry.locator('ion-checkbox').click();
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
    await expect(tabs.locator('.text-counter')).toHaveText(['0', '5', '0']);
    await expect(tabs.nth(0)).not.toHaveTheClass('active');
    await expect(tabs.nth(1)).toHaveTheClass('active');
    await expect(tabs.nth(2)).not.toHaveTheClass('active');

    const container = uploadMenu.locator('.element-container');
    const elements = container.locator('.element');
    await expect(elements).toHaveCount(5);

    await expect(elements.nth(0).locator('.element-details__name')).toHaveText('wksp1_ROOT.zip');
    await expect(elements.nth(0).locator('.element-details-info__size')).toHaveText('130 KB');

    const zipContent = await getDownloadedFile(documents);
    expect(zipContent).toBeTruthy();
    if (!zipContent) {
      throw new Error('No downloaded file');
    }
    expect(zipContent.length).toEqual(120947);
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

  msTest('Download warning', async ({ documents }, testInfo: TestInfo) => {
    await importDefaultFiles(documents, testInfo, ImportDocuments.Mp3 | ImportDocuments.Py, false);

    const audioEntry = documents.locator('.folder-container').locator('.file-list-item').first();
    await audioEntry.hover();
    await audioEntry.locator('.options-button').click();
    await documents.locator('.file-context-menu').getByRole('listitem').filter({ hasText: 'Download' }).click();

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
    await expect(tabs.locator('.text-counter')).toHaveText(['0', '3', '0']);
    const elements = uploadMenu.locator('.element-container').locator('.element');
    await expect(elements).toHaveCount(3);

    const minimizeButton = documents.locator('.upload-menu .menu-header-icons__item').nth(0);
    if (await minimizeButton.isVisible()) {
      await minimizeButton.click();
      await expect(documents.locator('.upload-menu')).toHaveTheClass('minimize');
    }

    const pyEntry = documents.locator('.folder-container').locator('.file-list-item').last();
    await pyEntry.hover();
    await pyEntry.locator('.options-button').click();
    await documents.locator('.file-context-menu').getByRole('listitem').filter({ hasText: 'Download' }).click();
    // This time the warning doesn't show up
    await documents.waitForTimeout(1000);
    await expect(tabs.locator('.text-counter')).toHaveText(['0', '4', '0']);
    await expect(elements).toHaveCount(4);
  });

  msTest('Download archive filenames from different alphabets', async ({ documents }, testInfo: TestInfo) => {
    msTest.setTimeout(45_000);
    const entries = documents.locator('.folder-container').locator('.file-list-item');
    const actionBar = documents.locator('#folders-ms-action-bar');

    // Sort by size to make the renaming easier
    const sorterPopoverButton = actionBar.locator('#select-popover-button');
    await expect(sorterPopoverButton).toHaveText('Name');
    await sorterPopoverButton.click();
    const sorterPopover = documents.locator('.sorter-popover');
    await expect(sorterPopover).toBeVisible();
    await expect(sorterPopover.getByRole('listitem').nth(4)).toHaveText('Size');
    await sorterPopover.getByRole('listitem').nth(4).click();
    await expect(sorterPopover).toBeHidden();

    await createFolder(documents, 'Folder');
    await entries.nth(0).dblclick();
    const dropZone = documents.locator('.folder-container').locator('.drop-zone').nth(0);
    await dragAndDropFile(documents, dropZone, [
      path.join(testInfo.config.rootDir, 'data', 'imports', 'image.png'),
      path.join(testInfo.config.rootDir, 'data', 'imports', 'hell_yeah.png'),
    ]);
    await documents.waitForTimeout(1000);
    const uploadMenu = documents.locator('.upload-menu');
    await expect(uploadMenu).toBeVisible();
    const tabs = uploadMenu.locator('.upload-menu-tabs').getByRole('listitem');
    await expect(tabs.locator('.text-counter')).toHaveText(['0', '2', '0']);
    await uploadMenu.locator('.menu-header-icons').locator('ion-icon').nth(1).click();
    await expect(documents.locator('.folder-container').locator('.no-files-content')).toBeHidden();
    // cspell:disable-next-line
    await renameDocument(documents, entries.nth(0), '文件名.png');
    // cspell:disable-next-line
    await renameDocument(documents, entries.nth(1), 'Имя файла.png');
    // cspell:disable-next-line
    await expect(entries.nth(0).locator('.file-name').locator('.label-name')).toHaveText('文件名.png');
    // cspell:disable-next-line
    await expect(entries.nth(1).locator('.file-name').locator('.label-name')).toHaveText('Имя файла.png');

    await documents.locator('#connected-header').locator('.topbar-left__breadcrumb').locator('ion-breadcrumb').nth(1).click();
    await expect(documents).toHaveHeader(['wksp1'], true, true);
    await expect(entries).toHaveCount(1);

    await entries.nth(0).hover();
    await entries.nth(0).locator('ion-checkbox').click();
    await expect(entries.nth(0).locator('ion-checkbox')).toHaveTheClass('checkbox-checked');

    await expect(actionBar.locator('.counter')).toHaveText('1 selected item');
    await expect(actionBar.locator('.ms-action-bar-button')).toHaveCount(7);
    await expect(actionBar.locator('.ms-action-bar-button').nth(4)).toHaveText('Download');
    await actionBar.locator('.ms-action-bar-button').nth(4).click();
    await confirmDownload(documents, true);
    await answerQuestion(documents, true);

    await documents.waitForTimeout(1000);

    await expect(uploadMenu).toBeVisible();
    await expect(tabs.locator('.text-counter')).toHaveText(['0', '3', '0']);

    const container = uploadMenu.locator('.element-container');
    const elements = container.locator('.element');
    await expect(elements).toHaveCount(3);

    await expect(elements.nth(0).locator('.element-details__name')).toHaveText('wksp1_ROOT.zip');
    await expect(elements.nth(0).locator('.element-details-info__size')).toHaveText('244 KB');

    const zipContent = await getDownloadedFile(documents);
    expect(zipContent).toBeTruthy();
    if (!zipContent) {
      throw new Error('No downloaded file');
    }
    expect(zipContent.length).toEqual(248683);
    const zip = new AdmZip(Buffer.from(zipContent));
    const zipEntries = zip.getEntries().sort();
    expect(zipEntries.length).toBe(2);
    // cspell:disable-next-line
    expect(zipEntries.at(0)?.entryName).toBe('Folder/Имя файла.png');
    expect(zipEntries.at(0)?.header.size).toBe(243871);
    // cspell:disable-next-line
    expect(zipEntries.at(1)?.entryName).toBe('Folder/文件名.png');
    expect(zipEntries.at(1)?.header.size).toBe(6335);
  });

  msTest('Download archive too many recursion', async ({ documents }, testInfo: TestInfo) => {
    msTest.setTimeout(45_000);
    const entries = documents.locator('.folder-container').locator('.file-list-item');

    for (let i = 0; i < 15; i++) {
      await createFolder(documents, `Folder${i}`);
      await expect(entries.nth(0).locator('.file-name').locator('.label-name')).toHaveText(`Folder${i}`);
      await expect(documents.locator('.folder-container').locator('.no-files')).toBeHidden();
      await entries.nth(0).dblclick();
      await expect(documents.locator('.folder-container').locator('.no-files')).toBeVisible();
    }
    const dropZone = documents.locator('.folder-container').locator('.drop-zone').nth(0);
    await dragAndDropFile(documents, dropZone, [path.join(testInfo.config.rootDir, 'data', 'imports', 'hell_yeah.png')]);
    await documents.waitForTimeout(1000);
    const uploadMenu = documents.locator('.upload-menu');
    await expect(uploadMenu).toBeVisible();
    const tabs = uploadMenu.locator('.upload-menu-tabs').getByRole('listitem');
    await expect(tabs.locator('.text-counter')).toHaveText(['0', '1', '0']);
    await uploadMenu.locator('.menu-header-icons').locator('ion-icon').nth(1).click();
    await expect(entries.nth(0).locator('.file-name').locator('.label-name')).toHaveText('hell_yeah.png');

    await documents.locator('#connected-header').locator('.topbar-left__breadcrumb').locator('ion-breadcrumb').nth(1).click();
    await documents.waitForTimeout(500);
    await expect(documents).toHaveHeader(['wksp1'], true, true);
    await expect(entries).toHaveCount(1);

    await entries.nth(0).hover();
    await entries.nth(0).locator('ion-checkbox').click();
    await expect(entries.nth(0).locator('ion-checkbox')).toHaveTheClass('checkbox-checked');

    const actionBar = documents.locator('#folders-ms-action-bar');
    await expect(actionBar.locator('.counter')).toHaveText('1 selected item');
    await expect(actionBar.locator('.ms-action-bar-button')).toHaveCount(7);
    await documents.waitForTimeout(100);
    await expect(actionBar.locator('.ms-action-bar-button').nth(4)).toHaveText('Download');
    await actionBar.locator('.ms-action-bar-button').nth(4).click();
    await confirmDownload(documents, true);
    await expect(documents).toShowToast('Maximum subfolder depth reached, cannot download', 'Error');
  });
});
