// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { Page } from '@playwright/test';
import { expect, fillInputModal, getClipboardText, msTest } from '@tests/e2e/helpers';

type Mode = 'grid' | 'list' | 'sidebar';

enum OpenMenuMethod {
  Button = 'button',
  RightClick = 'rightClick',
}

async function isInGridMode(page: Page): Promise<boolean> {
  return (await page.locator('#workspaces-ms-action-bar').locator('#grid-view').getAttribute('disabled')) !== null;
}

async function toggleViewMode(page: Page): Promise<void> {
  if (await isInGridMode(page)) {
    await page.locator('#workspaces-ms-action-bar').locator('#list-view').click();
  } else {
    await page.locator('#workspaces-ms-action-bar').locator('#grid-view').click();
  }
}

async function openContextMenu(page: Page, mode: Mode, method: OpenMenuMethod): Promise<void> {
  if (mode === 'grid') {
    const wk = page.locator('.workspaces-container-grid').locator('.workspace-card-item').nth(1);
    if (method === OpenMenuMethod.Button) {
      await wk.locator('.card-option').click();
    } else {
      await wk.click({ button: 'right' });
    }
  } else if (mode === 'list') {
    await toggleViewMode(page);
    const wk = page.locator('.workspaces-container').locator('.workspace-list-item').nth(1);
    if (method === OpenMenuMethod.Button) {
      await wk.locator('.workspace-options').click();
    } else {
      await wk.click({ button: 'right' });
    }
  } else {
    // Sidebar only shows after a workspace has been accessed recently
    await page.locator('.workspaces-container-grid').locator('.workspace-card-item').nth(1).click();
    await page.locator('#connected-header').locator('.topbar-left').locator('ion-breadcrumb').nth(0).click();
    const sidebar = page.locator('.sidebar');
    const a = sidebar.locator('.organization-workspaces').locator('.workspaces');
    const wk = a.locator('.list-sidebar-content').locator('.sidebar-item-workspace').nth(0);
    if (method === OpenMenuMethod.Button) {
      await wk.hover();
      await wk.locator('.sidebar-item-workspace__option').click();
    } else {
      await wk.click({ button: 'right' });
    }
  }
}

const MENU = [
  {
    title: 'Manage workspace',
    actions: ['Rename', 'History'],
  },
  {
    title: 'Collaboration',
    actions: ['Copy link', 'Sharing and roles'],
  },
  {
    title: 'Miscellaneous',
    actions: ['Pin'],
  },
];

for (const mode of ['grid', 'list', 'sidebar']) {
  msTest(`Checks workspace context menu ${mode}`, async ({ workspaces }) => {
    await expect(workspaces.locator('.workspace-context-menu')).toBeHidden();
    await openContextMenu(workspaces, mode as Mode, OpenMenuMethod.Button);
    const contextMenu = workspaces.locator('.workspace-context-menu');
    await expect(contextMenu).toBeVisible();
    await expect(contextMenu.getByRole('group')).toHaveCount(MENU.length);
    for (const [index, group] of MENU.entries()) {
      await expect(contextMenu.getByRole('group').nth(index).getByRole('listitem').nth(0)).toHaveText(group.title);
      for (const [actionIndex, action] of group.actions.entries()) {
        await expect(
          contextMenu
            .getByRole('group')
            .nth(index)
            .getByRole('listitem')
            .nth(actionIndex + 1),
        ).toHaveText(action);
      }
    }
  });

  msTest(`Checks workspace context menu with right click ${mode}`, async ({ workspaces }) => {
    await expect(workspaces.locator('.workspace-context-menu')).toBeHidden();
    await openContextMenu(workspaces, mode as Mode, OpenMenuMethod.RightClick);

    const contextMenu = workspaces.locator('.workspace-context-menu');
    await expect(contextMenu).toBeVisible();
    await expect(contextMenu.getByRole('group')).toHaveCount(MENU.length);
    for (const [index, group] of MENU.entries()) {
      await expect(contextMenu.getByRole('group').nth(index).getByRole('listitem').nth(0)).toHaveText(group.title);
      for (const [actionIndex, action] of group.actions.entries()) {
        await expect(
          contextMenu
            .getByRole('group')
            .nth(index)
            .getByRole('listitem')
            .nth(actionIndex + 1),
        ).toHaveText(action);
      }
    }
  });

  msTest(`Navigate into a workspace ${mode}`, async ({ workspaces }) => {
    if (mode === 'grid') {
      await workspaces.locator('.workspaces-container-grid').locator('.workspace-card-item').nth(1).click();
    } else if (mode === 'list') {
      await toggleViewMode(workspaces);
      await workspaces.locator('.workspaces-container').locator('.workspace-list-item').nth(1).click();
    } else {
      await workspaces.locator('.workspaces-container-grid').locator('.workspace-card-item').nth(1).click();
      await workspaces.locator('#connected-header').locator('.topbar-left').locator('ion-breadcrumb').nth(0).click();
      await workspaces.locator('.sidebar').locator('.list-sidebar.workspaces').getByRole('listitem').nth(0).click();
    }
    await expect(workspaces).toHaveHeader(['Trademeet'], true, true);
  });

  msTest.fail(`Rename a workspace ${mode}`, async ({ workspaces }) => {
    await openContextMenu(workspaces, mode as Mode, OpenMenuMethod.Button);
    const popover = workspaces.locator('.workspace-context-menu');
    await popover.getByRole('listitem').nth(1).click();
    await fillInputModal(workspaces, 'New Workspace Name', true);
    await expect(workspaces).toShowToast('Workspace has been successfully renamed to New Workspace Name.', 'Success');
  });

  msTest.fail(`Check copy link workspace action with permission ${mode}`, async ({ workspaces, context }) => {
    await context.grantPermissions(['clipboard-write']);
    await openContextMenu(workspaces, mode as Mode, OpenMenuMethod.Button);
    const contextMenu = workspaces.locator('.workspace-context-menu');
    await expect(contextMenu).toBeVisible();
    await expect(contextMenu.getByRole('group').nth(1).getByRole('listitem').nth(1)).toHaveText('Copy link');
    await contextMenu.getByRole('group').nth(1).getByRole('listitem').nth(1).click();
    await expect(workspaces).toShowToast('Workspace link has been copied to clipboard.', 'Info');
    // cspell:disable-next-line
    const payload = 'k8QY94a350f2f629403db2269c44583f7aa1AcQ0Zkd8YbWfYF19LMwc55HjBOvI8LA8c_9oU2xaBJ0u2Ou0AFZYA4-QHhi2FprzAtUoAgMYwg';
    const link = `parsec3://parsec.cloud/Org?a=path&p=${payload}`;
    expect(await getClipboardText(workspaces)).toBe(link);
  });

  msTest.fail(`Check copy link workspace action without permission ${mode}`, async ({ workspaces }) => {
    await openContextMenu(workspaces, mode as Mode, OpenMenuMethod.Button);
    const contextMenu = workspaces.locator('.workspace-context-menu');
    await expect(contextMenu).toBeVisible();
    await expect(contextMenu.getByRole('group').nth(1).getByRole('listitem').nth(1)).toHaveText('Copy link');
    await contextMenu.getByRole('group').nth(1).getByRole('listitem').nth(1).click();
    await expect(workspaces).toShowToast('Failed to copy the link. Your browser or device does not seem to support copy/paste.', 'Error');
  });

  msTest(`Toggle workspace favorite ${mode}`, async ({ workspaces }) => {
    const favorites = workspaces.locator('.sidebar').locator('.favorites');
    await expect(favorites).toBeHidden();
    await openContextMenu(workspaces, mode as Mode, OpenMenuMethod.Button);
    const popover = workspaces.locator('.workspace-context-menu');
    await popover.getByRole('listitem').nth(7).click();
    let wk;
    if (await isInGridMode(workspaces)) {
      wk = workspaces.locator('.workspaces-container-grid').locator('.workspace-card-item').nth(0);
      await expect(wk.locator('.workspace-card__title')).toHaveText('Trademeet');
    } else {
      wk = workspaces.locator('.workspaces-container').locator('.workspace-list-item').nth(0);
      await expect(wk.locator('.workspace-name')).toHaveText('Trademeet');
    }
    await expect(wk.locator('.workspace-favorite-icon')).toHaveTheClass('workspace-favorite-icon__on');
    await expect(favorites).toBeVisible();
    await expect(favorites.locator('.list-sidebar-header')).toHaveText('Pinned');
    await expect(favorites.getByRole('listitem').nth(0)).toHaveText('Trademeet');
  });

  msTest(`Open workspace sharing ${mode}`, async ({ workspaces }) => {
    await expect(workspaces.locator('.workspace-sharing-modal')).toBeHidden();
    await openContextMenu(workspaces, mode as Mode, OpenMenuMethod.Button);
    const popover = workspaces.locator('.workspace-context-menu');
    await popover.getByRole('listitem').nth(5).click();
    await expect(workspaces.locator('.workspace-sharing-modal')).toBeVisible();
    await expect(workspaces.locator('.workspace-sharing-modal').locator('.ms-modal-header__title')).toHaveText('Trademeet');
  });
}
