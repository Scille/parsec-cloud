// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { Page } from '@playwright/test';
import { expect } from '@tests/pw/helpers/assertions';
import { msTest } from '@tests/pw/helpers/fixtures';
import { fillInputModal } from '@tests/pw/helpers/utils';

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

const workspaces = [
  {
    name: 'The Copper Coronet',
    sharedWith: ['Ko', 'Ce'],
  },
  {
    name: 'Trademeet',
    sharedWith: ['Ko'],
  },
  {
    name: "Watcher's Keep",
    sharedWith: [],
  },
];

msTest('List the workspaces', async ({ connected }) => {
  const actionBar = connected.locator('#workspaces-ms-action-bar');
  await expect(actionBar.locator('.ms-action-bar-button')).toHaveCount(1);
  await expect(actionBar.locator('#button-new-workspace')).toHaveText('New workspace');
  await expect(actionBar.locator('.counter')).toHaveText('3 items');
  await expect(actionBar.locator('#workspace-filter-select')).toHaveText('Name');
  await expect(actionBar.locator('.ms-grid-list-toggle').locator('#grid-view')).toHaveDisabledAttribute();
  await expect(actionBar.locator('.ms-grid-list-toggle').locator('#list-view')).toBeEnabled();
  await expect(connected.locator('.workspaces-grid-item')).toHaveCount(3);
});

for (const workspace of workspaces) {
  msTest(`Check workspace card of ${workspace.name}`, async ({ connected }) => {
    const workspaceCard = connected.locator('.workspaces-grid-item', { hasText: workspace.name });
    await expect(workspaceCard).toContainText(workspace.name);
    expect(workspaceCard).toBeDefined();
    const workspaceInfo = workspaceCard.locator('.workspace-info');
    if (workspace.sharedWith.length === 0) {
      await expect(workspaceInfo.locator('.shared-group')).toBeHidden();
      await expect(workspaceInfo.locator('.not-shared-label')).toBeVisible();
      await expect(workspaceInfo.locator('.not-shared-label')).toHaveText('Not shared');
    } else {
      await expect(workspaceInfo.locator('.shared-group')).toBeVisible();
      await expect(workspaceInfo.locator('.not-shared-label')).toBeHidden();
      await expect(workspaceInfo.locator('.shared-group').locator('.person-avatar')).toHaveCount(workspace.sharedWith.length);
      await expect(workspaceInfo.locator('.shared-group').locator('.person-avatar')).toHaveText(workspace.sharedWith);
    }
  });
}

for (const gridMode of [false, true]) {
  msTest(`Workspace sort order in ${gridMode ? 'grid' : 'list'} mode`, async ({ connected }) => {
    if (!gridMode) {
      await toggleViewMode(connected);
    }
    // Order by name asc (default)
    let names = workspaces.map((w) => w.name).sort((wName1, wName2) => wName1.localeCompare(wName2));
    if (gridMode) {
      await expect(connected.locator('.workspaces-container').locator('.card-content__title')).toHaveText(names);
    } else {
      await expect(connected.locator('.workspaces-container').locator('.workspace-name__label')).toHaveText(names);
    }
    const actionBar = connected.locator('#workspaces-ms-action-bar');
    const sortSelector = actionBar.locator('#workspace-filter-select');
    await expect(sortSelector).toHaveText('Name');
    await expect(connected.locator('.popover-viewport')).toBeHidden();
    await sortSelector.click();
    const popover = connected.locator('.popover-viewport');
    const sortItems = popover.getByRole('listitem');
    await expect(sortItems).toHaveCount(4);
    await expect(sortItems).toHaveText(['Ascending', 'Name', 'Size', 'Last updated']);
    for (const [index, checked] of [false, true, false, false].entries()) {
      if (checked) {
        await expect(sortItems.nth(index)).toHaveTheClass('selected');
      } else {
        await expect(sortItems.nth(index)).not.toHaveTheClass('selected');
      }
    }
    await sortItems.nth(0).click();
    await expect(connected.locator('.popover-viewport')).toBeHidden();
    // Order by name desc
    names = workspaces.map((w) => w.name).sort((wName1, wName2) => wName2.localeCompare(wName1));
    await sortSelector.click();
    if (gridMode) {
      await expect(connected.locator('.workspaces-container').locator('.card-content__title')).toHaveText(names);
    } else {
      await expect(connected.locator('.workspaces-container').locator('.workspace-name__label')).toHaveText(names);
    }
    await expect(connected.locator('.popover-viewport').getByRole('listitem').nth(0)).toHaveText('Descending');
  });
}

for (const createWithSidebar of [false, true]) {
  msTest(`Create new workspace ${createWithSidebar ? 'with sidebar' : 'with action bar'}`, async ({ connected }) => {
    if (createWithSidebar) {
      await connected.locator('#workspaces-ms-action-bar').locator('#button-new-workspace').click();
    } else {
      await connected.locator('.sidebar').locator('.organization-workspaces').locator('#new-workspace').click();
    }
    await fillInputModal(connected, 'My Workspace');
    await expect(connected).toShowToast("The workspace 'My Workspace' has been created!", 'Success');
  });
}

async function ensureFavorite(page: Page, favoritesCount: number): Promise<void> {
  for (let i = 0; i < 3; i++) {
    let item;
    if (await isInGridMode(page)) {
      item = page.locator('.workspaces-grid-item').nth(i);
    } else {
      item = page.locator('.workspaces-container').locator('.workspace-list-item').nth(i);
    }
    const expectedClass = `card-favorite-${i < favoritesCount ? 'on' : 'off'}`;
    await expect(item.locator('.card-favorite')).toHaveTheClass(expectedClass);
  }
  const sidebarFavorites = page.locator('.sidebar').locator('.organization-workspaces').locator('.favorites');
  if (favoritesCount > 0) {
    await expect(sidebarFavorites).toBeVisible();
    await expect(sidebarFavorites.getByRole('listitem')).toHaveCount(favoritesCount);
  } else {
    await expect(sidebarFavorites).toBeHidden();
  }
}

msTest('Checks favorites', async ({ connected }) => {
  await expect(connected.locator('.workspaces-grid-item').locator('.card-content__title')).toHaveText([
    'The Copper Coronet',
    'Trademeet',
    "Watcher's Keep",
  ]);
  await ensureFavorite(connected, 0);
  await connected.locator('.workspaces-grid-item').nth(1).locator('.card-favorite').click();
  // Put favorite in first
  await expect(connected.locator('.workspaces-grid-item').locator('.card-content__title')).toHaveText([
    'Trademeet',
    'The Copper Coronet',
    "Watcher's Keep",
  ]);
  await ensureFavorite(connected, 1);
  // Check in list mode too
  await toggleViewMode(connected);
  await ensureFavorite(connected, 1);

  await connected.locator('.workspaces-container').locator('.workspace-list-item').nth(1).locator('.card-favorite').click();
  await expect(connected.locator('.workspace-list-item').locator('.workspace-name__label')).toHaveText([
    'The Copper Coronet',
    'Trademeet',
    "Watcher's Keep",
  ]);
  await ensureFavorite(connected, 2);
  // Check in grid mode too
  await toggleViewMode(connected);
  await ensureFavorite(connected, 2);

  await connected.locator('.workspaces-grid-item').nth(1).locator('.card-favorite').click();
  await ensureFavorite(connected, 1);
  await toggleViewMode(connected);

  await connected.locator('.workspaces-container').locator('.workspace-list-item').nth(0).locator('.card-favorite').click();
  await ensureFavorite(connected, 0);
});
