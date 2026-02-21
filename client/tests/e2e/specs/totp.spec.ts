// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { expect, fillIonInput, msTest } from '@tests/e2e/helpers';

// Need some way to generate TOTP codes first
msTest.skip('Setup totp modal', async ({ myProfilePage }) => {
  await expect(myProfilePage.locator('.menu-list__item').nth(2)).toHaveText('Authentication');
  await myProfilePage.locator('.menu-list__item').nth(2).click();
  await expect(myProfilePage.locator('.profile-content-item').locator('.item-header__title')).toHaveText('Authentication');

  const totpStatus = myProfilePage.locator('.totp-status');
  await expect(totpStatus).toBeVisible();
  await expect(totpStatus.locator('ion-button')).toBeVisible();
  await expect(totpStatus.locator('ion-button')).toHaveText('Set up MFA');
  await totpStatus.locator('ion-button').click();
  const modal = myProfilePage.locator('.activate-totp-modal');
  await expect(modal).toBeHidden();
  await expect(modal).toBeVisible();
  const nextButton = modal.locator('#next-button');

  const firstPage = modal.locator('.modal-current-authentication');
  const secondPage = modal.locator('.modal-content-information');
  const thirdPage = modal.locator('.modal-content-enter-code');
  const fourthPage = modal.locator('.modal-content-finalization');

  await expect(firstPage).toBeVisible();
  await expect(secondPage).toBeHidden();
  await expect(thirdPage).toBeHidden();
  await expect(fourthPage).toBeHidden();

  await expect(modal.locator('.ms-modal-header')).toHaveText('Multi-Factor Authentication (MFA)');
  await expect(nextButton).toBeTrulyDisabled();
  await fillIonInput(firstPage.locator('ion-input'), 'P@ssw0rd.');
  await expect(nextButton).toBeTrulyEnabled();
  await nextButton.click();

  await expect(firstPage).toBeHidden();
  await expect(secondPage).toBeVisible();
  await expect(thirdPage).toBeHidden();
  await expect(fourthPage).toBeHidden();

  await expect(secondPage.locator('.step-code-qrcode')).toBeVisible();
  await expect(secondPage.locator('.step-code-copy-text')).toHaveText('ABCDEF');
  await expect(secondPage.locator('.step-code-copy-button')).toHaveText('Copy key', { useInnerText: true });
  await nextButton.click();

  await expect(firstPage).toBeHidden();
  await expect(secondPage).toBeHidden();
  await expect(thirdPage).toBeVisible();
  await expect(fourthPage).toBeHidden();

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
  await expect(myProfilePage).toShowToast('TOTP IS SETUP', 'Success');
  await expect(myProfilePage).toBeWorkspacePage();
});
