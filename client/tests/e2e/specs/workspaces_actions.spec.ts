// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { Page } from '@playwright/test';
import { expect, fillInputModal, getClipboardText, msTest } from '@tests/e2e/helpers';

type Mode = 'grid' | 'list' | 'sidebar';

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

async function openContextMenu(page: Page, mode: Mode): Promise<void> {
  if (mode === 'grid') {
    const wk = page.locator('.workspaces-container-grid').locator('.workspace-card-item').nth(1);
    await wk.locator('.icon-option-container').nth(1).click();
  } else if (mode === 'list') {
    await toggleViewMode(page);
    const wk = page.locator('.workspaces-container').locator('.workspace-list-item').nth(1);
    await wk.locator('.workspace-options').click();
  } else {
    const sidebar = page.locator('.sidebar');
    const a = sidebar.locator('.organization-workspaces').locator('.workspaces');
    const wk = a.locator('.list-sidebar-content').locator('.sidebar-item-workspace').nth(0);
    await wk.hover();
    await wk.locator('.sidebar-item-workspace__option').click();
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
  msTest(`Checks workspace context menu ${mode}`, async ({ connected }) => {
    await expect(connected.locator('.workspace-context-menu')).toBeHidden();
    await openContextMenu(connected, mode as Mode);
    const contextMenu = connected.locator('.workspace-context-menu');
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

  msTest(`Checks workspace context menu with right click ${mode}`, async ({ connected }) => {
    await expect(connected.locator('.workspace-context-menu')).toBeHidden();

    if (mode === 'grid') {
      const wk = connected.locator('.workspaces-container-grid').locator('.workspace-card-item').nth(1);
      await wk.click({ button: 'right' });
    } else if (mode === 'list') {
      await toggleViewMode(connected);
      const wk = connected.locator('.workspaces-container').locator('.workspace-list-item').nth(1);
      await wk.click({ button: 'right' });
    } else {
      const wk = connected.locator('.sidebar').locator('.list-sidebar.workspaces').getByRole('listitem').nth(0);
      await wk.click({ button: 'right' });
    }

    const contextMenu = connected.locator('.workspace-context-menu');
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

  msTest(`Navigate into a workspace ${mode}`, async ({ connected }) => {
    if (mode === 'grid') {
      await connected.locator('.workspaces-container-grid').locator('.workspace-card-item').nth(1).click();
    } else if (mode === 'list') {
      await toggleViewMode(connected);
      await connected.locator('.workspaces-container').locator('.workspace-list-item').nth(1).click();
    } else {
      await connected.locator('.sidebar').locator('.list-sidebar.workspaces').getByRole('listitem').nth(0).click();
    }
    await expect(connected).toHaveHeader(['Trademeet'], true, true);
  });

  msTest(`Rename a workspace ${mode}`, async ({ connected }) => {
    await openContextMenu(connected, mode as Mode);
    const popover = connected.locator('.workspace-context-menu');
    await popover.getByRole('listitem').nth(1).click();
    await fillInputModal(connected, 'New Workspace Name', true);
    await expect(connected).toShowToast('Workspace has been successfully renamed to New Workspace Name.', 'Success');
  });

  msTest(`Check copy link workspace action with permission ${mode}`, async ({ connected, context }) => {
    await context.grantPermissions(['clipboard-write']);
    await openContextMenu(connected, mode as Mode);
    const contextMenu = connected.locator('.workspace-context-menu');
    await expect(contextMenu).toBeVisible();
    await expect(contextMenu.getByRole('group').nth(1).getByRole('listitem').nth(1)).toHaveText('Copy link');
    await contextMenu.getByRole('group').nth(1).getByRole('listitem').nth(1).click();
    await expect(connected).toShowToast('Workspace link has been copied to clipboard.', 'Info');
    // cspell:disable-next-line
    const payload = 'k8QY94a350f2f629403db2269c44583f7aa1AcQ0Zkd8YbWfYF19LMwc55HjBOvI8LA8c_9oU2xaBJ0u2Ou0AFZYA4-QHhi2FprzAtUoAgMYwg';
    const link = `parsec3://parsec.cloud/Org?a=path&p=${payload}`;
    expect(await getClipboardText(connected)).toBe(link);
  });

  msTest(`Check copy link workspace action without permission ${mode}`, async ({ connected }) => {
    await openContextMenu(connected, mode as Mode);
    const contextMenu = connected.locator('.workspace-context-menu');
    await expect(contextMenu).toBeVisible();
    await expect(contextMenu.getByRole('group').nth(1).getByRole('listitem').nth(1)).toHaveText('Copy link');
    await contextMenu.getByRole('group').nth(1).getByRole('listitem').nth(1).click();
    await expect(connected).toShowToast('Failed to copy the link. Your browser or device does not seem to support copy/paste.', 'Error');
  });

  msTest(`Toggle workspace favorite ${mode}`, async ({ connected }) => {
    const favorites = connected.locator('.sidebar').locator('.favorites');
    await expect(favorites).toBeHidden();
    await openContextMenu(connected, mode as Mode);
    const popover = connected.locator('.workspace-context-menu');
    await popover.getByRole('listitem').nth(7).click();
    let wk;
    if (await isInGridMode(connected)) {
      wk = connected.locator('.workspaces-container-grid').locator('.workspace-card-item').nth(0);
      await expect(wk.locator('.workspace-card-content__title')).toHaveText('Trademeet');
    } else {
      wk = connected.locator('.workspaces-container').locator('.workspace-list-item').nth(0);
      await expect(wk.locator('.workspace-name')).toHaveText('Trademeet');
    }
    await expect(wk.locator('.workspace-favorite-icon')).toHaveTheClass('workspace-favorite-icon__on');
    await expect(favorites).toBeVisible();
    await expect(favorites.locator('.list-sidebar-header')).toHaveText('Pinned');
    await expect(favorites.getByRole('listitem').nth(0)).toHaveText('Trademeet');
  });

  msTest(`Open workspace sharing ${mode}`, async ({ connected }) => {
    await expect(connected.locator('.workspace-sharing-modal')).toBeHidden();
    await openContextMenu(connected, mode as Mode);
    const popover = connected.locator('.workspace-context-menu');
    await popover.getByRole('listitem').nth(5).click();
    await expect(connected.locator('.workspace-sharing-modal')).toBeVisible();
    await expect(connected.locator('.workspace-sharing-modal').locator('.ms-modal-header__title')).toHaveText('Trademeet');
  });
}
