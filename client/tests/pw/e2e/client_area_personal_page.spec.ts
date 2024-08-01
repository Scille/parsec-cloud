// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { expect } from '@tests/pw/helpers/assertions';
import { DEFAULT_USER_INFORMATION } from '@tests/pw/helpers/data';
import { msTest } from '@tests/pw/helpers/fixtures';

msTest('Switch pages', async ({ clientArea }) => {
  const title = clientArea.locator('.header-content').locator('.header-title');
  await expect(clientArea).toHaveURL(/.+\/clientArea$/);
  await expect(title).toHaveText('Dashboard');
  const avatar = clientArea.locator('.header-content').locator('.header-right-profile');
  await expect(avatar.locator('.person-name')).toHaveText(DEFAULT_USER_INFORMATION.name);
  await avatar.click();
  await expect(title).toHaveText('My profile');
  const container = clientArea.locator('.personal-data-container');
  const items = container.locator('.ms-summary-card-item');
  await expect(items.locator('.ms-summary-card-item__label')).toHaveText([
    'Firstname',
    'Lastname',
    'Phone',
    'Company',
    'Job',
    'Email',
    'Password',
  ]);
  await expect(items.locator('.ms-summary-card-item__text')).toHaveText([
    DEFAULT_USER_INFORMATION.firstName,
    DEFAULT_USER_INFORMATION.lastName,
    '',
    '',
    '',
    DEFAULT_USER_INFORMATION.email,
    '*********',
  ]);
});
