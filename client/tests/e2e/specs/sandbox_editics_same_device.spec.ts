// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// Temporary sandbox for developing the collaborative editor tests.
// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { TestInfo } from '@playwright/test';
import {
  enableCollaborativeMode,
  expect,
  expectParagraphTexts,
  getParagraphTexts,
  importDefaultFiles,
  ImportDocuments,
  msTest,
  pressInDocument,
  promoteSecondUserToContributor,
  typeInDocument,
  waitForDocumentLoaded,
} from '@tests/e2e/helpers';

msTest.use({ documentsOptions: { empty: true } });

msTest('Sandbox: collaborative editing with same device two tabs', async ({ documents }, testInfo: TestInfo) => {
  msTest.setTimeout(240_000);
  await importDefaultFiles(documents, testInfo, ImportDocuments.Docx, false);
  await documents.waitForTimeout(500);

  // Enable collaborative mode (this is what we want to test)
  await enableCollaborativeMode(documents);

  // Promote Bob to Contributor
  await promoteSecondUserToContributor(documents);

  // Open the editor with Alice
  const aliceEntries = documents.locator('.folder-container').locator('.file-list-item');
  await aliceEntries.nth(0).click({ button: 'right' });
  const menu = documents.locator('#file-context-menu');
  await expect(menu.getByRole('listitem').nth(2)).toHaveText('Edit');
  await menu.getByRole('listitem').nth(2).click();
  await waitForDocumentLoaded(documents);

  // Open the editor with a second Alice tab (same device, another connection)
  const secondTab = await documents.openNewTab();
  await enableCollaborativeMode(secondTab);
  // The org is already active from the first tab: clicking the card either
  // asks for the password (fresh session) or enters directly.
  const card = secondTab.locator('.organization-card', { hasText: 'Alicey McAliceFace' }).first();
  await card.waitFor({ state: 'visible', timeout: 30000 });
  await card.click();
  const password = secondTab.locator('#password-input');
  if (await password.isVisible().catch(() => false)) {
    await password.locator('input').fill('P@ssw0rd.');
    await secondTab.locator('.login-button').click();
  }
  await secondTab.locator('.workspaces-container-grid').waitFor({ state: 'visible', timeout: 30000 });
  await secondTab.locator('.workspaces-container-grid').locator('.workspace-card-item').nth(0).click();
  await expect(secondTab).toBeDocumentPage();
  const bobEntries = secondTab.locator('.folder-container').locator('.file-list-item');
  await bobEntries.nth(0).click({ button: 'right' });
  const bobMenu = secondTab.locator('#file-context-menu');
  await expect(bobMenu.getByRole('listitem').nth(2)).toHaveText('Edit');
  await bobMenu.getByRole('listitem').nth(2).click();
  await waitForDocumentLoaded(secondTab);

  console.log('BOB JOINED, ALICE PARAGRAPHS:', JSON.stringify(await getParagraphTexts(documents)));
  console.log('BOB PARAGRAPHS:', JSON.stringify(await getParagraphTexts(secondTab)));

  // Type with Alice
  await typeInDocument(documents, 'HELLOALICE');
  await expectParagraphTexts(documents, ['HELLOALICETitle\r\n']);
  console.log('ALICE TYPED TEXT, ALICE PARAGRAPHS:', JSON.stringify(await getParagraphTexts(documents)));
  // Bob should see the text
  await expectParagraphTexts(secondTab, ['HELLOALICETitle\r\n']);
  console.log('BOB PARAGRAPHS AFTER ALICE TYPED:', JSON.stringify(await getParagraphTexts(secondTab)));

  // Press Enter with Alice (insert a new line)
  await pressInDocument(documents, 'Enter');
  await expectParagraphTexts(documents, ['HELLOALICE\r\n', 'Title\r\n']);
  console.log('ALICE PRESSED ENTER, ALICE PARAGRAPHS:', JSON.stringify(await getParagraphTexts(documents)));
  await expectParagraphTexts(secondTab, ['HELLOALICE\r\n', 'Title\r\n']);
  console.log('BOB PARAGRAPHS AFTER ALICE ENTER:', JSON.stringify(await getParagraphTexts(secondTab)));

  // Type with Bob (at the very beginning of the document: his cursor has not
  // been moved by Alice's changes, so his text lands before hers)
  await pressInDocument(secondTab, 'Control+Home');
  await typeInDocument(secondTab, 'HELLOBOB');
  await expectParagraphTexts(secondTab, ['HELLOBOBHELLOALICE\r\n', 'Title\r\n']);
  console.log('BOB TYPED, BOB PARAGRAPHS:', JSON.stringify(await getParagraphTexts(secondTab)));
  await expectParagraphTexts(documents, ['HELLOBOBHELLOALICE\r\n', 'Title\r\n']);
  console.log('ALICE PARAGRAPHS AFTER BOB TYPED:', JSON.stringify(await getParagraphTexts(documents)));
});
