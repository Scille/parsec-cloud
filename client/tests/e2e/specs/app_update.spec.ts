// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { Page } from '@playwright/test';
import { expect, MsPage, msTest, sendUpdateEvent, setupNewPage } from '@tests/e2e/helpers';

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
  await expect(newTab).toHaveURL(new RegExp('^https://docs\\.parsec\\.cloud/en/v[a-z0-9\\.\\-_]+/history\\.html$'));

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
  await expect(home.locator('.settings-modal')).toBeVisible();
  await sendUpdateEvent(home);
  await expect(home.locator('.update-app-modal')).toBeHidden();
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

msTest('Opens app update modal with profile popover', async ({ connected }) => {
  await sendUpdateEvent(connected);
  const modal = connected.locator('.update-app-modal');
  await modal.locator('.update-footer').locator('ion-button').nth(1).click();

  const sidebar = connected.locator('.sidebar');
  await expect(sidebar.locator('#profile-button').locator('.text-content-update')).toBeVisible();
  await expect(sidebar.locator('#profile-button').locator('.text-content-update')).toHaveText('Update available');
  await sidebar.locator('#profile-button').click();
  const profilePopover = connected.locator('.profile-header-organization-popover');
  await expect(profilePopover).toBeVisible();
  await expect(profilePopover.locator('.update-item')).toBeVisible();
  await expect(profilePopover.locator('.update-item').locator('.update-item-text')).toHaveText('New update available');
  await expect(profilePopover.locator('.update-item').locator('.update-item-version')).toHaveText('Parsec Cloud(v13.37)');
  await profilePopover.locator('.update-item').click();
  await expect(profilePopover).toBeHidden();
  await checkAppUpdateModal(connected);
});

for (const version of ['same', 'different']) {
  msTest(`Detect app update ${version} version`, async ({ context }) => {
    const page = (await context.newPage()) as MsPage;
    // Loads the app on its origin; first visit stores the current APP_VERSION
    await setupNewPage(page);

    const banner = page.locator('#app-updated-banner');
    await expect(banner).toBeHidden();

    if (version === 'different') {
      // Set a different version in the stored config
      await page.evaluate(async () => {
        // Helper to make indexedDB API async
        const p = (req: any): Promise<any> =>
          new Promise((res, rej) => {
            req.onsuccess = (): void => res(req.result);
            req.onerror = (): void => rej(req.error);
          });

        const db = await p(indexedDB.open('_ionicstorage'));
        const store = db.transaction('_ionickv', 'readwrite').objectStore('_ionickv');
        const cfg = (await p(store.get('config'))) ?? {};
        cfg.appVersionLastVisit = 'v42.0.0';
        await p(store.put(cfg, 'config'));
        db.close();
      });
    }

    // Can't access storage before the page is initialized so we initialize it first,
    // and then reload with the new config set.
    await page.reload();
    await setupNewPage(page, { skipTestbed: true, skipGoto: true });

    if (version === 'same') {
      await expect(banner).toBeHidden();
    } else {
      await expect(banner.locator('.update-banner-whats-new-text')).toHaveText(/Parsec has been updated to [a-z\.\-_0-9\+]+\!/);
      await expect(banner).toBeVisible();
      const newTabPromise = page.waitForEvent('popup');
      await banner.locator('.update-banner-whats-new').click();
      const newTab = await newTabPromise;
      await newTab.waitForLoadState();
      await expect(newTab).toHaveURL(new RegExp('^https://parsec\\.cloud/blog/releases/\\d{1,2}\\.\\d{1,2}/$'));
    }
    await page.release();
  });
}
