// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { Page } from '@playwright/test';
import { createWorkspace, expect, fillIonInput, msTest } from '@tests/e2e/helpers';

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

const WORKSPACES = ['The Copper Coronet', 'Trademeet', "Watcher's Keep", 'wksp1'];

msTest('Check workspace card', async ({ workspaces }) => {
  const workspaceCard = workspaces.locator('.workspace-card-item').nth(0);
  await expect(workspaceCard.locator('.workspace-card-content__title')).toHaveText('wksp1');
  const workspaceRole = workspaceCard.locator('.workspace-card-bottom');
  await expect(workspaceRole.locator('.workspace-card-bottom__role')).toHaveText(/^(Reader|Manager|Owner|Contributor)$/);
  const icons = workspaceRole.locator('.workspace-card-bottom__icons').locator('ion-icon');
  await expect(icons).toHaveCount(2);
});

for (const gridMode of [false, true]) {
  msTest.fail(`Empty workspaces in ${gridMode ? 'grid' : 'list'} mode`, async ({ connected }) => {
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
    for (const wk of WORKSPACES) {
      if (wk !== 'wksp1') {
        await createWorkspace(workspaces, wk);
      }
    }
    // Order by name asc (default)
    // Don't use `sort` because it sorts in place
    let names = [...WORKSPACES].sort((wName1, wName2) => wName1.localeCompare(wName2));
    if (gridMode) {
      await expect(workspaces.locator('.workspaces-container').locator('.workspace-card-content__title')).toHaveText(names);
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
    names = [...WORKSPACES].sort((wName1, wName2) => wName2.localeCompare(wName1));
    await sortSelector.click();
    if (gridMode) {
      await expect(workspaces.locator('.workspaces-container').locator('.workspace-card-content__title')).toHaveText(names);
    } else {
      await expect(workspaces.locator('.workspaces-container').locator('.workspace-name__label')).toHaveText(names);
    }
    await expect(workspaces.locator('.popover-viewport').getByRole('listitem').nth(0)).toHaveText('Descending');
  });
}

async function toggleFavorite(page: Page, index: number): Promise<void> {
  let item;
  if (await isInGridMode(page)) {
    item = page.locator('.workspace-card-item').nth(index);
  } else {
    item = page.locator('.workspaces-container').locator('.workspace-list-item').nth(index);
  }
  await item.locator('.workspace-favorite-icon').click();
}

msTest('Checks favorites', async ({ workspaces }) => {
  await createWorkspace(workspaces, 'The Copper Coronet');
  await expect(workspaces.locator('.workspace-card-item').locator('.workspace-card-content__title')).toHaveText([
    'The Copper Coronet',
    'wksp1',
  ]);
  await toggleFavorite(workspaces, 1);
  // Put favorite in first
  await expect(workspaces.locator('.workspace-card-item').locator('.workspace-card-content__title')).toHaveText([
    'wksp1',
    'The Copper Coronet',
  ]);
  // Check in list mode too
  await toggleViewMode(workspaces);
  await expect(workspaces.locator('.workspace-list-item').locator('.workspace-name__label')).toHaveText(['wksp1', 'The Copper Coronet']);
  await toggleFavorite(workspaces, 1);
  await expect(workspaces.locator('.workspace-list-item').locator('.workspace-name__label')).toHaveText(['The Copper Coronet', 'wksp1']);
});

for (const gridMode of [false, true]) {
  msTest(`Workspace filter in ${gridMode ? 'grid' : 'list'} mode`, async ({ workspaces }) => {
    if (!gridMode) {
      await toggleViewMode(workspaces);
    }
    for (const wk of WORKSPACES) {
      if (wk !== 'wksp1') {
        await createWorkspace(workspaces, wk);
      }
    }

    const searchInput = workspaces.locator('#workspaces-ms-action-bar').locator('#search-input-workspace').locator('ion-input');
    const container = workspaces.locator('.workspaces-container');
    const titles = gridMode ? container.locator('.workspace-card-content__title') : container.locator('.workspace-name__label');

    await expect(titles).toHaveCount(4);
    await expect(workspaces.locator('.no-workspaces')).toBeHidden();

    // await expect(titles).toHaveText(WORKSPACES);
    await fillIonInput(searchInput, 'ee');
    await expect(titles).toHaveText(WORKSPACES.filter((name) => name.includes('ee')));
    await fillIonInput(searchInput, 'eep');
    await expect(titles).toHaveText(WORKSPACES.filter((name) => name.includes('eep')));
    await fillIonInput(searchInput, '');
    await expect(workspaces.locator('.no-workspaces')).toBeHidden();
    await expect(titles).toHaveText(WORKSPACES);
    await fillIonInput(searchInput, 'No match');
    await expect(titles).not.toBeVisible();
    await expect(workspaces.locator('.no-workspaces')).toBeVisible();
    await expect(workspaces.locator('.no-workspaces').locator('ion-text')).toHaveText('No workspace match this search filter.');
    await fillIonInput(searchInput, '');
    await expect(workspaces.locator('.no-workspaces')).toBeHidden();
    await expect(titles).toHaveText(WORKSPACES);
  });
}

msTest('Back from files with back button', async ({ workspaces }) => {
  await workspaces.locator('.workspace-card-item').nth(0).click();
  await expect(workspaces.locator('.topbar-left').locator('.topbar-left__breadcrumb').locator('ion-breadcrumb').nth(1)).toHaveText('wksp1');
  await workspaces.locator('.topbar-left').locator('.back-button').click();
  await expect(workspaces.locator('.topbar-left').locator('.topbar-left__breadcrumb').locator('ion-breadcrumb').nth(0)).toHaveText(
    'My workspaces',
  );
});

msTest('Back from files with side menu', async ({ workspaces }) => {
  await workspaces.locator('.workspace-card-item').nth(0).click();
  await expect(workspaces.locator('.topbar-left').locator('.topbar-left__breadcrumb').locator('ion-breadcrumb').nth(1)).toHaveText('wksp1');
  await workspaces.locator('.sidebar').locator('.sidebar-header').locator('#goHome').click();
  await expect(workspaces.locator('.topbar-left').locator('.topbar-left__breadcrumb').locator('ion-breadcrumb').nth(0)).toHaveText(
    'My workspaces',
  );
});
