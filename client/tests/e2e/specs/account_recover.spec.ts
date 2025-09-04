// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { expect, fillIonInput, msTest } from '@tests/e2e/helpers';
import { testbedGetAccountCreationCode } from '@tests/e2e/helpers/testbed';

msTest('Parsec account recover account', async ({ parsecAccountLoggedIn }) => {
  const page = parsecAccountLoggedIn;
  await page.locator('.profile-header-homepage').click();
  await expect(page.locator('.profile-header-homepage-popover')).toBeVisible();
  const email = (await page.locator('.profile-header-homepage-popover').locator('.header-list-email').textContent()) as string;
  await page.locator('.profile-header-homepage-popover').locator('.logout').click();
  await expect(page).toHaveURL(/.+\/account$/);
  await page.locator('.input-password__forgot-password').click();
  await expect(page).toHaveURL(/.+\/recoverAccount$/);
  const container = page.locator('.recover-account-content');
  const title = container.locator('.recover-account-text__title');
  const subtitle = container.locator('.recover-account-text__subtitle');
  await expect(title).toHaveText('Recover your account');
  await expect(subtitle).toHaveText('Please enter the email address used for your account to proceed.');
  const steps = page.locator('.recover-account-step');
  for (const [index, step] of (await steps.all()).entries()) {
    index === 0 ? await expect(step).toBeVisible() : await expect(step).toBeHidden();
  }
  await expect(steps.nth(0).locator('ion-button')).toHaveText('Validate');
  await expect(steps.nth(0).locator('ion-button')).toBeTrulyDisabled();
  await fillIonInput(steps.nth(0).locator('ion-input').nth(2), email);
  await expect(steps.nth(0).locator('ion-button')).toBeTrulyEnabled();
  await steps.nth(0).locator('ion-button').click();
  for (const [index, step] of (await steps.all()).entries()) {
    index === 1 ? await expect(step).toBeVisible() : await expect(step).toBeHidden();
  }

  await expect(title).toHaveText('Email Verification');
  await expect(subtitle).toHaveText('A verification code has been sent to your email address. Please enter it to update your password.');
  await expect(steps.nth(1).locator('ion-button')).toHaveText('Validate');
  await expect(steps.nth(1).locator('ion-button')).toBeTrulyDisabled();
  const codeInputs = steps.nth(1).locator('ion-input');
  const code = await testbedGetAccountCreationCode(page, email);
  for (let i = 0; i < code.length; i++) {
    await fillIonInput(codeInputs.nth(i), code[i]);
  }
  await expect(steps.nth(1).locator('ion-button')).toBeTrulyEnabled();
  await steps.nth(1).locator('ion-button').click();

  for (const [index, step] of (await steps.all()).entries()) {
    index === 2 ? await expect(step).toBeVisible() : await expect(step).toBeHidden();
  }
  await expect(title).toHaveText('Choose your new password');
  await expect(subtitle).toHaveText('Make sure your new password meets the minimum strength criteria.');
  await expect(steps.nth(2).locator('ion-button')).toHaveText('Update password');
  await expect(steps.nth(2).locator('ion-button')).toBeTrulyDisabled();
  await fillIonInput(steps.nth(2).locator('ion-input').nth(0), 'BigP@ssw0rd.');
  await expect(steps.nth(2).locator('ion-button')).toBeTrulyDisabled();
  await fillIonInput(steps.nth(2).locator('ion-input').nth(1), 'BigP@ssw0rd.');
  await expect(steps.nth(2).locator('ion-button')).toBeTrulyEnabled();
  await steps.nth(2).locator('ion-button').click();

  for (const [index, step] of (await steps.all()).entries()) {
    index === 3 ? await expect(step).toBeVisible() : await expect(step).toBeHidden();
  }
  await expect(title).toHaveText('Your password has been updated');
  await expect(subtitle).toBeHidden();
  await expect(steps.nth(3).locator('ion-button')).toHaveText('Log in');
  await steps.nth(3).locator('ion-button').click();
  await expect(page).toBeHomePage();
  await expect(page.locator('.profile-header-homepage')).toHaveText(/^Agent\d+$/);
});

msTest('Parsec account recover account back to login', async ({ parsecAccountLoggedIn }) => {
  const page = parsecAccountLoggedIn;
  await page.locator('.profile-header-homepage').click();
  await expect(page.locator('.profile-header-homepage-popover')).toBeVisible();
  const email = (await page.locator('.profile-header-homepage-popover').locator('.header-list-email').textContent()) as string;
  await page.locator('.profile-header-homepage-popover').locator('.logout').click();
  await expect(page).toHaveURL(/.+\/account$/);
  await page.locator('.input-password__forgot-password').click();
  await expect(page).toHaveURL(/.+\/recoverAccount$/);
  const container = page.locator('.recover-account-content');
  const title = container.locator('.recover-account-text__title');
  await expect(title).toHaveText('Recover your account');
  const steps = page.locator('.recover-account-step');
  for (const [index, step] of (await steps.all()).entries()) {
    index === 0 ? await expect(step).toBeVisible() : await expect(step).toBeHidden();
  }
  await expect(steps.nth(0).locator('ion-button')).toBeTrulyDisabled();
  await fillIonInput(steps.nth(0).locator('ion-input').nth(2), email);
  await steps.nth(0).locator('ion-button').click();
  for (const [index, step] of (await steps.all()).entries()) {
    index === 1 ? await expect(step).toBeVisible() : await expect(step).toBeHidden();
  }
  await container.locator('.recover-account__back-button').click();
  await expect(page).toHaveURL(/.+\/account$/);
  await page.locator('.input-password__forgot-password').click();
  await expect(page).toHaveURL(/.+\/recoverAccount$/);
  // Check that it resets properly
  await expect(title).toHaveText('Recover your account');
  await expect(steps.nth(0).locator('ion-input').nth(2).locator('input')).toHaveValue('');
});

msTest('Parsec account recover account wrong code', async ({ parsecAccountLoggedIn }) => {
  const page = parsecAccountLoggedIn;
  await page.locator('.profile-header-homepage').click();
  await expect(page.locator('.profile-header-homepage-popover')).toBeVisible();
  const email = (await page.locator('.profile-header-homepage-popover').locator('.header-list-email').textContent()) as string;
  await page.locator('.profile-header-homepage-popover').locator('.logout').click();
  await expect(page).toHaveURL(/.+\/account$/);
  await page.locator('.input-password__forgot-password').click();
  await expect(page).toHaveURL(/.+\/recoverAccount$/);
  const container = page.locator('.recover-account-content');
  const title = container.locator('.recover-account-text__title');
  await expect(title).toHaveText('Recover your account');
  const steps = page.locator('.recover-account-step');
  for (const [index, step] of (await steps.all()).entries()) {
    index === 0 ? await expect(step).toBeVisible() : await expect(step).toBeHidden();
  }
  await expect(steps.nth(0).locator('ion-button')).toHaveText('Validate');
  await expect(steps.nth(0).locator('ion-button')).toBeTrulyDisabled();
  await fillIonInput(steps.nth(0).locator('ion-input').nth(2), email);
  await expect(steps.nth(0).locator('ion-button')).toBeTrulyEnabled();
  await steps.nth(0).locator('ion-button').click();
  for (const [index, step] of (await steps.all()).entries()) {
    index === 1 ? await expect(step).toBeVisible() : await expect(step).toBeHidden();
  }

  await expect(title).toHaveText('Email Verification');
  const codeInputs = steps.nth(1).locator('ion-input');
  for (let i = 0; i < 6; i++) {
    await fillIonInput(codeInputs.nth(i), 'ABCDEF'[i]);
  }
  await expect(steps.nth(1).locator('ion-button')).toBeTrulyEnabled();
  await steps.nth(1).locator('ion-button').click();

  for (const [index, step] of (await steps.all()).entries()) {
    index === 2 ? await expect(step).toBeVisible() : await expect(step).toBeHidden();
  }
  await expect(title).toHaveText('Choose your new password');
  await expect(steps.nth(2).locator('ion-button')).toBeTrulyDisabled();
  await fillIonInput(steps.nth(2).locator('ion-input').nth(0), 'BigP@ssw0rd.');
  await fillIonInput(steps.nth(2).locator('ion-input').nth(1), 'BigP@ssw0rd.');
  await expect(steps.nth(2).locator('ion-button')).toBeTrulyEnabled();
  await steps.nth(2).locator('ion-button').click();

  for (const [index, step] of (await steps.all()).entries()) {
    index === 1 ? await expect(step).toBeVisible() : await expect(step).toBeHidden();
  }
  await expect(title).toHaveText('Email Verification');
  await expect(container.locator('.recover-account-text__error')).toBeVisible();
  await expect(container.locator('.recover-account-text__error')).toHaveText(
    'The verification code is invalid, make sure you entered the correct code.',
  );
});
