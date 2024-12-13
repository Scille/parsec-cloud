// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { Locator } from '@playwright/test';
import { expect, fillIonInput, msTest } from '@tests/e2e/helpers';

// cspell:disable-next-line
const INVITATION_LINK = 'parsec3://parsec.cloud/Test?a=claim_device&p=xBBHJlEjlpxNZYTCvBWWDPIS';
const WAIT_TIME = 1000;

msTest('Claim device process', async ({ home }) => {
  async function checkStepper(steps: Locator, activeIndex: number): Promise<void> {
    for (let i = 0; i < 2; i++) {
      if (i < activeIndex) {
        expect(steps.nth(i).locator('.circle').locator('.inner-circle-done')).toBeDefined();
      } else if (i === activeIndex) {
        expect(steps.nth(i).locator('.circle').locator('.inner-circle-active')).toBeDefined();
      } else {
        expect(steps.nth(i).locator('.circle').locator('.inner-circle-active').locator('div')).toHaveCount(0);
      }
    }
  }

  const modal = home.locator('.join-organization-modal');
  await expect(modal).toBeHidden();

  await home.locator('#create-organization-button').click();
  await home.locator('.homepage-popover').locator('ion-item').nth(1).click();
  await fillIonInput(home.locator('.text-input-modal').locator('ion-input'), INVITATION_LINK);
  await home.locator('.text-input-modal').locator('#next-button').click();

  await expect(modal).toBeVisible();

  const title = modal.locator('.modal-header__title');
  const subtitle = modal.locator('.modal-header__text');
  const nextButton = modal.locator('#next-button');

  await expect(title).toHaveText('Add a new device');
  await expect(nextButton).toHaveText('I understand!');

  await nextButton.click();

  const steps = modal.locator('.ms-wizard-stepper__step');
  await expect(title).toHaveText('Get host code');
  await expect(subtitle).toHaveText('Click on the code you see on the main device.');
  await expect(steps).toHaveText(['Host code', 'Guest code', 'Authentication']);
  await checkStepper(steps, 0);
  await expect(nextButton).toBeHidden();

  const choices = modal.locator('.button-choice');
  await expect(choices).toHaveText(['5MNO', '6PQR', '7STU', '8VWX']);
  await choices.nth(2).click();

  await checkStepper(steps, 1);

  await home.waitForTimeout(WAIT_TIME);
  await checkStepper(steps, 2);

  await expect(title).toHaveText('Authentication');
  await expect(subtitle).toHaveText('Lastly, choose an authentication method for your new device.');
  await expect(nextButton).toBeVisible();
  await expect(nextButton).toBeTrulyDisabled();
  await expect(nextButton).toHaveText('Confirm');

  const authChoices = modal.locator('.choose-auth-page').locator('ion-radio');

  await expect(authChoices.nth(0).locator('.item-radio__label')).toHaveText('Use System Authentication');
  await expect(authChoices.nth(0).locator('.item-radio__text:visible')).toHaveText('Unavailable on web');
  await expect(authChoices.nth(0)).toHaveTheClass('radio-disabled');

  const inputs = modal.locator('.choose-auth-page').locator('.choose-password').locator('ion-input');
  await fillIonInput(inputs.nth(0), 'Password23;-$aze');
  await expect(nextButton).toBeTrulyDisabled();
  await fillIonInput(inputs.nth(1), 'Password23;-$aze');
  await nextButton.scrollIntoViewIfNeeded();
  await expect(nextButton).toBeTrulyEnabled();
  await nextButton.click();

  await expect(title).toHaveText('Device has been added!');
  await expect(nextButton).toHaveText('Log in');
  await nextButton.click();

  await expect(modal).toBeHidden();
});
