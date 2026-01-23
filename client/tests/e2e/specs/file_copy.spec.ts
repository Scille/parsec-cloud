// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { TestInfo } from '@playwright/test';
import {
  createFolder,
  DisplaySize,
  DocumentsPage,
  dragAndDropFile,
  expect,
  importDefaultFiles,
  ImportDocuments,
  msTest,
} from '@tests/e2e/helpers';

msTest.describe(() => {
  msTest.use({
    documentsOptions: {
      empty: true,
    },
  });

  for (const gridMode of [false, true]) {
    msTest(`Copy document in ${gridMode ? 'grid' : 'list'} mode`, async ({ documents }, testInfo: TestInfo) => {
      await importDefaultFiles(documents, testInfo, ImportDocuments.Png, true);

      let entries;
      if (gridMode) {
        await DocumentsPage.toggleViewMode(documents);
        entries = documents.locator('.folder-container').locator('.file-card-item');
        await expect(entries.nth(1).locator('.file-card__title')).toHaveText('image.png');
      } else {
        entries = documents.locator('.folder-container').locator('.file-list-item');
        await expect(entries.nth(1).locator('.file-name')).toHaveText('image.png');
      }
      await expect(documents.locator('.folder-selection-modal')).toBeHidden();
      await DocumentsPage.clickAction(await DocumentsPage.openActionPopover(documents, 1), 'Make a copy');
      const modal = documents.locator('.folder-selection-modal');
      await expect(modal).toBeVisible();
      await expect(modal.locator('.ms-modal-header__title')).toHaveText('Copy one item');
      const okButton = modal.locator('#next-button');
      await expect(okButton).toHaveText('Copy here');
      await modal.locator('.folder-container').getByRole('listitem').nth(0).click();
      await expect(modal.locator('.current-folder__text')).toHaveText(/^Dir_[A-Za-z_]+$/);
      await expect(okButton).toNotHaveDisabledAttribute();
      await okButton.click();
      await expect(modal).toBeHidden();

      const uploadMenu = documents.locator('.upload-menu');
      await expect(uploadMenu).toBeVisible();
      const opItems = uploadMenu.locator('.upload-menu-list').locator('.file-operation-item');
      await expect(opItems).toHaveCount(2);
      await expect(opItems.nth(0).locator('.element-details-title__name')).toHaveText('Copying image.png');
      await expect(opItems.nth(0).locator('.element-details-info')).toHaveText('wksp1');

      // File should still be here
      if (gridMode) {
        await expect(entries.nth(1).locator('.file-card__title')).toHaveText('image.png');
      } else {
        await expect(entries.nth(1).locator('.file-name')).toHaveText('image.png');
      }
      // And also in the folder
      await entries.nth(0).dblclick();

      await expect(documents).toHaveHeader(['wksp1', 'Dir_Folder'], true, true);
      await expect(entries).toHaveCount(1);

      if (gridMode) {
        await expect(entries.nth(0).locator('.file-card__title')).toHaveText('image.png');
      } else {
        await expect(entries.nth(0).locator('.file-name')).toHaveText('image.png');
      }
    });

    msTest(`Small display copy document in ${gridMode ? 'grid' : 'list'} mode`, async ({ documents }, testInfo: TestInfo) => {
      await importDefaultFiles(documents, testInfo, ImportDocuments.Png, true);
      await documents.setDisplaySize(DisplaySize.Small);
      let entries;
      if (gridMode) {
        await DocumentsPage.toggleViewMode(documents);
        entries = documents.locator('.folder-container').locator('.file-card-item');
        await expect(entries.nth(1).locator('.file-card__title')).toHaveText('image.png');
      } else {
        entries = documents.locator('.folder-container').locator('.file-list-item');
        await expect(entries.nth(1).locator('.label-name')).toHaveText('image.png');
      }
      await expect(documents.locator('.folder-selection-modal')).toBeHidden();
      await DocumentsPage.clickAction(await DocumentsPage.openActionPopover(documents, 1), 'Make a copy');
      const modal = documents.locator('.folder-selection-modal');
      await expect(modal).toBeVisible();
      await expect(modal.locator('.ms-modal-header__title')).toHaveText('Copy one item');
      const okButton = modal.locator('#next-button');
      await expect(okButton).toHaveText('Copy here');
      await modal.locator('.folder-container').getByRole('listitem').nth(0).click();
      await expect(modal.locator('.current-folder__text')).toHaveText(/^Dir_[A-Za-z_]+$/);
      await expect(okButton).toNotHaveDisabledAttribute();
      await okButton.click();
      await expect(modal).toBeHidden();

      const uploadMenu = documents.locator('.upload-menu');
      await expect(uploadMenu).toBeVisible();
      const opItems = uploadMenu.locator('.upload-menu-list').locator('.file-operation-item');
      await expect(opItems).toHaveCount(2);
      await expect(opItems.nth(0).locator('.element-details-title__name')).toHaveText('Copying image.png');
      await expect(opItems.nth(0).locator('.element-details-info')).toHaveText('wksp1');
      await uploadMenu.locator('.menu-header-icons').locator('ion-icon').nth(1).click();

      // File should still be here
      if (gridMode) {
        await expect(entries.nth(1).locator('.file-card__title')).toHaveText('image.png');
      } else {
        await expect(entries.nth(1).locator('.label-name')).toHaveText('image.png');
      }
      // And also in the folder
      await entries.nth(0).dblclick();

      await expect(documents).toHaveHeader(['wksp1', 'Dir_Folder'], true, true);
      await expect(entries).toHaveCount(1);

      if (gridMode) {
        await expect(entries.nth(0).locator('.file-card__title')).toHaveText('image.png');
      } else {
        await expect(entries.nth(0).locator('.label-name')).toHaveText('image.png');
      }
    });
  }

  for (const dupPolicy of ['replace', 'ignore', 'addCount']) {
    msTest(`Copy document with duplicate and ${dupPolicy}`, async ({ documents }) => {
      await createFolder(documents, 'Folder');
      const folderDropZone = documents.locator('.folder-container').locator('.drop-zone-item').nth(0);
      const dropZone = documents.locator('.folder-container').locator('.file-drop-zone');
      // Drop a file with the content '1' in the 'Folder' folder
      await dragAndDropFile(documents, folderDropZone, [], [{ name: 'file.txt', content: 'MQ==' }]);
      // Drop a file with the content '12345' in the current folder
      await dragAndDropFile(documents, dropZone, [], [{ name: 'file.txt', content: 'MTIzNDU=' }]);

      const uploadMenu = documents.locator('.upload-menu');
      await expect(uploadMenu).toBeVisible();
      const opItems = uploadMenu.locator('.upload-menu-list').locator('.file-operation-item');
      await expect(opItems).toHaveCount(2);
      await expect(opItems.nth(0).locator('.element-details-title__name')).toHaveText('file.txt');
      await expect(opItems.nth(0).locator('.element-details-info')).toHaveText('5 B •  wksp1');
      await expect(opItems.nth(1).locator('.element-details-title__name')).toHaveText('file.txt');
      await expect(opItems.nth(1).locator('.element-details-info')).toHaveText('1 B •  wksp1');
      await uploadMenu.locator('.menu-header-icons').locator('ion-icon').nth(1).click();

      const entries = documents.locator('.folder-container').locator('.file-list-item');
      await expect(entries).toHaveCount(2);
      await expect(entries.locator('.label-name')).toHaveText(['Folder', 'file.txt']);

      const modal = documents.locator('.folder-selection-modal');
      const dupModal = documents.locator('.file-operation-conflicts-modal');

      await expect(modal).toBeHidden();
      await expect(dupModal).toBeHidden();
      await DocumentsPage.clickAction(await DocumentsPage.openActionPopover(documents, 1), 'Make a copy');
      await expect(modal).toBeVisible();
      await expect(modal.locator('.ms-modal-header__title')).toHaveText('Copy one item');
      const okButton = modal.locator('#next-button');
      await expect(okButton).toHaveText('Copy here');
      await modal.locator('.folder-container').getByRole('listitem').nth(0).click();
      await expect(modal.locator('.current-folder__text')).toHaveText('Folder');
      await okButton.click();
      await expect(modal).toBeHidden();
      await expect(dupModal).toBeVisible();

      const buttons = dupModal.locator('.modal-footer-buttons__item');
      await expect(buttons).toHaveText(['Ignore', 'Replace', 'Keep both']);

      if (dupPolicy === 'replace') {
        await buttons.nth(1).click();
        await expect(dupModal).toBeHidden();
        await expect(uploadMenu).toBeVisible();
        await expect(opItems).toHaveCount(3);
        await expect(opItems.nth(0).locator('.element-details-title__name')).toHaveText('Copying file.txt');
        await expect(entries).toHaveCount(2);
        await expect(entries.locator('.label-name')).toHaveText(['Folder', 'file.txt']);
        await entries.nth(0).dblclick();
        await expect(entries).toHaveCount(1);
        await expect(entries.locator('.label-name')).toHaveText('file.txt');
        await expect(entries.locator('.file-size')).toHaveText('5 B');
      } else if (dupPolicy === 'ignore') {
        await buttons.nth(0).click();
        await expect(dupModal).toBeHidden();
        await expect(uploadMenu).toBeVisible();
        await expect(opItems).toHaveCount(3);
        await expect(opItems.nth(0).locator('.element-details-title__name')).toHaveText('Copying file.txt');
        await expect(entries).toHaveCount(2);
        await expect(entries.locator('.label-name')).toHaveText(['Folder', 'file.txt']);
        await entries.nth(0).dblclick();
        await expect(entries).toHaveCount(1);
        await expect(entries.locator('.label-name')).toHaveText('file.txt');
        await expect(entries.locator('.file-size')).toHaveText('1 B');
      } else {
        await buttons.nth(2).click();
        await expect(dupModal).toBeHidden();
        await expect(uploadMenu).toBeVisible();
        await expect(opItems).toHaveCount(3);
        await expect(opItems.nth(0).locator('.element-details-title__name')).toHaveText('Copying file.txt');
        await expect(entries).toHaveCount(2);
        await expect(entries.locator('.label-name')).toHaveText(['Folder', 'file.txt']);
        await entries.nth(0).dblclick();
        await expect(entries).toHaveCount(2);
        await expect(entries.locator('.label-name')).toHaveText(['file (2).txt', 'file.txt']);
        await expect(entries.locator('.file-size')).toHaveText(['5 B', '1 B']);
      }
    });

    msTest(`Copy folder with duplicate and ${dupPolicy}`, async ({ documents }) => {
      await createFolder(documents, 'FolderA');
      await createFolder(documents, 'FolderB');

      const entries = documents.locator('.folder-container').locator('.file-list-item');
      await expect(entries).toHaveCount(2);
      await expect(entries.locator('.label-name')).toHaveText(['FolderA', 'FolderB']);

      await entries.nth(0).dblclick();
      await expect(documents).toHaveHeader(['wksp1', 'FolderA'], true, true);
      await createFolder(documents, 'Folder');
      // Drop a file with the content '1' in the 'FolderA/Folder' folder
      await dragAndDropFile(
        documents,
        documents.locator('.folder-container').locator('.drop-zone-item').nth(0),
        [],
        [{ name: 'file.txt', content: 'MQ==' }],
      );
      await documents.locator('#connected-header').locator('.topbar-left').locator('.back-button').click();
      await expect(documents).toHaveHeader(['wksp1'], true, true);

      await entries.nth(1).dblclick();
      await expect(documents).toHaveHeader(['wksp1', 'FolderB'], true, true);
      await createFolder(documents, 'Folder');
      // Drop a file with the content '12345' in the 'FolderB/Folder' folder
      await dragAndDropFile(
        documents,
        documents.locator('.folder-container').locator('.drop-zone-item').nth(0),
        [],
        [{ name: 'file.txt', content: 'MTIzNDU=' }],
      );

      const uploadMenu = documents.locator('.upload-menu');
      await expect(uploadMenu).toBeVisible();
      const opItems = uploadMenu.locator('.upload-menu-list').locator('.file-operation-item');
      await expect(opItems).toHaveCount(2);
      await expect(opItems.nth(0).locator('.element-details-title__name')).toHaveText('file.txt');
      await expect(opItems.nth(0).locator('.element-details-info')).toHaveText('5 B •  wksp1');
      await expect(opItems.nth(1).locator('.element-details-title__name')).toHaveText('file.txt');
      await expect(opItems.nth(1).locator('.element-details-info')).toHaveText('1 B •  wksp1');
      await uploadMenu.locator('.menu-header-icons').locator('ion-icon').nth(1).click();

      const modal = documents.locator('.folder-selection-modal');
      const dupModal = documents.locator('.file-operation-conflicts-modal');

      await expect(modal).toBeHidden();
      await expect(dupModal).toBeHidden();
      await DocumentsPage.clickAction(await DocumentsPage.openActionPopover(documents, 0), 'Make a copy');
      await expect(modal).toBeVisible();
      await expect(modal.locator('.ms-modal-header__title')).toHaveText('Copy one item');
      const okButton = modal.locator('#next-button');
      await expect(okButton).toHaveText('Copy here');
      // Go back, then into FolderA
      await expect(modal.locator('.current-folder__text')).toHaveText('FolderB');
      await modal.locator('ion-breadcrumb').nth(0).click();
      await expect(modal.locator('.current-folder__text')).toBeHidden();
      await modal.locator('.folder-container').getByRole('listitem').nth(0).click();
      await expect(modal.locator('.current-folder__text')).toHaveText('FolderA');
      await expect(okButton).toNotHaveDisabledAttribute();
      await okButton.click();
      await expect(modal).toBeHidden();

      await expect(dupModal).toBeVisible();
      const buttons = dupModal.locator('.modal-footer-buttons__item');
      await expect(buttons).toHaveText(['Ignore', 'Replace', 'Keep both']);

      if (dupPolicy === 'replace') {
        await buttons.nth(1).click();
      } else if (dupPolicy === 'ignore') {
        await buttons.nth(0).click();
      } else {
        await buttons.nth(2).click();
      }

      await expect(dupModal).toBeHidden();
      await expect(uploadMenu).toBeVisible();
      await expect(opItems).toHaveCount(3);
      await expect(opItems.nth(0).locator('.element-details-title__name')).toHaveText('Copying Folder');
      // Folder is still there
      await expect(entries.locator('.label-name')).toHaveText('Folder');

      await documents.locator('#connected-header').locator('.topbar-left').locator('.back-button').click();
      await expect(documents).toHaveHeader(['wksp1'], true, true);
      await entries.nth(0).dblclick();
      await expect(documents).toHaveHeader(['wksp1', 'FolderA'], true, true);

      if (dupPolicy === 'replace') {
        await expect(entries.locator('.label-name')).toHaveCount(1);
        await expect(entries.locator('.label-name')).toHaveText('Folder');
        await entries.nth(0).dblclick();
        await expect(documents).toHaveHeader(['wksp1', '', 'Folder'], true, true);
        await expect(entries.locator('.label-name')).toHaveCount(1);
        await expect(entries.locator('.label-name')).toHaveText('file.txt');
        await expect(entries.locator('.file-size')).toHaveText('5 B');
      } else if (dupPolicy === 'ignore') {
        await expect(entries.locator('.label-name')).toHaveCount(1);
        await expect(entries.locator('.label-name')).toHaveText('Folder');
        await entries.nth(0).dblclick();
        await expect(documents).toHaveHeader(['wksp1', '', 'Folder'], true, true);
        await expect(entries.locator('.label-name')).toHaveCount(1);
        await expect(entries.locator('.label-name')).toHaveText('file.txt');
        await expect(entries.locator('.file-size')).toHaveText('1 B');
      } else {
        await expect(entries.locator('.label-name')).toHaveCount(2);
        await expect(entries.locator('.label-name')).toHaveText(['Folder', 'Folder (2)']);
        await entries.nth(0).dblclick();
        await expect(documents).toHaveHeader(['wksp1', '', 'Folder'], true, true);
        await expect(entries.locator('.label-name')).toHaveCount(1);
        await expect(entries.locator('.label-name')).toHaveText('file.txt');
        await expect(entries.locator('.file-size')).toHaveText('1 B');
        await documents.locator('#connected-header').locator('.topbar-left').locator('.back-button').click();
        await entries.nth(1).dblclick();
        await expect(documents).toHaveHeader(['wksp1', '', 'Folder (2)'], true, true);
        await expect(entries.locator('.label-name')).toHaveCount(1);
        await expect(entries.locator('.label-name')).toHaveText('file.txt');
        await expect(entries.locator('.file-size')).toHaveText('5 B');
      }
    });
  }
});
