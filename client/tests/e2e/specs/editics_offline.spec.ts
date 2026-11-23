// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { TestInfo } from '@playwright/test';
import {
  checkEntryContextMenu,
  expect,
  expectParagraphTexts,
  getEditorFrame,
  importDefaultFiles,
  ImportDocuments,
  msTest,
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
    editicsMode: 'offline',
  });

  msTest('Open, edit, save and reopen a document in offline mode', async ({ documents }, testInfo: TestInfo) => {
    msTest.setTimeout(240_000);
    await importDefaultFiles(documents, testInfo, ImportDocuments.Docx, false);
    // Let it process everything, avoid the refresh
    await documents.waitForTimeout(500);

    // Open the document in the editor (forcing the offline mode skips the
    // collaborative session altogether, even though the client is online)
    const entry = documents.locator('.folder-container').locator('.file-list-item').nth(0);
    await entry.click({ button: 'right' });
    await checkEntryContextMenu(documents, 'file-full', 'Edit', { canEdit: true });
    await waitForDocumentLoaded(documents);

    // Modify the document
    await typeInDocument(documents, 'Offline edit!');
    await expectParagraphTexts(documents, ['Offline edit!Title\r\n']);

    // Save the document and leave the editor
    const frame = await getEditorFrame(documents);
    await saveDocument(frame);
    await waitUntilSaved(documents);
    const topbar = documents.locator('.file-handler-topbar');
    await topbar.locator('.back-button').click();
    await expect(documents).toBeDocumentPage();

    // Reopen the document: the changes must have been persisted into the
    // workspace file
    await entry.click({ button: 'right' });
    await checkEntryContextMenu(documents, 'file-full', 'Edit', { canEdit: true });
    await waitForDocumentLoaded(documents);
    await expectParagraphTexts(documents, ['Offline edit!Title\r\n']);
  });
});
