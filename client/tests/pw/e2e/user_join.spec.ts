// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { Locator } from '@playwright/test';
import { expect } from '@tests/pw/helpers/assertions';
import { msTest } from '@tests/pw/helpers/fixtures';
import { fillInputModal, fillIonInput } from '@tests/pw/helpers/utils';

// cspell:disable-next-line
const INVITATION_LINK = 'parsec3://parsec.cloud/Test?a=claim_user&p=xBBHJlEjlpxNZYTCvBWWDPIS';

const userJoinTest = msTest.extend<{ userJoinModal: Locator }>({
  userJoinModal: async ({ home }, use) => {
    await home.locator('#create-organization-button').click();
    await expect(home.locator('.homepage-popover')).toBeVisible();
    await home.locator('.homepage-popover').getByRole('listitem').nth(1).click();
    await fillInputModal(home, INVITATION_LINK);
    const modal = home.locator('.join-organization-modal');
    await expect(home.locator('.join-organization-modal')).toBeVisible();
    await use(modal);
  },
});

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

userJoinTest('Go through the join user process', async ({ home, userJoinModal }) => {
  const nextButton = userJoinModal.locator('#next-button');
  const title = userJoinModal.locator('.modal-header__title');
  await expect(title).toHaveText('Welcome to Parsec!');
  await expect(nextButton).toHaveText('I understand!');
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
  await expect(nextButton).not.toHaveDisabledAttribute();
  await nextButton.click();
  await expect(title).toHaveText('Authentication');
  await expect(userJoinModal).toHaveWizardStepper(['Host code', 'Guest code', 'Contact details', 'Authentication'], 3);
  await expect(nextButton).toHaveText('Join the organization');
  await expect(nextButton).toHaveDisabledAttribute();
  const passwordChoice = userJoinModal.locator('#get-password').locator('.choose-password');
  await fillIonInput(passwordChoice.locator('ion-input').nth(0), 'AVeryL0ngP@ssw0rd');
  await expect(nextButton).toHaveDisabledAttribute();
  await fillIonInput(passwordChoice.locator('ion-input').nth(1), 'AVeryL0ngP@ssw0rd');
  await expect(nextButton).not.toHaveDisabledAttribute();
  await nextButton.click();
  await expect(title).toHaveText('You have joined the organization Test!');
  await expect(nextButton).not.toHaveDisabledAttribute();
  await nextButton.click();
  await expect(userJoinModal).toBeHidden();
  await expect(home).toShowToast('You successfully joined the organization.', 'Success');
});
