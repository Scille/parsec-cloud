// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { expect } from '@playwright/test';
import { MsPage, msTest } from '@tests/e2e/helpers';

async function isSidebarVisible(page: MsPage): Promise<boolean> {
  const sidebar = page.locator('.sidebar');
  return await sidebar.isVisible();
}

async function toggleSidebar(page: MsPage): Promise<void> {
  // Look for toggle button in standard locations - simplified approach
  const toggleButton = page.locator('#trigger-toggle-menu-button').first();

  // Wait for button to be available and click it
  await toggleButton.waitFor({ state: 'visible', timeout: 10000 });
  await toggleButton.click();
}

msTest('Sidebar state persists across page navigation', async ({ documents, workspaces }) => {
  // Start on documents page
  await expect(documents.locator('.sidebar')).toBeVisible();

  // Hide sidebar
  await toggleSidebar(documents);
  await expect(documents.locator('.sidebar')).toBeHidden();

  // Navigate to workspaces
  await documents.locator('#connected-header').locator('.topbar-left').locator('ion-breadcrumb').nth(0).click();
  await expect(workspaces.locator('.workspaces-container')).toBeVisible();

  // Verify sidebar stays hidden
  await expect(workspaces.locator('.sidebar')).toBeHidden();

  // Navigate back to documents
  await workspaces.locator('.workspace-card-item').nth(0).click();
  await expect(documents.locator('.folder-container')).toBeVisible();

  // Verify sidebar still hidden
  await expect(documents.locator('.sidebar')).toBeHidden();
});

msTest('Sidebar loading state prevents flash on page load', async ({ documents }) => {
  // Hide sidebar first
  await toggleSidebar(documents);
  await expect(documents.locator('.sidebar')).toBeHidden();

  // Refresh page and immediately check sidebar state
  const reloadPromise = documents.reload();

  // Check that sidebar doesn't flash visible during loading
  // This test relies on the loading state behavior implemented in the service
  await documents.waitForTimeout(100);
  const sidebarVisibleDuringLoad = await isSidebarVisible(documents);

  await reloadPromise;
  await documents.waitForLoadState('domcontentloaded');

  // Sidebar should remain hidden after load
  await expect(documents.locator('.sidebar')).toBeHidden();

  // The sidebar shouldn't have flashed visible during loading
  // This is a best-effort test since timing can be tricky
  expect(sidebarVisibleDuringLoad).toBe(false);
});
