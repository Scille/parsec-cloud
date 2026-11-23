// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { TestInfo } from '@playwright/test';
import {
  checkEntryContextMenu,
  expect,
  expectParagraphTexts,
  importDefaultFiles,
  ImportDocuments,
  login,
  msTest,
  pressInDocument,
  promoteSecondUserToContributor,
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

  msTest('Two users edit the same document and see each other changes', async ({ documents }, testInfo: TestInfo) => {
    msTest.setTimeout(240_000);
    await importDefaultFiles(documents, testInfo, ImportDocuments.Docx, false);
    // Let it process everything, avoid the refresh
    await documents.waitForTimeout(500);

    // Promote Bob to Contributor so that he can edit the document too
    await promoteSecondUserToContributor(documents);

    // Open the document in the editor with Alice
    const aliceEntry = documents.locator('.folder-container').locator('.file-list-item').nth(0);
    await aliceEntry.click({ button: 'right' });
    await checkEntryContextMenu(documents, 'file-full', 'Edit', { canEdit: true });
    await waitForDocumentLoaded(documents);

    // Open the document in the editor with Bob: he joins the collaborative
    // session opened by Alice
    const secondTab = await documents.openNewTab();
    await login(secondTab, 'Boby McBobFace', 'bob@example.com');
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

  msTest('Open document in editor while other user is editing', async ({ documents }, testInfo: TestInfo) => {
    msTest.setTimeout(240_000);
    await importDefaultFiles(documents, testInfo, ImportDocuments.Docx, false);
    // Let it process everything, avoid the refresh
    await documents.waitForTimeout(500);

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
    await login(secondTab, 'Boby McBobFace', 'bob@example.com');
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

  msTest('Save document edited by a user and reopen it', async ({ documents }, testInfo: TestInfo) => {
    msTest.setTimeout(240_000);
    await importDefaultFiles(documents, testInfo, ImportDocuments.Docx, false);
    // Let it process everything, avoid the refresh
    await documents.waitForTimeout(500);

    // Promote Bob to Contributor so that he can open the document in the editor too
    await promoteSecondUserToContributor(documents);

    // Open the document in the editor with Alice
    const aliceEntry = documents.locator('.folder-container').locator('.file-list-item').nth(0);
    await aliceEntry.click({ button: 'right' });
    await checkEntryContextMenu(documents, 'file-full', 'Edit', { canEdit: true });
    await waitForDocumentLoaded(documents);

    // Make some edits and wait for the document to be saved into the workspace file
    await typeInDocument(documents, 'New first line!');
    await expectParagraphTexts(documents, ['New first line!Title\r\n']);
    await waitUntilSaved(documents);

    // Leave the editor
    const topbar = documents.locator('.file-handler-topbar');
    await topbar.locator('.back-button').click();
    await expect(documents).toBeDocumentPage();

    // Reopen the document in the editor with Alice: the changes must have been
    // persisted into the workspace file
    const reopenedEntry = documents.locator('.folder-container').locator('.file-list-item').nth(0);
    await reopenedEntry.click({ button: 'right' });
    await checkEntryContextMenu(documents, 'file-full', 'Edit', { canEdit: true });
    await waitForDocumentLoaded(documents);
    await expectParagraphTexts(documents, ['New first line!Title\r\n']);

    // Bob also sees the modification when he opens the document
    const secondTab = await documents.openNewTab();
    await login(secondTab, 'Boby McBobFace', 'bob@example.com');
    await secondTab.locator('.workspaces-container-grid').locator('.workspace-card-item').nth(0).click();
    await expect(secondTab).toBeDocumentPage();
    const bobEntry = secondTab.locator('.folder-container').locator('.file-list-item').nth(0);
    await bobEntry.click({ button: 'right' });
    await checkEntryContextMenu(secondTab, 'file-full', 'Edit', { canEdit: true });
    await waitForDocumentLoaded(secondTab);
    await expectParagraphTexts(secondTab, ['New first line!Title\r\n']);
  });
});
