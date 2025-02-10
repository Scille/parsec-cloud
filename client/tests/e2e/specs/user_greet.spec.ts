// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { answerQuestion, expect, msTest } from '@tests/e2e/helpers';

msTest('Open the greet user modal', async ({ userGreetModal }) => {
  await expect(userGreetModal.locator('.modal-header__title')).toHaveText('Onboard a new user');
  await expect(userGreetModal.locator('#next-button')).toHaveText('Start');
  await expect(userGreetModal.locator('.closeBtn')).toBeVisible();
});

msTest('Go through the greet process', async ({ userGreetModal }) => {
  const title = userGreetModal.locator('.modal-header__title');
  const subtitle = userGreetModal.locator('.modal-header__text');
  const nextButton = userGreetModal.locator('#next-button');
  const modalContent = userGreetModal.locator('.modal-content');

  await nextButton.click();

  await expect(userGreetModal).toHaveWizardStepper(['Host code', 'Guest code', 'Contact details'], 0);

  await expect(title).toHaveText('Share your code');
  await expect(subtitle).toHaveText('Give the code below to the guest.');
  await expect(modalContent.locator('.code:visible')).toHaveText('2EDF');
  await expect(nextButton).toBeHidden();
  await userGreetModal.page().waitForTimeout(200);

  await expect(userGreetModal).toHaveWizardStepper(['Host code', 'Guest code', 'Contact details'], 1);
  await expect(title).toHaveText('Get guest code');
  await expect(subtitle).toHaveText('Click on the code given to you by the guest.');
  const choices = modalContent.locator('.code:visible');
  await expect(choices).toHaveText(['1ABC', '2DEF', '3GHI', '4JKL']);
  await expect(nextButton).toBeHidden();
  await choices.nth(1).click();

  await expect(title).toHaveText('Contact details');
  await expect(userGreetModal).toHaveWizardStepper(['Host code', 'Guest code', 'Contact details'], 2);
  await expect(nextButton).toBeHidden();
  await userGreetModal.page().waitForTimeout(200);

  await expect(title).toHaveText('Contact details');
  await expect(subtitle).toHaveText('You can update the user name, device and profile.');
  await expect(userGreetModal).toHaveWizardStepper(['Host code', 'Guest code', 'Contact details'], 2);
  await expect(nextButton).toBeVisible();
  await expect(nextButton).toHaveDisabledAttribute();

  await expect(modalContent.locator('.user-info-page').locator('ion-input').nth(0).locator('input')).toHaveValue('Gordon Freeman');
  await expect(modalContent.locator('.user-info-page').locator('ion-input').nth(1)).toHaveTheClass('input-disabled');
  await expect(modalContent.locator('.user-info-page').locator('ion-input').nth(1).locator('input')).toHaveValue(
    'gordon.freeman@blackmesa.nm',
  );
  const profileButton = modalContent.locator('.user-info-page').locator('.filter-button');
  await expect(profileButton).toHaveText('Choose a profile');
  await profileButton.click();
  const profileDropdown = userGreetModal.page().locator('.dropdown-popover');
  await expect(profileDropdown.getByRole('listitem').locator('.option-text__label')).toHaveText(['Administrator', 'Member', 'External']);
  await profileDropdown.getByRole('listitem').nth(1).click();
  await expect(profileButton).toHaveText('Member');
  await expect(nextButton).toNotHaveDisabledAttribute();
  await expect(nextButton).toHaveText('Approve');
  await nextButton.click();

  await expect(title).toHaveText('User has been added successfully!');
  await expect(nextButton).toNotHaveDisabledAttribute();
  await expect(nextButton).toBeVisible();
  await expect(modalContent.locator('.final-step').locator('.person-name')).toHaveText('Gordon Freeman');
  await expect(modalContent.locator('.final-step').locator('.user-info__email').locator('.cell')).toHaveText('gordon.freeman@blackmesa.nm');
  await expect(modalContent.locator('.final-step').locator('.user-info__role').locator('.label-profile')).toHaveText('Member');
  await nextButton.click();

  await expect(userGreetModal.page().locator('.greet-organization-modal')).toBeHidden();
  await expect(userGreetModal.page()).toShowToast('Gordon Freeman can now access to the organization.', 'Success');
});

msTest('User greet select invalid SAS code', async ({ userGreetModal }) => {
  const title = userGreetModal.locator('.modal-header__title');
  const nextButton = userGreetModal.locator('#next-button');
  await nextButton.click();

  await expect(title).toHaveText('Get guest code');
  const choices = userGreetModal.locator('.modal-content').locator('.code:visible');
  await choices.nth(0).click();

  await expect(userGreetModal.page()).toShowToast('You did not select the correct code. Please restart the onboarding process.', 'Error');
  await expect(title).toHaveText('Onboard a new user');
});

msTest('User greet select no SAS code', async ({ userGreetModal }) => {
  const title = userGreetModal.locator('.modal-header__title');
  const nextButton = userGreetModal.locator('#next-button');
  await nextButton.click();

  await expect(title).toHaveText('Get guest code');
  await userGreetModal.locator('.modal-content').locator('.button-none').click();

  await expect(userGreetModal.page()).toShowToast(
    'If you did not see the correct code, this could be a sign of a security issue during the onboarding. Please restart the process.',
    'Error',
  );
  await expect(title).toHaveText('Onboard a new user');
});

msTest('Close user greet process', async ({ userGreetModal }) => {
  const title = userGreetModal.locator('.modal-header__title');
  const nextButton = userGreetModal.locator('#next-button');
  const closeButton = userGreetModal.locator('.closeBtn');
  const modalContent = userGreetModal.locator('.modal-content');

  await nextButton.click();
  await expect(title).toHaveText('Get guest code');

  await closeButton.click();
  await answerQuestion(userGreetModal.page(), false, {
    expectedTitleText: 'Cancel the onboarding',
    expectedQuestionText:
      'Are you sure you want to cancel the onboarding process? Information will not be saved, you will have to restart.',
    expectedPositiveText: 'Cancel process',
    expectedNegativeText: 'Resume',
  });

  await modalContent.locator('.code:visible').nth(1).click();

  await closeButton.click();
  await answerQuestion(userGreetModal.page(), false, {
    expectedTitleText: 'Cancel the onboarding',
    expectedQuestionText:
      'Are you sure you want to cancel the onboarding process? Information will not be saved, you will have to restart.',
    expectedPositiveText: 'Cancel process',
    expectedNegativeText: 'Resume',
  });

  await expect(title).toHaveText('Contact details');
  await closeButton.click();
  await answerQuestion(userGreetModal.page(), false, {
    expectedTitleText: 'Cancel the onboarding',
    expectedQuestionText:
      'Are you sure you want to cancel the onboarding process? Information will not be saved, you will have to restart.',
    expectedPositiveText: 'Cancel process',
    expectedNegativeText: 'Resume',
  });

  await modalContent.locator('.user-info-page').locator('.filter-button').click();
  const profileDropdown = userGreetModal.page().locator('.dropdown-popover');
  await profileDropdown.getByRole('listitem').nth(1).click();
  await nextButton.click();

  await expect(closeButton).toBeHidden();
});
