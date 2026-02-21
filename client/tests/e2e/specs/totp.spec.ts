// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { expect, fillIonInput, msTest, sendEvent } from '@tests/e2e/helpers';

msTest('Setup totp modal', async ({ connected }) => {
  const modal = connected.locator('.activate-totp-modal');
  await expect(modal).toBeHidden();
  await sendEvent(connected, 'ClientTotpRequired');
  await expect(modal).toBeVisible();

  const firstPage = modal.locator('.modal-content-information');
  const secondPage = modal.locator('.modal-content-enter-code');
  const thirdPage = modal.locator('.modal-content-finalization');

  await expect(firstPage).toBeVisible();
  await expect(secondPage).toBeHidden();
  await expect(thirdPage).toBeHidden();

  await expect(modal.locator('.ms-modal-header')).toHaveText('Multi-Factor Authentication (MFA)');
  await expect(firstPage.locator('.step-code-qrcode')).toBeVisible();
  await expect(firstPage.locator('.step-code-copy-text')).toHaveText('ABCDEF');
  await expect(firstPage.locator('.step-code-copy-button')).toHaveText('Copy key', { useInnerText: true });
  await expect(modal.locator('#next-button')).toHaveText('Next');
  await modal.locator('#next-button').click();
  await expect(firstPage).toBeHidden();
  await expect(secondPage).toBeVisible();
  await expect(thirdPage).toBeHidden();
  await expect(modal.locator('#next-button')).toBeTrulyDisabled();
  await fillIonInput(secondPage.locator('ion-input'), '123456');
  await expect(modal.locator('#next-button')).toBeTrulyEnabled();
  await modal.locator('#next-button').click();
  await expect(firstPage).toBeHidden();
  await expect(secondPage).toBeHidden();
  await expect(thirdPage).toBeVisible();
  await expect(modal.locator('#next-button')).toHaveText('Done');
  await modal.locator('#next-button').click();
  await expect(modal).toBeHidden();
  await expect(connected).toShowToast('TOTP IS SETUP', 'Success');
  await expect(connected).toBeWorkspacePage();
});

msTest('Decline TOTP setup', async ({ connected }) => {
  const modal = connected.locator('.activate-totp-modal');
  await expect(modal).toBeHidden();
  await sendEvent(connected, 'ClientTotpRequired');
  await expect(modal).toBeVisible();
  await modal.locator('.closeBtn').click();
  await expect(modal).toBeHidden();
  await expect(connected).toBeHomePage();
});
