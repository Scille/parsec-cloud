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
  const container = parsecAccountLoggedIn.locator('.manage-account-page-container ');
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
