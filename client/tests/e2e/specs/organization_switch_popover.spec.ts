// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { createFolder, expect, msTest } from '@tests/e2e/helpers';

msTest('Switch back and forth', async ({ connected }) => {
  // Logged in as Alice

  const profileName = connected.locator('#connected-header').locator('.topbar-right').locator('.text-content-name');
  await expect(profileName).toHaveText('Alicey McAliceFace');

  // Add a folder
  await connected.locator('.workspaces-container-grid').locator('.workspace-card-item').nth(0).click();
  await expect(connected).toHaveHeader(['wksp1'], true, true);
  await expect(connected).toBeDocumentPage();
  await expect(connected.locator('.folder-container').locator('.no-files')).toBeVisible();
  await createFolder(connected, 'Folder');
  const entryNames = connected.locator('.folder-container').locator('.file-list-item').locator('.file-name').locator('.file-name__label');
  await expect(entryNames).toHaveText(['Folder']);
  await connected.locator('.topbar-left').locator('.back-button').click();

  // Switch to home page
  connected.locator('.sidebar').locator('.sidebar-header').locator('.organization-card-header-desktop').click();
  const popover = connected.locator('.popover-switch');
  const switchButton = popover.locator('.section-buttons');
  const connectedOrgs = popover.locator('.connected-organization');
  const currentOrg = popover.locator('.current-organization');
  await expect(currentOrg.locator('.organization-name')).toHaveText(/^TestbedOrg\d+$/);
  await expect(connectedOrgs).toBeHidden();
  await expect(switchButton).toHaveText('Switch organization');
  await switchButton.click();
  await expect(popover).toBeHidden();
  await expect(connected).toBeHomePage();

  // Check that Alice is marked as logged in
  await expect(connected.locator('.organization-card').nth(0).locator('.connected-text')).toBeVisible();
  await expect(connected.locator('.organization-card').nth(0).locator('.connected-text')).toHaveText('Logged in');

  // Log in with Bob
  await connected.locator('.organization-card').nth(1).click();
  await expect(connected.locator('#password-input')).toBeVisible();
  await expect(connected.locator('.login-button')).toHaveDisabledAttribute();
  await connected.locator('#password-input').locator('input').fill('P@ssw0rd.');
  await expect(connected.locator('.login-button')).toBeEnabled();
  await connected.locator('.login-button').click();
  await expect(connected.locator('#connected-header')).toContainText('My workspaces');
  await expect(connected).toBeWorkspacePage();
  await expect(profileName).toHaveText('Boby McBobFace');

  // Go to the workspace and check that the folder is here
  await connected.locator('.workspaces-container-grid').locator('.workspace-card-item').nth(0).click();
  await expect(connected).toHaveHeader(['wksp1'], true, true);
  await expect(connected).toBeDocumentPage();
  await expect(entryNames).toHaveText(['Folder']);

  // Go back to workspace list
  await connected.locator('.topbar-left').locator('.back-button').click();
  await expect(connected).toBeWorkspacePage();

  // Check switch popover
  connected.locator('.sidebar').locator('.sidebar-header').locator('.organization-card-header-desktop').click();
  await expect(popover).toBeVisible();
  await expect(connectedOrgs).toBeVisible();
  await expect(connectedOrgs.locator('.organization-list__item').locator('.organization-text-content__name')).toHaveText(/^TestbedOrg\d+$/);
  await expect(connectedOrgs.locator('.organization-list__item').locator('.organization-text-content__email')).toHaveText(
    'Alicey McAliceFace',
  );
  await switchButton.click();
  await expect(popover).toBeHidden();
  await expect(connected).toBeHomePage();

  await connected.locator('.organization-list').locator('.organization-card').nth(0).click();
  // Should be Alice on the documents page
  await expect(profileName).toHaveText('Alicey McAliceFace');
  await expect(connected).toBeWorkspacePage();

  connected.locator('.sidebar').locator('.sidebar-header').locator('.organization-card-header-desktop').click();
  await expect(popover).toBeVisible();
  await expect(connectedOrgs.locator('.organization-list__item').locator('.organization-text-content__name')).toHaveText(/^TestbedOrg\d+$/);
  await expect(connectedOrgs.locator('.organization-list__item').locator('.organization-text-content__email')).toHaveText('Boby McBobFace');
  await switchButton.click();

  // Check that both Alice and Bob are marked as logged in
  await expect(connected.locator('.organization-card').nth(0).locator('.connected-text')).toBeVisible();
  await expect(connected.locator('.organization-card').nth(1).locator('.connected-text')).toBeVisible();
  await expect(connected.locator('.organization-card').nth(0).locator('.connected-text')).toHaveText('Logged in');
  await expect(connected.locator('.organization-card').nth(1).locator('.connected-text')).toHaveText('Logged in');
});
