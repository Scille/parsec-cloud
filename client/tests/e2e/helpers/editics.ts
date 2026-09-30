// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { BrowserContext, expect, FrameLocator } from '@playwright/test';
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

export async function checkDocumentTitle(frame: FrameLocator, expectedTitle: string): Promise<void> {
  await expect(frame.locator('#title-doc-name')).toHaveValue(expectedTitle);
}

export async function mockEditicsRequest(context: BrowserContext, error?: '404' | 'timeout'): Promise<void> {
  await context.route(
    (url) => url.hostname.startsWith('editics.') && url.pathname === '/editics/offline.html',
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
