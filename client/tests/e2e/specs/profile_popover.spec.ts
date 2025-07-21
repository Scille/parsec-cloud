// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { expect, msTest } from '@tests/e2e/helpers';

msTest('Profile popover default state', async ({ connected }) => {
  await expect(connected.locator('.profile-header-organization-popover')).toBeHidden();
  await expect(connected.locator('.topbar').locator('.profile-header').locator('.text-content-name')).toHaveText('Alicey McAliceFace');
  await connected.locator('.topbar').locator('.profile-header').click();
  await expect(connected.locator('.profile-header-organization-popover')).toBeVisible();
  const popover = connected.locator('.profile-header-organization-popover');
  const header = popover.locator('.header-list');
  await expect(header.locator('.header-list-email')).toHaveText('alice@example.com');
  await expect(header.locator('.profile')).toHaveText('Administrator');

  const buttons = popover.locator('.main-list').getByRole('listitem');
  await expect(buttons).toHaveText(['Settings', 'My devices', 'Authentication', 'Recovery file', 'Log out']);

  const footer = popover.locator('.footer-list');
  await expect(footer.locator('ion-item')).toHaveText(['Documentation', 'Feedback', /About \(v.+\)/, 'Report a bug']);
});

msTest('Profile popover open documentation', async ({ connected }) => {
  await connected.locator('.topbar').locator('.profile-header').click();
  const popover = connected.locator('.profile-header-organization-popover');
  const newTabPromise = connected.waitForEvent('popup');
  await popover.locator('.footer-list').locator('ion-item').nth(0).click();
  const newTab = await newTabPromise;
  await newTab.waitForLoadState();
  await expect(newTab).toHaveURL(new RegExp('https://docs.parsec.cloud/(en|fr)/[a-z0-9-+.]+'));
});

msTest('Profile popover open feedback', async ({ connected }) => {
  await connected.locator('.topbar').locator('.profile-header').click();
  const popover = connected.locator('.profile-header-organization-popover');
  const newTabPromise = connected.waitForEvent('popup');
  await popover.locator('.footer-list').locator('ion-item').nth(1).click();
  const newTab = await newTabPromise;
  await newTab.waitForLoadState();
  await expect(newTab).toHaveURL(new RegExp('https://sign(-dev)?.parsec.cloud/contact'));
});

msTest('Profile popover go to about', async ({ connected }) => {
  await connected.locator('.topbar').locator('.profile-header').click();
  const popover = connected.locator('.profile-header-organization-popover');
  await popover.locator('.footer-list').locator('ion-item').nth(2).click();
  await expect(connected.locator('.profile-content-item').locator('.item-header__title')).toHaveText('About');
});

msTest('Profile popover go to profile', async ({ connected }) => {
  await connected.locator('.topbar').locator('.profile-header').click();
  const popover = connected.locator('.profile-header-organization-popover');
  await popover.locator('.main-list').getByRole('listitem').nth(0).click();
  await expect(connected.locator('#connected-header').locator('.topbar-left-text__title')).toHaveText('My profile');
});

msTest('Profile popover go to settings', async ({ connected }) => {
  await connected.locator('.topbar').locator('.profile-header').click();
  const popover = connected.locator('.profile-header-organization-popover');
  await popover.locator('.main-list').getByRole('listitem').nth(0).click();
  await expect(connected.locator('.profile-content-item').locator('.item-header__title')).toHaveText('Settings');
});
