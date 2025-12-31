// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { Page } from '@playwright/test';
import { DEFAULT_USER_INFORMATION, MockBms, expect, fillIonInput, msTest } from '@tests/e2e/helpers';

async function goToPersonalPage(page: Page): Promise<void> {
  await page.locator('.topbar').locator('.profile-header').click();
  await expect(page.locator('.topbar').locator('.topbar-left-text__title')).toHaveText('My profile');
}

msTest('Check personal data page', async ({ clientArea }) => {
  const title = clientArea.locator('.topbar').locator('.topbar-left-text__title');
  await expect(title).toHaveText('Dashboard');
  const avatar = clientArea.locator('.topbar-right').locator('.profile-header');
  await expect(avatar.locator('.text-content__name')).toHaveText(DEFAULT_USER_INFORMATION.name);
  await avatar.click();
  await expect(title).toHaveText('My profile');
  const container = clientArea.locator('.personal-data-page');
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
    'Not defined',
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
    'Not defined',
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
  await expect(clientArea).toShowToast('Personal information has been updated.', 'Success');
  await expect(dataContainer.locator('.ms-summary-card-item__text')).toHaveText(['Gregory', 'House', 'Not defined']);

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
  await expect(clientArea).toShowToast('Personal information has been updated.', 'Success');
  await expect(dataContainer.locator('.ms-summary-card-item__text')).toHaveText(['Gregory', 'House', '+1 609-258-3000']);

  await MockBms.mockUserRoute(clientArea, {}, { PATCH: { errors: { status: 401, attribute: 'client.phone' } } });
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
  await MockBms.mockUserRoute(clientArea, {}, { PATCH: { errors: { status: 401 } } });
  await goToPersonalPage(clientArea);
  const dataContainer = clientArea.locator('.personal-data-content').nth(0).locator('.ms-summary-card').nth(0);
  await expect(dataContainer.locator('.ms-summary-card-item__text')).toHaveText([
    DEFAULT_USER_INFORMATION.firstName,
    DEFAULT_USER_INFORMATION.lastName,
    'Not defined',
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
  await expect(modal.locator('.report-error')).toHaveText('An unexpected error occurred. Please try again.');
  await modal.locator('.closeBtn').click();
  await expect(modal).toBeHidden();
  await expect(dataContainer.locator('.ms-summary-card-item__text')).toHaveText([
    DEFAULT_USER_INFORMATION.firstName,
    DEFAULT_USER_INFORMATION.lastName,
    'Not defined',
  ]);
});

msTest('Update personal information timeout', async ({ clientArea }) => {
  await MockBms.mockUserRoute(clientArea, {}, { PATCH: { timeout: true } });
  await goToPersonalPage(clientArea);
  const dataContainer = clientArea
    .locator('.personal-data-page')
    .locator('.personal-data-content')
    .nth(0)
    .locator('.ms-summary-card')
    .nth(0);
  await expect(dataContainer.locator('.ms-summary-card-item__text')).toHaveText([
    DEFAULT_USER_INFORMATION.firstName,
    DEFAULT_USER_INFORMATION.lastName,
    'Not defined',
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
    'Not defined',
  ]);
});

msTest('Update email', async ({ clientArea }) => {
  await MockBms.mockUpdateEmailSendCode(clientArea);
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
  const title = modal.locator('.ms-modal-header__title-container');
  const subtitle = modal.locator('.ms-modal-header__text');
  const inputs = modal.locator('ion-input');
  await expect(title).toHaveText('Change your email');
  await expect(subtitle).toHaveText('If you update your email address, keep in mind that you will have to log in with this new address.');
  await expect(inputs.nth(0).locator('input')).toHaveValue('');
  await fillIonInput(inputs.nth(0), 'gregory.house@hospital.pu');
  await expect(okButton).toBeTrulyEnabled();
  await okButton.click();

  await expect(title).toHaveText('Enter your password');
  await expect(subtitle).toBeHidden();
  await expect(inputs.nth(1).locator('input')).toHaveValue('');
  await expect(okButton).toBeTrulyDisabled();
  await fillIonInput(inputs.nth(1), 'Hydrocodone/paracetamol');
  await expect(okButton).toBeTrulyEnabled();
  await okButton.click();

  const codeInputs = modal.locator('.code-input-list').locator('ion-input');
  await expect(title).toHaveText('Validate your new email');
  await expect(subtitle).toHaveText('You should have received a code on your new email address.');
  await expect(okButton).toBeTrulyDisabled();
  let i = 1;
  for (const codeInput of await codeInputs.all()) {
    await fillIonInput(codeInput, i.toString());
    i++;
  }
  await expect(okButton).toBeTrulyEnabled();
  await okButton.click();

  await expect(modal).toBeHidden();
  await expect(dataContainer.locator('.ms-summary-card-item__text')).toHaveText('gregory.house@hospital.pu');
  await expect(clientArea).toShowToast('Email has been updated.', 'Success');
});

for (const params of [
  { status: 400, code: 'EMAIL_ALREADY_VALIDATED', expectedMsg: 'This email is already used.' },
  { status: 400, code: 'EMAIL_VALIDATION_CODE_TRIES_EXCEEDED', expectedMsg: 'Too many tries.' },
  { status: 400, code: 'EMAIL_VALIDATION_INVALID_CODE', expectedMsg: 'The code is invalid.' },
  { status: 400, code: 'error', expectedMsg: 'An unexpected error occurred. Please try again.' },
  { status: 403, code: 'error', expectedMsg: 'The password is incorrect.' },
  { status: 401, code: 'error', expectedMsg: 'An unexpected error occurred. Please try again.' },
])
  msTest(`Update email fail (status: ${params.status} ${params.code})`, async ({ clientArea }) => {
    await MockBms.mockUpdateEmailSendCode(clientArea);
    await MockBms.mockUpdateEmail(clientArea, { POST: { errors: { status: params.status, code: params.code } } });
    await goToPersonalPage(clientArea);
    const dataContainer = clientArea
      .locator('.personal-data-page')
      .locator('.personal-data-content')
      .nth(1)
      .locator('.ms-summary-card')
      .nth(0);
    await expect(dataContainer.locator('.ms-summary-card-item__text')).toHaveText(DEFAULT_USER_INFORMATION.email);
    const modal = clientArea.locator('.authentication-modal');
    await expect(modal).toBeHidden();
    await dataContainer.locator('.update-button').click();
    await expect(modal).toBeVisible();

    const okButton = modal.locator('#next-button');
    await expect(okButton).toBeTrulyDisabled();
    const title = modal.locator('.ms-modal-header__title-container');
    const subtitle = modal.locator('.ms-modal-header__text');
    const inputs = modal.locator('ion-input');
    await expect(title).toHaveText('Change your email');
    await expect(subtitle).toHaveText('If you update your email address, keep in mind that you will have to log in with this new address.');
    await expect(inputs.nth(0).locator('input')).toHaveValue('');
    await fillIonInput(inputs.nth(0), 'gregory.house@hospital.pu');
    await expect(okButton).toBeTrulyEnabled();
    await okButton.click();

    await expect(title).toHaveText('Enter your password');
    await expect(subtitle).toBeHidden();
    await expect(inputs.nth(1).locator('input')).toHaveValue('');
    await expect(okButton).toBeTrulyDisabled();
    await fillIonInput(inputs.nth(1), 'Hydrocodone/paracetamol');
    await expect(okButton).toBeTrulyEnabled();
    await okButton.click();

    const codeInputs = modal.locator('.code-input-list').locator('ion-input');
    await expect(title).toHaveText('Validate your new email');
    await expect(subtitle).toHaveText('You should have received a code on your new email address.');
    await expect(okButton).toBeTrulyDisabled();
    let i = 1;
    for (const codeInput of await codeInputs.all()) {
      await fillIonInput(codeInput, i.toString());
      i++;
    }
    await expect(okButton).toBeTrulyEnabled();
    await okButton.click();

    const error = modal.locator('.change-password-error');
    await expect(error).toBeVisible();
    await expect(error).toHaveText(params.expectedMsg);
    await expect(modal).toBeVisible();
  });

msTest('Update email timeout', async ({ clientArea }) => {
  await MockBms.mockUpdateEmailSendCode(clientArea);
  await MockBms.mockUpdateEmail(clientArea, { POST: { timeout: true } });
  await goToPersonalPage(clientArea);
  const dataContainer = clientArea.locator('.personal-data-content').nth(1).locator('.ms-summary-card').nth(0);
  await expect(dataContainer.locator('.ms-summary-card-item__text')).toHaveText(DEFAULT_USER_INFORMATION.email);
  const modal = clientArea.locator('.authentication-modal');
  await dataContainer.locator('.update-button').click();
  const okButton = modal.locator('#next-button');
  const inputs = modal.locator('ion-input');
  await expect(inputs.nth(0).locator('input')).toHaveValue('');
  await fillIonInput(inputs.nth(0), 'gregory.house@hospital.pu');
  await okButton.click();
  await expect(inputs.nth(1).locator('input')).toHaveValue('');
  await fillIonInput(inputs.nth(1), 'Hydrocodone/paracetamol');
  await okButton.click();
  const codeInputs = modal.locator('.code-input-list').locator('ion-input');
  let i = 1;
  for (const codeInput of await codeInputs.all()) {
    await fillIonInput(codeInput, i.toString());
    i++;
  }
  await okButton.click();
  await expect(modal.locator('.change-password-error')).toHaveText('An unexpected error occurred. Please try again.');
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
  await MockBms.mockUserRoute(clientArea, {}, { PATCH: { errors: { status: 401 } } });
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
  await MockBms.mockUserRoute(clientArea, {}, { PATCH: { timeout: true } });
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
