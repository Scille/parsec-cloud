// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { answerQuestion, expect, fillIonInput, msTest } from '@tests/e2e/helpers';

msTest('Opens the user join organization modal', async ({ home }) => {
  await expect(home.locator('#create-organization-button')).toHaveText('Create or join');
  await expect(home.locator('.homepage-popover')).toBeHidden();
  await home.locator('#create-organization-button').click();
  await expect(home.locator('.homepage-popover')).toBeVisible();
  await expect(home.locator('.join-organization-modal')).toBeHidden();
  const joinButton = home.locator('.homepage-popover').getByRole('listitem').nth(1);
  await expect(joinButton.locator('ion-label')).toHaveText('Join');
  await expect(joinButton.locator('ion-text')).toHaveText('I received an invitation to join an organization');
  await joinButton.click();
  const modal = home.locator('.text-input-modal');
  await expect(modal).toBeVisible();
  await expect(modal.locator('#next-button')).toHaveText('Join');
});

msTest('Go through the join user process', async ({ home, userJoinModal }) => {
  const nextButton = userJoinModal.locator('#next-button');
  const title = userJoinModal.locator('.modal-header__title');
  await expect(title).toHaveText('Welcome to Parsec!');
  await expect(nextButton).toHaveText('Continue with ');
  await nextButton.click();
  await expect(title).toHaveText('Get host code');
  await expect(userJoinModal).toHaveWizardStepper(['Host code', 'Guest code', 'Contact details', 'Authentication'], 0);
  const sasCodeButtons = userJoinModal.locator('.button-choice');
  await expect(sasCodeButtons).toHaveCount(4);
  await sasCodeButtons.nth(1).click();
  await expect(title).toHaveText('Your contact details');
  await expect(userJoinModal).toHaveWizardStepper(['Host code', 'Guest code', 'Contact details', 'Authentication'], 2);
  await expect(nextButton).toHaveDisabledAttribute();
  // cspell:disable-next-line
  await fillIonInput(userJoinModal.locator('#get-user-info').locator('ion-input').nth(0), 'Shadowheart');
  await expect(userJoinModal.locator('#get-user-info').locator('ion-input').nth(1).locator('input')).toHaveValue(
    'shadowheart@swordcoast.faerun',
  );
  await expect(nextButton).toNotHaveDisabledAttribute();
  await nextButton.click();
  await expect(title).toHaveText('Authentication');
  await expect(userJoinModal).toHaveWizardStepper(['Host code', 'Guest code', 'Contact details', 'Authentication'], 3);
  await expect(nextButton).toHaveText('Join the organization');
  await expect(nextButton).toHaveDisabledAttribute();
  const authRadio = userJoinModal.locator('.choose-auth-page').locator('.radio-list-item');
  await expect(authRadio).toHaveCount(2);
  await expect(authRadio.nth(0)).toHaveTheClass('radio-disabled');
  await expect(authRadio.nth(0).locator('.item-radio__label')).toHaveText('Use System Authentication');
  await expect(authRadio.nth(0).locator('.item-radio__text:visible')).toHaveText('Unavailable on web');
  await expect(authRadio.nth(1)).toHaveText('Use Password');
  const passwordChoice = userJoinModal.locator('#get-password').locator('.choose-password');
  await fillIonInput(passwordChoice.locator('ion-input').nth(0), 'AVeryL0ngP@ssw0rd');
  await expect(nextButton).toHaveDisabledAttribute();
  await fillIonInput(passwordChoice.locator('ion-input').nth(1), 'AVeryL0ngP@ssw0rd');
  await nextButton.scrollIntoViewIfNeeded();
  await expect(nextButton).toNotHaveDisabledAttribute();
  await nextButton.click();
  await expect(title).toHaveText('You have joined the organization Test!');
  await expect(nextButton).toNotHaveDisabledAttribute();
  await nextButton.click();
  await expect(userJoinModal).toBeHidden();
  await expect(home).toShowToast('You successfully joined the organization.', 'Success');
});

msTest('User join select invalid SAS code', async ({ userJoinModal }) => {
  const nextButton = userJoinModal.locator('#next-button');
  const title = userJoinModal.locator('.modal-header__title');
  await expect(title).toHaveText('Welcome to Parsec!');
  await expect(nextButton).toHaveText('Continue with ');
  await nextButton.click();
  await expect(title).toHaveText('Get host code');
  await expect(userJoinModal).toHaveWizardStepper(['Host code', 'Guest code', 'Contact details', 'Authentication'], 0);
  const sasCodeButtons = userJoinModal.locator('.button-choice');
  await sasCodeButtons.nth(0).click();
  await expect(userJoinModal.page()).toShowToast('You did not select the correct code. Please restart the onboarding process.', 'Error');
  await expect(title).toHaveText('Welcome to Parsec!');
  await expect(nextButton).toHaveText('Continue with ');
});

msTest('User join select no SAS code', async ({ userJoinModal }) => {
  const nextButton = userJoinModal.locator('#next-button');
  const title = userJoinModal.locator('.modal-header__title');
  await expect(title).toHaveText('Welcome to Parsec!');
  await expect(nextButton).toHaveText('Continue with ');
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
  await expect(nextButton).toHaveText('Continue with ');
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
