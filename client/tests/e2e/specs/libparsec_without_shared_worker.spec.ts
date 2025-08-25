// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { test } from '@playwright/test';
import { expect } from '@tests/e2e/helpers';
import { initTestBed } from '@tests/e2e/helpers/testbed';

test('Use libparsec in a browser without SharedWorker support', async ({ page }) => {
  page.on('console', (msg) => console.log('> ', msg.text()));

  await page.addInitScript(() => {
    (window as any).TESTING = true;
    // Disable the SharedWorker, this typically simulates Chrome on Android
    (window as any).SharedWorker = undefined;
  });

  await page.goto('/');
  await page.waitForLoadState('domcontentloaded');
  await expect(page.locator('#app')).toHaveAttribute('app-state', 'initializing');

  await initTestBed(page);

  const devices = await page.evaluate(async () => {
    const configDir = window.TESTING_CONFIG_PATH;
    const result = await window.libparsec.listAvailableDevices(configDir);
    return result.ok ? result.value : [];
  });

  expect(devices).toHaveLength(5);
});
