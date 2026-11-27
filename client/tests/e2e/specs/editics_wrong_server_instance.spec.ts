// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { TestInfo } from '@playwright/test';
import {
  checkEntryContextMenu,
  expect,
  expectEditorError,
  importDefaultFiles,
  ImportDocuments,
  mockWrongEditicsInstance,
  msTest,
  trackEditicsSessionRequests,
} from '@tests/e2e/helpers';

msTest.describe(() => {
  msTest.use({
    documentsOptions: {
      empty: true,
    },
  });

  msTest('Editor shows an error when the editics server does not host the document session', async ({ documents }, testInfo: TestInfo) => {
    msTest.setTimeout(240_000);
    await importDefaultFiles(documents, testInfo, ImportDocuments.Docx, false);
    // Let it process everything, avoid the refresh
    await documents.waitForTimeout(500);

    // Answer the editics session requests with the refusal the server sends
    // when the client reached an editics server instance that does not host
    // the document's session (see `mockWrongEditicsInstance`)
    const getSessionRequestCount = trackEditicsSessionRequests(documents);
    await mockWrongEditicsInstance(documents.context());

    // Open the document in the collaborative editor
    const entry = documents.locator('.folder-container').locator('.file-list-item').nth(0);
    await entry.click({ button: 'right' });
    await checkEntryContextMenu(documents, 'file-full', 'Edit', { canEdit: true });

    // Sanity check: the client did try to open a collaborative session
    await expect(async () => {
      expect(getSessionRequestCount()).toBeGreaterThan(0);
    }).toPass({ timeout: 30_000 });

    // The refusal is fatal (retrying against the same wrong editics server
    // instance would loop forever): the error flows back to the client, which
    // shows the editor error placeholder
    await expectEditorError(documents);

    // The user can leave the errored editor and land back on the documents
    // (the document never loaded, so there is no unsaved changes to ask about)
    await documents.locator('.file-editor-error').locator('.error-content-buttons__item').click();
    await expect(documents).toBeDocumentPage();
  });
});
