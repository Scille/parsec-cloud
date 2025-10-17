// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { FrameLocator } from '@playwright/test';
import { expect } from '@tests/e2e/helpers/assertions';
import { MsPage } from '@tests/e2e/helpers/types';

// cspell:disable-next-line
export const CRYPTPAD_SERVER = process.env.CRYPTPAD_SERVER || 'https://cryptpad-dev.parsec.cloud';

interface MockCryptpadOptions {
  timeout?: boolean;
  httpErrorCode?: number;
}

export async function mockCryptpadServer(page: MsPage, opts?: MockCryptpadOptions): Promise<void> {
  await page.route(`${CRYPTPAD_SERVER}/**`, async (route) => {
    if (opts?.timeout) {
      await route.abort('timedout');
    } else if (opts?.httpErrorCode) {
      await route.fulfill({ status: opts.httpErrorCode });
    } else {
      await route.continue();
    }
  });
}

export async function waitUntilSaved(page: MsPage, timeout = 10000): Promise<void> {
  await expect(page.locator('#unsaved-changes')).toBeHidden();
  await expect(page.locator('#saved-changes')).toBeVisible({ timeout: timeout });
}

export async function waitUntilLoaded(page: MsPage): Promise<FrameLocator> {
  await expect(page.locator('#cryptpad-editor')).toBeVisible();
  const mainFrame = page.locator('#cryptpad-editor').contentFrame();
  await expect(mainFrame.locator('.placeholder-message-container')).toBeVisible({ timeout: 30000 });
  await expect(mainFrame.locator('.placeholder-message-container')).toHaveText('Loading...');
  await expect(mainFrame.locator('#sbox-iframe')).toBeVisible({ timeout: 30000 });
  const sboxFrame = mainFrame.locator('#sbox-iframe').contentFrame();
  await expect(sboxFrame.locator('.cp-loading-progress-list')).toBeVisible();

  return sboxFrame;
}
