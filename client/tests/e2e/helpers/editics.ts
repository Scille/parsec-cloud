// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { BrowserContext, Frame, FrameLocator } from '@playwright/test';
import { expect } from '@tests/e2e/helpers/assertions';
import { MsPage } from '@tests/e2e/helpers/types';

export async function waitUntilSaved(page: MsPage, timeout = 3000): Promise<void> {
  await expect(page.locator('#saved-changes')).toBeVisible({ timeout: timeout });
}

export async function getEditorFrame(page: MsPage): Promise<FrameLocator> {
  const offlineFrame = page.frameLocator('.file-editor');
  await expect(offlineFrame.locator('#editics-container')).toBeVisible();
  return offlineFrame.locator('#editics-container').frameLocator('iframe');
}

// The editics host page (i.e. the iframe serving `editics/index.html`): it
// is the one plugging the OnlyOffice editor
// in, hence where the editor's API can be reached (see `getParagraphTexts`).
export async function getHostFrame(page: MsPage): Promise<Frame> {
  let hostFrame: Frame | undefined;
  await expect(async () => {
    hostFrame = page.frames().find((frame) => /\/editics\/index/.test(frame.url()));
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

// Promotes the second user of the default organization (Boby McBobFace,
// a Reader of wksp1 by default) to Contributor, so that he can edit the
// workspace files too. The testbed organization is left with Alice as the
// workspace Owner.
export async function promoteSecondUserToContributor(page: MsPage): Promise<void> {
  await changeSecondUserRole(page, 'Reader', 'Contributor');
}

// Tracks the requests sent to the editics session routes of the editics
// server (`{serverUrl}/authenticated/{organizationId}/editics/...`, see the
// routes in `client/editics/collaborative.ts`): those requests only ever
// happen in collaborative edition mode, and are all of them (the editics
// host page is served from a different origin, under an `editics/` path but
// never with the `authenticated/` prefix). Returns a getter on the requests
// seen so far.
export function trackEditicsSessionRequests(page: MsPage): () => number {
  let count = 0;
  page.on('request', (request) => {
    const url = request.url();
    if (url.includes('/authenticated/') && url.includes('/editics/')) {
      count += 1;
    }
  });
  return (): number => count;
}

// Asserts the error UI shown by the file editor (see `FileEditor.vue`) when
// the document cannot be opened, kept as-is, or the session was closed by the
// editics server (e.g. in collaborative mode when the user lost access to the
// realm, or when the realm got archived).
export async function expectEditorError(page: MsPage, timeout = 30_000): Promise<void> {
  const editorError = page.locator('.file-editor-error');
  await expect(editorError).toBeVisible({ timeout });
  await expect(editorError.locator('.error-content-text__title')).toHaveText('Cannot open file');
  await expect(editorError.locator('.error-content-text__message')).toHaveText('Failed to open the file');
  await expect(editorError.locator('.error-advices')).toBeVisible();
  // The editor iframe is replaced by the error UI
  await expect(page.locator('.file-editor')).toBeHidden();
}

// Opens the sharing modal of the first workspace from its owner's page
// (the `documents` fixture, logged in as Alice), and changes the role of the
// second user of the default organization (Boby McBobFace), leaving the page
// on the documents of the first workspace, like it was found.
// `expectedRole` is asserted first for sanity (the caller is expected to know
// the current role, typically set by `promoteSecondUserToContributor` above).
export async function changeSecondUserRole(
  page: MsPage,
  expectedRole: 'Owner' | 'Manager' | 'Contributor' | 'Reader',
  newRole: 'Owner' | 'Manager' | 'Contributor' | 'Reader' | 'Not shared',
): Promise<void> {
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
  await expect(bob.locator('.dropdown-button')).toHaveText(expectedRole);
  await bob.locator('.dropdown-button').click();
  const roles = page.locator('.dropdown-popover').getByRole('listitem');
  await expect(roles.locator('.option-text__label')).toHaveText(['Owner', 'Manager', 'Contributor', 'Reader', 'Not shared']);
  await roles.locator('.option-text__label', { hasText: newRole }).click();
  if (newRole === 'Not shared') {
    await expect(page).toShowToast('The workspace is no longer shared with Boby McBobFace.', 'Success');
  } else {
    await expect(page).toShowToast(`Boby McBobFace's role has been updated to ${newRole}.`, 'Success');
  }
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
    (url) => url.hostname.startsWith('editics.') && /^\/editics\/index(-[0-9a-f]+)?\.html$/.test(url.pathname),
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
