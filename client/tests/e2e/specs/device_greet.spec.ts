// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { answerQuestion, expect, msTest } from '@tests/e2e/helpers';

msTest('Open the greet device modal', async ({ deviceGreetModal }) => {
  await expect(deviceGreetModal.locator('.modal-header__title')).toHaveText('Create a new device');
  await expect(deviceGreetModal.locator('#next-button')).toHaveText('Start');
  await expect(deviceGreetModal.locator('.closeBtn')).toBeVisible();
});

msTest('Go through the greet process', async ({ deviceGreetModal }) => {
  const title = deviceGreetModal.locator('.modal-header__title');
  const subtitle = deviceGreetModal.locator('.modal-header__text');
  const nextButton = deviceGreetModal.locator('#next-button');
  const modalContent = deviceGreetModal.locator('.modal-content');

  await nextButton.click();

  await expect(deviceGreetModal).toHaveWizardStepper(['Host code', 'Guest code'], 0);

  await expect(title).toHaveText('Share your code');
  await expect(subtitle).toHaveText('Click on the code that appears on the guest device.');
  await expect(nextButton).toBeHidden();
  await deviceGreetModal.page().waitForTimeout(200);

  await expect(deviceGreetModal).toHaveWizardStepper(['Host code', 'Guest code'], 1);
  await expect(title).toHaveText('Get guest code');
  await expect(subtitle).toHaveText('Click on the code that appears on the guest device.');
  const choices = modalContent.locator('.code:visible');
  await expect(choices).toHaveText(['1ABC', '2DEF', '3GHI', '4JKL']);
  await expect(nextButton).toBeHidden();
  await choices.nth(1).click();

  await expect(title).toHaveText('Waiting for device information');
  await expect(deviceGreetModal).toHaveWizardStepper(['Host code', 'Guest code'], 1);
  await expect(nextButton).toBeHidden();
  await deviceGreetModal.page().waitForTimeout(200);

  await expect(title).toHaveText('New device added');
  await expect(nextButton).toNotHaveDisabledAttribute();
  await expect(nextButton).toBeVisible();
  await expect(nextButton).toHaveText('Finish');
  await nextButton.click();

  await expect(deviceGreetModal.page().locator('.greet-organization-modal')).toBeHidden();
  await expect(deviceGreetModal.page()).toShowToast('You can connect to this organization from your new device.', 'Success');
});

msTest('Device greet select invalid SAS code', async ({ deviceGreetModal }) => {
  const title = deviceGreetModal.locator('.modal-header__title');
  const nextButton = deviceGreetModal.locator('#next-button');
  await nextButton.click();

  await expect(title).toHaveText('Get guest code');
  const choices = deviceGreetModal.locator('.modal-content').locator('.code:visible');
  await choices.nth(0).click();

  await expect(deviceGreetModal.page()).toShowToast('You did not select the correct code. Please restart the onboarding process.', 'Error');
  await expect(title).toHaveText('Create a new device');
});

msTest('Device greet select no SAS code', async ({ deviceGreetModal }) => {
  const title = deviceGreetModal.locator('.modal-header__title');
  const nextButton = deviceGreetModal.locator('#next-button');
  await nextButton.click();

  await expect(title).toHaveText('Get guest code');
  await deviceGreetModal.locator('.modal-content').locator('.button-none').click();

  await expect(deviceGreetModal.page()).toShowToast(
    'If you did not see the correct code, this could be a sign of a security issue during the onboarding. Please restart the process.',
    'Error',
  );
  await expect(title).toHaveText('Create a new device');
});

for (const cancel of [true, false]) {
  msTest(`Try closing device greet process${cancel ? ' and cancel' : ''}`, async ({ deviceGreetModal }) => {
    const title = deviceGreetModal.locator('.modal-header__title');
    const nextButton = deviceGreetModal.locator('#next-button');
    const closeButton = deviceGreetModal.locator('.closeBtn');

    await nextButton.click();
    await expect(title).toHaveText('Get guest code');

    await closeButton.click();
    await answerQuestion(deviceGreetModal.page(), !cancel, {
      expectedTitleText: 'Cancel',
      expectedQuestionText: 'Are you sure you want to cancel the onboarding process?',
      expectedPositiveText: 'Cancel process',
      expectedNegativeText: 'Resume',
    });
    if (cancel) {
      await expect(deviceGreetModal.page().locator('.greet-organization-modal')).toBeVisible();
    } else {
      await expect(deviceGreetModal.page().locator('.greet-organization-modal')).toBeHidden();
    }
  });
}
