// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { BrowserContext, Frame, FrameLocator } from '@playwright/test';
import { expect } from '@tests/e2e/helpers/assertions';
import { MsPage } from '@tests/e2e/helpers/types';

export async function waitUntilSaved(page: MsPage, timeout = 10000): Promise<void> {
  await expect(page.locator('#unsaved-changes')).toBeHidden();
  await expect(page.locator('#saved-changes')).toBeVisible({ timeout: timeout });
}

export async function getEditorFrame(page: MsPage): Promise<FrameLocator> {
  const offlineFrame = page.frameLocator('.file-editor');
  await expect(offlineFrame.locator('#editics-container')).toBeVisible();
  return offlineFrame.locator('#editics-container').frameLocator('iframe');
}

// The editics host page (i.e. the iframe serving `editics/offline.html` or
// `editics/collaborative.html`): it is the one plugging the OnlyOffice editor
// in, hence where the editor's API can be reached (see `getParagraphTexts`).
export async function getHostFrame(page: MsPage): Promise<Frame> {
  let hostFrame: Frame | undefined;
  await expect(async () => {
    hostFrame = page.frames().find((frame) => /\/editics\/(offline|collaborative)\.html/.test(frame.url()));
    if (hostFrame === undefined) {
      throw new Error('Cannot find the editics host frame');
    }
  }).toPass({ timeout: 30_000 });
  return hostFrame!;
}

export async function waitForDocumentLoaded(page: MsPage): Promise<void> {
  const hostFrame = await getHostFrame(page);
  await hostFrame.waitForFunction(() => (window as any).ooDocumentReady === true, undefined, { timeout: 90_000 });
}

// Reads the text of the document paragraphs through the editor's API: the
// OnlyOffice editor only renders the visible part of the document, so the
// DOM cannot be used to assert on the document content.
export async function getParagraphTexts(page: MsPage): Promise<string[] | null> {
  const hostFrame = await getHostFrame(page);
  return (await hostFrame.evaluate(() => {
    const api = (window as any).ooDocEditor?.getApi();
    if (!api) {
      return null;
    }
    const doc = api.WordControl?.m_oLogicDocument;
    if (!doc || !doc.Content) {
      return null;
    }
    return doc.Content.map((element: any) => {
      if (typeof element.SetApplyToAll !== 'function' || typeof element.GetSelectedText !== 'function') {
        return `<${element.Type}>`;
      }
      element.SetApplyToAll(true);
      const text = element.GetSelectedText(false, { Numbering: false });
      element.SetApplyToAll(false);
      return text ?? '';
    });
  })) as string[] | null;
}

// Checks the text of the document's first paragraphs (a `null` entry skips
// the corresponding paragraph), polling until OnlyOffice has applied the
// expected changes (the collaborative changes are not applied instantly).
export async function expectParagraphTexts(page: MsPage, expected: (string | null)[], timeout = 20_000): Promise<void> {
  await expect(async () => {
    const texts = await getParagraphTexts(page);
    for (const [index, text] of expected.entries()) {
      if (text !== null && (texts?.[index] ?? null) !== text) {
        throw new Error(`paragraph ${index}: expected ${JSON.stringify(text)}, got ${JSON.stringify(texts?.[index])}`);
      }
    }
  }).toPass({ timeout });
}

// Types some text into the currently opened document (at the current cursor
// position, like a user typing in the editor).
export async function typeInDocument(page: MsPage, text: string): Promise<void> {
  const editorFrame = await getEditorFrame(page);
  await editorFrame.locator('#editor-container').focus();
  await page.keyboard.insertText(text);
}

// Sends a key (or key combination) to the currently opened document.
export async function pressInDocument(page: MsPage, key: string): Promise<void> {
  const editorFrame = await getEditorFrame(page);
  await editorFrame.locator('#editor-container').focus();
  await page.keyboard.press(key);
}

// The e2e tests keep the editor in offline mode by default (so the suite
// doesn't depend on the still work in progress collaborative stack): a test
// can opt in with this helper (see `isCollaborativeEnabledInTests` in
// `FileEditor.vue`).
export async function enableCollaborativeMode(page: MsPage): Promise<void> {
  await page.evaluate(() => {
    (window as any).TESTING_ENABLE_EDITICS_COLLABORATIVE = true;
  });
}

// Promotes the second user of the default organization (Boby McBobFace,
// a Reader of wksp1 by default) to Contributor, so that he can edit the
// workspace files too. The testbed organization is left with Alice as the
// workspace Owner.
export async function promoteSecondUserToContributor(page: MsPage): Promise<void> {
  // The sharing modal is only accessible from the workspaces page
  await page.locator('.topbar-left').locator('.back-button').click();
  await expect(page).toBeWorkspacePage();
  await page
    .locator('.workspaces-container-grid')
    .locator('.workspace-card-item')
    .nth(0)
    .locator('.workspace-card-bottom__icons')
    .locator('.icon-share-container')
    .nth(0)
    .click();
  const sharingModal = page.locator('.workspace-sharing-modal');
  await expect(sharingModal).toBeVisible();
  const users = sharingModal.locator('.ms-modal-content').locator('.user-member-item');
  const bob = users.filter({ hasText: 'Boby McBobFace' });
  await expect(bob.locator('.dropdown-button')).toHaveText('Reader');
  await bob.locator('.dropdown-button').click();
  const roles = page.locator('.dropdown-popover').getByRole('listitem');
  await expect(roles.locator('.option-text__label')).toHaveText(['Owner', 'Manager', 'Contributor', 'Reader', 'Not shared']);
  await roles.nth(2).click();
  await expect(page).toShowToast("Boby McBobFace's role has been updated to Contributor.", 'Success');
  await sharingModal.locator('.closeBtn').click();
  await expect(sharingModal).toBeHidden();
  // Back to the documents of the first workspace
  await page.locator('.workspaces-container-grid').locator('.workspace-card-item').nth(0).click();
  await expect(page).toBeDocumentPage();
}

export async function checkDocumentTitle(frame: FrameLocator, expectedTitle: string): Promise<void> {
  await expect(frame.locator('#title-doc-name')).toHaveValue(expectedTitle);
}

export async function mockEditicsRequest(context: BrowserContext, error?: '404' | 'timeout'): Promise<void> {
  await context.route(
    (url) => url.hostname.startsWith('editics.') && /^\/editics\/(offline|collaborative)\.html$/.test(url.pathname),
    async (route) => {
      console.log('MATCHING ROUTE');
      if (error === '404') {
        await route.fulfill({
          status: 404,
          contentType: 'text/plain',
          body: '',
        });
      } else if (error === 'timeout') {
        await route.abort('timedout');
      } else {
        await route.fulfill({
          status: 200,
          contentType: 'text/html',
          body: '<html><body></body></html>',
        });
      }
    },
  );
}

export async function saveDocument(frame: FrameLocator): Promise<void> {
  await frame.locator('#box-document-title').locator('#slot-btn-dt-save').click();
}
