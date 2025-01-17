// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { answerQuestion, expect, fillIonInput, msTest } from '@tests/e2e/helpers';

msTest('User join select invalid SAS code', async ({ userJoinModal }) => {
  const nextButton = userJoinModal.locator('#next-button');
  const title = userJoinModal.locator('.modal-header__title');
  await expect(title).toHaveText('Welcome to Parsec!');
  await expect(nextButton).toHaveText('I understand!');
  await nextButton.click();
  await expect(title).toHaveText('Get host code');
  await expect(userJoinModal).toHaveWizardStepper(['Host code', 'Guest code', 'Contact details', 'Authentication'], 0);
  const sasCodeButtons = userJoinModal.locator('.button-choice');
  await sasCodeButtons.nth(0).click();
  await expect(userJoinModal.page()).toShowToast('You did not select the correct code. Please restart the onboarding process.', 'Error');
  await expect(title).toHaveText('Welcome to Parsec!');
  await expect(nextButton).toHaveText('I understand!');
});

msTest('User join select no SAS code', async ({ userJoinModal }) => {
  const nextButton = userJoinModal.locator('#next-button');
  const title = userJoinModal.locator('.modal-header__title');
  await expect(title).toHaveText('Welcome to Parsec!');
  await expect(nextButton).toHaveText('I understand!');
  await nextButton.click();
  await expect(title).toHaveText('Get host code');
  await expect(userJoinModal).toHaveWizardStepper(['Host code', 'Guest code', 'Contact details', 'Authentication'], 0);
  const sasCodeNone = userJoinModal.locator('.button-none');
  await sasCodeNone.click();
  await expect(userJoinModal.page()).toShowToast(
    'If you did not see the correct code, this could be a sign of a security issue during the onboarding. Please restart the process.',
    'Error',
  );
  await expect(title).toHaveText('Welcome to Parsec!');
  await expect(nextButton).toHaveText('I understand!');
});

msTest('Close user join process', async ({ userJoinModal }) => {
  const closeButton = userJoinModal.locator('.closeBtn');
  await expect(closeButton).toBeVisible();

  await closeButton.click();
  await answerQuestion(userJoinModal.page(), false, {
    expectedTitleText: 'Cancel the onboarding',
    expectedQuestionText:
      'Are you sure you want to cancel the onboarding process? Information will not be saved, you will have to restart.',
    expectedPositiveText: 'Cancel process',
    expectedNegativeText: 'Resume',
  });

  const nextButton = userJoinModal.locator('#next-button');
  await nextButton.click();

  await closeButton.click();
  await answerQuestion(userJoinModal.page(), false);

  const sasCodeButtons = userJoinModal.locator('.button-choice');
  await sasCodeButtons.nth(1).click();

  // cspell:disable-next-line
  await fillIonInput(userJoinModal.locator('#get-user-info').locator('ion-input').nth(0), 'Shadowheart');
  await nextButton.click();

  await closeButton.click();
  await answerQuestion(userJoinModal.page(), false);

  const passwordChoice = userJoinModal.locator('#get-password').locator('.choose-password');
  await fillIonInput(passwordChoice.locator('ion-input').nth(0), 'AVeryL0ngP@ssw0rd');
  await fillIonInput(passwordChoice.locator('ion-input').nth(1), 'AVeryL0ngP@ssw0rd');
  await nextButton.click();

  await expect(closeButton).toBeHidden();
});
