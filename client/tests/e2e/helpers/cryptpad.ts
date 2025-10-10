// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { expect, request } from '@playwright/test';
import { MsPage } from '@tests/e2e/helpers/types';

// cspell:disable-next-line
export const CRYPTPAD_SERVER = 'cryptpad-dev.parsec.cloud';

/**
 * Check if the CryptPad server is responding and available
 * @param serverUrl - The CryptPad server URL to check
 * @param timeout - Timeout in milliseconds (default: 10000)
 * @returns Promise<boolean> - true if server is available, false otherwise
 */
export async function isCryptpadServerAvailable(serverUrl: string = CRYPTPAD_SERVER, timeout: number = 10000): Promise<boolean> {
  try {
    const apiRequestContext = await request.newContext();
    // Use cryptpad-api.js - this is the fundamental entry point that Parsec loads to initialize CryptPad.
    // If this file isn't available, CryptPad integration cannot work at all.
    // This matches exactly what the CryptPad service does in src/services/cryptpad.ts
    const response = await apiRequestContext.get(`https://${serverUrl}/cryptpad-api.js`, {
      timeout: timeout,
    });
    await apiRequestContext.dispose();
    return response.ok();
  } catch (error) {
    // Server is not responding, connection timeout, or any other network error
    console.log(`CryptPad server ${serverUrl} is not available:`, error);
    return false;
  }
}

interface MockCryptpadOptions {
  timeout?: boolean;
  httpErrorCode?: number;
}

export async function mockCryptpadServer(page: MsPage, opts?: MockCryptpadOptions): Promise<void> {
  await page.route(`https://${CRYPTPAD_SERVER}/**`, async (route) => {
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

/**
 * Helper function to conditionally skip CryptPad tests based on server availability
 * Use this function to wrap your test function when you want to skip only if server is unavailable
 * @param testFn - The Playwright test function to run
 * @returns A function that checks server availability before running the test
 */
export function skipIfCryptpadUnavailable(testFn: (fixtures: any) => Promise<void>): (fixtures: any) => Promise<void> {
  return async (fixtures: any): Promise<void> => {
    const isAvailable = await isCryptpadServerAvailable();
    if (!isAvailable) {
      console.log(`Skipping test because CryptPad server ${CRYPTPAD_SERVER} is not available`);
      return; // Skip the test by returning early
    }
    return testFn(fixtures);
  };
}
