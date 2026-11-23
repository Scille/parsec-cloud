// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { TestInfo } from '@playwright/test';
import {
  answerQuestion,
  changeSecondUserRole,
  checkEntryContextMenu,
  checkWorkspaceContextMenu,
  expect,
  expectEditorError,
  expectParagraphTexts,
  importDefaultFiles,
  ImportDocuments,
  login,
  MsPage,
  msTest,
  promoteSecondUserToContributor,
  trackEditicsSessionRequests,
  typeInDocument,
  waitForDocumentLoaded,
} from '@tests/e2e/helpers';

msTest.describe(() => {
  msTest.use({
    documentsOptions: {
      empty: true,
    },
  });

  // Open the imported document in the collaborative editor as Bob from a second tab.
  async function openDocumentCollaborativelyAsBob(page: MsPage): Promise<{ bobTab: MsPage; getSessionRequestCount: () => number }> {
    // Promote Bob to Contributor so that he can edit the document too
    await promoteSecondUserToContributor(page);

    const bobTab = await page.openNewTab();
    await login(bobTab, 'Boby McBobFace', 'bob@example.com');
    const getSessionRequestCount = trackEditicsSessionRequests(bobTab);
    await bobTab.locator('.workspaces-container-grid').locator('.workspace-card-item').nth(0).click();
    await expect(bobTab).toBeDocumentPage();
    const bobEntry = bobTab.locator('.folder-container').locator('.file-list-item').nth(0);
    await bobEntry.click({ button: 'right' });
    await checkEntryContextMenu(bobTab, 'file-full', 'Edit', { canEdit: true });
    await waitForDocumentLoaded(bobTab);
    return { bobTab, getSessionRequestCount };
  }

  msTest('Open a document of an archived workspace must be in offline mode', async ({ documents }, testInfo: TestInfo) => {
    msTest.setTimeout(240_000);
    await importDefaultFiles(documents, testInfo, ImportDocuments.Docx, false);
    // Let it process everything, avoid the refresh
    await documents.waitForTimeout(500);

    // Archive wksp1 (the collaborative edition mode is the default, the client
    // being online)
    await documents.locator('.topbar-left').locator('.back-button').click();
    await expect(documents).toBeWorkspacePage();
    const wk = documents.locator('.workspaces-container-grid').locator('.workspace-card-item').nth(0);
    await wk.click({ button: 'right' });
    await checkWorkspaceContextMenu(documents, 'owner', 'Archive this workspace');
    await answerQuestion(documents, true);
    await expect(documents).toShowToast('The workspace wksp1 has successfully been archived.', 'Success');
    await expect(wk).not.toBeVisible();

    // A collaborative session is only ever opened through the editics session
    // routes: no request on them means the offline editor is used
    const getSessionRequestCount = trackEditicsSessionRequests(documents);

    // Open the document from the archived workspace (in read-only)
    const sidebarArchiveButton = documents.locator('.sidebar').locator('#sidebar-workspaces').locator('#sidebar-archived-workspaces');
    await sidebarArchiveButton.click();
    await expect(documents).toHavePageTitle('Archived workspaces');
    await expect(wk).toBeVisible();
    await wk.click();
    await expect(documents).toBeDocumentPage();
    const entry = documents.locator('.folder-container').locator('.file-list-item').nth(0);
    await entry.click({ button: 'right' });
    await checkEntryContextMenu(documents, 'file-readonly', 'Preview');

    // The document opens in the editor, in offline mode
    await waitForDocumentLoaded(documents);
    await expectParagraphTexts(documents, ['Title\r\n']);
    expect(getSessionRequestCount()).toBe(0);
  });

  msTest('Editor shows an error when the user loses access to the realm while editing', async ({ documents }, testInfo: TestInfo) => {
    msTest.setTimeout(240_000);
    await importDefaultFiles(documents, testInfo, ImportDocuments.Docx, false);
    // Let it process everything, avoid the refresh
    await documents.waitForTimeout(500);

    // Bob opens the document in the collaborative editor
    const { bobTab, getSessionRequestCount } = await openDocumentCollaborativelyAsBob(documents);
    // Sanity check: Bob is in a collaborative session, so his editor talks
    // to the editics server through the session routes
    await expect(async () => {
      expect(getSessionRequestCount()).toBeGreaterThan(0);
    }).toPass({ timeout: 30_000 });

    // Bob has some (unsaved) changes when he loses the access to the workspace
    await typeInDocument(bobTab, 'Bob edit!');
    await expectParagraphTexts(bobTab, ['Bob edit!Title\r\n']);
    await changeSecondUserRole(documents, 'Contributor', 'Not shared');

    // The editics server closed Bob's session and now refuses his reconnection
    // attempts for good (no realm role anymore): the error flows back to the
    // client, which shows the editor error placeholder
    await expectEditorError(bobTab);

    // The user can leave the errored editor: the changes cannot be saved
    // anymore (the session and the workspace are both gone), so the user is
    // asked whether to discard them
    await bobTab.locator('.file-editor-error').locator('.error-content-buttons__item').click();
    await answerQuestion(bobTab, true, { expectedTitleText: 'Discard changes?' });
    await expect(bobTab.locator('.file-editor-error')).toBeHidden();
    await expect(bobTab.locator('.file-editor')).toBeHidden();
  });

  msTest('Editor shows an error when the realm gets archived while editing', async ({ documents }, testInfo: TestInfo) => {
    msTest.setTimeout(240_000);
    await importDefaultFiles(documents, testInfo, ImportDocuments.Docx, false);
    // Let it process everything, avoid the refresh
    await documents.waitForTimeout(500);

    // Bob opens the document in the collaborative editor
    const { bobTab, getSessionRequestCount } = await openDocumentCollaborativelyAsBob(documents);
    // Sanity check: Bob is in a collaborative session, so his editor talks
    // to the editics server through the session routes
    await expect(async () => {
      expect(getSessionRequestCount()).toBeGreaterThan(0);
    }).toPass({ timeout: 30_000 });

    // Bob has some (unsaved) changes when the workspace gets archived
    await typeInDocument(bobTab, 'Bob edit!');
    await expectParagraphTexts(bobTab, ['Bob edit!Title\r\n']);

    // Alice archives wksp1, kicking every participant of its realms out of
    // their editics sessions
    await documents.locator('.topbar-left').locator('.back-button').click();
    await expect(documents).toBeWorkspacePage();
    const wk = documents.locator('.workspaces-container-grid').locator('.workspace-card-item').nth(0);
    await wk.click({ button: 'right' });
    await checkWorkspaceContextMenu(documents, 'owner', 'Archive this workspace');
    await answerQuestion(documents, true);
    await expect(documents).toShowToast('The workspace wksp1 has successfully been archived.', 'Success');

    // The editics server closed Bob's session and now refuses his reconnection
    // attempts for good (the realm is archived, hence read-only): the error
    // flows back to the client, which shows the editor error placeholder
    await expectEditorError(bobTab);

    // The user can leave the errored editor: the changes cannot be saved
    // anymore, so the user is asked whether to discard them, then lands back
    // on the (read-only) documents of the archived workspace
    await bobTab.locator('.file-editor-error').locator('.error-content-buttons__item').click();
    await answerQuestion(bobTab, true, { expectedTitleText: 'Discard changes?' });
    await expect(bobTab).toBeDocumentPage();
  });
});
