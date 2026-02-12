// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { Page } from '@playwright/test';
import { expect, fillInputModal, getClipboardText, login, msTest, resizePage } from '@tests/e2e/helpers';

type Mode = 'grid' | 'list' | 'sidebar' | 'breadcrumb';

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
    const wk = page.locator('.workspaces-container-grid').locator('.workspace-card-item').nth(0);
    if (method === OpenMenuMethod.Button) {
      await wk.locator('.icon-option-container').click();
    } else {
      await wk.click({ button: 'right' });
    }
  } else if (mode === 'list') {
    await toggleViewMode(page);
    const wk = page.locator('.workspaces-container').locator('.workspace-list-item').nth(0);
    if (method === OpenMenuMethod.Button) {
      await wk.locator('.workspace-options').click();
    } else {
      await wk.click({ button: 'right' });
    }
  } else if (mode === 'sidebar') {
    await page.locator('.workspaces-container-grid').locator('.workspace-card-item').nth(0).click();
    const sidebar = page.locator('.sidebar');
    const wk = sidebar.locator('#sidebar-workspaces').locator('.sidebar-item').nth(0);
    if (method === OpenMenuMethod.Button) {
      await wk.hover();
      await wk.locator('.sidebar-item-workspace__option').click();
    } else {
      await wk.click({ button: 'right' });
    }
  } else {
    await page.locator('.workspaces-container-grid').locator('.workspace-card-item').nth(0).click();
    await page.locator('.topbar-left').locator('.breadcrumb-element--active').click();
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
    actions: ['Add to favorites'],
  },
];

for (const mode of ['grid', 'list', 'sidebar', 'breadcrumb']) {
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
      await workspaces.locator('.workspaces-container-grid').locator('.workspace-card-item').nth(0).click();
    } else if (mode === 'list') {
      await toggleViewMode(workspaces);
      await workspaces.locator('.workspaces-container').locator('.workspace-list-item').locator('.workspace-name').nth(0).click();
    } else {
      await workspaces.locator('.workspaces-container-grid').locator('.workspace-card-item').nth(0).click();
      await expect(workspaces).toBeDocumentPage();
      await expect(workspaces).toHaveHeader(['wksp1'], true, true);
      await workspaces.locator('.sidebar').locator('#sidebar-workspaces').locator('#sidebar-all-workspaces').click();
      await expect(workspaces).toBeWorkspacePage();
      await workspaces.locator('.workspaces-container-grid').locator('.workspace-card-item').nth(0).click();
    }
    await expect(workspaces).toBeDocumentPage();
    await expect(workspaces).toHaveHeader(['wksp1'], true, true);
  });

  msTest(`Rename a workspace ${mode}`, async ({ workspaces }) => {
    await openContextMenu(workspaces, mode as Mode, OpenMenuMethod.Button);
    const popover = workspaces.locator('.workspace-context-menu');
    await popover.getByRole('listitem').nth(1).click();
    await fillInputModal(workspaces, 'New Workspace Name', true);
    await expect(workspaces).toShowToast('Workspace has been successfully renamed to New Workspace Name.', 'Success');
  });

  msTest(`Check copy link workspace action with permission ${mode}`, async ({ workspaces, context }) => {
    await context.grantPermissions(['clipboard-write']);
    await openContextMenu(workspaces, mode as Mode, OpenMenuMethod.Button);
    const contextMenu = workspaces.locator('.workspace-context-menu');
    await expect(contextMenu).toBeVisible();
    await expect(contextMenu.getByRole('group').nth(1).getByRole('listitem').nth(1)).toHaveText('Copy link');
    await contextMenu.getByRole('group').nth(1).getByRole('listitem').nth(1).click();
    await expect(workspaces).toShowToast('Workspace link has been copied to clipboard.', 'Info');
    expect(await getClipboardText(workspaces)).toMatch(/^parsec3:\/\/.+$/);
  });

  msTest(`Check copy link workspace action without permission ${mode}`, async ({ workspaces }) => {
    await openContextMenu(workspaces, mode as Mode, OpenMenuMethod.Button);
    const contextMenu = workspaces.locator('.workspace-context-menu');
    await expect(contextMenu).toBeVisible();
    await expect(contextMenu.getByRole('group').nth(1).getByRole('listitem').nth(1)).toHaveText('Copy link');
    await contextMenu.getByRole('group').nth(1).getByRole('listitem').nth(1).click();
    await expect(workspaces).toShowToast('Failed to copy the link. Your browser or device does not seem to support copy/paste.', 'Error');
  });

  msTest(`Toggle workspace favorite ${mode}`, async ({ workspaces }) => {
    const favorites = workspaces.locator('.sidebar').locator('#sidebar-favorite-workspaces');
    await expect(favorites).toBeVisible();
    await openContextMenu(workspaces, mode as Mode, OpenMenuMethod.Button);
    const popover = workspaces.locator('.workspace-context-menu');
    await popover.getByRole('listitem').nth(8).click();
    await expect(popover).toBeHidden();
    if (mode === 'sidebar') {
      await workspaces.locator('.sidebar').locator('#sidebar-all-workspaces').click();
    }

    let wk;
    if (mode === 'breadcrumb') {
      await workspaces.locator('.topbar-left').locator('.breadcrumb-element').nth(0).click();
    }
    if (await isInGridMode(workspaces)) {
      wk = workspaces.locator('.workspaces-container-grid').locator('.workspace-card-item').nth(0);
      await expect(wk.locator('.workspace-card-content__title')).toHaveText('wksp1');
    } else {
      wk = workspaces.locator('.workspaces-container').locator('.workspace-list-item').nth(0);
      await expect(wk.locator('.workspace-name')).toHaveText('wksp1');
    }
    await expect(wk.locator('.workspace-favorite-icon')).toHaveTheClass('workspace-favorite-icon__on');
    await expect(favorites).toBeVisible();
    await expect(favorites.locator('.sidebar-content-organization-button__text')).toHaveText('Favorites');
  });

  msTest(`Open workspace sharing ${mode}`, async ({ workspaces }) => {
    await expect(workspaces.locator('.workspace-sharing-modal')).toBeHidden();
    await openContextMenu(workspaces, mode as Mode, OpenMenuMethod.Button);
    const popover = workspaces.locator('.workspace-context-menu');
    await popover.getByRole('listitem').nth(6).click();
    await expect(workspaces.locator('.workspace-sharing-modal')).toBeVisible();
    await expect(workspaces.locator('.ms-modal-header__title')).toHaveText('Share the workspace');
    await expect(workspaces.locator('.sharing-modal__title')).toHaveText('wksp1');
  });
}

msTest('Check workspace rename in header breadcrumb', async ({ workspaces }) => {
  msTest.setTimeout(45_000);
  const card = workspaces.locator('.workspaces-container-grid').locator('.workspace-card-item').nth(0);
  await expect(card.locator('.workspace-card-content__title')).toHaveText('wksp1');
  await card.click();
  await expect(workspaces).toHaveHeader(['wksp1'], true, true);

  // Rename workspace and check both headers
  const sidebar = workspaces.locator('.sidebar');
  const sidebarWorkspaceButton = sidebar.locator('#sidebar-workspaces').locator('.sidebar-item').nth(0);

  const bobTab = await workspaces.openNewTab();
  await login(bobTab, 'Boby McBobFace');
  const bobCard = bobTab.locator('.workspaces-container-grid').locator('.workspace-card-item').nth(0);
  await expect(bobCard.locator('.workspace-card-content__title')).toHaveText('wksp1');
  await bobCard.click();
  await expect(bobTab).toBeDocumentPage();
  await expect(bobTab).toHaveHeader(['wksp1'], true, true);

  const popover = workspaces.locator('.workspace-context-menu');
  await expect(popover).toBeHidden();
  await sidebarWorkspaceButton.click({ button: 'right' });
  await expect(popover).toBeVisible();
  await expect(popover.getByRole('listitem').nth(1)).toHaveText('Rename');
  await popover.getByRole('listitem').nth(1).click();
  await fillInputModal(workspaces, 'New-wksp1', true);
  await expect(workspaces).toShowToast('Workspace has been successfully renamed to New-wksp1.', 'Success');
  await expect(bobTab).toHaveHeader(['New-wksp1'], true, true);
  await expect(workspaces).toHaveHeader(['New-wksp1'], true, true);
});

msTest('Check if action bar updates when resizing the window', async ({ connected }) => {
  const actionBar = connected.locator('#workspaces-ms-action-bar');
  const actionsBarButtons = actionBar.locator('.ms-action-bar-button');
  const actionBarMoreButton = actionBar.locator('#action-bar-more-button');
  await expect(actionBar).toBeVisible();
  await expect(actionsBarButtons).toBeVisible();
  await expect(actionsBarButtons).toHaveCount(1);
  await expect(actionsBarButtons.nth(0)).toHaveText('New workspace');
  await resizePage(connected, 1000);
  await expect(actionsBarButtons).toHaveCount(1);
  await expect(actionsBarButtons.nth(0)).toBeHidden();
  await expect(actionBarMoreButton).toBeVisible();
  await actionBarMoreButton.click();
  await expect(connected.locator('.popover-viewport').getByRole('listitem').nth(0)).toHaveText('New workspace');
  await connected.keyboard.press('Escape');
  await expect(connected.locator('.popover-viewport')).toBeHidden();
});
