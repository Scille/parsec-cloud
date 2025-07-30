// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { answerQuestion, expect, fillIonInput, msTest } from '@tests/e2e/helpers';
import { testbedGetAccountCreationCode } from '@tests/e2e/helpers/testbed';

msTest('Manage account state', async ({ parsecAccountLoggedIn }) => {
  await parsecAccountLoggedIn.locator('.profile-header-homepage').click();
  const popover = parsecAccountLoggedIn.locator('.profile-header-homepage-popover');
  await expect(popover).toBeVisible();
  await popover.locator('.main-list__item').nth(1).click();
  const container = parsecAccountLoggedIn.locator('.manage-account-page-container ');
  await expect(container).toBeVisible();
  const inputs = container.locator('ion-input');
  await expect(inputs.nth(0).locator('input')).toHaveValue(/^Agent\d+$/);
  await expect(inputs.nth(1).locator('input')).toHaveValue(/^agent\d+@example\.com+$/);
});

msTest('Delete account', async ({ parsecAccountLoggedIn }) => {
  await parsecAccountLoggedIn.locator('.profile-header-homepage').click();
  const popover = parsecAccountLoggedIn.locator('.profile-header-homepage-popover');
  await expect(popover).toBeVisible();
  await popover.locator('.main-list__item').nth(1).click();
  const container = parsecAccountLoggedIn.locator('.manage-account-page-container');
  await expect(container).toBeVisible();
  const email = await container.locator('ion-input').nth(1).locator('input').inputValue();
  await container.locator('.delete-account__button').click();
  const modal = parsecAccountLoggedIn.locator('.code-validation-modal');
  await expect(modal).toBeHidden();
  await answerQuestion(parsecAccountLoggedIn, true, {
    expectedNegativeText: 'Cancel',
    expectedPositiveText: 'Send code',
    expectedQuestionText: 'In order to delete your Parsec account, you must confirm your action with a code sent by email.',
    expectedTitleText: 'Send a validation code by email',
  });
  await expect(modal).toBeVisible();
  await expect(modal.locator('.ms-modal-header__title')).toHaveText('Enter the verification code to delete your account');
  const confirmButton = modal.locator('#next-button');
  await expect(confirmButton).toBeTrulyDisabled();

  const codeInputs = modal.locator('ion-input');

  for (const [index, char] of ['A', 'B', 'C', 'D', 'E', 'F'].entries()) {
    await fillIonInput(codeInputs.nth(index), char);
    if (index < 5) {
      await expect(confirmButton).toBeTrulyDisabled();
    }
  }
  await expect(confirmButton).toBeTrulyEnabled();

  const code = await testbedGetAccountCreationCode(parsecAccountLoggedIn, email);
  // Try with a wrong code first
  for (let i = 0; i < code.length; i++) {
    await fillIonInput(codeInputs.nth(i), code[i]);
  }
  await expect(confirmButton).toBeTrulyEnabled();
  await confirmButton.click();
  await expect(modal).toBeHidden();
  await expect(parsecAccountLoggedIn).toShowToast('Your account has been successfully deleted.', 'Success');
  await expect(parsecAccountLoggedIn).toHaveURL(/.+\/account$/);
});

msTest('Update password', async ({ parsecAccountLoggedIn }) => {
  await parsecAccountLoggedIn.locator('.profile-header-homepage').click();
  const popover = parsecAccountLoggedIn.locator('.profile-header-homepage-popover');
  await expect(popover).toBeVisible();
  await popover.locator('.main-list__item').nth(2).click();
  const container = parsecAccountLoggedIn.locator('.account-authentication-page-container');
  await expect(container).toBeVisible();
  const authItems = container.locator('.authentication-method-item');
  await expect(authItems).toHaveCount(2);
  await expect(authItems.nth(0).locator('.authentication-method-item-details__title')).toHaveText(/^Other$/);
  await expect(authItems.nth(1).locator('.authentication-method-item-details__title')).toHaveText(/^Password$/);
  await expect(authItems.nth(0).locator('.authentication-method-item-details__date ')).toHaveText(/^Created on:[a-zA-Z]{3} \d{2}, \d{4}$/);
  await expect(authItems.nth(1).locator('.authentication-method-item-details__date ')).toHaveText(/^Created on:[a-zA-Z]{3} \d{2}, \d{4}$/);
  const changeButton = container.locator('ion-button');
  await expect(changeButton).toBeVisible();
  await expect(changeButton).toHaveText('Change password');
  const passwordModal = parsecAccountLoggedIn.locator('.account-update-authentication');
  await expect(passwordModal).toBeHidden();
  await changeButton.click();
  await expect(passwordModal).toBeVisible();
  const confirmButton = passwordModal.locator('#next-button');
  await expect(confirmButton).toHaveText('Update password');
  await expect(confirmButton).toBeTrulyDisabled();
  const passwordInputs = passwordModal.locator('ion-input');
  await fillIonInput(passwordInputs.nth(0), 'BigP@ssw0rd.');
  await expect(confirmButton).toBeTrulyDisabled();
  await fillIonInput(passwordInputs.nth(1), 'BigP@ssw0rd.');
  await expect(confirmButton).toBeTrulyEnabled();
  await confirmButton.click();
  await expect(parsecAccountLoggedIn).toShowToast('Your authentication method has been successfully updated.', 'Success');
  await expect(passwordModal).toBeHidden();
});
