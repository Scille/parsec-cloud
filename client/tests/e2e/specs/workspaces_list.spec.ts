// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { Page } from '@playwright/test';
import { answerQuestion, createWorkspace, DisplaySize, expect, fillInputModal, fillIonInput, msTest } from '@tests/e2e/helpers';

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

async function verifyActiveCategory(page: Page, activeIndex: number, totalCategories: number = 3): Promise<void> {
  const categoriesMenu = page.locator('.workspace-categories-menu');
  const menuItems = categoriesMenu.locator('.workspace-categories-menu-item');

  await expect(menuItems.nth(activeIndex)).toHaveClass(/active/);

  for (let i = 0; i < totalCategories; i++) {
    if (i === activeIndex) continue;
    await expect(menuItems.nth(i)).not.toHaveClass(/active/);
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
  msTest.fail(`Empty workspaces in ${gridMode ? 'grid' : 'list'} mode`, { tag: '@important' }, async ({ connected }) => {
    if (!gridMode) {
      await toggleViewMode(connected);
    }
    const actionBar = connected.locator('#workspaces-ms-action-bar');
    await expect(actionBar.locator('.ms-action-bar-button')).toHaveCount(1);
    await expect(actionBar.getByText('New workspace')).toHaveText('New workspace');
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

  msTest(`Workspace sort order in ${gridMode ? 'grid' : 'list'} mode`, async ({ workspacesStandard }) => {
    if (!gridMode) {
      await toggleViewMode(workspacesStandard);
    }
    for (const wk of WORKSPACES) {
      if (wk !== 'wksp1') {
        await createWorkspace(workspacesStandard, wk);
      }
    }
    // Order by name asc (default)
    // Don't use `sort` because it sorts in place
    let names = [...WORKSPACES].sort((wName1, wName2) => wName1.localeCompare(wName2));
    if (gridMode) {
      await expect(workspacesStandard.locator('.workspaces-container').locator('.workspace-card-content__title')).toHaveText(names);
    } else {
      await expect(workspacesStandard.locator('.workspaces-container').locator('.workspace-name__label')).toHaveText(names);
    }
    const actionBar = workspacesStandard.locator('#workspaces-ms-action-bar');
    const sortSelector = actionBar.locator('#workspace-filter-select');
    await expect(sortSelector).toHaveText('Name');
    await expect(workspacesStandard.locator('.popover-viewport')).toBeHidden();
    await sortSelector.click();
    const popover = workspacesStandard.locator('.popover-viewport');
    const sortItems = popover.getByRole('listitem');
    await expect(sortItems).toHaveCount(3);
    await expect(sortItems).toHaveText(['Ascending', 'Name', 'Role']);
    for (const [index, checked] of [false, true].entries()) {
      if (checked) {
        await expect(sortItems.nth(index)).toHaveTheClass('selected');
      } else {
        await expect(sortItems.nth(index)).not.toHaveTheClass('selected');
      }
    }
    await sortItems.nth(0).click();
    await expect(workspacesStandard.locator('.popover-viewport')).toBeHidden();
    // Order by name desc
    names = [...WORKSPACES].sort((wName1, wName2) => wName2.localeCompare(wName1));
    await sortSelector.click();
    if (gridMode) {
      await expect(workspacesStandard.locator('.workspaces-container').locator('.workspace-card-content__title')).toHaveText(names);
    } else {
      await expect(workspacesStandard.locator('.workspaces-container').locator('.workspace-name__label')).toHaveText(names);
    }
    await expect(workspacesStandard.locator('.popover-viewport').getByRole('listitem').nth(0)).toHaveText('Descending');

    // Order by role desc
    await sortItems.nth(2).click();
    // wksp1 is Reader, should be shown first, other are Owner
    if (gridMode) {
      await expect(workspacesStandard.locator('.workspaces-container').locator('.workspace-card-content__title').first()).toHaveText(
        'wksp1',
      );
    } else {
      await expect(workspacesStandard.locator('.workspaces-container').locator('.workspace-name__label').first()).toHaveText('wksp1');
    }

    // Order by role asc
    await sortSelector.click();
    await sortItems.nth(0).click();
    // wksp1 is Reader, should be shown last, other are Owner
    if (gridMode) {
      await expect(workspacesStandard.locator('.workspaces-container').locator('.workspace-card-content__title').last()).toHaveText(
        'wksp1',
      );
    } else {
      await expect(workspacesStandard.locator('.workspaces-container').locator('.workspace-name__label').last()).toHaveText('wksp1');
    }
  });

  msTest(`Switch to recent workspaces  in ${gridMode ? 'grid' : 'list'} mode`, async ({ workspaces }) => {
    if (!gridMode) {
      await toggleViewMode(workspaces);
      await workspaces.locator('.workspace-list-item').nth(0).click();
    } else {
      await workspaces.locator('.workspace-card-item').nth(0).click();
    }

    await workspaces.locator('#connected-header').locator('.topbar-left').locator('ion-breadcrumb').nth(0).click();

    const categoriesMenu = workspaces.locator('.workspace-categories-menu');
    const recentWorkspacesButton = categoriesMenu.locator('.workspace-categories-menu-item').nth(1);
    await recentWorkspacesButton.click();
    await verifyActiveCategory(workspaces, 1);
    const recentWorkspaces = workspaces.locator('.workspaces-container');

    if (!gridMode) {
      await expect(recentWorkspaces.locator('.workspace-list-item').locator('.workspace-name__label').nth(0)).toHaveText('wksp1');
    } else {
      await expect(recentWorkspaces.locator('.workspace-card-content__title').nth(0)).toHaveText('wksp1');
    }
  });
}

async function toggleFavorite(page: Page, index: number, displaySize: DisplaySize): Promise<void> {
  let item;
  if (displaySize === DisplaySize.Large) {
    if (await isInGridMode(page)) {
      item = page.locator('.workspace-card-item').nth(index);
    } else {
      item = page.locator('.workspace-list-item').nth(index);
      await expect(item).toBeVisible();
      await expect(item.locator('.workspace-favorite-icon')).toBeVisible();
    }
  } else {
    item = page.locator('.workspace-card-item').nth(index);
  }
  await item.locator('.workspace-favorite-icon').click();
}

for (const displaySize of [DisplaySize.Small, DisplaySize.Large]) {
  msTest(`Checks favorites ${displaySize} display`, async ({ workspaces }) => {
    if (displaySize === DisplaySize.Small) {
      await workspaces.setDisplaySize(DisplaySize.Small);
    }
    await createWorkspace(workspaces, 'The Copper Coronet');
    await expect(workspaces.locator('.workspace-card-item').locator('.workspace-card-content__title')).toHaveText([
      'The Copper Coronet',
      'wksp1',
    ]);
    await toggleFavorite(workspaces, 1, displaySize);
    // Put favorite in first
    await expect(workspaces.locator('.workspace-card-item').locator('.workspace-card-content__title')).toHaveText([
      'wksp1',
      'The Copper Coronet',
    ]);
    const workspaceCategoriesMenu = workspaces.locator('.workspace-categories-menu');
    const allWorkspacesButton = workspaceCategoriesMenu.locator('.workspace-categories-menu-item').nth(0);
    const favoriteWorkspacesButton = workspaceCategoriesMenu.locator('.workspace-categories-menu-item').nth(2);
    if (displaySize === DisplaySize.Small) {
      await expect(workspaceCategoriesMenu.locator('.workspace-categories-menu-item__text')).toHaveText([
        'All',
        'Recent',
        'Favorites',
        'Hidden',
      ]);
    } else {
      await expect(workspaceCategoriesMenu.locator('.workspace-categories-menu-item__text')).toHaveText([
        'All workspaces',
        'Recently viewed',
        'Favorites',
        'Hidden',
      ]);
    }
    await expect(workspaceCategoriesMenu).toBeVisible();
    await expect(favoriteWorkspacesButton).toBeVisible();
    await favoriteWorkspacesButton.click({ force: true });
    await allWorkspacesButton.click({ force: true });
    await expect(workspaces.locator('.workspace-card-item').locator('.workspace-card-content__title')).toHaveText([
      'wksp1',
      'The Copper Coronet',
    ]);

    if (displaySize === DisplaySize.Large) {
      await toggleViewMode(workspaces);
      await expect(workspaces.locator('.workspace-list-item').locator('.workspace-name__label')).toHaveText([
        'wksp1',
        'The Copper Coronet',
      ]);
      await toggleFavorite(workspaces, 1, displaySize);
      await expect(workspaces.locator('.workspace-list-item').locator('.workspace-name__label')).toHaveText([
        'The Copper Coronet',
        'wksp1',
      ]);
    } else {
      await toggleFavorite(workspaces, 1, displaySize);
      await expect(workspaces.locator('.workspace-card-item').locator('.workspace-card-content__title')).toHaveText([
        'The Copper Coronet',
        'wksp1',
      ]);
    }
  });

  msTest(`Switch between tabs in ${displaySize} display`, async ({ workspaces }) => {
    if (displaySize === DisplaySize.Small) {
      await workspaces.setDisplaySize(DisplaySize.Small);
    }
    const workspaceCategoriesMenu = workspaces.locator('.workspace-categories-menu');

    await expect(workspaceCategoriesMenu).toBeVisible();
    if (displaySize === DisplaySize.Small) {
      await expect(workspaceCategoriesMenu.locator('.workspace-categories-menu-item__text')).toHaveText([
        'All',
        'Recent',
        'Favorites',
        'Hidden',
      ]);
    } else {
      await expect(workspaceCategoriesMenu.locator('.workspace-categories-menu-item__text')).toHaveText([
        'All workspaces',
        'Recently viewed',
        'Favorites',
        'Hidden',
      ]);
    }
    const allWorkspacesButton = workspaceCategoriesMenu.locator('.workspace-categories-menu-item').nth(0);
    const recentsWorkspacesButton = workspaceCategoriesMenu.locator('.workspace-categories-menu-item').nth(1);
    const favoriteWorkspacesButton = workspaceCategoriesMenu.locator('.workspace-categories-menu-item').nth(2);
    const hiddenWorkspacesButton = workspaceCategoriesMenu.locator('.workspace-categories-menu-item').nth(3);

    await verifyActiveCategory(workspaces, 0);

    await recentsWorkspacesButton.click({ force: true });
    await verifyActiveCategory(workspaces, 1);

    await allWorkspacesButton.click({ force: true });
    await verifyActiveCategory(workspaces, 0);

    await favoriteWorkspacesButton.click({ force: true });
    await verifyActiveCategory(workspaces, 2);

    await hiddenWorkspacesButton.click({ force: true });
    await verifyActiveCategory(workspaces, 3);
  });

  msTest(`Show/hide workspace ${displaySize} display`, async ({ workspaces }) => {
    if (displaySize === DisplaySize.Small) {
      await workspaces.setDisplaySize(DisplaySize.Small);
    }

    await createWorkspace(workspaces, 'The Copper Coronet');
    const workspaceCard = workspaces.locator('.workspace-card-item');

    await expect(workspaceCard.locator('.workspace-card-content__title')).toHaveText(['The Copper Coronet', 'wksp1']);

    await workspaceCard.nth(0).click({ button: 'right' });
    let popover;

    if (displaySize === DisplaySize.Large) {
      popover = workspaces.locator('#workspace-context-menu');
      await expect(popover).toBeVisible();
      await expect(popover.getByRole('group')).toHaveCount(3);
      await expect(popover.getByRole('listitem')).toHaveText([
        'Manage workspace',
        'Rename',
        'History',
        'Hide this workspace',
        'Collaboration',
        'Copy link',
        'Sharing and roles',
        'Miscellaneous',
        'Add to favorites',
      ]);
      await popover.getByRole('listitem').nth(3).click();
    } else {
      popover = workspaces.locator('.workspace-context-sheet-modal');
      await expect(popover).toBeVisible();
      await expect(popover.getByRole('listitem')).toHaveText([
        'Rename',
        'History',
        'Hide this workspace',
        'Copy link',
        'Sharing and roles',
        'Add to favorites',
      ]);
      await popover.getByRole('listitem').nth(2).click();
    }

    await expect(workspaces).toShowToast('The workspace is now hidden in Parsec.', 'Success');

    await expect(workspaceCard.nth(0).locator('.workspace-card-content__title')).toHaveText('wksp1');
    await expect(workspaceCard.nth(1)).toBeHidden();
    await workspaceCard.nth(0).click({ button: 'right' });
    if (displaySize === DisplaySize.Large) {
      popover = workspaces.locator('#workspace-context-menu');
      await expect(popover).toBeVisible();
      await popover.getByRole('listitem').nth(3).click();
    } else {
      popover = workspaces.locator('.workspace-context-sheet-modal');
      await expect(popover).toBeVisible();
      await popover.getByRole('listitem').nth(2).click();
    }
    await expect(workspaceCard.nth(0)).toBeHidden();
    await expect(workspaceCard.nth(1)).toBeHidden();

    await workspaces.locator('.workspace-categories-menu-item').nth(3).click();
    await expect(workspaceCard.nth(0).locator('.workspace-hidden')).toHaveText('Hidden');
    await expect(workspaceCard.nth(0)).toBeVisible();
    await expect(workspaceCard.nth(1).locator('.workspace-hidden')).toHaveText('Hidden');
    await expect(workspaceCard.nth(0)).toBeVisible();

    await workspaceCard.nth(0).click({ button: 'right' });
    await expect(popover).toBeVisible();
    if (displaySize === DisplaySize.Large) {
      popover = workspaces.locator('#workspace-context-menu');
      await expect(popover).toBeVisible();
      await expect(popover.getByRole('group')).toHaveCount(3);
      await expect(popover.getByRole('listitem')).toHaveText([
        'Manage workspace',
        'Rename',
        'History',
        'Show this workspace',
        'Collaboration',
        'Copy link',
        'Sharing and roles',
        'Miscellaneous',
        'Add to favorites',
      ]);
      await popover.getByRole('listitem').nth(3).click();
    } else {
      popover = workspaces.locator('.workspace-context-sheet-modal');
      await expect(popover).toBeVisible();
      await expect(popover.getByRole('listitem')).toHaveText([
        'Rename',
        'History',
        'Show this workspace',
        'Copy link',
        'Sharing and roles',
        'Add to favorites',
      ]);
      await popover.getByRole('listitem').nth(2).click();
    }
  });
}

for (const gridMode of [false, true]) {
  msTest(`Workspace search filter in ${gridMode ? 'grid' : 'list'} mode`, async ({ workspaces }) => {
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

  msTest(`Workspace list filter in ${gridMode ? 'grid' : 'list'} mode`, async ({ workspacesStandard }) => {
    if (!gridMode) {
      await toggleViewMode(workspacesStandard);
    }
    for (const wk of WORKSPACES) {
      if (wk !== 'wksp1') {
        await createWorkspace(workspacesStandard, wk);
      }
    }
    const container = workspacesStandard.locator('.workspaces-container');
    const titles = gridMode ? container.locator('.workspace-card-content__title') : container.locator('.workspace-name__label');
    const roles = gridMode ? container.locator('.workspace-card-bottom').locator('.label-role') : container.locator('.label-role-text');
    await expect(roles).toHaveText(['Owner', 'Owner', 'Owner', 'Reader']);

    const filterButton = workspacesStandard.locator('.filter-button');
    await filterButton.click();
    const filterPopover = workspacesStandard.locator('.filter-popover');
    await expect(filterPopover).toBeVisible();
    await expect(filterPopover.locator('.list-group-title')).toHaveText('Role');
    const popoverItems = filterPopover.locator('.list-group-item');
    await expect(popoverItems).toHaveCount(4);
    await expect(popoverItems).toHaveText(['Owner', 'Manager', 'Contributor', 'Reader']);

    await expect(titles).toHaveCount(4);
    await expect(roles).toHaveText(['Owner', 'Owner', 'Owner', 'Reader']);
    await popoverItems.nth(2).click();
    await expect(titles).toHaveCount(4);
    await expect(roles).toHaveText(['Owner', 'Owner', 'Owner', 'Reader']);
    await popoverItems.nth(0).click();
    await expect(titles).toHaveCount(1);
    await expect(roles).toHaveText(['Reader']);
    await popoverItems.nth(0).click();
    await popoverItems.nth(3).click();
    await expect(titles).toHaveCount(3);
    await expect(roles).toHaveText(['Owner', 'Owner', 'Owner']);
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
  await workspaces.locator('.sidebar').locator('#sidebar-workspaces').locator('#sidebar-all-workspaces').click();
  await expect(workspaces.locator('.topbar-left').locator('.topbar-left__breadcrumb').locator('ion-breadcrumb').nth(0)).toHaveText(
    'My workspaces',
  );
});

msTest('Check no create workspace button as external', async ({ workspacesExternal }) => {
  // Header button
  await expect(workspacesExternal.locator('#workspaces-ms-action-bar').getByText('New workspace')).toBeHidden();
  // Center button with no workspace
  await expect(workspacesExternal.locator('.workspaces-container').locator('.workspace-list-item')).toHaveCount(0);
  await expect(workspacesExternal.locator('#new-workspace')).toBeHidden();
});

msTest('Create new workspace name too long', async ({ workspaces }) => {
  await expect(workspaces.locator('.workspace-card-content__title')).toHaveText(['wksp1']);
  await workspaces.locator('#workspaces-ms-action-bar').getByText('New workspace').click();
  const modal = workspaces.locator('.text-input-modal');
  await expect(modal).toBeVisible();
  const okButton = modal.locator('.ms-modal-footer-buttons').locator('#next-button');
  await fillIonInput(modal.locator('ion-input'), 'A'.repeat(132));
  await expect(modal.locator('.form-error')).toBeVisible();
  await expect(modal.locator('.form-error')).toHaveText('Workspace name is too long, limit is 128 characters.');
  await fillIonInput(modal.locator('ion-input'), 'A'.repeat(64));
  await expect(modal.locator('.form-error')).toBeHidden();
  await expect(okButton).toBeTrulyEnabled();
  await okButton.click();
  await expect(modal).toBeHidden();
  await expect(workspaces).toShowToast(`The workspace '${'A'.repeat(64)}' has been created!`, 'Success');
  await expect(workspaces.locator('.workspace-card-content__title')).toHaveText(['A'.repeat(64), 'wksp1']);
});

msTest('Create new workspace with similar name', async ({ workspaces }) => {
  await expect(workspaces.locator('.workspace-card-content__title')).toHaveText(['wksp1']);
  await workspaces.locator('#workspaces-ms-action-bar').getByText('New workspace').click();
  await fillInputModal(workspaces, 'Workspace');
  await expect(workspaces).toShowToast("The workspace 'Workspace' has been created!", 'Success');
  await expect(workspaces.locator('.workspace-card-content__title')).toHaveText(['wksp1', 'Workspace']);

  const wkList = ['wksp1', 'Workspace'];
  for (const newWorkspace of ['Workspace', 'workspace', 'Workspace 1', 'A Workspace']) {
    await workspaces.locator('#workspaces-ms-action-bar').getByText('New workspace').click();
    // Similar name, saying no, doesn't create the workspace
    await fillInputModal(workspaces, newWorkspace);
    await answerQuestion(workspaces, false, {
      expectedNegativeText: 'Cancel',
      expectedPositiveText: 'Create anyway',
      expectedQuestionText: 'Another workspace already exists with the same or a similar name. Do you still want to create this workspace?',
      expectedTitleText: 'Duplicate workspace name',
    });
    // Similar name, saying yes, does create the workspace
    await fillInputModal(workspaces, newWorkspace);
    await answerQuestion(workspaces, true);
    await expect(workspaces).toShowToast(`The workspace '${newWorkspace}' has been created!`, 'Success');
    wkList.push(newWorkspace);
    wkList.sort((a, b) => a.localeCompare(b));
    await expect(workspaces.locator('.workspace-card-content__title')).toHaveText(wkList);
  }
});

msTest('Check no favorite or recent workspaces', async ({ connected }) => {
  const workspaceCategoriesMenu = connected.locator('.workspace-categories-menu');

  const recentsWorkspacesButton = workspaceCategoriesMenu.locator('.workspace-categories-menu-item').nth(1);
  await expect(recentsWorkspacesButton).toBeVisible();
  await recentsWorkspacesButton.click({ force: true });
  await expect(connected.locator('.workspaces-container').locator('.no-recent-workspaces')).toBeVisible();
  await expect(connected.locator('.workspaces-container').locator('.no-recent-workspaces').locator('ion-text')).toHaveText(
    'You have not consulted any workspaces yet. Recently consulted workspaces will be listed here.',
  );

  const favoriteWorkspacesButton = workspaceCategoriesMenu.locator('.workspace-categories-menu-item').nth(2);
  await expect(favoriteWorkspacesButton).toBeVisible();
  await favoriteWorkspacesButton.click({ force: true });
  await expect(connected.locator('.workspaces-container').locator('.no-favorite-workspaces')).toBeVisible();
  await expect(connected.locator('.workspaces-container').locator('.no-favorite-workspaces').locator('ion-text')).toHaveText(
    'You have not set any workspaces as favorite. Favorite a workspace to have it be listed here.',
  );
});
