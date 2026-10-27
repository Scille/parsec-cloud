// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { TestInfo } from '@playwright/test';
import {
  checkDocumentTitle,
  checkEntryContextMenu,
  enableCollaborativeMode,
  expect,
  expectParagraphTexts,
  fillInputModal,
  getClipboardText,
  getEditorFrame,
  importDefaultFiles,
  ImportDocuments,
  login,
  mockEditicsRequest,
  msTest,
  pressInDocument,
  promoteSecondUserToContributor,
  saveDocument,
  typeInDocument,
  waitForDocumentLoaded,
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

  // Both tests below exercise the collaborative edition mode (the default mode
  // when the client is online, see `editics/README.md`): the e2e tests keep the
  // editor in offline mode otherwise, so they explicitly opt in with the
  // `enableCollaborativeMode` helper (see `FileEditor.vue`).

  msTest('Edit file in editor with two users', async ({ documents }, testInfo: TestInfo) => {
    msTest.setTimeout(240_000);
    await importDefaultFiles(documents, testInfo, ImportDocuments.Docx, false);
    // Let it process everything, avoid the refresh
    await documents.waitForTimeout(500);

    await enableCollaborativeMode(documents);

    // Promote Bob to Contributor so that he can edit the document too
    await promoteSecondUserToContributor(documents);

    // Open the document in the editor with Alice
    const aliceEntry = documents.locator('.folder-container').locator('.file-list-item').nth(0);
    await aliceEntry.click({ button: 'right' });
    await checkEntryContextMenu(documents, 'file-full', 'Edit', { canEdit: true });
    await waitForDocumentLoaded(documents);

    // Open the document in the editor with Bob
    const secondTab = await documents.openNewTab();
    await enableCollaborativeMode(secondTab);
    await login(secondTab, 'Boby McBobFace');
    await secondTab.locator('.workspaces-container-grid').locator('.workspace-card-item').nth(0).click();
    await expect(secondTab).toBeDocumentPage();
    const bobEntry = secondTab.locator('.folder-container').locator('.file-list-item').nth(0);
    await bobEntry.click({ button: 'right' });
    await checkEntryContextMenu(secondTab, 'file-full', 'Edit', { canEdit: true });
    await waitForDocumentLoaded(secondTab);

    // Alice types at the very beginning of the document
    await typeInDocument(documents, 'New first line!');
    await expectParagraphTexts(documents, ['New first line!Title\r\n']);
    // Bob should see Alice's changes
    await expectParagraphTexts(secondTab, ['New first line!Title\r\n']);

    // Bob types at the very beginning of the document
    await pressInDocument(secondTab, 'Control+Home');
    await typeInDocument(secondTab, 'New NEWER first line!');
    await expectParagraphTexts(secondTab, ['New NEWER first line!New first line!Title\r\n']);
    // Alice should see Bob's changes
    await expectParagraphTexts(documents, ['New NEWER first line!New first line!Title\r\n']);

    // Alice inserts a new line at the very beginning of the document (the
    // paragraph insertion is a structural change, unlike the plain text edits)
    await pressInDocument(documents, 'Control+Home');
    await pressInDocument(documents, 'Enter');
    await expectParagraphTexts(documents, ['\r\n', 'New NEWER first line!New first line!Title\r\n']);
    // Bob should see the new line too
    await expectParagraphTexts(secondTab, ['\r\n', 'New NEWER first line!New first line!Title\r\n']);
  });

  msTest('Open file in editor while other user is editing', async ({ documents }, testInfo: TestInfo) => {
    msTest.setTimeout(240_000);
    await importDefaultFiles(documents, testInfo, ImportDocuments.Docx, false);
    // Let it process everything, avoid the refresh
    await documents.waitForTimeout(500);

    await enableCollaborativeMode(documents);

    // Promote Bob to Contributor so that he can edit the document too
    await promoteSecondUserToContributor(documents);

    // Open the document in the editor with Alice
    const aliceEntry = documents.locator('.folder-container').locator('.file-list-item').nth(0);
    await aliceEntry.click({ button: 'right' });
    await checkEntryContextMenu(documents, 'file-full', 'Edit', { canEdit: true });
    await waitForDocumentLoaded(documents);

    // Alice types at the very beginning of the document and inserts a new line
    await typeInDocument(documents, 'New first line!');
    await expectParagraphTexts(documents, ['New first line!Title\r\n']);
    await pressInDocument(documents, 'Enter');
    await expectParagraphTexts(documents, ['New first line!\r\n', 'Title\r\n']);

    // Open the document in the editor with Bob while Alice is still editing: Bob
    // joins the session and catches up with Alice's changes (delivered as the
    // session's changes backlog)
    const secondTab = await documents.openNewTab();
    await enableCollaborativeMode(secondTab);
    await login(secondTab, 'Boby McBobFace');
    await secondTab.locator('.workspaces-container-grid').locator('.workspace-card-item').nth(0).click();
    await expect(secondTab).toBeDocumentPage();
    const bobEntry = secondTab.locator('.folder-container').locator('.file-list-item').nth(0);
    await bobEntry.click({ button: 'right' });
    await checkEntryContextMenu(secondTab, 'file-full', 'Edit', { canEdit: true });
    await waitForDocumentLoaded(secondTab);

    // Bob must see Alice's changes, including the inserted line
    await expectParagraphTexts(secondTab, ['New first line!\r\n', 'Title\r\n']);

    // Bob can also edit the document, Alice should see his changes
    await pressInDocument(secondTab, 'Control+Home');
    await typeInDocument(secondTab, 'New NEWER first line!');
    await expectParagraphTexts(secondTab, ['New NEWER first line!New first line!\r\n', 'Title\r\n']);
    await expectParagraphTexts(documents, ['New NEWER first line!New first line!\r\n', 'Title\r\n']);
  });

  msTest('Check file edited by other user', async ({ documents }, testInfo: TestInfo) => {
    msTest.setTimeout(240_000);
    await importDefaultFiles(documents, testInfo, ImportDocuments.Docx, false);
    // Let it process everything, avoid the refresh
    await documents.waitForTimeout(500);

    await enableCollaborativeMode(documents);

    // Promote Bob to Contributor so that he can open the document in the editor too
    await promoteSecondUserToContributor(documents);

    // Open the document in the editor with Alice
    const aliceEntry = documents.locator('.folder-container').locator('.file-list-item').nth(0);
    await aliceEntry.click({ button: 'right' });
    await checkEntryContextMenu(documents, 'file-full', 'Edit', { canEdit: true });
    await waitForDocumentLoaded(documents);

    // Make some edits and wait for the changes to be sent to the session
    await typeInDocument(documents, 'New first line!');
    await expectParagraphTexts(documents, ['New first line!Title\r\n']);
    await waitUntilSaved(documents);

    // Leaving the editor persists the document into the workspace file (this is
    // the only way the collaborative mode persists it, see `onSaveRequest` in
    // `editics/common.ts`)
    const topbar = documents.locator('.file-handler-topbar');
    await topbar.locator('.back-button').click();
    await expect(documents).toBeDocumentPage();

    // Open the document in the editor with Bob and check he sees the modification
    const secondTab = await documents.openNewTab();
    await enableCollaborativeMode(secondTab);
    await login(secondTab, 'Boby McBobFace');
    await secondTab.locator('.workspaces-container-grid').locator('.workspace-card-item').nth(0).click();
    await expect(secondTab).toBeDocumentPage();
    const bobEntry = secondTab.locator('.folder-container').locator('.file-list-item').nth(0);
    await bobEntry.click({ button: 'right' });
    await checkEntryContextMenu(secondTab, 'file-full', 'Edit', { canEdit: true });
    await waitForDocumentLoaded(secondTab);
    await expectParagraphTexts(secondTab, ['New first line!Title\r\n']);
  });
});
