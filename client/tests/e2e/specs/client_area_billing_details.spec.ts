// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { DEFAULT_USER_INFORMATION, MockBms, clientAreaNavigateTo, expect, fillIonInput, msTest } from '@tests/e2e/helpers';

msTest('Test initial status', async ({ clientArea }) => {
  await clientAreaNavigateTo(clientArea, 'Billing details');

  const inputs = clientArea.locator('.main-content').locator('ion-input');
  await expect(inputs.nth(0).locator('input')).toHaveValue(DEFAULT_USER_INFORMATION.address.line1);
  await expect(inputs.nth(0).locator('input')).toHaveDisabledAttribute();
  await expect(inputs.nth(1).locator('input')).toHaveValue(DEFAULT_USER_INFORMATION.address.postalCode);
  await expect(inputs.nth(1).locator('input')).toHaveDisabledAttribute();
  await expect(inputs.nth(2).locator('input')).toHaveValue(DEFAULT_USER_INFORMATION.address.city);
  await expect(inputs.nth(2).locator('input')).toHaveDisabledAttribute();
  await expect(inputs.nth(3).locator('input')).toHaveValue(DEFAULT_USER_INFORMATION.address.country);
  await expect(inputs.nth(3).locator('input')).toHaveDisabledAttribute();

  const toggleButton = clientArea.locator('.main-content').locator('.update-button');
  const submitButton = clientArea.locator('.main-content').locator('.submit-button');
  const cancelButton = clientArea.locator('.main-content').locator('.cancel-button');
  await expect(toggleButton).toBeTrulyEnabled();
  await expect(toggleButton).toBeVisible();
  await expect(submitButton).toBeTrulyEnabled();
  await expect(submitButton).not.toBeVisible();
  await expect(cancelButton).toBeTrulyEnabled();
  await expect(cancelButton).not.toBeVisible();
  await toggleButton.click();
  await expect(toggleButton).not.toBeVisible();
  await expect(submitButton).toBeVisible();
  await expect(cancelButton).toBeVisible();
  await expect(inputs.nth(0).locator('input')).toNotHaveDisabledAttribute();
  await expect(inputs.nth(1).locator('input')).toNotHaveDisabledAttribute();
  await expect(inputs.nth(2).locator('input')).toNotHaveDisabledAttribute();
  await expect(inputs.nth(3).locator('input')).toNotHaveDisabledAttribute();
  await expect(inputs.nth(4).locator('input')).toNotHaveDisabledAttribute();
});

msTest('Test update address', async ({ clientArea }) => {
  await clientAreaNavigateTo(clientArea, 'Billing details');
  await clientArea.locator('.main-content').locator('.update-button').click();

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

  await clientAreaNavigateTo(clientArea, 'Billing details');
  await clientArea.locator('.main-content').locator('.update-button').click();

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

  await clientAreaNavigateTo(clientArea, 'Billing details');
  await clientArea.locator('.main-content').locator('.update-button').click();

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

msTest('Test cancel update address', async ({ clientArea }) => {
  await clientAreaNavigateTo(clientArea, 'Billing details');
  await clientArea.locator('.main-content').locator('.update-button').click();

  const inputs = clientArea.locator('.main-content').locator('ion-input');
  await fillIonInput(inputs.nth(0), '221B Baker Street');
  await fillIonInput(inputs.nth(1), 'Marylebone');
  await fillIonInput(inputs.nth(2), 'NW1 6XE');
  await fillIonInput(inputs.nth(3), 'London');
  await fillIonInput(inputs.nth(4), 'United Kingdom');

  const cancelButton = clientArea.locator('.main-content').locator('.cancel-button');
  await expect(cancelButton).toBeTrulyEnabled();
  await cancelButton.click();
  await expect(inputs.nth(0).locator('input')).toHaveValue(DEFAULT_USER_INFORMATION.address.line1);
  await expect(inputs.nth(0).locator('input')).toHaveDisabledAttribute();
  await expect(inputs.nth(1).locator('input')).toHaveValue(DEFAULT_USER_INFORMATION.address.postalCode);
  await expect(inputs.nth(1).locator('input')).toHaveDisabledAttribute();
  await expect(inputs.nth(2).locator('input')).toHaveValue(DEFAULT_USER_INFORMATION.address.city);
  await expect(inputs.nth(2).locator('input')).toHaveDisabledAttribute();
  await expect(inputs.nth(3).locator('input')).toHaveValue(DEFAULT_USER_INFORMATION.address.country);
  await expect(inputs.nth(3).locator('input')).toHaveDisabledAttribute();
});
