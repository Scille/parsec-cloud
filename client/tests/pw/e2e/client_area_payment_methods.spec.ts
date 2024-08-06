// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { expect } from '@tests/pw/helpers/assertions';
import { MockBms } from '@tests/pw/helpers/bms';
import { msTest } from '@tests/pw/helpers/fixtures';
import { DateTime } from 'luxon';

[
  { fail: false, setDefault: { set: true, fail: false } },
  { fail: true, setDefault: { set: true, fail: false } },
  { fail: false, setDefault: { set: true, fail: true } },
].forEach(({ fail, setDefault }) => {
  msTest(`Add payment method Fail(${fail}) SetDefault(${setDefault.set}) SetDefaultFail(${setDefault.fail})`, async ({ clientArea }) => {
    await MockBms.mockBillingDetails(clientArea);
    await MockBms.mockAddPaymentMethod(clientArea, fail ? { PUT: { errors: { status: 401, attribute: 'payment_method' } } } : undefined);
    await MockBms.mockSetDefaultPaymentMethod(
      clientArea,
      setDefault.fail ? { PATCH: { errors: { status: 401, attribute: 'payment_method' } } } : undefined,
    );

    const title = clientArea.locator('.header-content').locator('.header-title');
    await clientArea.locator('.menu-client').locator('.menu-client-list').getByRole('listitem').nth(3).click();
    await expect(title).toHaveText('Payment methods');
    const addButton = clientArea.locator('.client-page-method').locator('ion-button').nth(0);
    await expect(addButton).toHaveText('Add');
    const modal = clientArea.locator('.credit-card-modal');
    await expect(modal).toBeHidden();
    await addButton.click();
    await expect(modal).toBeVisible();
    const modalButton = modal.locator('.ms-modal-footer').locator('ion-button');
    await expect(modalButton).toBeTrulyDisabled();
    const toggle = modal.locator('ion-toggle');
    const inputs = modal.locator('.input-container');
    await inputs.nth(0).frameLocator('iframe').locator('.InputContainer').locator('input').fill('4242 4242 4242 4242');
    await inputs
      .nth(1)
      .frameLocator('iframe')
      .locator('.InputContainer')
      .locator('input')
      .fill(DateTime.now().plus({ years: 3 }).toFormat('MM/yy'));
    await inputs.nth(2).frameLocator('iframe').locator('.InputContainer').locator('input').fill('123');
    await expect(modalButton).toBeTrulyEnabled();
    if (setDefault.set) {
      await toggle.click();
    }
    await modalButton.click();
    if (fail) {
      await expect(clientArea).toShowToast('Failed to register this card.', 'Error');
    } else {
      if (setDefault.set && setDefault.fail) {
        await expect(clientArea).toShowToast(
          'The card was successfully added but it could not be set as the default payment method.',
          'Success',
        );
      } else {
        await expect(clientArea).toShowToast('The card was successfully added.', 'Success');
      }
    }
  });
});
