// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { Page } from '@playwright/test';
import { expect, fillIonInput, msTest } from '@tests/e2e/helpers';

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

const WORKSPACES = [
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

for (const workspace of WORKSPACES) {
  msTest(`Check workspace card of ${workspace.name}`, async ({ workspaces }) => {
    const workspaceCard = workspaces.locator('.workspace-card-item', { hasText: workspace.name });
    await expect(workspaceCard).toContainText(workspace.name);
    expect(workspaceCard).toBeDefined();
    const workspaceInfo = workspaceCard.locator('.workspace-info');
    await expect(workspaceInfo.locator('.shared-group')).toBeHidden();
    await expect(workspaceInfo.locator('.not-shared-label')).toBeVisible();
    await expect(workspaceInfo.locator('.not-shared-label')).toHaveText('Not shared');
    // await expect(workspaceInfo.locator('.shared-group')).toBeVisible();
    // await expect(workspaceInfo.locator('.not-shared-label')).toBeHidden();
    // await expect(workspaceInfo.locator('.shared-group').locator('.person-avatar')).toHaveCount(workspace.sharedWith.length);
    // await expect(workspaceInfo.locator('.shared-group').locator('.person-avatar')).toHaveText(workspace.sharedWith);
    const workspaceRole = workspaceCard.locator('.card-bottom');
    await expect(workspaceRole.locator('.card-bottom__role')).toHaveText(/^(Reader|Manager|Owner|Contributor)$/);
    const role = await workspaceRole.locator('.card-bottom__role').textContent();
    const icons = workspaceRole.locator('.card-bottom__icons').locator('ion-icon');
    await expect(icons).toHaveCount(3);

    switch (role) {
      case 'Reader': {
        await expect(icons.nth(0)).toHaveTheClass('icon-disabled');
        await expect(icons.nth(1)).toHaveTheClass('icon-disabled');
        await expect(icons.nth(2)).not.toHaveTheClass('icon-disabled');
        break;
      }
      case 'Manager': {
        await expect(icons.nth(0)).not.toHaveTheClass('icon-disabled');
        await expect(icons.nth(1)).not.toHaveTheClass('icon-disabled');
        await expect(icons.nth(2)).not.toHaveTheClass('icon-disabled');
        break;
      }
      case 'Contributor': {
        await expect(icons.nth(0)).toHaveTheClass('icon-disabled');
        await expect(icons.nth(1)).not.toHaveTheClass('icon-disabled');
        await expect(icons.nth(2)).not.toHaveTheClass('icon-disabled');
        break;
      }
      case 'Owner': {
        await expect(icons.nth(0)).not.toHaveTheClass('icon-disabled');
        await expect(icons.nth(1)).not.toHaveTheClass('icon-disabled');
        await expect(icons.nth(2)).not.toHaveTheClass('icon-disabled');
        break;
      }
      default:
        break;
    }
  });
}

for (const gridMode of [false, true]) {
  msTest(`Empty workspaces in ${gridMode ? 'grid' : 'list'} mode`, async ({ connected }) => {
    if (!gridMode) {
      await toggleViewMode(connected);
    }
    const actionBar = connected.locator('#workspaces-ms-action-bar');
    await expect(actionBar.locator('.ms-action-bar-button')).toHaveCount(1);
    await expect(actionBar.locator('#button-new-workspace')).toHaveText('New workspace');
    await expect(actionBar.locator('.counter')).toHaveText('No items');
    await expect(actionBar.locator('#workspace-filter-select')).toHaveText('Name');
    if (gridMode) {
      await expect(actionBar.locator('.ms-grid-list-toggle').locator('#grid-view')).toHaveDisabledAttribute();
      await expect(actionBar.locator('.ms-grid-list-toggle').locator('#list-view')).toBeEnabled();
      await expect(connected.locator('.workspaces-container').locator('.workspace-card-item')).toHaveCount(0);
      await expect(connected.locator('.workspaces-container').locator('.no-workspaces-content')).toBeVisible();
      await expect(connected.locator('.workspaces-container').locator('.no-workspaces-content').locator('ion-text')).toHaveText(
        'You do not have access to any workspace yet. Workspaces that you create or have been shared with you will be listed here.',
      );
    } else {
      await expect(actionBar.locator('.ms-grid-list-toggle').locator('#grid-view')).toBeEnabled();
      await expect(actionBar.locator('.ms-grid-list-toggle').locator('#list-view')).toHaveDisabledAttribute();
      await expect(connected.locator('.workspaces-container').locator('.no-workspaces-content')).toBeVisible();
      await expect(connected.locator('.workspaces-container').locator('.workspace-list-item')).toHaveCount(0);
      await expect(connected.locator('.workspaces-container').locator('.no-workspaces-content').locator('ion-text')).toHaveText(
        'You do not have access to any workspace yet. Workspaces that you create or have been shared with you will be listed here.',
      );
    }
  });

  msTest(`Workspace sort order in ${gridMode ? 'grid' : 'list'} mode`, async ({ workspaces }) => {
    if (!gridMode) {
      await toggleViewMode(workspaces);
    }
    // Order by name asc (default)
    let names = WORKSPACES.map((w) => w.name).sort((wName1, wName2) => wName1.localeCompare(wName2));
    if (gridMode) {
      await expect(workspaces.locator('.workspaces-container').locator('.workspace-card__title')).toHaveText(names);
    } else {
      await expect(workspaces.locator('.workspaces-container').locator('.workspace-name__label')).toHaveText(names);
    }
    const actionBar = workspaces.locator('#workspaces-ms-action-bar');
    const sortSelector = actionBar.locator('#workspace-filter-select');
    await expect(sortSelector).toHaveText('Name');
    await expect(workspaces.locator('.popover-viewport')).toBeHidden();
    await sortSelector.click();
    const popover = workspaces.locator('.popover-viewport');
    const sortItems = popover.getByRole('listitem');
    await expect(sortItems).toHaveCount(2);
    await expect(sortItems).toHaveText(['Ascending', 'Name']);
    for (const [index, checked] of [false, true].entries()) {
      if (checked) {
        await expect(sortItems.nth(index)).toHaveTheClass('selected');
      } else {
        await expect(sortItems.nth(index)).not.toHaveTheClass('selected');
      }
    }
    await sortItems.nth(0).click();
    await expect(workspaces.locator('.popover-viewport')).toBeHidden();
    // Order by name desc
    names = WORKSPACES.map((w) => w.name).sort((wName1, wName2) => wName2.localeCompare(wName1));
    await sortSelector.click();
    if (gridMode) {
      await expect(workspaces.locator('.workspaces-container').locator('.workspace-card__title')).toHaveText(names);
    } else {
      await expect(workspaces.locator('.workspaces-container').locator('.workspace-name__label')).toHaveText(names);
    }
    await expect(workspaces.locator('.popover-viewport').getByRole('listitem').nth(0)).toHaveText('Descending');
  });
}

async function ensureFavorite(page: Page, favoritesCount: number): Promise<void> {
  for (let i = 0; i < 3; i++) {
    let item;
    if (await isInGridMode(page)) {
      item = page.locator('.workspace-card-item').nth(i);
    } else {
      item = page.locator('.workspaces-container').locator('.workspace-list-item').nth(i);
    }
    const expectedClass = `workspace-favorite-icon__${i < favoritesCount ? 'on' : 'off'}`;
    await expect(item.locator('.workspace-favorite-icon')).toHaveTheClass(expectedClass);
  }
  const sidebarFavorites = page.locator('.sidebar').locator('.organization-workspaces').locator('.favorites');
  if (favoritesCount > 0) {
    await expect(sidebarFavorites).toBeVisible();
    await expect(sidebarFavorites.getByRole('listitem')).toHaveCount(favoritesCount);
  } else {
    await expect(sidebarFavorites).toBeHidden();
  }
}

msTest('Checks favorites', async ({ workspaces }) => {
  await expect(workspaces.locator('.workspace-card-item').locator('.workspace-card__title')).toHaveText([
    'The Copper Coronet',
    'Trademeet',
    "Watcher's Keep",
  ]);
  await ensureFavorite(workspaces, 0);
  await workspaces.locator('.workspace-card-item').nth(1).locator('.workspace-favorite-icon').click();
  // Put favorite in first
  await expect(workspaces.locator('.workspace-card-item').locator('.workspace-card__title')).toHaveText([
    'Trademeet',
    'The Copper Coronet',
    "Watcher's Keep",
  ]);
  await ensureFavorite(workspaces, 1);
  // Check in list mode too
  await toggleViewMode(workspaces);
  await ensureFavorite(workspaces, 1);

  await workspaces.locator('.workspaces-container').locator('.workspace-list-item').nth(1).locator('.workspace-favorite-icon').click();
  await expect(workspaces.locator('.workspace-list-item').locator('.workspace-name__label')).toHaveText([
    'The Copper Coronet',
    'Trademeet',
    "Watcher's Keep",
  ]);
  await ensureFavorite(workspaces, 2);
  // Check in grid mode too
  await toggleViewMode(workspaces);
  await ensureFavorite(workspaces, 2);

  await workspaces.locator('.workspace-card-item').nth(1).locator('.workspace-favorite-icon').click();
  await ensureFavorite(workspaces, 1);
  await toggleViewMode(workspaces);

  await workspaces.locator('.workspaces-container').locator('.workspace-list-item').nth(0).locator('.workspace-favorite-icon').click();
  await ensureFavorite(workspaces, 0);
});

for (const gridMode of [false, true]) {
  msTest(`Workspace filter in ${gridMode ? 'grid' : 'list'} mode`, async ({ workspaces }) => {
    if (!gridMode) {
      await toggleViewMode(workspaces);
    }
    const searchInput = workspaces.locator('#workspaces-ms-action-bar').locator('#search-input-workspace').locator('ion-input');
    const container = workspaces.locator('.workspaces-container');
    const titles = gridMode ? container.locator('.workspace-card__title') : container.locator('.workspace-name__label');

    await expect(titles).toHaveText(['The Copper Coronet', 'Trademeet', "Watcher's Keep"]);
    await fillIonInput(searchInput, 'ee');
    await expect(titles).toHaveText(['Trademeet', "Watcher's Keep"]);
    await fillIonInput(searchInput, 'eep');
    await expect(titles).toHaveText(["Watcher's Keep"]);
    await fillIonInput(searchInput, '');
    await expect(workspaces.locator('.no-workspaces')).not.toBeVisible();
    await expect(titles).toHaveText(['The Copper Coronet', 'Trademeet', "Watcher's Keep"]);
    await fillIonInput(searchInput, 'No match');
    await expect(titles).not.toBeVisible();
    await expect(workspaces.locator('.no-workspaces')).toBeVisible();
    await expect(workspaces.locator('.no-workspaces').locator('ion-text')).toHaveText('No workspace match this search filter.');
  });
}

msTest('Back from files with back button', async ({ workspaces }) => {
  await workspaces.locator('.workspace-card-item').nth(0).click();
  await expect(workspaces.locator('.topbar-left').locator('.topbar-left__breadcrumb').locator('ion-breadcrumb').nth(1)).toHaveText(
    'The Copper Coronet',
  );
  await workspaces.locator('.topbar-left').locator('.back-button').click();
  await expect(workspaces.locator('.topbar-left').locator('.topbar-left__breadcrumb').locator('ion-breadcrumb').nth(0)).toHaveText(
    'My workspaces',
  );
});

msTest('Back from files with side menu', async ({ workspaces }) => {
  await workspaces.locator('.workspace-card-item').nth(0).click();
  await expect(workspaces.locator('.topbar-left').locator('.topbar-left__breadcrumb').locator('ion-breadcrumb').nth(1)).toHaveText(
    'The Copper Coronet',
  );
  await workspaces.locator('.sidebar').locator('.sidebar-header').locator('#goHome').click();
  await expect(workspaces.locator('.topbar-left').locator('.topbar-left__breadcrumb').locator('ion-breadcrumb').nth(0)).toHaveText(
    'My workspaces',
  );
});
