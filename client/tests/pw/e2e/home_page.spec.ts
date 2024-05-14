// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { expect } from '@tests/pw/helpers/assertions';
import { msTest } from '@tests/pw/helpers/fixtures';
import { answerQuestion, fillIonInput } from '@tests/pw/helpers/utils';

msTest('Home default state with devices', async ({ home }) => {
  await expect(home.locator('.organization-title')).toHaveText('Your organizations');
  const cards = home.locator('.organization-list').locator('.organization-card');

  await expect(cards).toHaveCount(5);

  for (const card of await cards.all()) {
    await expect(card.locator('.organization-info').locator('.title-h4')).toHaveText(/Org\d+/);
    await expect(card.locator('.card-content-footer-login').nth(0)).toHaveText('--');
  }
  await expect(cards.nth(0).locator('.organization-info').locator('.subtitles-sm')).toHaveText('Alicey McAliceFace');
});

msTest('Login', async ({ home }) => {
  await home.locator('.organization-list').locator('.organization-card').nth(0).click();
  await expect(home.locator('.login-header__title')).toHaveText('Log in');
  const loginButton = home.locator('.login-card-footer').locator('.login-button');
  await expect(loginButton).toHaveDisabledAttribute();
  const input = home.locator('#password-input').locator('ion-input');
  await fillIonInput(input, 'P@ssw0rd.');
  await expect(loginButton).not.toHaveDisabledAttribute();
  await loginButton.click();
  await expect(home).toBeWorkspacePage();
  await expect(home).toHaveHeader(['My workspaces'], false);
  const profile = home.locator('.topbar').locator('.profile-header');
  await expect(profile.locator('.text-content-name')).toHaveText('Gordon Freeman');
});

msTest('Login page and back to device list', async ({ home }) => {
  await home.locator('.organization-list').locator('.organization-card').nth(0).click();
  await expect(home.locator('.login-header__title')).toHaveText('Log in');
  await expect(home.locator('.organization-list')).toBeHidden();
  const backButton = home.locator('.topbar-left__back-button');
  await expect(backButton).toHaveText('Return to organizations');
  await backButton.click();
  await expect(home.locator('.organization-list')).toBeVisible();
});

msTest('Login with invalid password', async ({ home }) => {
  await home.locator('.organization-list').locator('.organization-card').nth(0).click();
  const loginButton = home.locator('.login-card-footer').locator('.login-button');
  await expect(loginButton).toHaveDisabledAttribute();
  await expect(home.locator('#password-input').locator('.form-error')).toBeHidden();
  const input = home.locator('#password-input').locator('ion-input');
  await fillIonInput(input, 'InvalidP@ssw0rd.');
  await expect(loginButton).not.toHaveDisabledAttribute();
  await loginButton.click();
  await expect(home.locator('#password-input').locator('.form-error')).toBeVisible();
  await expect(home.locator('#password-input').locator('.form-error')).toHaveText('Incorrect password.');
  await expect(home).toBeHomePage();
});

msTest('Logout and go back to devices list', async ({ home }) => {
  await home.locator('.organization-list').locator('.organization-card').nth(0).click();
  await fillIonInput(home.locator('#password-input').locator('ion-input'), 'P@ssw0rd.');
  await home.locator('.login-card-footer').locator('.login-button').click();
  await expect(home).toBeWorkspacePage();
  await home.locator('.topbar').locator('.profile-header').click();
  const buttons = home.locator('.profile-header-popover').locator('.main-list').getByRole('listitem');
  await buttons.nth(2).click();
  await answerQuestion(home, true);
  await expect(home.locator('.organization-title')).toHaveText('Your organizations');
  await expect(home).toBeHomePage();
});
