// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { TestInfo } from '@playwright/test';
import {
  checkDocumentTitle,
  checkEntryContextMenu,
  expect,
  fillInputModal,
  getClipboardText,
  getEditorFrame,
  importDefaultFiles,
  ImportDocuments,
  login,
  mockEditicsRequest,
  msTest,
  saveDocument,
  waitUntilSaved,
} from '@tests/e2e/helpers';

msTest.describe(() => {
  msTest.use({
    documentsOptions: {
      empty: true,
    },
  });

  for (const [mode, method] of [
    ['edit', 'context'],
    ['view', 'context'],
    ['edit', 'header'],
    ['view', 'header'],
  ]) {
    msTest(`Open editics in ${mode} mode with ${method}`, async ({ documents }, testInfo: TestInfo) => {
      await importDefaultFiles(documents, testInfo, ImportDocuments.Docx, false);
      const entry = documents.locator('.folder-container').locator('.file-list-item').nth(0);

      // Let it process everything, avoid the refresh
      await documents.waitForTimeout(500);

      if (method === 'header') {
        await entry.hover();
        await expect(entry.locator('.checkbox-container')).toBeVisible();
        await entry.locator('.checkbox-container').locator('input').check();
        const actionBar = documents.locator('#folders-ms-action-bar');
        await expect(actionBar.locator('.item-selected')).toHaveText('1 selected item');
        if (mode === 'edit') {
          await expect(actionBar.locator('ion-button').nth(1)).toHaveText('Edit');
          await actionBar.locator('ion-button').nth(1).click();
        } else {
          await expect(actionBar.locator('ion-button').nth(0)).toHaveText('Preview');
          await actionBar.locator('ion-button').nth(0).click();
        }
      } else {
        await entry.click({ button: 'right' });
        if (mode === 'edit') {
          await checkEntryContextMenu(documents, 'file-full', 'Edit', { canEdit: true });
        } else {
          await checkEntryContextMenu(documents, 'file-full', 'Preview', { canEdit: true });
        }
      }
      await expect(documents.locator('.file-editor')).toBeVisible();
      const topbar = documents.locator('.file-handler-topbar');
      await expect(topbar.locator('.file-handler-topbar__title')).toHaveText('document.docx');
      await expect(topbar.locator('.back-button')).toBeVisible();
      const topbarButtons = topbar.locator('.file-handler-topbar-buttons').locator('.file-handler-topbar-buttons__item:visible');
      if (mode === 'edit') {
        await expect(topbar.locator('.save-info')).toBeHidden();
        await expect(topbarButtons).toHaveCount(4);
        await expect(topbarButtons).toHaveText(['Details', 'Copy link', 'Download', 'Show menu']);
      } else {
        await expect(topbar.locator('.save-info')).toBeVisible();
        await expect(topbar.locator('.save-info')).toHaveText('Read only');
        await expect(topbarButtons).toHaveCount(5);
        await expect(topbarButtons).toHaveText(['Details', 'Copy link', 'Edit', 'Download', 'Show menu']);
      }
      const editicsFrame = await getEditorFrame(documents);
      await expect(editicsFrame.locator('#editor-container')).toBeVisible();
      if (mode === 'edit') {
        await checkDocumentTitle(editicsFrame, 'document.docx');
      }
    });
  }

  for (const action of ['details', 'copy_link', 'edit', 'download', 'show_menu']) {
    msTest(`File editor header '${action}' action`, async ({ documents }, testInfo: TestInfo) => {
      await importDefaultFiles(documents, testInfo, ImportDocuments.Docx, false);
      const entry = documents.locator('.folder-container').locator('.file-list-item').nth(0);

      await entry.click({ button: 'right' });
      await checkEntryContextMenu(documents, 'file-full', 'Preview', { canEdit: true });

      await expect(documents.locator('.file-editor')).toBeVisible();
      const editicsFrame = await getEditorFrame(documents);
      await expect(editicsFrame.locator('#editor-container')).toBeVisible();

      const topbar = documents.locator('.file-handler-topbar');
      await expect(topbar.locator('.file-handler-topbar__title')).toHaveText('document.docx');
      await expect(topbar.locator('.back-button')).toBeVisible();
      const topbarButtons = topbar.locator('.file-handler-topbar-buttons').locator('.file-handler-topbar-buttons__item:visible');
      await expect(topbarButtons).toHaveText(['Details', 'Copy link', 'Edit', 'Download', 'Show menu']);
      await expect(topbar.locator('.save-info')).toBeVisible();
      await expect(topbar.locator('.save-info')).toHaveText('Read only');

      if (action === 'details') {
        const modal = documents.locator('.file-details-modal');
        await expect(modal).toBeHidden();
        await topbarButtons.nth(0).click();
        await expect(modal).toBeVisible();
      } else if (action === 'copy_link') {
        await documents.context().grantPermissions(['clipboard-write']);
        await topbarButtons.nth(1).click();
        await expect(documents).toShowToast('Link has been copied to clipboard.', 'Info');
        expect(await getClipboardText(documents)).toMatch(/^https?:\/\/.+\/redirect\/.+a=path&p=.+$/);
      } else if (action === 'edit') {
        await topbarButtons.nth(2).click();
        await expect(topbar.locator('.save-info')).toBeHidden();
        await expect(topbarButtons).toHaveText(['Details', 'Copy link', 'Download', 'Show menu']);
      } else if (action === 'download') {
        const modal = documents.locator('.download-warning-modal');
        await expect(modal).toBeHidden();
        await topbarButtons.nth(3).click();
        await expect(modal).toBeVisible();
      } else if (action === 'show_menu') {
        const header = documents.locator('#connected-header');
        await expect(header).toBeHidden();
        await topbarButtons.nth(4).click();
        await expect(header).toBeVisible();
      }
    });
  }

  for (const error of ['404', 'timeout']) {
    msTest(`File editor failing to get frame because of ${error}`, async ({ documents }, testInfo: TestInfo) => {
      await importDefaultFiles(documents, testInfo, ImportDocuments.Docx, false);
      const entry = documents.locator('.folder-container').locator('.file-list-item').nth(0);

      await mockEditicsRequest(documents.context(), error as any);

      await entry.click({ button: 'right' });
      const menu = documents.locator('#file-context-menu');
      await expect(menu).toBeVisible();
      await expect(menu.getByRole('listitem').nth(1)).toHaveText('Preview');
      await menu.getByRole('listitem').nth(1).click();

      await expect(documents.locator('.file-editor')).toBeHidden();
      const errorContainer = documents.locator('.file-editor-error');
      await expect(errorContainer).toBeVisible();
      await expect(errorContainer.locator('.error-content-text__title')).toHaveText('Cannot open file');
      await expect(errorContainer.locator('.error-content-text__message')).toHaveText(
        'Could not load the editor. Please check your network connection.',
      );
    });
  }

  msTest('File editor save status', async ({ documents }, testInfo: TestInfo) => {
    await importDefaultFiles(documents, testInfo, ImportDocuments.Docx, false);
    const entry = documents.locator('.folder-container').locator('.file-list-item').nth(0);

    await entry.click({ button: 'right' });
    const menu = documents.locator('#file-context-menu');
    await expect(menu).toBeVisible();
    await expect(menu.getByRole('listitem').nth(2)).toHaveText('Edit');
    await menu.getByRole('listitem').nth(2).click();

    await documents.waitForTimeout(5000);
    const frame = await getEditorFrame(documents);
    await expect(frame.locator('#editor-container')).toBeVisible();
    const topbar = documents.locator('.file-handler-topbar');
    await expect(topbar.locator('.save-info-text')).toBeHidden();
    await frame.locator('#editor-container').focus();
    await documents.keyboard.insertText('TEST');
    await expect(topbar.locator('.save-info-text')).toBeVisible();
    await expect(topbar.locator('.save-info-text')).toHaveText('Changes unsaved');
    await saveDocument(frame);
    await waitUntilSaved(documents);
  });

  msTest('Go back with unsaved status', async ({ documents }, testInfo: TestInfo) => {
    await importDefaultFiles(documents, testInfo, ImportDocuments.Docx, false);
    const entry = documents.locator('.folder-container').locator('.file-list-item').nth(0);

    await entry.click({ button: 'right' });
    const menu = documents.locator('#file-context-menu');
    await expect(menu).toBeVisible();
    await expect(menu.getByRole('listitem').nth(2)).toHaveText('Edit');
    await menu.getByRole('listitem').nth(2).click();

    await documents.waitForTimeout(5000);
    const frame = await getEditorFrame(documents);
    await expect(frame.locator('#editor-container')).toBeVisible();
    const topbar = documents.locator('.file-handler-topbar');
    await expect(topbar.locator('.save-info-text')).toBeHidden();
    await frame.locator('#editor-container').focus();
    await documents.keyboard.insertText('TEST');
    await expect(topbar.locator('.save-info-text')).toBeVisible();
    await expect(topbar.locator('.save-info-text')).toHaveText('Changes unsaved');
    await topbar.locator('.back-button').click();
    await expect(documents).toBeDocumentPage();
  });

  msTest('Check files handled', async ({ documents }, testInfo: TestInfo) => {
    // Makes sure that some files cannot be opened, and also checks that the opening + back + opening + ...
    // works properly.

    msTest.setTimeout(90_000);

    const FILES = [
      { fileName: 'file.txt', opener: 'viewer', renameIndex: 3 },
      { fileName: 'file.html', opener: 'viewer', renameIndex: 3 },
      { fileName: 'file.odt', opener: 'editor', renameIndex: 3 },
      { fileName: 'file.docx', opener: 'editor', renameIndex: 3 },
      { fileName: 'file.doc', opener: undefined, renameIndex: 3 },
      { fileName: 'file.xls', opener: 'editor', renameIndex: 2 },
      { fileName: 'file.xlsx', opener: 'editor', renameIndex: 3 },
      { fileName: 'file.ods', opener: 'editor', renameIndex: 3 },
      { fileName: 'file.pptx', opener: 'editor', renameIndex: 3 },
      { fileName: 'file.odp', opener: 'editor', renameIndex: 3 },
      { fileName: 'file.ppt', opener: undefined, renameIndex: 3 },
      { fileName: 'file.rtf', opener: undefined, renameIndex: 2 },
      { fileName: 'file.log', opener: 'viewer', renameIndex: 2 },
      { fileName: 'file.png', opener: 'viewer', renameIndex: 3 },
      { fileName: 'file.pdf', opener: 'viewer', renameIndex: 2 },
      { fileName: 'file.mp3', opener: 'viewer', renameIndex: 2 },
      { fileName: 'file.mp4', opener: 'viewer', renameIndex: 2 },
      { fileName: 'file', opener: undefined, renameIndex: 2 },
    ];

    // Doesn't matter which one we import initially
    await importDefaultFiles(documents, testInfo, ImportDocuments.Txt, false);
    const entry = documents.locator('.folder-container').locator('.file-list-item').nth(0);

    await mockEditicsRequest(documents.context());

    for (const fileData of FILES) {
      const menu = documents.locator('#file-context-menu');

      console.log(`Checking file '${fileData.fileName}'`);

      // Rename the file first, we match the extension
      await entry.click({ button: 'right' });
      await expect(menu).toBeVisible();
      await expect(menu.getByRole('listitem').nth(fileData.renameIndex)).toHaveText('Rename');
      await menu.getByRole('listitem').nth(fileData.renameIndex).click();
      await fillInputModal(documents, fileData.fileName);
      expect(menu).toBeHidden();
      await expect(entry.locator('.label-name')).toHaveText(fileData.fileName);
      await entry.click({ button: 'right' });
      await expect(menu).toBeVisible();
      await expect(menu.getByRole('listitem').nth(1)).toHaveText('Preview');
      await menu.getByRole('listitem').nth(1).click();

      if (!fileData.opener) {
        await expect(documents).toShowInformationModal(
          'Parsec cannot preview this type of document. You can download it ' +
            "by selecting the file or showing options then click 'Download'.",
          'Info',
          'Cannot preview this file',
        );
        await expect(documents).toBeDocumentPage();
      } else if (fileData.opener === 'viewer') {
        await expect(documents).toBeViewerPage();
        const topbar = documents.locator('.file-handler-topbar');
        await expect(topbar.locator('.file-handler-topbar__title')).toHaveText(fileData.fileName);
        await expect(topbar.locator('.back-button')).toBeVisible();
        await topbar.locator('.back-button').click();
        await expect(documents).toBeDocumentPage();
      } else {
        await expect(documents).toBeEditorPage();
        await expect(documents.locator('.file-editor')).toBeHidden();
        const errorContainer = documents.locator('.file-editor-error');
        await expect(errorContainer).toBeVisible();
        await expect(errorContainer.locator('.error-content-text__title')).toHaveText('Cannot open file');
        await expect(errorContainer.locator('.error-content-text__message')).toHaveText(
          'Could not load the editor. Please check your network connection.',
        );
        const topbar = documents.locator('.file-handler-topbar');
        await expect(topbar.locator('.file-handler-topbar__title')).toHaveText(fileData.fileName);
        await expect(topbar.locator('.back-button')).toBeVisible();
        await topbar.locator('.back-button').click();
        await expect(documents).toBeDocumentPage();
      }
    }
  });
});

// TODO: re-enable when collaborative editing is properly supported
msTest.skip('Edit file in editor with two users', async ({ documents }) => {
  msTest.setTimeout(120_000);
  const entries = documents.locator('.folder-container').locator('.file-list-item');

  // Promote Bob
  await documents.locator('.sidebar').locator('.sidebar-content-workspaces').nth(1).getByRole('listitem').click({ button: 'right' });
  await expect(documents.locator('ion-popover').locator('ion-item').nth(9)).toHaveText('Sharing and roles');
  await documents.locator('ion-popover').locator('ion-item').nth(9).click();
  const bobDropdown = documents.locator('ion-modal').locator('.user-list-members-item').locator('.dropdown-container');
  await expect(bobDropdown).toHaveText('Reader');
  await bobDropdown.click();
  await documents.locator('ion-popover').locator('ion-item').nth(0).click();
  await expect(bobDropdown).toHaveText('Owner');
  await documents.locator('ion-modal').locator('.closeBtn').click();

  // Open in editor with Alice
  await entries.nth(2).click({ button: 'right' });
  const menu = documents.locator('#file-context-menu');
  await expect(menu).toBeVisible();
  await expect(menu.getByRole('listitem').nth(2)).toHaveText('Edit');
  await menu.getByRole('listitem').nth(2).click();
  await expect(documents.locator('#cryptpad-editor')).toBeVisible();
  const mainFrameAlice = documents.locator('#cryptpad-editor').contentFrame();
  await expect(mainFrameAlice.locator('.placeholder-message-container')).toBeVisible();
  await expect(mainFrameAlice.locator('.placeholder-message-container')).toHaveText('Loading...');
  // Takes an incredibly long time to load on the CI
  await documents.waitForTimeout(10000);

  // Open editor with Bob
  const secondTab = await documents.openNewTab();
  await login(secondTab, 'Boby McBobFace');
  await secondTab.locator('.workspaces-container-grid').locator('.workspace-card-item').click();
  const secondEntries = secondTab.locator('.folder-container').locator('.file-list-item');
  await secondEntries.nth(2).hover();
  await secondEntries.nth(2).locator('.ms-checkbox').check();
  const actionBar = secondTab.locator('#folders-ms-action-bar');
  await expect(actionBar.locator('ion-button').nth(1)).toHaveText('Edit');
  await actionBar.locator('ion-button').nth(1).click();
  await expect(secondTab.locator('#cryptpad-editor')).toBeVisible();
  const mainFrameBob = secondTab.locator('#cryptpad-editor').contentFrame();
  await expect(mainFrameBob.locator('.placeholder-message-container')).toBeVisible();
  await expect(mainFrameBob.locator('.placeholder-message-container')).toHaveText('Loading...');
  // Takes an incredibly long time to load on the CI
  await secondTab.waitForTimeout(10000);

  // Make some edits and check from the other user
  await expect(mainFrameAlice.locator('#sbox-iframe')).toBeVisible();
  const editorAlice = documents
    .locator('#cryptpad-editor')
    .contentFrame()
    .locator('#sbox-iframe')
    .contentFrame()
    .locator('#cp-app-code-editor')
    .locator('.CodeMirror-code')
    .locator('pre')
    .nth(0);
  const editorBob = secondTab
    .locator('#cryptpad-editor')
    .contentFrame()
    .locator('#sbox-iframe')
    .contentFrame()
    .locator('#cp-app-code-editor')
    .locator('.CodeMirror-code')
    .locator('pre')
    .nth(0);
  await expect(editorAlice).toHaveText('# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS');
  await expect(editorBob).toHaveText('# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS');

  await editorAlice.fill('New first line!');
  await expect(editorAlice).toHaveText('New first line!');
  await waitUntilSaved(documents);
  await expect(editorBob).toHaveText('New first line!');
  await editorBob.fill('New NEWER first line!');
  await expect(editorBob).toHaveText('New NEWER first line!');
  await waitUntilSaved(secondTab);
  await expect(editorAlice).toHaveText('New NEWER first line!');
});

msTest.skip('Check file edited by other user', async ({ documents }) => {
  msTest.setTimeout(120_000);
  await documents.locator('.header-label-name').click();
  const entries = documents.locator('.folder-container').locator('.file-list-item');

  // Promote Bob
  await documents.locator('.sidebar').locator('.sidebar-content-workspaces').nth(1).getByRole('listitem').click({ button: 'right' });
  await expect(documents.locator('ion-popover').locator('ion-item').nth(9)).toHaveText('Sharing and roles');
  await documents.locator('ion-popover').locator('ion-item').nth(9).click();
  const bobDropdown = documents.locator('ion-modal').locator('.user-list-members-item').locator('.dropdown-container');
  await expect(bobDropdown).toHaveText('Reader');
  await bobDropdown.click();
  await documents.locator('ion-popover').locator('ion-item').nth(0).click();
  await expect(bobDropdown).toHaveText('Owner');
  await documents.locator('ion-modal').locator('.closeBtn').click();

  // Open in editor with Alice
  await entries.nth(2).click({ button: 'right' });
  const menu = documents.locator('#file-context-menu');
  await expect(menu).toBeVisible();
  await expect(menu.getByRole('listitem').nth(2)).toHaveText('Edit');
  await menu.getByRole('listitem').nth(2).click();
  await expect(documents.locator('#cryptpad-editor')).toBeVisible();
  const mainFrameAlice = documents.locator('#cryptpad-editor').contentFrame();
  await expect(mainFrameAlice.locator('.placeholder-message-container')).toBeVisible();
  await expect(mainFrameAlice.locator('.placeholder-message-container')).toHaveText('Loading...');
  // Takes an incredibly long time to load on the CI
  await documents.waitForTimeout(10000);

  // Make some edits and check from the other user
  await expect(mainFrameAlice.locator('#sbox-iframe')).toBeVisible();
  const editorAlice = documents
    .locator('#cryptpad-editor')
    .contentFrame()
    .locator('#sbox-iframe')
    .contentFrame()
    .locator('#cp-app-code-editor')
    .locator('.CodeMirror-code')
    .locator('pre')
    .nth(0);
  await expect(editorAlice).toBeVisible();
  await expect(editorAlice).toHaveText('# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS');
  await editorAlice.fill('New first line!');
  await waitUntilSaved(documents);

  // Open editor with Bob
  const secondTab = await documents.openNewTab();
  await login(secondTab, 'Boby McBobFace');
  await secondTab.locator('.workspaces-container-grid').locator('.workspace-card-item').click();
  const secondEntries = secondTab.locator('.folder-container').locator('.file-list-item');
  await secondEntries.nth(2).hover();
  await secondEntries.nth(2).locator('.ms-checkbox').check();
  const actionBar = secondTab.locator('#folders-ms-action-bar');
  await expect(actionBar.locator('ion-button').nth(1)).toHaveText('Edit');
  await actionBar.locator('ion-button').nth(1).click();
  await expect(secondTab.locator('#cryptpad-editor')).toBeVisible();

  const mainFrameBob = secondTab.locator('#cryptpad-editor').contentFrame();
  await expect(mainFrameBob.locator('.placeholder-message-container')).toBeVisible();
  await expect(mainFrameBob.locator('.placeholder-message-container')).toHaveText('Loading...');
  // Takes an incredibly long time to load on the CI
  await secondTab.waitForTimeout(10000);

  // Check modified text
  const editorBob = secondTab
    .locator('#cryptpad-editor')
    .contentFrame()
    .locator('#sbox-iframe')
    .contentFrame()
    .locator('#cp-app-code-editor')
    .locator('.CodeMirror-code')
    .locator('pre')
    .nth(0);
  await expect(editorBob).toBeVisible();
  await expect(editorBob).toHaveText('New first line!');
});
