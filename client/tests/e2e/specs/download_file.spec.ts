// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { Download, Locator, TestInfo } from '@playwright/test';
import {
  createFolder,
  dragAndDropFile,
  expect,
  expectSameContent,
  importDefaultFiles,
  ImportDocuments,
  MsPage,
  msTest,
  openExternalLink,
  readDownload,
  readImportedFile,
  renameDocument,
  waitForDownload,
} from '@tests/e2e/helpers';
import AdmZip from 'adm-zip';
import { randomBytes } from 'crypto';
import { writeFileSync } from 'fs';
import path from 'path';

async function confirmDownload(page: MsPage, noReminder = false): Promise<void> {
  const modal = page.locator('.download-warning-modal');

  await expect(modal).toBeVisible();
  if (noReminder) {
    await modal.locator('.ms-checkbox').check();
  }
  await modal.locator('#next-button').click();
  await expect(modal).toBeHidden();
}

// Clicks "Download" in the context menu that is open (whatever else the menu contains)
async function clickContextMenuDownload(page: MsPage): Promise<void> {
  await page
    .locator('.file-context-menu')
    .getByRole('listitem')
    .filter({ hasText: /^Download$/ })
    .click();
}

// Downloads the entry with its context menu. The warning is only shown until it is asked not to (see `confirmDownload`).
async function downloadFromContextMenu(page: MsPage, entry: Locator, warningShown = true): Promise<Download> {
  return await waitForDownload(page, async () => {
    await entry.hover();
    await entry.locator('.options-button').click();
    await clickContextMenuDownload(page);
    if (warningShown) {
      await confirmDownload(page, true);
    }
  });
}

// Downloads the selected entries with the action bar
async function downloadFromActionBar(page: MsPage): Promise<Download> {
  const actionBar = page.locator('#folders-ms-action-bar');
  const downloadButton = actionBar.locator('.ms-action-bar-button:visible').filter({ hasText: 'Download' });
  const moreButton = actionBar.locator('#action-bar-more-button');
  const morePopover = page.locator('.action-bar-more-popover');
  return await waitForDownload(page, async () => {
    // The button is in the "more" menu if there is not enough room for it in the bar
    await expect(downloadButton.or(moreButton).filter({ visible: true }).first()).toBeVisible();
    if (await downloadButton.isVisible()) {
      await downloadButton.click();
    } else {
      await moreButton.click();
      await expect(morePopover).toBeVisible();
      await morePopover
        .getByRole('listitem')
        .filter({ hasText: /^Download$/ })
        .click();
      await expect(morePopover).toBeHidden();
    }
    await confirmDownload(page, true);
  });
}

// `expected` associates the name of each entry of the archive to its content (`null` for a folder)
function expectZipContent(archive: Buffer, expected: Record<string, Buffer | null>): void {
  const zipEntries = new AdmZip(archive).getEntries();
  expect(zipEntries.map((entry) => entry.entryName).sort()).toEqual(Object.keys(expected).sort());
  for (const entry of zipEntries) {
    const content = expected[entry.entryName];
    if (content === null) {
      expect(entry.isDirectory).toBe(true);
    } else {
      expect(entry.isDirectory).toBe(false);
      expectSameContent(entry.getData(), content);
    }
  }
}

msTest.describe(() => {
  msTest.use({
    documentsOptions: {
      empty: true,
    },
  });

  msTest('Download file', async ({ documents }, testInfo: TestInfo) => {
    await importDefaultFiles(documents, testInfo, ImportDocuments.Mp3, false);

    const entry = documents.locator('.folder-container').locator('.file-list-item').first();
    const download = await downloadFromContextMenu(documents, entry);

    expect(download.suggestedFilename()).toBe('audio.mp3');
    expectSameContent(await readDownload(download), readImportedFile(testInfo, 'audio.mp3'));
  });

  msTest('Download multiple files and folder as a zip', async ({ documents }, testInfo: TestInfo) => {
    msTest.setTimeout(45_000);

    await importDefaultFiles(documents, testInfo, ImportDocuments.Mp3 | ImportDocuments.Pdf | ImportDocuments.Xlsx, true);

    const entries = documents.locator('.folder-container').locator('.file-list-item');
    await entries.nth(0).dblclick();
    const dropZone = documents.locator('.folder-container').locator('.drop-zone').nth(0);
    await dragAndDropFile(documents, dropZone, [path.join(testInfo.config.rootDir, 'data', 'imports', 'image.png')]);
    await documents.waitForTimeout(1000);
    await expect(documents.locator('.folder-container').locator('.no-files-content')).toBeHidden();

    await documents.locator('#connected-header').locator('.topbar-left__breadcrumb').locator('ion-breadcrumb').nth(1).click();
    await expect(documents).toHaveHeader(['wksp1'], true, true);
    await expect(entries).toHaveCount(4);

    await documents.locator('.folder-container').locator('.header-label-selected').click();
    await expect(documents.locator('#folders-ms-action-bar').locator('.counter')).toHaveText('4 selected items');

    const download = await downloadFromActionBar(documents);

    expect(download.suggestedFilename()).toBe('wksp1.zip');
    expectZipContent(await readDownload(download), {
      'audio.mp3': readImportedFile(testInfo, 'audio.mp3'),
      'Dir_Folder/image.png': readImportedFile(testInfo, 'image.png'),
      'pdfDocument.pdf': readImportedFile(testInfo, 'pdfDocument.pdf'),
      'spreadsheet.xlsx': readImportedFile(testInfo, 'spreadsheet.xlsx'),
    });
  });

  msTest('Download warning', async ({ documents }, testInfo: TestInfo) => {
    await importDefaultFiles(documents, testInfo, ImportDocuments.Mp3 | ImportDocuments.Py, false);

    const audioEntry = documents.locator('.folder-container').locator('.file-list-item').first();
    await audioEntry.hover();
    await audioEntry.locator('.options-button').click();
    await clickContextMenuDownload(documents);

    // Download warning modal is visible
    const warningModal = documents.locator('.download-warning-modal');
    await expect(warningModal).toBeVisible();
    await expect(warningModal.locator('.ms-modal-header__title')).toHaveText('Are you sure you want to download this file?');
    const docLink = warningModal.locator('.download-warning-documentation').locator('a');

    // Check the link to the doc
    await expect(docLink).toHaveText('documentation');
    await openExternalLink(documents, docLink, new RegExp('^https://docs.parsec.cloud/.+$'));

    // Check the do not remind me
    await warningModal.locator('.ms-checkbox').check();

    // The file is downloaded once the warning is accepted
    const audioDownload = await waitForDownload(documents, async () => {
      await warningModal.locator('#next-button').click();
      await expect(warningModal).toBeHidden();
    });
    expect(audioDownload.suggestedFilename()).toBe('audio.mp3');
    expectSameContent(await readDownload(audioDownload), readImportedFile(testInfo, 'audio.mp3'));

    // This time the warning doesn't show up
    const pyEntry = documents.locator('.folder-container').locator('.file-list-item').last();
    const pyDownload = await waitForDownload(documents, async () => {
      await pyEntry.hover();
      await pyEntry.locator('.options-button').click();
      await clickContextMenuDownload(documents);
    });
    await expect(warningModal).toBeHidden();
    expect(pyDownload.suggestedFilename()).toBe('code.py');
    expectSameContent(await readDownload(pyDownload), readImportedFile(testInfo, 'code.py'));
  });

  msTest('Download a folder with names from different alphabets', async ({ documents }, testInfo: TestInfo) => {
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
    await uploadMenu.locator('.menu-header-icons').locator('ion-icon').nth(1).click();
    await expect(documents.locator('.folder-container').locator('.no-files-content')).toBeHidden();
    // cspell:disable-next-line
    await renameDocument(documents, entries.nth(1), '文件名.png');
    // cspell:disable-next-line
    await renameDocument(documents, entries.nth(0), 'Имя файла.png');
    // cspell:disable-next-line
    await expect(entries.nth(1).locator('.file-name').locator('.label-name')).toHaveText('文件名.png');
    // cspell:disable-next-line
    await expect(entries.nth(0).locator('.file-name').locator('.label-name')).toHaveText('Имя файла.png');

    await documents.locator('#connected-header').locator('.topbar-left__breadcrumb').locator('ion-breadcrumb').nth(1).click();
    await expect(documents).toHaveHeader(['wksp1'], true, true);
    await expect(entries).toHaveCount(1);

    await documents.waitForTimeout(500);
    await entries.nth(0).hover();
    await documents.waitForTimeout(300);
    await entries.nth(0).locator('.ms-checkbox').check();
    await documents.waitForTimeout(300);
    await expect(entries.nth(0).locator('.ms-checkbox')).toBeChecked();
    await expect(actionBar.locator('.counter')).toHaveText('1 selected item');

    const download = await downloadFromActionBar(documents);

    expect(download.suggestedFilename()).toBe('Folder.zip');
    expectZipContent(await readDownload(download), {
      // cspell:disable-next-line
      'Folder/Имя файла.png': readImportedFile(testInfo, 'image.png'),
      // cspell:disable-next-line
      'Folder/文件名.png': readImportedFile(testInfo, 'hell_yeah.png'),
    });
  });

  msTest('Download a folder with an empty subfolder', async ({ documents }) => {
    const entries = documents.locator('.folder-container').locator('.file-list-item');

    await createFolder(documents, 'Parent');
    await entries.nth(0).dblclick();
    await createFolder(documents, 'Empty');
    await expect(entries).toHaveCount(1);

    await documents.locator('#connected-header').locator('.topbar-left__breadcrumb').locator('ion-breadcrumb').nth(1).click();
    await expect(documents).toHaveHeader(['wksp1'], true, true);
    await expect(entries).toHaveCount(1);

    await documents.waitForTimeout(500);
    await entries.nth(0).hover();
    await documents.waitForTimeout(300);
    await entries.nth(0).locator('.ms-checkbox').check();
    await expect(entries.nth(0).locator('.ms-checkbox')).toBeChecked();

    const download = await downloadFromActionBar(documents);

    expect(download.suggestedFilename()).toBe('Parent.zip');
    // The empty folder is in the archive, nothing else is
    expectZipContent(await readDownload(download), { 'Parent/Empty/': null });
  });

  msTest('Download a file with a name that must be encoded', async ({ documents }, testInfo: TestInfo) => {
    await importDefaultFiles(documents, testInfo, ImportDocuments.Png, false);

    const entries = documents.locator('.folder-container').locator('.file-list-item');
    // Non-ASCII characters, spaces, parentheses and an apostrophe: nothing is allowed as is in a header
    // cspell:disable-next-line
    const name = 'Имя файла (1) file name 文件名.png';
    await renameDocument(documents, entries.nth(0), name);
    await expect(entries.nth(0).locator('.file-name').locator('.label-name')).toHaveText(name);

    const download = await downloadFromContextMenu(documents, entries.nth(0));

    expect(download.suggestedFilename()).toBe(name);
    expectSameContent(await readDownload(download), readImportedFile(testInfo, 'image.png'));
  });

  msTest('Download a file larger than what the worker reads at once', async ({ documents }, testInfo: TestInfo) => {
    // The worker reads 2MB at a time
    const content = randomBytes(5 * 1024 * 1024);
    const filePath = testInfo.outputPath('large.bin');
    writeFileSync(filePath, content);

    await expect(documents).toBeDocumentPage();
    const dropZone = documents.locator('.folder-container').locator('.drop-zone').nth(0);
    await dragAndDropFile(documents, dropZone, [filePath]);
    const uploadMenu = documents.locator('.upload-menu');
    await expect(uploadMenu.locator('.file-operation-item').locator('.folder-icon')).toBeVisible({ timeout: 60_000 });
    await uploadMenu.locator('.menu-header-icons').locator('ion-icon').nth(1).click();

    const entry = documents.locator('.folder-container').locator('.file-list-item').first();
    const download = await downloadFromContextMenu(documents, entry);

    expect(download.suggestedFilename()).toBe('large.bin');
    expectSameContent(await readDownload(download), content);
  });

  msTest('Download once the browser stopped the worker', async ({ documents }, testInfo: TestInfo) => {
    await importDefaultFiles(documents, testInfo, ImportDocuments.Mp3 | ImportDocuments.Py, false);
    const entries = documents.locator('.folder-container').locator('.file-list-item');

    // The first download is what makes the page learn who it is for the worker, it doesn't ask again afterwards
    const firstDownload = await downloadFromContextMenu(documents, entries.nth(0));
    expect(firstDownload.suggestedFilename()).toBe('audio.mp3');
    expectSameContent(await readDownload(firstDownload), readImportedFile(testInfo, 'audio.mp3'));

    // Browsers stop the workers whenever they want, and they lose everything they have in memory.
    // The state of the worker is followed to be sure that it was really stopped, and started again.
    const cdp = await documents.context().newCDPSession(documents);
    const states: Array<string> = [];
    cdp.on('ServiceWorker.workerVersionUpdated', (event) => {
      for (const version of event.versions.filter((version) => version.scriptURL.endsWith('/streaming-worker.js'))) {
        states.push(version.runningStatus);
      }
    });
    await cdp.send('ServiceWorker.enable');
    await expect.poll(() => states.at(-1)).toBe('running');
    await cdp.send('ServiceWorker.stopAllWorkers');
    await expect.poll(() => states.at(-1)).toBe('stopped');

    const secondDownload = await downloadFromContextMenu(documents, entries.nth(1), false);

    expect(states.slice(states.lastIndexOf('stopped') + 1)).toContain('running');
    expect(secondDownload.suggestedFilename()).toBe('code.py');
    expectSameContent(await readDownload(secondDownload), readImportedFile(testInfo, 'code.py'));
  });
});
