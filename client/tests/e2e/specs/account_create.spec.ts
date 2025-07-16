// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { Locator } from '@playwright/test';
import { DEFAULT_USER_INFORMATION, expect, fillIonInput, generateUniqueEmail, msTest } from '@tests/e2e/helpers';
import { testbedGetAccountCreationCode } from '@tests/e2e/helpers/testbed';

msTest('Parsec account create account', async ({ parsecAccount }) => {
  async function checkVisible(containers: Array<Locator>, visibleIndex: number): Promise<void> {
    for (const [index, ctn] of containers.entries()) {
      if (index === visibleIndex) {
        await expect(ctn).toBeVisible();
      } else {
        await expect(ctn).toBeHidden();
      }
    }
  }

  const EMAIL = generateUniqueEmail();

  const container = parsecAccount.locator('.homepage-content');
  await expect(container.locator('.account-login__title')).toHaveText('My Parsec account');
  await expect(container.locator('.account-create__button')).toHaveText('Create an account');
  await container.locator('.account-create__button').click();
  await expect(parsecAccount).toHaveURL(/.+\/createAccount$/);

  const userInfoContainer = parsecAccount.locator('.user-information-step');
  const passwordContainer = parsecAccount.locator('.authentication-step');
  const validationContainer = parsecAccount.locator('.validation-email-step');
  const creatingContainer = parsecAccount.locator('.creating-step');
  const createdContainer = parsecAccount.locator('.created-step');
  const title = parsecAccount.locator('.create-account-page').locator('.create-account-page-header__title');

  const containers = [userInfoContainer, validationContainer, passwordContainer, creatingContainer, createdContainer];
  await checkVisible(containers, 0);
  await expect(title).toHaveText('Create an account');
  const inputContainers = userInfoContainer.locator('div.account-login-content__input');
  await expect(inputContainers.locator('.form-label')).toHaveText(['Server address', 'First name', 'Last name', 'Email address']);
  const userInfoNext = userInfoContainer.locator('.account-login-content-button__item');
  await expect(userInfoNext).toHaveText('Next');
  await expect(userInfoNext).toBeTrulyDisabled();
  await fillIonInput(inputContainers.nth(0).locator('ion-input'), process.env.TESTBED_SERVER as string);
  await expect(userInfoNext).toBeTrulyDisabled();
  await fillIonInput(inputContainers.nth(1).locator('ion-input'), DEFAULT_USER_INFORMATION.firstName);
  await expect(userInfoNext).toBeTrulyDisabled();
  await fillIonInput(inputContainers.nth(2).locator('ion-input'), DEFAULT_USER_INFORMATION.lastName);
  await expect(userInfoNext).toBeTrulyDisabled();
  await fillIonInput(inputContainers.nth(3).locator('ion-input'), EMAIL);
  await expect(userInfoNext).toBeTrulyEnabled();
  await userInfoNext.click();
  await checkVisible(containers, 1);
  await expect(title).toHaveText('Validate your email');

  const validationNext = validationContainer.locator('ion-button').nth(0);
  await expect(validationNext).toHaveText('Next');
  await expect(validationNext).toBeTrulyDisabled();
  const codeInputs = validationContainer.locator('ion-input');

  // Try with a wrong code first
  for (const [index, char] of ['A', 'B', 'C', 'D', 'E', 'F'].entries()) {
    await fillIonInput(codeInputs.nth(index), char);
    if (index < 5) {
      await expect(validationNext).toBeTrulyDisabled();
    }
  }
  await expect(validationNext).toBeTrulyEnabled();
  await validationNext.click();

  await checkVisible(containers, 1);
  await expect(validationContainer.locator('.form-error')).toBeVisible();
  await expect(validationContainer.locator('.form-error')).toHaveText('The code is invalid.');

  const code = await testbedGetAccountCreationCode(parsecAccount, EMAIL);
  // Try with a wrong code first
  for (let i = 0; i < code.length; i++) {
    await fillIonInput(codeInputs.nth(i), code[i]);
  }
  await expect(validationNext).toBeTrulyEnabled();
  await validationNext.click();

  await checkVisible(containers, 2);
  await expect(title).toHaveText('Choose your authentication method');
  const passwordNext = passwordContainer.locator('ion-button');
  await expect(passwordNext).toBeTrulyDisabled();
  await fillIonInput(passwordContainer.locator('ion-input').nth(0), 'BigP@ssw0rd.');
  await expect(passwordNext).toBeTrulyDisabled();
  await fillIonInput(passwordContainer.locator('ion-input').nth(1), 'BigP@ssw0rd.');
  await expect(passwordNext).toBeTrulyEnabled();
  await passwordNext.click();

  await checkVisible(containers, 3);
  await expect(title).toHaveText('Creating your account');
  await expect(parsecAccount.locator('.create-account-page-header__title')).toHaveText('Creating your account');
  await parsecAccount.waitForTimeout(2000);
  await checkVisible(containers, 4);
  await expect(createdContainer.locator('.created-step-welcome__name')).toHaveText('Gordon Freeman');
  const createdNext = createdContainer.locator('.created-step-welcome-login__button');
  await expect(createdNext).toHaveText('Access organizations');
  await createdNext.click();
  await expect(parsecAccount).toBeHomePage();
});
