// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { DEFAULT_USER_INFORMATION, MockBms, MsPage, expect, fillIonInput, msTest, setupNewPage } from '@tests/e2e/helpers';

msTest('Client area forgot password', async ({ context }) => {
  const home = (await context.newPage()) as MsPage;
  await setupNewPage(home, { enableStripe: true });

  await MockBms.mockChangePassword(home);
  const button = home.locator('.homepage-header').locator('#trigger-customer-area-button');
  await expect(button).toHaveText('Customer area');
  await button.click();
  await expect(home.locator('.saas-login-link__forgotten-password')).toHaveText('Forgot your password?');
  await home.locator('.saas-login-link__forgotten-password').click();
  const container = home.locator('.saas-forgot-password');
  await expect(container).toBeVisible();
  await expect(container.locator('.modal-header-title__text')).toHaveText('Forgot password');
  const buttons = container.locator('.saas-forgot-password-button').locator('.saas-forgot-password-button__item');
  await expect(buttons.nth(1)).toHaveText('Submit');
  await expect(buttons.nth(1)).toBeTrulyDisabled();
  await fillIonInput(container.locator('ion-input'), DEFAULT_USER_INFORMATION.email);
  await expect(buttons.nth(1)).toBeTrulyEnabled();
  await buttons.nth(1).click();
  await expect(container.locator('.modal-header-title__text')).toHaveText('Check Your Inbox');
  await expect(container.locator('.modal-header-title__text')).toBeVisible();
  await expect(buttons.nth(0)).toBeTrulyEnabled();
  await expect(buttons.nth(0)).toHaveText('Log in');
  await buttons.nth(0).click();
  await expect(home.locator('.saas-login-container').locator('.saas-login__title')).toHaveText('Log in to your customer account');
});

msTest('Client area forgot password failed', async ({ context }) => {
  const home = (await context.newPage()) as MsPage;
  await setupNewPage(home, { enableStripe: true });

  await MockBms.mockChangePassword(home, { POST: { errors: { status: 400 } } });
  const button = home.locator('.homepage-header').locator('#trigger-customer-area-button');
  await button.click();
  await home.locator('.saas-login-link__forgotten-password').click();
  const container = home.locator('.saas-forgot-password');
  await expect(container).toBeVisible();
  const buttons = container.locator('.saas-forgot-password-button').locator('.saas-forgot-password-button__item');
  await expect(buttons.nth(1)).toHaveText('Submit');
  await expect(buttons.nth(1)).toBeTrulyDisabled();
  await fillIonInput(container.locator('ion-input'), DEFAULT_USER_INFORMATION.email);
  await expect(buttons.nth(1)).toBeTrulyEnabled();
  await buttons.nth(1).click();
  await expect(container.locator('.forgot-password-button-error')).toBeVisible();
  await expect(container.locator('.forgot-password-button-error')).toHaveText(
    'Could not reset your password. Please check your email address.',
  );
  await expect(buttons.nth(1)).toBeTrulyEnabled();
});

msTest('Client area forgot password go back', async ({ context }) => {
  const home = (await context.newPage()) as MsPage;
  await setupNewPage(home, { enableStripe: true });

  await MockBms.mockChangePassword(home);
  const button = home.locator('.homepage-header').locator('#trigger-customer-area-button');
  await button.click();
  await home.locator('.saas-login-link__forgotten-password').click();
  const container = home.locator('.saas-forgot-password');
  await expect(container).toBeVisible();
  const buttons = container.locator('.saas-forgot-password-button').locator('.saas-forgot-password-button__item');
  await expect(buttons.nth(0)).toHaveText('Back to login');
  await buttons.nth(0).click();
  await expect(home.locator('.saas-login-container').locator('.saas-login__title')).toHaveText('Log in to your customer account');
});
