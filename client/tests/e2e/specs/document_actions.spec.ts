// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { TestInfo } from '@playwright/test';
import {
  answerQuestion,
  DisplaySize,
  documentsToggleViewMode,
  dragAndDropFile,
  expect,
  fillInputModal,
  fillIonInput,
  importDefaultFiles,
  ImportDocuments,
  login,
  mockLibParsec,
  msTest,
} from '@tests/e2e/helpers';
import path from 'path';

msTest.describe(() => {
  msTest.use({
    documentsOptions: {
      empty: true,
    },
  });

  for (const gridMode of [false, true]) {
    msTest(`Open file in ${gridMode ? 'grid' : 'list'} mode`, async ({ documents }, testInfo) => {
      await importDefaultFiles(documents, testInfo, ImportDocuments.Png, true);

      await expect(documents.locator('.information-modal')).toBeHidden();
      await expect(documents).toHaveHeader(['wksp1'], true, true);
      if (gridMode) {
        await documentsToggleViewMode(documents);
        await documents.locator('.folder-container').locator('.file-card-item').nth(1).dblclick();
      } else {
        await documents.locator('.folder-container').getByRole('listitem').nth(1).dblclick();
      }

      await expect(documents).toBeViewerPage();
    });
  }

  for (const displaySize of [DisplaySize.Small, DisplaySize.Large]) {
    msTest(`Create a folder ${displaySize} display`, async ({ documents }, testInfo: TestInfo) => {
      await importDefaultFiles(documents, testInfo, ImportDocuments.Png, false);

      const entries = documents.locator('.folder-container').locator('.file-list-item');

      if (displaySize === DisplaySize.Small) {
        await documents.setDisplaySize(DisplaySize.Small);
      } else {
        const actionBar = documents.locator('#folders-ms-action-bar');
        await expect(entries).toHaveCount(1);
        await expect(actionBar.locator('.counter')).toHaveText('1 item');
      }

      if (displaySize === DisplaySize.Small) {
        const addButton = documents.locator('.tab-bar-menu').locator('#add-menu-fab-button');
        await expect(addButton).toBeVisible();
        await addButton.click();
        const modal = documents.locator('.tab-menu-modal');
        await expect(modal).toBeVisible();
        await modal.locator('.list-group-item').filter({ hasText: 'New folder' }).click();
      } else {
        const actionBar = documents.locator('#folders-ms-action-bar');
        await actionBar.getByText('New folder').click();
      }

      await fillInputModal(documents, 'My folder');

      await expect(entries).toHaveCount(2);
      await expect(entries.locator('.file-name').locator('.label-name')).toHaveText(['My folder', 'image.png']);
    });

    msTest(`Delete all documents with action bar in ${displaySize} display`, async ({ documents }, testInfo: TestInfo) => {
      await importDefaultFiles(documents, testInfo, ImportDocuments.Pdf | ImportDocuments.Png, true);
      await documents.waitForTimeout(500);
      const entries = documents.locator('.folder-container').locator('.file-list-item');
      await expect(entries).toHaveCount(3);
      const globalCheckbox = documents.locator('.folder-container').locator('.files-list-header').locator('.ms-checkbox');
      await expect(globalCheckbox).not.toBeChecked();
      await globalCheckbox.check();
      await documents.waitForTimeout(100);
      await expect(globalCheckbox).toBeChecked();

      if (displaySize === DisplaySize.Small) {
        await documents.setDisplaySize(DisplaySize.Small);
        await expect(documents.locator('#tab-bar-options')).toBeVisible();
        await expect(documents.locator('#tab-bar-options').locator('.tab-bar-menu-button').nth(2)).toHaveText('Delete');
        await documents.locator('#tab-bar-options').locator('.tab-bar-menu-button').nth(2).click();
      } else {
        const actionBar = documents.locator('#folders-ms-action-bar');
        const deleteButton = actionBar.locator('.ms-action-bar-button:visible').nth(2);
        await expect(deleteButton).toBeVisible();
        await expect(deleteButton).toHaveText('Delete');
        await deleteButton.click();
      }

      await answerQuestion(documents, true, {
        expectedTitleText: 'Delete multiple items',
        expectedQuestionText: /Are you sure you want to delete these 3 items\?/,
        expectedPositiveText: /Delete 3 items/,
        expectedNegativeText: 'Keep items',
      });
      await expect(entries).toHaveCount(0);
      await expect(documents.locator('.folder-container').locator('.no-files')).toBeVisible();
    });
  }

  msTest('Create folder error', async ({ documents }) => {
    await mockLibParsec(documents, [
      {
        name: 'workspaceCreateFolderAll',
        result: { ok: false, error: { tag: 'WorkspaceCreateFolderErrorInternal', error: 'failed' } },
      },
    ]);

    const entries = documents.locator('.folder-container').locator('.file-list-item');
    await expect(entries).toHaveCount(0);

    const actionBar = documents.locator('#folders-ms-action-bar');
    await actionBar.getByText('New folder').click();

    await fillInputModal(documents, 'My folder');
    await expect(documents).toShowToast('Failed to create folder `My folder`, please try again.', 'Error');

    await expect(entries).toHaveCount(0);
  });

  msTest('Create a folder with a name too long', async ({ documents }) => {
    const entries = documents.locator('.folder-container').locator('.file-list-item');
    await expect(entries).toHaveCount(0);

    const actionBar = documents.locator('#folders-ms-action-bar');
    await actionBar.getByText('New folder').click();

    const modal = documents.locator('.text-input-modal');
    await expect(modal).toBeVisible();
    const okButton = modal.locator('.ms-modal-footer-buttons').locator('#next-button');
    await fillIonInput(modal.locator('ion-input'), 'A'.repeat(132));
    await expect(modal.locator('.form-error')).toBeVisible();
    await expect(modal.locator('.form-error')).toHaveText('Folder name is too long, limit is 128 characters.');
    await fillIonInput(modal.locator('ion-input'), 'A'.repeat(64));
    await expect(modal.locator('.form-error')).toBeHidden();
    await expect(okButton).toBeTrulyEnabled();
    await okButton.click();

    await expect(entries).toHaveCount(1);
    await expect(entries.locator('.file-name').locator('.label-name').nth(0)).toHaveText('A'.repeat(64));
  });

  for (const file of [true, false]) {
    msTest(`Rename a ${file ? 'file' : 'folder'} with a name too long`, async ({ documents }, testInfo: TestInfo) => {
      await importDefaultFiles(documents, testInfo, ImportDocuments.Png, true);
      const entries = documents.locator('.folder-container').locator('.file-list-item');
      await expect(entries).toHaveCount(2);

      const entry = file ? entries.nth(1) : entries.nth(0);
      await entry.click({ button: 'right' });
      await expect(documents.locator('.file-context-menu')).toBeVisible();
      await documents.locator('.file-context-menu').getByRole('listitem').filter({ hasText: 'Rename' }).click();

      const modal = documents.locator('.text-input-modal');
      await expect(modal).toBeVisible();
      const okButton = modal.locator('.ms-modal-footer-buttons').locator('#next-button');
      await fillIonInput(modal.locator('ion-input'), 'A'.repeat(132));
      await expect(modal.locator('.form-error')).toBeVisible();
      await expect(modal.locator('.form-error')).toHaveText(`${file ? 'File' : 'Folder'} name is too long, limit is 128 characters.`);
      await expect(okButton).toBeTrulyDisabled();
    });
  }

  msTest('Drop file in empty read only workspace', async ({ home }, testInfo: TestInfo) => {
    await login(home, 'Boby McBobFace');
    await expect(home).toBeWorkspacePage();
    await home.locator('.workspaces-container-grid').locator('.workspace-card-item').nth(0).click();
    await expect(home).toBeDocumentPage();
    const dropZone = home.locator('.folder-container').locator('.drop-zone').nth(0);
    await dragAndDropFile(home, dropZone, [path.join(testInfo.config.rootDir, 'data', 'imports', 'image.png')]);
    await expect(home).toShowToast('You are a Reader on this workspace and cannot import files.', 'Error');
    await expect(home.locator('.folder-container').locator('.no-files')).toBeVisible();
  });

  msTest('Test drag&drop message', async ({ documents }) => {
    const entries = documents.locator('.folder-container').locator('.file-list-item');
    await expect(entries).toHaveCount(0);

    const dropZone = documents.locator('.folder-container').locator('.drop-zone').nth(0);
    const dropMessage = dropZone.locator('.drop-message').nth(0);
    await expect(dropMessage).toBeHidden();

    await dropZone.dispatchEvent('dragenter');
    await expect(dropMessage).toBeVisible();
    await expect(dropMessage).toHaveText('Drag & drop your files to import them');
    await dropZone.dispatchEvent('dragleave');
    await expect(dropMessage).toBeHidden();
    await dropZone.dispatchEvent('dragenter');
    await expect(dropMessage).toBeVisible();

    const dataTransfer = await documents.evaluateHandle(
      async (fileInfo: { content: string; name: string }) => {
        const dt = new DataTransfer();

        const blobData = await fetch(`data:application/octet-stream;base64,${fileInfo.content}`).then((res) => res.blob());
        const upload = new File([blobData], fileInfo.name, { type: 'application/octet-stream' });
        dt.items.add(upload);
        return dt;
      },
      { content: 'dGVzdA==', name: 'text.txt' },
    );

    await dropZone.dispatchEvent('drop', { dataTransfer });
    await expect(entries).toHaveCount(1);
    await expect(dropMessage).toBeHidden();
  });
});

msTest('Drop file in full read only workspace', async ({ documentsReadOnly }, testInfo: TestInfo) => {
  const dropZone = documentsReadOnly.locator('.folder-container').locator('.drop-zone').nth(0);
  await dragAndDropFile(documentsReadOnly, dropZone, [path.join(testInfo.config.rootDir, 'data', 'imports', 'image.png')]);
  await expect(documentsReadOnly).toShowToast('You are a Reader on this workspace and cannot import files.', 'Error');
});
