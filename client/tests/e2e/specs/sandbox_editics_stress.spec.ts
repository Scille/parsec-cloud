// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// Temporary sandbox: advanced collaborative-editing stress scenarios:
//   - lots of interleaved/simultaneous changes and new lines in multiple
//     places from both participants
//   - strict co-editing mode: paragraph locks from both sides, a held lock
//     must protect its paragraph (no lock lost), released and reused
//   - a save from one side while the other types (save-lock contention)
//   - a late third participant joining (backlog replay)
//   - everything must converge to the same content on all sides.
import type { Frame } from '@playwright/test';
import { TestInfo } from '@playwright/test';
import {
  enableCollaborativeMode,
  expect,
  getParagraphTexts,
  importDefaultFiles,
  ImportDocuments,
  login,
  msTest,
  pressInDocument,
  promoteSecondUserToContributor,
  saveDocument,
  typeInDocument,
  waitForDocumentLoaded,
} from '@tests/e2e/helpers';
import { appendFileSync, writeFileSync } from 'fs';

function mark(label: string): void {
  appendFileSync(PROTO_LOG, `${JSON.stringify({ t: Date.now(), mark: label })}\n`);
}

// Full session-protocol capture for offline analysis (request + reply bodies).
const PROTO_LOG = '/tmp/stress-protocol.log';
function captureSession(page: any, name: string): void {
  page.on('response', async (response: any) => {
    try {
      const url = response.url();
      if (!url.includes('/editics/sessions/') || !url.includes('/send/')) return;
      const req = JSON.parse(response.request().postData() ?? '{}');
      const text = await response.text().catch(() => '');
      const rep = text ? JSON.parse(text) : null;
      if (req.type === 'saveChanges' || req.type === 'isSaveLock' || req.type === 'unLockDocument') {
        appendFileSync(
          PROTO_LOG,
          `${JSON.stringify({
            t: Date.now(),
            tab: name,
            req,
            rep: rep?.type === 'saveChanges' ? null : rep,
            repType: rep?.type ?? null,
          })}\n`,
        );
      }
    } catch {
      // ignore
    }
  });
}

msTest.use({ documentsOptions: { empty: true } });

async function getHostFrame(page: any): Promise<Frame> {
  const frame = page.frames().find((f) => /\/editics\/(offline|collaborative)\.html/.test(f.url()));
  if (frame === undefined) throw new Error('Cannot find the editics host frame');
  return frame;
}

// Editor-side helpers, run in the host frame.
async function setCoEditingMode(page: any, fast: boolean): Promise<void> {
  const hostFrame = await getHostFrame(page);
  await hostFrame.evaluate((isFast) => {
    const api = (window as any).ooDocEditor?.getApi();
    if (!api || typeof api.asc_SetFastCollaborative !== 'function') {
      throw new Error('asc_SetFastCollaborative not available');
    }
    api.asc_SetFastCollaborative(isFast);
  }, fast);
}

async function editorSave(page: any): Promise<void> {
  const hostFrame = await getHostFrame(page);
  await hostFrame.evaluate(() => {
    (window as any).ooDocEditor?.save();
  });
}

async function isDocumentModified(page: any): Promise<boolean | null> {
  const hostFrame = await getHostFrame(page);
  return hostFrame.evaluate(() => {
    const api = (window as any).ooDocEditor?.getApi();
    return api ? !!api.isDocumentModified() : null;
  });
}

// The two documents must show the exact same paragraphs.
async function expectConvergence(pages: any[], label: string, timeout = 30_000): Promise<string[]> {
  let last: string[] | null = null;
  await expect(async () => {
    const texts = await Promise.all(pages.map((page) => getParagraphTexts(page)));
    last = texts[0] ?? [];
    for (const text of texts.slice(1)) {
      if (JSON.stringify(text) !== JSON.stringify(last)) {
        throw new Error(`no convergence (${label}):\n  A: ${JSON.stringify(last)}\n  B: ${JSON.stringify(text)}`);
      }
    }
  }).toPass({ timeout });
  return last ?? [];
}

// All the documents settled (all changes flushed to the session).
async function expectSettled(pages: any[], label: string, timeout = 60_000): Promise<void> {
  await expect(async () => {
    for (const page of pages) {
      const modified = await isDocumentModified(page);
      if (modified !== false) {
        throw new Error(`document still modified (${label}): ${modified}`);
      }
    }
  }).toPass({ timeout });
}

// The document must contain each expected line (as part of any paragraph).
async function expectContains(page: any, lines: string[], label: string, timeout = 30_000): Promise<void> {
  await expect(async () => {
    const texts = (await getParagraphTexts(page)) ?? [];
    const all = texts.join('\n');
    for (const line of lines) {
      if (!all.includes(line)) {
        throw new Error(`missing ${JSON.stringify(line)} (${label}) in ${JSON.stringify(texts)}`);
      }
    }
  }).toPass({ timeout });
}

msTest('Sandbox: collaborative editing stress', async ({ documents }, testInfo: TestInfo) => {
  msTest.setTimeout(300_000);
  writeFileSync(PROTO_LOG, '');
  captureSession(documents, 'A');
  await importDefaultFiles(documents, testInfo, ImportDocuments.Docx, false);
  await documents.waitForTimeout(500);

  await enableCollaborativeMode(documents);

  // Promote Bob to Contributor (so he can edit too)
  await promoteSecondUserToContributor(documents);

  // Open the editor with Alice (first tab)
  const aliceEntries = documents.locator('.folder-container').locator('.file-list-item');
  await aliceEntries.nth(0).click({ button: 'right' });
  const menu = documents.locator('#file-context-menu');
  await expect(menu.getByRole('listitem').nth(2)).toHaveText('Edit');
  await menu.getByRole('listitem').nth(2).click();
  await waitForDocumentLoaded(documents);

  // Open the editor with Bob (second tab)
  const secondTab = await documents.openNewTab();
  captureSession(secondTab, 'B');
  await enableCollaborativeMode(secondTab);
  await login(secondTab, 'Boby McBobFace');
  await secondTab.locator('.workspaces-container-grid').locator('.workspace-card-item').nth(0).click();
  await expect(secondTab).toBeDocumentPage();
  const bobEntries = secondTab.locator('.folder-container').locator('.file-list-item');
  await bobEntries.nth(0).click({ button: 'right' });
  const bobMenu = secondTab.locator('#file-context-menu');
  await expect(bobMenu.getByRole('listitem').nth(2)).toHaveText('Edit');
  await bobMenu.getByRole('listitem').nth(2).click();
  await waitForDocumentLoaded(secondTab);

  // =========================================================================
  // 1. Structure: Alice builds a 5-paragraph block
  // =========================================================================
  for (let i = 0; i < 4; i++) {
    await typeInDocument(documents, `P${i + 1} `);
    await pressInDocument(documents, 'Enter');
  }
  await typeInDocument(documents, 'P5 ');
  await expectConvergence([documents, secondTab], 'structure');
  await expectSettled([documents, secondTab], 'structure');
  await expectContains(documents, ['P1 ', 'P2 ', 'P3 ', 'P4 ', 'P5 '], 'structure');

  // =========================================================================
  // 2. Interleaved bursts: Alice appends to the top paragraph, Bob to the
  //    bottom one, 10 rounds
  // =========================================================================
  // Human-paced interleaved bursts (each round leaves time for the change to
  // flow and settle: a realistic co-editing rhythm).
  for (let round = 0; round < 10; round++) {
    await pressInDocument(documents, 'Control+Home');
    await typeInDocument(documents, `a${round} `);
    await secondTab.waitForTimeout(200);
    await pressInDocument(secondTab, 'Control+End');
    await pressInDocument(secondTab, 'Home');
    await typeInDocument(secondTab, `b${round} `);
    await secondTab.waitForTimeout(200);
  }
  await expectConvergence([documents, secondTab], 'interleaved bursts');
  await expectSettled([documents, secondTab], 'interleaved bursts');
  // All bursts present on both sides, and the same content everywhere.
  // (Note: the burst ORDER is not asserted: under concurrent remote changes
  // the editor leaves the cursor at a stuck anchor for a while — the same
  // happens against the real OnlyOffice server — so a burst sequence can
  // come out in a surprising order. What matters is that nothing is lost
  // and both sides converge to the same document.)
  for (const page of [documents, secondTab]) {
    await expect(async () => {
      const texts = ((await getParagraphTexts(page)) ?? []).join('\n');
      for (let i = 0; i < 10; i++) {
        if (!texts.includes(`a${i} `)) throw new Error(`missing a${i} on this side`);
        if (!texts.includes(`b${i} `)) throw new Error(`missing b${i} on this side`);
      }
    }).toPass({ timeout: 30_000 });
  }

  // =========================================================================
  // 3. Simultaneous typing: both type 10 chunks at the same time, each in
  //    its own paragraph (Alice in P2, Bob in P4)
  // =========================================================================
  const typeBursts = async (page: any, downs: number, prefix: string): Promise<void> => {
    await pressInDocument(page, 'Control+Home');
    for (let i = 0; i < downs; i++) await pressInDocument(page, 'ArrowDown');
    await pressInDocument(page, 'End');
    for (let chunk = 0; chunk < 10; chunk++) {
      await typeInDocument(page, `${prefix}${chunk} `);
    }
  };
  await Promise.all([typeBursts(documents, 1, 'A'), typeBursts(secondTab, 3, 'B')]);
  await expectConvergence([documents, secondTab], 'simultaneous typing', 60_000);
  await expectSettled([documents, secondTab], 'simultaneous typing');
  const burstA = 'A0 A1 A2 A3 A4 A5 A6 A7 A8 A9 ';
  const burstB = 'B0 B1 B2 B3 B4 B5 B6 B7 B8 B9 ';
  await expectContains(documents, [burstA, burstB], 'simultaneous typing');
  await expectContains(secondTab, [burstA, burstB], 'simultaneous typing');

  // =========================================================================
  // 4. New lines everywhere: 10 interleaved Enters from both sides
  // =========================================================================
  for (let round = 0; round < 10; round++) {
    await pressInDocument(documents, 'Control+Home');
    for (let i = 0; i < (round % 3) + 1; i++) await pressInDocument(documents, 'ArrowDown');
    await pressInDocument(documents, 'Enter');
    await typeInDocument(documents, `N${round} `);
    await pressInDocument(secondTab, 'End');
    await pressInDocument(secondTab, 'Enter');
    await typeInDocument(secondTab, `M${round} `);
  }
  await expectConvergence([documents, secondTab], 'new lines', 60_000);
  await expectSettled([documents, secondTab], 'new lines');
  await expectContains(documents, ['N0 ', 'N9 ', 'M0 ', 'M9 '], 'new lines');
  await expectContains(secondTab, ['N0 ', 'N9 ', 'M0 ', 'M9 '], 'new lines');

  // =========================================================================
  // 5. Strict mode: in strict co-editing the paragraph lock is taken as soon
  //    as the user starts typing in a paragraph, and the typed changes are
  //    only sent to the session on an explicit save (that's the point of
  //    strict mode). So:
  //    - Alice types in the first paragraph (lock taken, changes unsaved)
  //    - Bob attempts the same paragraph: his client must refuse (lock held)
  //    - Alice saves: changes propagate and her lock is released
  //    - Bob can now edit that paragraph too
  // =========================================================================
  await setCoEditingMode(documents, false);
  await setCoEditingMode(secondTab, false);
  await documents.waitForTimeout(1000);

  mark('phase5-strict-switch');
  // Alice takes the first paragraph's lock with unsaved changes
  await pressInDocument(documents, 'Control+Home');
  await typeInDocument(documents, 'LOCKEDA ');
  mark('phase5a-A-types-LOCKEDA');
  await expectContains(documents, ['LOCKEDA '], 'A lock typing (local)');

  // Bob tries to type in Alice's locked paragraph: the lock must protect it
  // (his client refuses the edit locally, based on the lock table).
  const bobTextsBefore = JSON.stringify((await getParagraphTexts(secondTab)) ?? []);
  await pressInDocument(secondTab, 'Control+Home');
  await pressInDocument(secondTab, 'End');
  await typeInDocument(secondTab, 'HACKED ');
  await secondTab.waitForTimeout(3000);
  const bobTextsAfter = JSON.stringify((await getParagraphTexts(secondTab)) ?? []);
  expect(bobTextsAfter).toBe(bobTextsBefore);

  // Alice saves: the changes propagate and her paragraph lock is released
  mark('phase5c-A-save');
  await editorSave(documents);
  await expect(async () => {
    const texts = ((await getParagraphTexts(secondTab)) ?? []).join('\n');
    if (!texts.includes('LOCKEDA ')) {
      throw new Error('LOCKEDA did not propagate after the save');
    }
  }).toPass({ timeout: 30_000 });
  await expectSettled([documents, secondTab], 'after A save');

  // Bob takes the (now released) paragraph and edits it, then saves
  await pressInDocument(secondTab, 'Control+Home');
  await typeInDocument(secondTab, 'AFTERLOCK ');
  await expectContains(secondTab, ['AFTERLOCK '], 'B edit after lock release (local)');
  await editorSave(secondTab);
  await expect(async () => {
    const texts = ((await getParagraphTexts(documents)) ?? []).join('\n');
    if (!texts.includes('AFTERLOCK ')) {
      throw new Error('AFTERLOCK did not propagate after the save');
    }
  }).toPass({ timeout: 30_000 });
  await expectConvergence([documents, secondTab], 'strict mode end');
  await expectSettled([documents, secondTab], 'strict mode end');

  // Back to fast mode
  await setCoEditingMode(documents, true);
  await setCoEditingMode(secondTab, true);
  await documents.waitForTimeout(1000);
  await expectSettled([documents, secondTab], 'back to fast');

  // =========================================================================
  // 6. Bob types while Alice saves (save-lock contention)
  // =========================================================================
  const typingWhileSaving = (async () => {
    for (let i = 0; i < 10; i++) {
      await pressInDocument(secondTab, 'End');
      await typeInDocument(secondTab, `W${i} `);
    }
  })();
  await editorSave(documents);
  await typingWhileSaving;
  await expectConvergence([documents, secondTab], 'typing while saving', 60_000);
  await expectContains(documents, ['W0 ', 'W9 '], 'typing while saving');
  await expectSettled([documents, secondTab], 'typing while saving');

  // =========================================================================
  // 7. Manual save (the workspace file persistence path) then a late joiner
  // =========================================================================
  await saveDocument((documents as any).frameLocator('.file-editor'));
  await documents.waitForTimeout(2000);

  const finalTexts = await getParagraphTexts(documents);
  const thirdTab = await documents.openNewTab();
  await enableCollaborativeMode(thirdTab);
  await login(thirdTab, 'Alicey McAliceFace');
  await thirdTab.locator('.workspaces-container-grid').locator('.workspace-card-item').nth(0).click();
  await expect(thirdTab).toBeDocumentPage();
  const thirdEntries = thirdTab.locator('.folder-container').locator('.file-list-item');
  await thirdEntries.nth(0).click({ button: 'right' });
  const thirdMenu = thirdTab.locator('#file-context-menu');
  await expect(thirdMenu.getByRole('listitem').nth(2)).toHaveText('Edit');
  await thirdMenu.getByRole('listitem').nth(2).click();
  await waitForDocumentLoaded(thirdTab);

  // The late joiner replays the whole backlog on top of the original content
  await expect(async () => {
    const texts = (await getParagraphTexts(thirdTab)) ?? [];
    if (JSON.stringify(texts) !== JSON.stringify(finalTexts)) {
      throw new Error('late joiner did not converge');
    }
  }).toPass({ timeout: 60_000 });
  await expectConvergence([documents, secondTab, thirdTab], 'late joiner');
  await expectSettled([documents, secondTab, thirdTab], 'late joiner');
});
