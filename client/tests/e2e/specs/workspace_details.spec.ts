// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { checkWorkspaceContextMenu, DisplaySize, expect, login, logout, msTest, shareWorkspace } from '@tests/e2e/helpers';

const ROLES = [
  {
    role: 'Owner',
    rights: [
      'Manage the workspace (rename, archive, delete)',
      'Manage access (share the workspace)',
      'Edit documents (add, edit, rename, delete)',
      'View documents (open, preview)',
    ],
  },
  {
    role: 'Manager',
    rights: ['Manage access (share the workspace)', 'Edit documents (add, edit, rename, delete)', 'View documents (open, preview)'],
  },
  {
    role: 'Contributor',
    rights: ['Edit documents (add, edit, rename, delete)', 'View documents (open, preview)'],
  },
  {
    role: 'Reader',
    rights: ['View documents (open, preview)'],
  },
] as const;

for (const { role, rights } of ROLES) {
  msTest(`Workspace details as ${role}`, async ({ workspaces }) => {
    // Alice owns wksp1 and Boby is Reader on it in the testbed
    if (role === 'Manager' || role === 'Contributor') {
      await shareWorkspace(workspaces, 'wksp1', 'Boby McBobFace', role);
    }
    if (role !== 'Owner') {
      await logout(workspaces);
      await login(workspaces, 'Boby McBobFace');
    }

    const modal = workspaces.locator('.workspace-details-modal');
    await expect(modal).toBeHidden();

    const wk = workspaces.locator('.workspaces-container-grid').locator('.workspace-card-item', { hasText: 'wksp1' });
    await wk.click({ button: 'right' });
    const menu = workspaces.locator('.workspace-context-menu');
    await expect(menu).toBeVisible();
    await menu.locator('.list-group-item__label', { hasText: 'Details' }).click();
    await expect(menu).toBeHidden();

    await expect(modal).toBeVisible();
    await expect(modal.locator('.ms-modal-header__title')).toHaveText('Workspace details');
    await expect(modal.locator('.workspace-header-text__name')).toHaveText('wksp1');
    await expect(modal.locator('.workspace-section__title')).toHaveText('General information');
    await expect(modal.locator('.workspace-info-item').nth(1).locator('.workspace-info-item__title')).toHaveText('Created on');
    await expect(modal.locator('.workspace-info-item').nth(1).locator('.workspace-info-item__value')).toHaveText(/^[0-9A-Z,:\/ ]+$/);
    await expect(modal.locator('.workspace-info-item').nth(0).locator('.workspace-info-item__title')).toHaveText('Access rights');
    await expect(modal.locator('.workspace-info-item__role')).toHaveText(role);
    await expect(modal.locator('.workspace-rights-item__text')).toHaveText([...rights]);
    await expect(modal.locator('.label-id')).toHaveText(/^(Internal ID: )[a-f0-9-]+$/);

    await modal.locator('.closeBtn').click();
    await expect(modal).toBeHidden();
  });
}

msTest('Workspace details in small display', async ({ workspaces }) => {
  await workspaces.setDisplaySize(DisplaySize.Small);
  const modal = workspaces.locator('.workspace-details-modal');
  await expect(modal).toBeHidden();

  const wk = workspaces.locator('.workspaces-container-grid').locator('.workspace-card-item').nth(0);
  await wk.locator('.icon-option-container').click();
  await checkWorkspaceContextMenu(workspaces, 'owner', 'Details');

  await expect(modal).toBeVisible();
  await expect(modal.locator('.ms-modal-header__title')).toHaveText('Workspace details');
  await expect(modal.locator('.workspace-header-text__name')).toHaveText('wksp1');
  await expect(modal.locator('.workspace-info-item__role')).toHaveText('Owner');
});
