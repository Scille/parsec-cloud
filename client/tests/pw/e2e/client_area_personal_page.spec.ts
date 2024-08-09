// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { Page } from '@playwright/test';
import { expect } from '@tests/pw/helpers/assertions';
import { MockBms } from '@tests/pw/helpers/bms';
import { DEFAULT_USER_INFORMATION } from '@tests/pw/helpers/data';
import { msTest } from '@tests/pw/helpers/fixtures';
import { fillIonInput } from '@tests/pw/helpers/utils';

async function goToPersonalPage(page: Page): Promise<void> {
  await page.locator('.header-content').locator('.header-right-profile').click();
  await expect(page.locator('.header-content').locator('.header-title')).toHaveText('My profile');
}

msTest('Check personal data page', async ({ clientArea }) => {
  const title = clientArea.locator('.header-content').locator('.header-title');
  await expect(title).toHaveText('Dashboard');
  const avatar = clientArea.locator('.header-content').locator('.header-right-profile');
  await expect(avatar.locator('.person-name')).toHaveText(DEFAULT_USER_INFORMATION.name);
  await avatar.click();
  await expect(title).toHaveText('My profile');
  const container = clientArea.locator('.personal-data-container');
  const items = container.locator('.ms-summary-card-item');
  await expect(items.locator('.ms-summary-card-item__label')).toHaveText([
    'Firstname',
    'Lastname',
    'Phone',
    'I represent a company',
    'Company',
    'Job',
    'Email',
    'Password',
  ]);
  await expect(items.locator('.ms-summary-card-item__text')).toHaveText([
    DEFAULT_USER_INFORMATION.firstName,
    DEFAULT_USER_INFORMATION.lastName,
    '',
    'Yes',
    DEFAULT_USER_INFORMATION.company,
    DEFAULT_USER_INFORMATION.job,
    DEFAULT_USER_INFORMATION.email,
    '*********',
  ]);
});

msTest('Update personal information', async ({ clientArea }) => {
  await MockBms.mockUserRoute(clientArea);
  await goToPersonalPage(clientArea);
  const dataContainer = clientArea.locator('.personal-data-content').nth(0).locator('.ms-summary-card').nth(0);
  await expect(dataContainer.locator('.ms-summary-card-item__text')).toHaveText([
    DEFAULT_USER_INFORMATION.firstName,
    DEFAULT_USER_INFORMATION.lastName,
    '',
  ]);
  const modal = clientArea.locator('.personal-info-modal');
  await expect(modal).toBeHidden();
  await dataContainer.locator('.update-button').click();
  await expect(modal).toBeVisible();
  await expect(modal.locator('.ms-modal-header__title')).toHaveText('Update personal information');
  const inputs = modal.locator('ion-input');
  await expect(inputs.nth(0).locator('input')).toHaveValue(DEFAULT_USER_INFORMATION.firstName);
  await expect(inputs.nth(1).locator('input')).toHaveValue(DEFAULT_USER_INFORMATION.lastName);
  await expect(inputs.nth(2).locator('input')).toHaveValue('');
  await fillIonInput(inputs.nth(0), 'Gregory');
  await fillIonInput(inputs.nth(1), 'House');
  const okButton = modal.locator('#next-button');
  await expect(okButton).toBeTrulyEnabled();
  await okButton.click();
  await expect(modal).toBeHidden();
  await expect(dataContainer.locator('.ms-summary-card-item__text')).toHaveText(['Gregory', 'House', '']);

  // Reopen to add a phone number now
  await dataContainer.locator('.update-button').click();
  await expect(modal).toBeVisible();
  await expect(inputs.nth(0).locator('input')).toHaveValue('Gregory');
  await expect(inputs.nth(1).locator('input')).toHaveValue('House');
  await expect(inputs.nth(2).locator('input')).toHaveValue('');
  await fillIonInput(inputs.nth(2), '+1 609-258-3000');
  await expect(okButton).toBeTrulyEnabled();
  await okButton.click();
  await expect(modal).toBeHidden();
  await expect(dataContainer.locator('.ms-summary-card-item__text')).toHaveText(['Gregory', 'House', '+1 609-258-3000']);

  await MockBms.mockUserRoute(clientArea, { PATCH: { errors: { status: 401, attribute: 'client.phone' } } });
  // Try to remove the phone number
  await dataContainer.locator('.update-button').click();
  await expect(modal).toBeVisible();
  await expect(modal.locator('.ms-modal-header__title')).toHaveText('Update personal information');
  await expect(inputs.nth(2).locator('input')).toHaveValue('+1 609-258-3000');
  await fillIonInput(inputs.nth(2), '');
  await expect(okButton).toBeTrulyEnabled();
  await okButton.click();
  await expect(modal).toBeVisible();
  await expect(modal.locator('.input-container').nth(2).locator('.form-error').nth(1)).toBeVisible();
  await expect(modal.locator('.input-container').nth(2).locator('.form-error').nth(1)).toHaveText('Invalid phone number');
});

msTest('Update personal information generic fail', async ({ clientArea }) => {
  await MockBms.mockUserRoute(clientArea, { PATCH: { errors: { status: 401 } } });
  await goToPersonalPage(clientArea);
  const dataContainer = clientArea.locator('.personal-data-content').nth(0).locator('.ms-summary-card').nth(0);
  await expect(dataContainer.locator('.ms-summary-card-item__text')).toHaveText([
    DEFAULT_USER_INFORMATION.firstName,
    DEFAULT_USER_INFORMATION.lastName,
    '',
  ]);
  const modal = clientArea.locator('.personal-info-modal');
  await expect(modal).toBeHidden();
  await dataContainer.locator('.update-button').click();
  await expect(modal).toBeVisible();
  const inputs = modal.locator('ion-input');
  await expect(inputs.nth(0).locator('input')).toHaveValue(DEFAULT_USER_INFORMATION.firstName);
  await expect(inputs.nth(1).locator('input')).toHaveValue(DEFAULT_USER_INFORMATION.lastName);
  await expect(inputs.nth(2).locator('input')).toHaveValue('');
  await fillIonInput(inputs.nth(0), 'Gregory');
  await fillIonInput(inputs.nth(1), 'House');
  await fillIonInput(inputs.nth(2), '+1 609-258-3000');
  const okButton = modal.locator('#next-button');
  await okButton.click();
  // TODO: Update when generic failures are handled
  // await expect(modal.locator('.error')).toHaveText('ERROR');
  await modal.locator('.closeBtn').click();
  await expect(modal).toBeHidden();
  await expect(dataContainer.locator('.ms-summary-card-item__text')).toHaveText([
    DEFAULT_USER_INFORMATION.firstName,
    DEFAULT_USER_INFORMATION.lastName,
    '',
  ]);
});

msTest('Update personal information timeout', async ({ clientArea }) => {
  await MockBms.mockUserRoute(clientArea, { PATCH: { timeout: true } });
  await goToPersonalPage(clientArea);
  const dataContainer = clientArea.locator('.personal-data-content').nth(0).locator('.ms-summary-card').nth(0);
  await expect(dataContainer.locator('.ms-summary-card-item__text')).toHaveText([
    DEFAULT_USER_INFORMATION.firstName,
    DEFAULT_USER_INFORMATION.lastName,
    '',
  ]);
  const modal = clientArea.locator('.personal-info-modal');
  await expect(modal).toBeHidden();
  await dataContainer.locator('.update-button').click();
  await expect(modal).toBeVisible();
  const inputs = modal.locator('ion-input');
  await expect(inputs.nth(0).locator('input')).toHaveValue(DEFAULT_USER_INFORMATION.firstName);
  await expect(inputs.nth(1).locator('input')).toHaveValue(DEFAULT_USER_INFORMATION.lastName);
  await expect(inputs.nth(2).locator('input')).toHaveValue('');
  await fillIonInput(inputs.nth(0), 'Gregory');
  await fillIonInput(inputs.nth(1), 'House');
  await fillIonInput(inputs.nth(2), '+1 609-258-3000');
  const okButton = modal.locator('#next-button');
  await okButton.click();
  // TODO: Update when generic failures are handled
  // await expect(modal.locator('.error')).toHaveText('ERROR');
  await modal.locator('.closeBtn').click();
  await expect(modal).toBeHidden();
  await expect(dataContainer.locator('.ms-summary-card-item__text')).toHaveText([
    DEFAULT_USER_INFORMATION.firstName,
    DEFAULT_USER_INFORMATION.lastName,
    '',
  ]);
});

msTest('Update email', async ({ clientArea }) => {
  await MockBms.mockUpdateEmail(clientArea);
  await goToPersonalPage(clientArea);
  const dataContainer = clientArea.locator('.personal-data-content').nth(1).locator('.ms-summary-card').nth(0);
  await expect(dataContainer.locator('.ms-summary-card-item__text')).toHaveText(DEFAULT_USER_INFORMATION.email);
  const modal = clientArea.locator('.authentication-modal');
  await expect(modal).toBeHidden();
  await dataContainer.locator('.update-button').click();
  await expect(modal).toBeVisible();
  const okButton = modal.locator('#next-button');
  await expect(okButton).toBeTrulyDisabled();
  await expect(modal.locator('.ms-modal-header__title-container')).toHaveText('Change your email');
  const inputs = modal.locator('ion-input');
  await expect(inputs.nth(0).locator('input')).toHaveValue('');
  await expect(inputs.nth(1).locator('input')).toHaveValue('');
  await fillIonInput(inputs.nth(0), 'gregory.house@hospital.pu');
  await expect(okButton).toBeTrulyDisabled();
  await fillIonInput(inputs.nth(1), 'Hydrocodone/paracetamol');
  await expect(okButton).toBeTrulyEnabled();
  await okButton.click();
  await expect(modal).toBeHidden();
  await expect(dataContainer.locator('.ms-summary-card-item__text')).toHaveText('gregory.house@hospital.pu');
});

for (const params of [
  { status: 400, expectedMsg: 'Wrong email address format.' },
  { status: 403, expectedMsg: 'The password is wrong.' },
  { status: 401, expectedMsg: 'An unexpected error occurred. Please try again.' },
])
  msTest(`Update email fail (status: ${params.status})`, async ({ clientArea }) => {
    await MockBms.mockUpdateEmail(clientArea, { POST: { errors: { status: params.status } } });
    await goToPersonalPage(clientArea);
    const dataContainer = clientArea.locator('.personal-data-content').nth(1).locator('.ms-summary-card').nth(0);
    await expect(dataContainer.locator('.ms-summary-card-item__text')).toHaveText(DEFAULT_USER_INFORMATION.email);
    const modal = clientArea.locator('.authentication-modal');
    await expect(modal).toBeHidden();
    await dataContainer.locator('.update-button').click();
    await expect(modal).toBeVisible();
    const inputs = modal.locator('ion-input');
    await fillIonInput(inputs.nth(0), 'gregory.house@hospital.pu');
    await fillIonInput(inputs.nth(1), 'Hydrocodone/paracetamol');
    await modal.locator('#next-button').click();
    await expect(modal.locator('.ms-error')).toHaveText(params.expectedMsg);
    await modal.locator('.closeBtn').click();
    await expect(modal).toBeHidden();
    await expect(dataContainer.locator('.ms-summary-card-item__text')).toHaveText(DEFAULT_USER_INFORMATION.email);
  });

msTest('Update email timeout', async ({ clientArea }) => {
  await MockBms.mockUpdateEmail(clientArea, { POST: { timeout: true } });
  await goToPersonalPage(clientArea);
  const dataContainer = clientArea.locator('.personal-data-content').nth(1).locator('.ms-summary-card').nth(0);
  await expect(dataContainer.locator('.ms-summary-card-item__text')).toHaveText(DEFAULT_USER_INFORMATION.email);
  const modal = clientArea.locator('.authentication-modal');
  await expect(modal).toBeHidden();
  await dataContainer.locator('.update-button').click();
  await expect(modal).toBeVisible();
  const inputs = modal.locator('ion-input');
  await fillIonInput(inputs.nth(0), 'gregory.house@hospital.pu');
  await fillIonInput(inputs.nth(1), 'Hydrocodone/paracetamol');
  await modal.locator('#next-button').click();
  await expect(modal.locator('.ms-error')).toHaveText('An unexpected error occurred. Please try again.');
  await modal.locator('.closeBtn').click();
  await expect(modal).toBeHidden();
  await expect(dataContainer.locator('.ms-summary-card-item__text')).toHaveText(DEFAULT_USER_INFORMATION.email);
});

msTest('Update professional information', async ({ clientArea }) => {
  await MockBms.mockUserRoute(clientArea);
  await goToPersonalPage(clientArea);
  const dataContainer = clientArea.locator('.personal-data-content').nth(0).locator('.ms-summary-card').nth(1);
  await expect(dataContainer.locator('.ms-summary-card-item__text')).toHaveText([
    'Yes',
    DEFAULT_USER_INFORMATION.company,
    DEFAULT_USER_INFORMATION.job,
  ]);
  const modal = clientArea.locator('.professional-info-modal');
  await expect(modal).toBeHidden();
  await dataContainer.locator('.update-button').click();
  await expect(modal).toBeVisible();
  await expect(modal.locator('.ms-boolean-toggle').locator('.button-view').nth(0)).toHaveTheClass('button-disabled');
  const inputs = modal.locator('ion-input');
  await expect(inputs.nth(0).locator('input')).toHaveValue(DEFAULT_USER_INFORMATION.company);
  await expect(inputs.nth(1).locator('input')).toHaveValue(DEFAULT_USER_INFORMATION.job);
  await fillIonInput(inputs.nth(0), 'The Resistance');
  await fillIonInput(inputs.nth(1), 'Resistant');
  await modal.locator('#next-button').click();
  await expect(modal).toBeHidden();
  await expect(dataContainer.locator('.ms-summary-card-item__text')).toHaveText(['Yes', 'The Resistance', 'Resistant']);
});

msTest('Update professional information no job/company', async ({ clientArea }) => {
  await MockBms.mockUserRoute(clientArea);
  await goToPersonalPage(clientArea);
  const dataContainer = clientArea.locator('.personal-data-content').nth(0).locator('.ms-summary-card').nth(1);
  await expect(dataContainer.locator('.ms-summary-card-item__text')).toHaveText([
    'Yes',
    DEFAULT_USER_INFORMATION.company,
    DEFAULT_USER_INFORMATION.job,
  ]);
  const modal = clientArea.locator('.professional-info-modal');
  await expect(modal).toBeHidden();
  await dataContainer.locator('.update-button').click();
  await expect(modal).toBeVisible();
  await expect(modal.locator('.ms-boolean-toggle').locator('.button-view').nth(0)).toHaveTheClass('button-disabled');
  await modal.locator('.ms-boolean-toggle').locator('.button-view').nth(1).click();
  await expect(modal.locator('.ms-boolean-toggle').locator('.button-view').nth(1)).toHaveTheClass('button-disabled');
  const inputs = modal.locator('ion-input');
  await expect(inputs.nth(0)).toBeHidden();
  await expect(inputs.nth(1)).toBeHidden();
  await modal.locator('#next-button').click();
  await expect(modal).toBeHidden();
  await expect(dataContainer.locator('.ms-summary-card-item__text')).toHaveText(['No']);
});

msTest('Update professional information fail', async ({ clientArea }) => {
  await MockBms.mockUserRoute(clientArea, { PATCH: { errors: { status: 401 } } });
  await goToPersonalPage(clientArea);
  const dataContainer = clientArea.locator('.personal-data-content').nth(0).locator('.ms-summary-card').nth(1);
  await expect(dataContainer.locator('.ms-summary-card-item__text')).toHaveText([
    'Yes',
    DEFAULT_USER_INFORMATION.company,
    DEFAULT_USER_INFORMATION.job,
  ]);
  const modal = clientArea.locator('.professional-info-modal');
  await expect(modal).toBeHidden();
  await dataContainer.locator('.update-button').click();
  await expect(modal).toBeVisible();
  const inputs = modal.locator('ion-input');
  await expect(inputs.nth(0).locator('input')).toHaveValue(DEFAULT_USER_INFORMATION.company);
  await expect(inputs.nth(1).locator('input')).toHaveValue(DEFAULT_USER_INFORMATION.job);
  await fillIonInput(inputs.nth(0), 'Resistant');
  await fillIonInput(inputs.nth(1), 'The Resistance');
  await modal.locator('#next-button').click();
  // TODO: Update when we have a generic error message
  // await expect(modal.locator('.error')).toHaveText('ERROR');
  await modal.locator('.closeBtn').click();
  await expect(modal).toBeHidden();
  await expect(dataContainer.locator('.ms-summary-card-item__text')).toHaveText([
    'Yes',
    DEFAULT_USER_INFORMATION.company,
    DEFAULT_USER_INFORMATION.job,
  ]);
});

msTest('Update professional information timeout', async ({ clientArea }) => {
  await MockBms.mockUserRoute(clientArea, { PATCH: { timeout: true } });
  await goToPersonalPage(clientArea);
  const dataContainer = clientArea.locator('.personal-data-content').nth(0).locator('.ms-summary-card').nth(1);
  await expect(dataContainer.locator('.ms-summary-card-item__text')).toHaveText([
    'Yes',
    DEFAULT_USER_INFORMATION.company,
    DEFAULT_USER_INFORMATION.job,
  ]);
  const modal = clientArea.locator('.professional-info-modal');
  await expect(modal).toBeHidden();
  await dataContainer.locator('.update-button').click();
  await expect(modal).toBeVisible();
  const inputs = modal.locator('ion-input');
  await expect(inputs.nth(0).locator('input')).toHaveValue(DEFAULT_USER_INFORMATION.company);
  await expect(inputs.nth(1).locator('input')).toHaveValue(DEFAULT_USER_INFORMATION.job);
  await fillIonInput(inputs.nth(0), 'The Resistance');
  await fillIonInput(inputs.nth(1), 'Resistant');
  await modal.locator('#next-button').click();
  // TODO: Update once we handle generic errors
  // await expect(modal.locator('.error')).toHaveText('ERROR');
  await modal.locator('.closeBtn').click();
  await expect(modal).toBeHidden();
  await expect(dataContainer.locator('.ms-summary-card-item__text')).toHaveText([
    'Yes',
    DEFAULT_USER_INFORMATION.company,
    DEFAULT_USER_INFORMATION.job,
  ]);
});

// msTest('Update password', async ({ clientArea }) => {
//   await MockBms.mockUpdateAuthentication(clientArea);
//   await goToPersonalPage(clientArea);
//   const modal = clientArea.locator('.security-modal');
//   await expect(modal).toBeHidden();
//   await clientArea.locator('.personal-data-content').nth(1).locator('.ms-summary-card').nth(1).locator('.update-button').click();
//   await expect(modal).toBeVisible();
//   const inputs = modal.locator('ion-input');
//   await fillIonInput(inputs.nth(0), 'D3@th2N1H1l@nth');
//   await fillIonInput(inputs.nth(1), 'D0wnW1thBr33n');
//   await fillIonInput(inputs.nth(2), 'D0wnW1thBr33n');
//   await modal.locator('.okButton').click();
//   await expect(clientArea).toShowToast('YEP', 'Success');
// });

// msTest('Update password fail', async ({ clientArea }) => {
//   await MockBms.mockUpdateAuthentication(clientArea, { errors: { status: 401 } });
//   await goToPersonalPage(clientArea);
//   const modal = clientArea.locator('.security-modal');
//   await expect(modal).toBeHidden();
//   await clientArea.locator('.personal-data-content').nth(1).locator('.ms-summary-card').nth(1).locator('.update-button').click();
//   await expect(modal).toBeVisible();
//   const inputs = modal.locator('ion-input');
//   await fillIonInput(inputs.nth(0), 'D3@th2N1H1l@nth');
//   await fillIonInput(inputs.nth(1), 'D0wnW1thBr33n');
//   await fillIonInput(inputs.nth(2), 'D0wnW1thBr33n');
//   await modal.locator('.okButton').click();
//   await expect(modal.locator('.error')).toHaveText('ERROR');
//   await modal.locator('.closeBtn').click();
//   await expect(modal).toBeHidden();
// });

// msTest('Update password timeout', async ({ clientArea }) => {
//   await MockBms.mockUpdateAuthentication(clientArea, { timeout: true });
//   await goToPersonalPage(clientArea);
//   const modal = clientArea.locator('.security-modal');
//   await expect(modal).toBeHidden();
//   await clientArea.locator('.personal-data-content').nth(1).locator('.ms-summary-card').nth(1).locator('.update-button').click();
//   await expect(modal).toBeVisible();
//   const inputs = modal.locator('ion-input');
//   await fillIonInput(inputs.nth(0), 'D3@th2N1H1l@nth');
//   await fillIonInput(inputs.nth(1), 'D0wnW1thBr33n');
//   await fillIonInput(inputs.nth(2), 'D0wnW1thBr33n');
//   await modal.locator('.okButton').click();
//   await expect(modal.locator('.error')).toHaveText('ERROR');
//   await modal.locator('.closeBtn').click();
//   await expect(modal).toBeHidden();
// });
