// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import {
  checkWorkspaceContextMenu,
  DisplaySize,
  expect,
  login,
  logout,
  msTest,
  readDownload,
  shareWorkspace,
  waitForDownload,
} from '@tests/e2e/helpers';

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
    await expect(modal.locator('.workspace-section__title').nth(0)).toHaveText('General information');
    await expect(modal.locator('.workspace-info-item').nth(1).locator('.workspace-info-item__title')).toHaveText('Created on');
    await expect(modal.locator('.workspace-info-item').nth(1).locator('.workspace-info-item__value')).toHaveText(/^[0-9A-Z,:\/ ]+$/);
    await expect(modal.locator('.workspace-info-item').nth(0).locator('.workspace-info-item__title')).toHaveText('Access rights');
    await expect(modal.locator('.workspace-info-item__role')).toHaveText(role);
    await expect(modal.locator('.workspace-rights-item__text')).toHaveText([...rights]);
    await expect(modal.locator('.label-id')).toHaveText(/^(Internal ID: )[a-f0-9-]+$/);
    await expect(modal.locator('.workspace-section__title').nth(1)).toHaveText('Export data');

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

msTest('Export workspace data as JSON', async ({ workspaces }) => {
  const modal = workspaces.locator('.workspace-details-modal');
  await expect(modal).toBeHidden();

  const wk = workspaces.locator('.workspaces-container-grid').locator('.workspace-card-item', { hasText: 'wksp1' });
  await wk.click({ button: 'right' });
  const menu = workspaces.locator('.workspace-context-menu');
  await expect(menu).toBeVisible();
  await menu.locator('.list-group-item__label', { hasText: 'Details' }).click();
  await expect(modal).toBeVisible();

  const exportButton = modal.locator('.workspace-export__button');
  await expect(exportButton).toBeVisible();
  await expect(exportButton).toHaveText('Download export (.json)');

  const download = await waitForDownload(workspaces, async () => {
    await exportButton.click();
  });

  expect(download.suggestedFilename()).toMatch(/^Export_TestbedOrg\d+_Workspace_wksp1_.+\.json$/);
  const content = JSON.parse((await readDownload(download)).toString('utf-8'));

  expect(content.export.organization).toMatch(/^TestbedOrg\d+$/);
  expect(content.workspace.name).toBe('wksp1');
  expect(content.workspace.archived).toBe(false);
  expect(content.workspace.id).toMatch(/^[a-f0-9-]+$/);

  expect(content.workspace.users.length).toBeGreaterThan(0);
  const alice = content.workspace.users.find((u: any) => u.name === 'Alicey McAliceFace');
  expect(alice).toBeDefined();
  expect(alice.currentRole).toBe('RealmRoleOwner');
  expect(alice.roleChanges.length).toBeGreaterThan(0);
  expect(alice.roleChanges[0]).toHaveProperty('grantedBy');
  expect(alice.roleChanges[0]).toHaveProperty('grantedOn');
  expect(alice.roleChanges[0]).toHaveProperty('role');

  const bob = content.workspace.users.find((u: any) => u.name === 'Boby McBobFace');
  expect(bob).toBeDefined();
  expect(bob.currentRole).toBe('RealmRoleReader');

  await modal.locator('.closeBtn').click();
  await expect(modal).toBeHidden();
});
