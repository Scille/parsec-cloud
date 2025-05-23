// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { clientAreaNavigateTo, expect, MockBms, msTest } from '@tests/e2e/helpers';
import { DateTime } from 'luxon';
import { setTimeout } from 'timers/promises';

msTest('List payment methods', async ({ clientArea }) => {
  await clientAreaNavigateTo(clientArea, 'Payment methods');

  const activeContainer = clientArea.locator('.client-page-method').locator('.method-cards-active');
  const activeCard = activeContainer.locator('.ms-stripe-card');
  await expect(activeCard.locator('.title-h4').nth(0)).toHaveText('**** **** **** 4444');
  await expect(activeCard.locator('.title-h4').nth(1)).toHaveText(/^\d{2}\/\d{2}$/);
  await expect(clientArea.locator('.client-page-method').locator('.no-additional-payment-method')).toBeVisible();
  await expect(clientArea.locator('.client-page-method').locator('.no-additional-payment-method')).toHaveText(
    "You haven't added additional payment methods.",
  );
});

[
  { fail: false, setDefault: { set: true, fail: false } },
  { fail: true, setDefault: { set: true, fail: false } },
  { fail: false, setDefault: { set: true, fail: true } },
].forEach(({ fail, setDefault }) => {
  msTest(`Add payment method Fail(${fail}) SetDefault(${setDefault.set}) SetDefaultFail(${setDefault.fail})`, async ({ clientArea }) => {
    await MockBms.mockAddPaymentMethod(clientArea, fail ? { PUT: { errors: { status: 401, attribute: 'payment_method' } } } : undefined);
    await MockBms.mockSetDefaultPaymentMethod(
      clientArea,
      setDefault.fail ? { PATCH: { errors: { status: 401, attribute: 'payment_method' } } } : undefined,
    );

    await clientAreaNavigateTo(clientArea, 'Payment methods');
    const addButton = clientArea.locator('.client-page-method').locator('.method-cards-saved-header').locator('.custom-button');
    await expect(addButton).toHaveText('Add a new card');
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
    await expect(modalButton).toBeTrulyDisabled();
    if (fail) {
      await expect(modal.locator('#modal-error')).toHaveText('Failed to register this card.');
      await expect(modal).toBeVisible();
      await expect(modalButton).toBeTrulyEnabled();
    } else {
      if (setDefault.set && setDefault.fail) {
        await expect(clientArea).toShowToast(
          'The card was successfully added but it could not be set as the default payment method.',
          'Success',
        );
      } else {
        setTimeout(1000);
        await expect(clientArea).toShowToast('The card was successfully added.', 'Success');
      }
      await expect(modal).toBeHidden();
    }
  });
});

msTest('No payment methods', async ({ clientArea }) => {
  await MockBms.mockBillingDetails(clientArea, { cardsCount: 0, sepaCount: 0 });
  await clientAreaNavigateTo(clientArea, 'Payment methods');

  await expect(clientArea.locator('.client-page-method').locator('.no-payment-method')).toBeVisible();
  await expect(clientArea.locator('.client-page-method').locator('.no-payment-method')).toHaveText("You don't have a payment method set.");
});
