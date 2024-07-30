// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { expect } from '@tests/pw/helpers/assertions';
import { MockBms } from '@tests/pw/helpers/bms';
import { DEFAULT_USER_INFORMATION } from '@tests/pw/helpers/data';
import { msTest } from '@tests/pw/helpers/fixtures';
import { fillIonInput } from '@tests/pw/helpers/utils';

msTest('Log into the customer area', async ({ home }) => {
  await MockBms.mockLogin(home, true);
  await MockBms.mockUserInfo(home);

  const button = home.locator('.topbar-right-buttons').locator('ion-button').nth(2);
  await expect(button).toHaveText('Customer Area');
  await button.click();
  await expect(home).toHaveURL(/.+\/clientLogin$/);
  await fillIonInput(home.locator('.input-container').nth(0).locator('ion-input'), DEFAULT_USER_INFORMATION.email);
  await fillIonInput(home.locator('.input-container').nth(1).locator('ion-input'), DEFAULT_USER_INFORMATION.password);
  await home.locator('.login-button').locator('ion-button').click();
  await expect(home).toHaveURL(/.+\/clientArea$/);
  await expect(home.locator('.header-content').locator('ion-button')).toHaveText('BYE BYE');
  await home.locator('.header-content').locator('ion-button').click();
  await expect(home).toHaveURL(/.+\/home$/);
});

msTest('Log into the customer area failed', async ({ home }) => {
  await MockBms.mockLogin(home, false);
  await MockBms.mockUserInfo(home);

  const button = home.locator('.topbar-right-buttons').locator('ion-button').nth(2);
  await expect(button).toHaveText('Customer Area');
  await button.click();
  await expect(home).toHaveURL(/.+\/clientLogin$/);
  await fillIonInput(home.locator('.input-container').nth(0).locator('ion-input'), DEFAULT_USER_INFORMATION.email);
  await fillIonInput(home.locator('.input-container').nth(1).locator('ion-input'), 'invalid_password');
  await home.locator('.login-button').locator('ion-button').click();
  const error = home.locator('.saas-login-container').locator('.login-button-error');
  await expect(error).toBeVisible();
  await expect(error).toHaveText('Cannot log in. Please check your email and password.');
});
