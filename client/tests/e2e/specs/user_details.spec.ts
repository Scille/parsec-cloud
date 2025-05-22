// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { Locator } from '@playwright/test';
import { answerQuestion, expect, MsPage, msTest } from '@tests/e2e/helpers';
import { DateTime } from 'luxon';

async function openModalWithUser(usersPage: MsPage, userIndex: number, revokeFirst?: boolean): Promise<Locator> {
  await expect(usersPage.locator('.user-details-modal')).toBeHidden();
  const user = usersPage.locator('#users-page-user-list').getByRole('listitem').nth(userIndex);
  const isOutsider = (await user.locator('.user-profile').innerText()) === 'External';

  if (revokeFirst) {
    await user.hover();
    await user.locator('.options-button').click();
    await usersPage.locator('.user-context-menu').getByRole('listitem').nth(1).click();
    await answerQuestion(usersPage, true);
    await expect(usersPage).toShowToast('Malloryy McMalloryFace has been revoked. They can no longer access this organization.', 'Success');
    await expect(user).toHaveTheClass('revoked');
  }
  await user.hover();
  await user.locator('.options-button').click();
  const popover = usersPage.locator('.user-context-menu');
  let nth = 5;
  if (isOutsider) {
    nth = 3;
  }
  if (revokeFirst) {
    nth = 1;
  }

  await popover.getByRole('listitem').nth(nth).click();
  const modal = usersPage.locator('.user-details-modal');
  await expect(modal).toBeVisible();
  return modal;
}

msTest('User details modal', async ({ usersPage }) => {
  const modal = await openModalWithUser(usersPage, 1);

  await expect(modal.locator('.ms-modal-header__title')).toHaveText('User details');
  const detailsItems = modal.locator('.ms-modal-content').locator('.details-item');
  await expect(detailsItems.nth(0).locator('.details-item-name__title')).toHaveText('Name');
  await expect(detailsItems.nth(0).locator('.details-item-name__text')).toHaveText('Boby McBobFace');
  const timeItems = modal.locator('.ms-modal-content').locator('.time-list-item');
  await expect(timeItems.nth(0).locator('.time-list-item__title')).toHaveText('Joined');
  await expect(timeItems.nth(0).locator('.time-list-item__text')).toHaveText('Jan 3, 2000');
  await expect(modal.locator('.label-id')).toHaveText(/^(Internal ID: )[a-f0-9]+$/);
  await expect(modal.locator('.workspace-list')).toBeVisible();
  await expect(modal.locator('.workspace-empty')).toBeHidden();
  await expect(modal.locator('.workspace-list').locator('.workspace-list-item').locator('.item-container__name')).toHaveText('wksp1');
  await expect(modal.locator('.workspace-list').locator('.workspace-list-item').locator('.label-role')).toHaveText('Reader');
});

msTest('User details modal no common workspaces', async ({ usersPage }) => {
  const modal = await openModalWithUser(usersPage, 2);

  await expect(modal.locator('.ms-modal-header__title')).toHaveText('User details');
  const detailsItems = modal.locator('.ms-modal-content').locator('.details-item');
  await expect(detailsItems.nth(0).locator('.details-item-name__title')).toHaveText('Name');
  // cspell:disable-next-line
  await expect(detailsItems.nth(0).locator('.details-item-name__text')).toHaveText('Malloryy McMalloryFace');
  await expect(detailsItems.nth(0).locator('.label-status')).toBeVisible();
  const timeItems = modal.locator('.ms-modal-content').locator('.time-list-item');
  await expect(timeItems.nth(0).locator('.time-list-item__title')).toHaveText('Joined');
  await expect(timeItems.nth(0).locator('.time-list-item__text')).toHaveText('Jan 6, 2000');
  await expect(modal.locator('.workspace-list')).toBeHidden();
  await expect(modal.locator('.workspace-empty')).toBeVisible();
  await expect(modal.locator('.workspace-empty')).toHaveText('You have no workspaces in common with this user.');
});

msTest('Revoked user details modal no common workspaces', async ({ usersPage }) => {
  const modal = await openModalWithUser(usersPage, 2, true);

  await expect(modal.locator('.ms-modal-header__title')).toHaveText('User details');
  const detailsItems = modal.locator('.ms-modal-content').locator('.details-item');
  await expect(detailsItems.nth(0).locator('.details-item-name__title')).toHaveText('Name');
  await expect(detailsItems.nth(0).locator('.details-item-name__text')).toHaveText('Malloryy McMalloryFace');
  await expect(detailsItems.nth(0).locator('.label-status')).toBeVisible();
  const timeItems = modal.locator('.ms-modal-content').locator('.time-list-item');
  await expect(timeItems.nth(0).locator('.time-list-item__title')).toHaveText('Joined');
  await expect(timeItems.nth(0).locator('.time-list-item__text')).toHaveText('Jan 6, 2000');
  await expect(timeItems.nth(1).locator('.time-list-item__title')).toHaveText('Revoked since');
  await expect(timeItems.nth(1).locator('.time-list-item__text')).toContainText(`${DateTime.now().day}`);
  await expect(modal.locator('.workspace-list')).toBeHidden();
  await expect(modal.locator('.workspace-empty')).toBeVisible();
  await expect(modal.locator('.workspace-empty')).toHaveText('You have no workspaces in common with this user.');
});

msTest('Close modal with X', async ({ usersPage }) => {
  const modal = await openModalWithUser(usersPage, 1);
  await modal.locator('.closeBtn').click();
  await expect(modal).toBeHidden();
});
