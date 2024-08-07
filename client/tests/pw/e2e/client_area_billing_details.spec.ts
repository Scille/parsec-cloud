// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { DEFAULT_USER_INFORMATION, MockBms, expect, fillIonInput, msTest } from '@tests/pw/helpers';

msTest('Test initial status', async ({ clientArea }) => {
  const title = clientArea.locator('.header-content').locator('.header-title');
  await clientArea.locator('.menu-client').locator('.menu-client-list').getByRole('listitem').nth(4).click();
  await expect(title).toHaveText('Billing details');

  const inputs = clientArea.locator('.main-content').locator('ion-input');
  await expect(inputs.nth(0).locator('input')).toHaveValue(DEFAULT_USER_INFORMATION.address.line1);
  await expect(inputs.nth(1).locator('input')).toHaveValue('');
  await expect(inputs.nth(2).locator('input')).toHaveValue(DEFAULT_USER_INFORMATION.address.postalCode);
  await expect(inputs.nth(3).locator('input')).toHaveValue(DEFAULT_USER_INFORMATION.address.city);
  await expect(inputs.nth(4).locator('input')).toHaveValue(DEFAULT_USER_INFORMATION.address.country);
  const okButton = clientArea.locator('.main-content').locator('.submit-button');
  await expect(okButton).toBeTrulyEnabled();
});

msTest('Test update address', async ({ clientArea }) => {
  const title = clientArea.locator('.header-content').locator('.header-title');
  await clientArea.locator('.menu-client').locator('.menu-client-list').getByRole('listitem').nth(4).click();
  await expect(title).toHaveText('Billing details');

  const inputs = clientArea.locator('.main-content').locator('ion-input');
  await fillIonInput(inputs.nth(0), '221B Baker Street');
  await fillIonInput(inputs.nth(1), 'Marylebone');
  await fillIonInput(inputs.nth(2), 'NW1 6XE');
  await fillIonInput(inputs.nth(3), 'London');
  await fillIonInput(inputs.nth(4), 'United Kingdom');
  const okButton = clientArea.locator('.main-content').locator('.submit-button');
  await expect(okButton).toBeTrulyEnabled();
  await okButton.click();
  await expect(clientArea).toShowToast('The information have been updated.', 'Success');
});

msTest('Test update address generic error', async ({ clientArea }) => {
  await MockBms.mockBillingDetails(clientArea, {}, { PATCH: { errors: { status: 400 } } });

  const title = clientArea.locator('.header-content').locator('.header-title');
  await clientArea.locator('.menu-client').locator('.menu-client-list').getByRole('listitem').nth(4).click();
  await expect(title).toHaveText('Billing details');

  const inputs = clientArea.locator('.main-content').locator('ion-input');
  await fillIonInput(inputs.nth(0), '221B Baker Street');
  await fillIonInput(inputs.nth(1), 'Marylebone');
  await fillIonInput(inputs.nth(2), 'NW1 6XE');
  await fillIonInput(inputs.nth(3), 'London');
  await fillIonInput(inputs.nth(4), 'United Kingdom');
  const okButton = clientArea.locator('.main-content').locator('.submit-button');
  await expect(okButton).toBeTrulyEnabled();
  await okButton.click();
  await expect(clientArea).toShowToast('Could not update the information.', 'Error');
});

msTest('Test update address timeout', async ({ clientArea }) => {
  await MockBms.mockBillingDetails(clientArea, {}, { PATCH: { timeout: true } });
  const title = clientArea.locator('.header-content').locator('.header-title');
  await clientArea.locator('.menu-client').locator('.menu-client-list').getByRole('listitem').nth(4).click();
  await expect(title).toHaveText('Billing details');

  const inputs = clientArea.locator('.main-content').locator('ion-input');
  await fillIonInput(inputs.nth(0), '221B Baker Street');
  await fillIonInput(inputs.nth(1), 'Marylebone');
  await fillIonInput(inputs.nth(2), 'NW1 6XE');
  await fillIonInput(inputs.nth(3), 'London');
  await fillIonInput(inputs.nth(4), 'United Kingdom');
  const okButton = clientArea.locator('.main-content').locator('.submit-button');
  await expect(okButton).toBeTrulyEnabled();
  await okButton.click();
  await expect(clientArea).toShowToast('Could not update the information.', 'Error');
});
