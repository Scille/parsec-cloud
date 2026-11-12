// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// Temporary sandbox: does rapid synthetic typing (insertText immediately
// followed by Enter) lose content in the OFFLINE editics mode (single user)?
// Same fork build + wrapper + loadBinary as the collaborative mode, minus
// the co-authoring protocol: isolates the editor's input handling from the
// editics session machinery.
import { TestInfo } from '@playwright/test';
import {
  expect,
  getParagraphTexts,
  importDefaultFiles,
  ImportDocuments,
  msTest,
  pressInDocument,
  typeInDocument,
  waitForDocumentLoaded,
} from '@tests/e2e/helpers';

msTest.describe(() => {
  msTest.use({ documentsOptions: { empty: true } });

  msTest('Sandbox: offline rapid typing does not lose content', async ({ documents }, testInfo: TestInfo) => {
    msTest.setTimeout(240_000);
    await importDefaultFiles(documents, testInfo, ImportDocuments.Docx, false);
    await documents.waitForTimeout(500);

    // Open the editor (offline is the default in e2e)
    const aliceEntries = documents.locator('.folder-container').locator('.file-list-item');
    await aliceEntries.nth(0).click({ button: 'right' });
    const menu = documents.locator('#file-context-menu');
    await expect(menu.getByRole('listitem').nth(2)).toHaveText('Edit');
    await menu.getByRole('listitem').nth(2).click();
    await waitForDocumentLoaded(documents);

    // Zero-delay type/Enter bursts in several places
    for (let i = 0; i < 4; i++) {
      await typeInDocument(documents, `P${i + 1} `);
      await pressInDocument(documents, 'Enter');
    }
    await typeInDocument(documents, 'P5 ');
    await expectParagraphs(documents, ['P1 \r\n', 'P2 \r\n', 'P3 \r\n', 'P4 \r\n', 'P5 \r\n']);

    // Char-by-char, no delay
    const hostFrame = documents.frames().find((frame) => /\/editics\/(offline|collaborative)\.html/.test(frame.url()));
    const editorFrame = documents.frameLocator('.file-editor').locator('#editics-container').frameLocator('iframe');
    await editorFrame.locator('#editor-container').focus();
    for (let i = 0; i < 4; i++) {
      await documents.keyboard.type(`C${i + 1} `, { delay: 5 });
      await documents.keyboard.press('Enter');
    }
    await documents.keyboard.type('C5 ', { delay: 5 });
    await expectParagraphs(documents, [
      'P1 \r\n',
      'P2 \r\n',
      'P3 \r\n',
      'P4 \r\n',
      'P5 \r\n',
      'C1 \r\n',
      'C2 \r\n',
      'C3 \r\n',
      'C4 \r\n',
      'C5 \r\n',
    ]);

    // Some back-and-forth edits in multiple places
    await pressInDocument(documents, 'Control+Home');
    await typeInDocument(documents, 'X1 ');
    await pressInDocument(documents, 'Control+End');
    await pressInDocument(documents, 'Enter');
    await typeInDocument(documents, 'X2 ');
    await expectParagraphs(documents, [
      'X1 P1 \r\n',
      'P2 \r\n',
      'P3 \r\n',
      'P4 \r\n',
      'P5 \r\n',
      'C1 \r\n',
      'C2 \r\n',
      'C3 \r\n',
      'C4 \r\n',
      'C5 \r\n',
      'X2 \r\n',
    ]);

    console.log('OFFLINE PARAGRAPHS:', JSON.stringify(await getParagraphTexts(documents)));
    console.log('HOST FRAME:', hostFrame?.url());
  });
});

async function expectParagraphs(documents: any, expected: string[]): Promise<void> {
  await expect(async () => {
    const texts = (await getParagraphTexts(documents)) ?? [];
    const all = texts.join('');
    for (const line of expected) {
      if (!all.includes(line)) {
        throw new Error(`missing ${JSON.stringify(line)} in ${JSON.stringify(texts)}`);
      }
    }
  }).toPass({ timeout: 20_000 });
}
