// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { Page } from '@playwright/test';
import { DisplaySize, expect, msTest, sendUpdateEvent } from '@tests/e2e/helpers';

async function checkAppUpdateModal(page: Page): Promise<void> {
  const modal = page.locator('.update-app-modal');
  await expect(modal).toBeVisible();
  await expect(modal.locator('.update-version').locator('.update-version__item')).toHaveText('13.37');
  await expect(modal.locator('.update-footer').locator('ion-button').nth(0)).toHaveText('Update Parsec');
  await expect(modal.locator('.update-footer').locator('ion-button').nth(1)).toHaveText('Later');

  const newTabPromise = page.waitForEvent('popup');
  await modal.locator('.update-version').locator('.update-version__item').click();
  const newTab = await newTabPromise;
  await newTab.waitForLoadState();
  await expect(newTab).toHaveURL(new RegExp('^https://docs\\.parsec\\.cloud/.+'));
  await newTab.close();
}

msTest('Opens app update modal automatically on home page', async ({ home }) => {
  await sendUpdateEvent(home);
  const updateContainer = home.locator('.update-container');
  await expect(updateContainer.locator('.update-text')).toHaveText('A new version is available');

  await checkAppUpdateModal(home);
});

msTest('Prevent to open app update modal automatically if a modal is already opened on home page', async ({ home }) => {
  await expect(home.locator('.settings-modal')).toBeHidden();
  await home.locator('.menu-secondary').locator('#trigger-settings-button').click();
  await sendUpdateEvent(home);
  await expect(home.locator('.settings-modal')).toBeVisible();

  const modal = home.locator('.update-app-modal');
  await expect(modal).toBeHidden();
});

msTest('Opens app update modal automatically after being connected', async ({ connected }) => {
  await sendUpdateEvent(connected);
  await checkAppUpdateModal(connected);
});

msTest('Prevent to open app update modal automatically if a modal is already opened on connected page', async ({ workspaces }) => {
  await expect(workspaces.locator('.workspace-sharing-modal')).toBeHidden();
  await workspaces.locator('.workspaces-container-grid').locator('.workspace-card-item').nth(0).hover();
  await workspaces.locator('.workspaces-container-grid').locator('.workspace-card-item').nth(0).locator('.icon-share-container').click();
  await expect(workspaces.locator('.workspace-sharing-modal')).toBeVisible();
  await sendUpdateEvent(workspaces);

  const modal = workspaces.locator('.update-app-modal');
  await expect(modal).toBeHidden();
});

msTest("Dismiss update on home page and update modal shouldn't be appear again", async ({ home }) => {
  await sendUpdateEvent(home);
  const modal = home.locator('.update-app-modal');
  await checkAppUpdateModal(home);
  await expect(modal).toBeVisible();
  await expect(modal.locator('.update-footer').locator('ion-button').nth(1)).toHaveText('Later');
  await modal.locator('.update-footer').locator('ion-button').nth(1).click();
  await expect(modal).toBeHidden();

  await home.locator('.organization-card').first().click();
  await expect(home.locator('#password-input')).toBeVisible();
  await sendUpdateEvent(home);
  await expect(modal).toBeHidden();
});

msTest("Dismiss update on connected page and update modal shouldn't be appear again", async ({ workspaces }) => {
  await sendUpdateEvent(workspaces);
  const modal = workspaces.locator('.update-app-modal');
  await checkAppUpdateModal(workspaces);
  await expect(modal).toBeVisible();
  await expect(modal.locator('.update-footer').locator('ion-button').nth(1)).toHaveText('Later');
  await modal.locator('.update-footer').locator('ion-button').nth(1).click();
  await expect(modal).toBeHidden();

  await sendUpdateEvent(workspaces);

  await expect(modal).toBeHidden();
});

for (const displaySize of [DisplaySize.Small, DisplaySize.Large]) {
  msTest(`Opens app update modal with notification on ${displaySize}`, async ({ connected }) => {
    await sendUpdateEvent(connected);
    const modal = connected.locator('.update-app-modal');
    await modal.locator('.update-footer').locator('ion-button').nth(1).click();
    const header = connected.locator('#connected-header');
    const notifButton = header.locator('#trigger-notifications-button');
    await expect(notifButton).toHaveTheClass('unread');
    const notifCenter =
      displaySize === DisplaySize.Small
        ? connected.locator('.notification-center-modal')
        : connected.locator('.notification-center-popover');

    if (displaySize === DisplaySize.Small) {
      await connected.setDisplaySize(DisplaySize.Small);
    }
    await expect(notifCenter).toBeHidden();
    await notifButton.click();
    await expect(notifCenter).toBeVisible();
    await expect(notifCenter.locator('.notification-center-header__counter')).toHaveText('1');
    const notifs = notifCenter.locator('.notification-container').locator('.notification');
    await expect(notifs).toHaveCount(1);
    await expect(notifs.nth(0).locator('.notification-details__message')).toHaveText('A new version is available (v13.37)');
    await notifs.nth(0).click();
    await expect(notifCenter).toBeHidden();

    await checkAppUpdateModal(connected);
  });
}

msTest('Opens app update modal with profile popover', async ({ connected }) => {
  await sendUpdateEvent(connected);
  const modal = connected.locator('.update-app-modal');
  await modal.locator('.update-footer').locator('ion-button').nth(1).click();

  const header = connected.locator('#connected-header');
  await expect(header.locator('#profile-button').locator('.text-content-update')).toBeVisible();
  await expect(header.locator('#profile-button').locator('.text-content-update')).toHaveText('Update available');
  await header.locator('#profile-button').click();
  const profilePopover = connected.locator('.profile-header-organization-popover');
  await expect(profilePopover).toBeVisible();
  await expect(profilePopover.locator('.update-item')).toBeVisible();
  await expect(profilePopover.locator('.update-item').locator('.update-item-text')).toHaveText('New update available');
  await expect(profilePopover.locator('.update-item').locator('.update-item-version')).toHaveText('Parsec Cloud(v13.37)');
  await profilePopover.locator('.update-item').click();
  await expect(profilePopover).toBeHidden();
  await checkAppUpdateModal(connected);
});
