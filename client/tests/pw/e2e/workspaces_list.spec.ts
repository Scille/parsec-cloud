// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { expect } from '@tests/pw/helpers/assertions';
import { msTest } from '@tests/pw/helpers/fixtures';

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
    await expect(workspaceCard).toContainText('Last updated:');
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

msTest('Workspace sort order', async ({ connected }) => {
  // Order by name asc (default)
  let names = workspaces.map((w) => w.name).sort((wName1, wName2) => wName1.localeCompare(wName2));
  await expect(connected.locator('.card-content__title')).toHaveText(names);
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
  await expect(connected.locator('.card-content__title')).toHaveText(names);
  await expect(connected.locator('.popover-viewport').getByRole('listitem').nth(0)).toHaveText('Descending');
});
