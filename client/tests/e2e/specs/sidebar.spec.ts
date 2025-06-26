// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { expect, msTest } from '@tests/e2e/helpers';

msTest('Sidebar in organization management', async ({ organizationPage }) => {
  const sidebar = organizationPage.locator('.sidebar');

  await expect(sidebar.locator('.back-button')).toBeVisible();

  const mainButtons = sidebar.locator('.organization-card-buttons').locator('.organization-card-buttons__item');
  await expect(mainButtons).toHaveText(['Manage my organization', 'My workspaces']);
  await expect(mainButtons.nth(0)).toHaveTheClass('active');
  await expect(mainButtons.nth(1)).not.toHaveTheClass('active');

  await expect(sidebar.locator('.file-workspaces')).toBeHidden();
  await expect(sidebar.locator('.favorites')).toBeHidden();
  await expect(sidebar.locator('.workspaces')).toBeHidden();

  await expect(sidebar.locator('.manage-organization')).toBeVisible();
  await expect(sidebar.locator('.manage-organization').locator('.list-sidebar-header')).toHaveText('Manage my organization');
  const items = sidebar.locator('.manage-organization').locator('.organization-card-buttons').getByRole('listitem');
  await expect(items).toHaveText(['Users', 'Information']);
});

msTest('Sidebar in workspaces page', async ({ workspaces }) => {
  const sidebar = workspaces.locator('.sidebar');

  await workspaces.locator('.workspaces-container-grid').locator('.workspace-card-item').nth(0).click();
  await workspaces.locator('#connected-header').locator('.topbar-left').locator('ion-breadcrumb').nth(0).click();

  await expect(sidebar.locator('.back-button')).toBeHidden();

  const mainButtons = sidebar.locator('.organization-card-buttons').locator('.organization-card-buttons__item');
  await expect(mainButtons).toHaveText(['Manage my organization', 'My workspaces']);
  await expect(mainButtons.nth(0)).not.toHaveTheClass('active');
  await expect(mainButtons.nth(1)).toHaveTheClass('active');

  await expect(sidebar.locator('.file-workspaces')).toBeHidden();
  await expect(sidebar.locator('.favorites')).toBeHidden();
  await expect(sidebar.locator('.workspaces')).toBeVisible();
  await expect(sidebar.locator('.workspaces').locator('.list-sidebar-header')).toHaveText('Recent workspaces');
  await expect(sidebar.locator('.workspaces').getByRole('listitem').nth(0)).toHaveText('wksp1');

  await expect(sidebar.locator('.manage-organization')).toBeHidden();
});
