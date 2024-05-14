// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { expect } from '@tests/pw/helpers/assertions';
import { msTest } from '@tests/pw/helpers/fixtures';

msTest('Profile popover default state', async ({ connected }) => {
  await expect(connected.locator('.profile-header-popover')).toBeHidden();
  await connected.locator('.topbar').locator('.profile-header').click();
  await expect(connected.locator('.profile-header-popover')).toBeVisible();
  const popover = connected.locator('.profile-header-popover');
  const header = popover.locator('.header-list');
  await expect(header.locator('.header-list-email')).toHaveText('user@host.com');
  await expect(header.locator('.profile')).toHaveText('Administrator');

  const buttons = popover.locator('.main-list').getByRole('listitem');
  await expect(buttons).toHaveText(['My profile', 'Settings', 'Log out']);

  const footer = popover.locator('.footer-list');
  await expect(footer.locator('ion-item')).toHaveText(['Help and feedback', /About \(v.+\)/]);
});
