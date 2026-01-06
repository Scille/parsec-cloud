// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { DisplaySize, expect, msTest } from '@tests/e2e/helpers';

msTest('Fab button and menu as administrator and read/write', async ({ workspaces }) => {
  await workspaces.setDisplaySize(DisplaySize.Small);
  const fabButton = workspaces.locator('#add-menu-fab-button');
  const tabBarButtons = workspaces.locator('#tab-bar').locator('.tab-bar-menu-button');
  const fabModal = workspaces.locator('.tab-menu-modal');
  const fabModalOptions = fabModal.locator('.list-group-item');

  // WorkspacesPage
  await expect(fabButton).toBeVisible();
  await expect(fabModal).toBeHidden();
  await fabButton.click();
  await expect(fabModal).toBeVisible();
  await expect(fabModalOptions).toHaveCount(2);
  await expect(fabModalOptions).toHaveText(['New workspace', 'Invite a user']);
  await fabButton.click();
  await expect(fabModal).toBeHidden();

  // Read/write workspace + admin
  await workspaces.locator('.workspaces-container-grid').locator('.workspace-card-item').nth(0).click();
  await expect(fabButton).toBeVisible();
  await fabButton.click();
  await expect(fabModal).toBeVisible();
  await expect(fabModalOptions).toHaveCount(4);
  await expect(fabModalOptions).toHaveText(['New folder', 'Import a folder', 'Import files', 'Invite a user']);
  await fabButton.click();
  await expect(fabModal).toBeHidden();

  // Organization management
  await tabBarButtons.nth(2).click();
  await expect(workspaces).toBeOrganizationPage();
  await fabButton.click();
  await expect(fabModal).toBeVisible();
  await expect(fabModalOptions).toHaveCount(1);
  await expect(fabModalOptions).toHaveText(['Invite a user']);
  await fabButton.click();
  await expect(fabModal).toBeHidden();

  // No button in profile page
  await tabBarButtons.nth(3).click();
  await expect(workspaces).toBeMyProfilePage();
  await fabButton.click();
  await expect(fabModal).toBeVisible();
  await expect(fabModalOptions).toHaveCount(1);
  await expect(fabModalOptions).toHaveText(['Invite a user']);
  await fabButton.click();
  await expect(fabModal).toBeHidden();
});

msTest('Fab button and menu as standard and reader', async ({ workspacesStandard }) => {
  await workspacesStandard.setDisplaySize(DisplaySize.Small);
  const fabButton = workspacesStandard.locator('#add-menu-fab-button');
  const tabBarButtons = workspacesStandard.locator('#tab-bar').locator('.tab-bar-menu-button');
  const fabModal = workspacesStandard.locator('.tab-menu-modal');
  const fabModalOptions = fabModal.getByRole('listitem');

  // WorkspacesPage
  await expect(fabModal).toBeHidden();
  await expect(fabButton).toBeVisible();
  await fabButton.click();
  await expect(fabModal).toBeVisible();
  await expect(fabModalOptions).toHaveCount(1);
  await expect(fabModalOptions).toHaveText(['New workspace']);
  await fabButton.click();
  await expect(fabModal).toBeHidden();

  // No button in reader workspace
  await workspacesStandard.locator('.workspaces-container-grid').locator('.workspace-card-item').nth(0).click();
  await expect(fabButton).toBeHidden();

  // No button in organization management
  await tabBarButtons.nth(2).click();
  await expect(workspacesStandard).toBeOrganizationPage();
  await expect(fabButton).toBeHidden();

  // No button in profile page
  await tabBarButtons.nth(3).click();
  await expect(workspacesStandard).toBeMyProfilePage();
  await expect(fabButton).toBeHidden();
});

msTest('Fab button and menu as external', async ({ workspacesExternal }) => {
  await workspacesExternal.setDisplaySize(DisplaySize.Small);
  const fabButton = workspacesExternal.locator('#add-menu-fab-button');
  const tabBarButtons = workspacesExternal.locator('#tab-bar').locator('.tab-bar-menu-button');

  // WorkspacesPage
  await expect(fabButton).toBeHidden();

  // No button to organization management
  await expect(tabBarButtons.nth(2)).toBeHidden();

  // No button in profile page
  await tabBarButtons.nth(3).click();
  await expect(workspacesExternal).toBeMyProfilePage();
  await expect(fabButton).toBeHidden();
});
