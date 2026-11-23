// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { TestInfo } from '@playwright/test';
import {
  checkEntryContextMenu,
  expect,
  expectParagraphTexts,
  importDefaultFiles,
  ImportDocuments,
  mockLibParsec,
  MsPage,
  msTest,
  typeInDocument,
  waitForDocumentLoaded,
} from '@tests/e2e/helpers';

msTest.describe(() => {
  msTest.use({
    documentsOptions: {
      empty: true,
    },
  });

  async function mockServerOffline(page: MsPage): Promise<void> {
    await mockLibParsec(page, [
      {
        name: 'clientInfo',
        result: {
          ok: true,
          value: {
            organizationAddr: 'parsec3://addr',
            organizationId: 'BlackMesa',
            deviceId: 'deviceId',
            userId: 'userId',
            deviceLabel: 'Device Label',
            humanHandle: { email: 'alice@example.com', label: 'Alicey McAliceFace' },
            currentProfile: 'UserProfileAdmin',
            serverOrganizationConfig: {
              userProfileOutsiderAllowed: false,
              activeUsersLimit: { tag: 'ActiveUsersLimitLimitedTo', x1: 5 },
              realmMinimumArchivingPeriodBeforeDeletion: 1200,
            },
            isServerOnline: false, // Here!
            isOrganizationExpired: false,
            mustAcceptTos: false,
          },
        },
      },
    ]);
  }

  async function openDocument(page: MsPage): Promise<void> {
    const entry = page.locator('.folder-container').locator('.file-list-item').nth(0);
    await entry.click({ button: 'right' });
    await checkEntryContextMenu(page, 'file-full', 'Edit', { canEdit: true });
  }

  msTest('Offline editing mode is proposed when the client is offline', async ({ documents }, testInfo: TestInfo) => {
    msTest.setTimeout(240_000);
    await importDefaultFiles(documents, testInfo, ImportDocuments.Docx, false);
    // Let it process everything, avoid the refresh
    await documents.waitForTimeout(500);

    await mockServerOffline(documents);
    await openDocument(documents);

    // The user is asked to go back or use the offline edition mode
    const modal = documents.locator('.editor-issue-modal');
    await expect(modal).toBeVisible();
    await expect(modal.locator('.ms-modal-header__title')).toHaveText('Collaborative editing is not available');
    await expect(modal.locator('#next-button')).toHaveText('Open in offline mode');
    await expect(modal.locator('#cancel-button')).toHaveText('Back to files');

    // Using the offline mode opens the document, which can then be edited
    // in isolation
    await modal.locator('#next-button').click();
    await expect(modal).toBeHidden();
    await waitForDocumentLoaded(documents);
    await typeInDocument(documents, 'Offline edit!');
    await expectParagraphTexts(documents, ['Offline edit!Title\r\n']);
  });

  msTest('Going back when asked about the offline editing mode', async ({ documents }, testInfo: TestInfo) => {
    msTest.setTimeout(240_000);
    await importDefaultFiles(documents, testInfo, ImportDocuments.Docx, false);
    // Let it process everything, avoid the refresh
    await documents.waitForTimeout(500);

    await mockServerOffline(documents);
    await openDocument(documents);

    // Going back leaves the document unopened
    const modal = documents.locator('.editor-issue-modal');
    await expect(modal).toBeVisible();
    await modal.locator('#cancel-button').click();
    await expect(modal).toBeHidden();
    await expect(documents).toBeDocumentPage();
    await expect(documents.locator('.file-editor')).toBeHidden();
  });
});
