// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { Locator, Page } from '@playwright/test';
import { expect } from '@tests/pw/helpers/assertions';
import { msTest } from '@tests/pw/helpers/fixtures';

async function openModalWithUser(usersPage: Page, userIndex: number): Promise<Locator> {
  await expect(usersPage.locator('.user-details-modal')).toBeHidden();
  const user = usersPage.locator('#users-page-user-list').getByRole('listitem').nth(userIndex);
  await user.hover();
  await user.locator('.options-button').click();
  const popover = usersPage.locator('.user-context-menu');
  await popover.getByRole('listitem').nth(3).click();
  const modal = usersPage.locator('.user-details-modal');
  await expect(modal).toBeVisible();
  return modal;
}

msTest('User details modal', async ({ usersPage }) => {
  const modal = await openModalWithUser(usersPage, 3);

  await expect(modal.locator('.ms-modal-header__title')).toHaveText('Member details');
  const detailsItems = modal.locator('.ms-modal-content').locator('.details-item');
  await expect(detailsItems.nth(0).locator('.details-item__title')).toHaveText('Name');
  // cspell:disable-next-line
  await expect(detailsItems.nth(0).locator('.details-item__text')).toHaveText('Cernd');
  await expect(detailsItems.nth(1).locator('.details-item__title')).toHaveText('Joined');
  await expect(detailsItems.nth(1).locator('.details-item__text')).toHaveText(/^now|< 1 minute$/);
  await expect(modal.locator('.workspace-list')).toBeVisible();
  await expect(modal.locator('.workspace-empty')).toBeHidden();
  await expect(modal.locator('.workspace-list').locator('.workspace-list-item').locator('.item-container__name')).toHaveText([
    'Trademeet',
    'The Copper Coronet',
  ]);
  await expect(modal.locator('.workspace-list').locator('.workspace-list-item').locator('.label-role')).toHaveText(['Reader', 'Reader']);
});

msTest('User details modal no common workspaces', async ({ usersPage }) => {
  const modal = await openModalWithUser(usersPage, 4);

  await expect(modal.locator('.ms-modal-header__title')).toHaveText('Member details');
  const detailsItems = modal.locator('.ms-modal-content').locator('.details-item');
  await expect(detailsItems.nth(0).locator('.details-item__title')).toHaveText('Name');
  await expect(detailsItems.nth(0).locator('.details-item__text')).toHaveText('Patches');
  await expect(detailsItems.nth(1).locator('.details-item__title')).toHaveText('Joined');
  await expect(detailsItems.nth(1).locator('.details-item__text')).toHaveText('Oct 6, 2009');
  await expect(modal.locator('.workspace-list')).toBeHidden();
  await expect(modal.locator('.workspace-empty')).toBeVisible();
  await expect(modal.locator('.workspace-empty')).toHaveText('You have no workspaces in common with this user.');
});

msTest('Close modal with X', async ({ usersPage }) => {
  const modal = await openModalWithUser(usersPage, 3);
  await modal.locator('.closeBtn').click();
  await expect(modal).toBeHidden();
});
