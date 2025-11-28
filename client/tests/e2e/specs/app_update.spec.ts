// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { Page } from '@playwright/test';
import { DisplaySize, expect, MsPage, msTest, setupNewPage } from '@tests/e2e/helpers';

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

msTest('Opens app update modal on home page', async ({ context }) => {
  const home = (await context.newPage()) as MsPage;
  await setupNewPage(home, { enableUpdateEvent: true });

  await checkAppUpdateModal(home);
});

for (const displaySize of [DisplaySize.Small, DisplaySize.Large]) {
  msTest(`Opens app update modal with notification on ${displaySize}`, async ({ context }) => {
    const page = (await context.newPage()) as MsPage;
    await setupNewPage(page, { enableUpdateEvent: true });
    const modal = page.locator('.update-app-modal');
    await expect(modal).toBeVisible();
    await modal.locator('.update-footer').locator('ion-button').nth(1).click();
    await expect(modal).toBeHidden();
    await page.locator('.organization-card').first().click();
    await expect(page.locator('#password-input')).toBeVisible();

    await expect(page.locator('.login-button')).toHaveDisabledAttribute();

    await page.locator('#password-input').locator('input').fill('P@ssw0rd.');
    await expect(page.locator('.login-button')).toBeEnabled();
    await page.locator('.login-button').click();
    await expect(page.locator('#connected-header')).toContainText('My workspaces');
    await expect(page.locator('.topbar-right').locator('.text-content-name')).toHaveText('Alicey McAliceFace');
    await expect(page).toBeWorkspacePage();
    const connected = page;

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

msTest('Opens app update modal with profile popover', async ({ context }) => {
  const page = (await context.newPage()) as MsPage;
  await setupNewPage(page, { enableUpdateEvent: true });
  const modal = page.locator('.update-app-modal');
  await expect(modal).toBeVisible();
  await modal.locator('.update-footer').locator('ion-button').nth(1).click();
  await expect(modal).toBeHidden();
  await page.locator('.organization-card').first().click();
  await expect(page.locator('#password-input')).toBeVisible();

  await expect(page.locator('.login-button')).toHaveDisabledAttribute();

  await page.locator('#password-input').locator('input').fill('P@ssw0rd.');
  await expect(page.locator('.login-button')).toBeEnabled();
  await page.locator('.login-button').click();
  await expect(page.locator('#connected-header')).toContainText('My workspaces');
  await expect(page.locator('.topbar-right').locator('.text-content-name')).toHaveText('Alicey McAliceFace');
  await expect(page).toBeWorkspacePage();
  const connected = page;

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
