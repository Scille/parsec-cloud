// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import {
  expect,
  fillInputModal,
  fillIonInput,
  generateTotpCode,
  getClipboardText,
  getServerHttpAddr,
  logout,
  MsPage,
  msTest,
  resetTotp,
  setWriteClipboardPermission,
} from '@tests/e2e/helpers';

async function setupTotp(page: MsPage): Promise<string> {
  const totpStatus = page.locator('.mfa');
  const modal = page.locator('.activate-totp-modal');
  await expect(modal).toBeHidden();
  await totpStatus.locator('ion-button').click();
  await expect(modal).toBeVisible();
  const nextButton = modal.locator('#next-button');

  const firstPage = modal.locator('.modal-current-authentication');
  const secondPage = modal.locator('.modal-content-information');
  const thirdPage = modal.locator('.modal-content-enter-code');
  const fourthPage = modal.locator('.modal-content-finalization');

  await fillIonInput(firstPage.locator('ion-input'), 'P@ssw0rd.');
  await expect(nextButton).toBeTrulyEnabled();
  await nextButton.click();

  await setWriteClipboardPermission(page.context(), true);
  await secondPage.locator('.step-code-copy-button').click();
  const totpSecret = await getClipboardText(page);

  await nextButton.click();

  const totpCode = await generateTotpCode(totpSecret);

  await fillIonInput(thirdPage.locator('ion-input'), totpCode);
  await expect(nextButton).toBeTrulyEnabled();
  await nextButton.click();
  await expect(fourthPage).toBeVisible();
  await expect(nextButton).toHaveText('Done');
  await nextButton.click();
  await expect(modal).toBeHidden();
  await expect(page).toShowToast(
    'Multi-factor authentication has been successfully enabled and will now be required to log in to this organization.',
    'Success',
  );
  return totpSecret;
}

msTest('Setup totp modal', async ({ myProfilePage }) => {
  await expect(myProfilePage.locator('.menu-list__item').nth(2)).toHaveText('Authentication');
  await myProfilePage.locator('.menu-list__item').nth(2).click();
  await expect(myProfilePage.locator('.profile-content-item').locator('.item-header__title')).toHaveText('Authentication');

  const totpStatus = myProfilePage.locator('.mfa');
  await expect(totpStatus.locator('ion-toggle')).toBeHidden();

  await expect(totpStatus).toBeVisible();
  await expect(totpStatus.locator('ion-button')).toBeVisible();
  await expect(totpStatus.locator('ion-button')).toHaveText('Enable MFA');
  const modal = myProfilePage.locator('.activate-totp-modal');
  await expect(modal).toBeHidden();
  await totpStatus.locator('ion-button').click();
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

  await expect(nextButton).toBeTrulyDisabled();
  await fillIonInput(firstPage.locator('ion-input'), 'P@ssw0rd.');
  await expect(nextButton).toBeTrulyEnabled();
  await nextButton.click();

  await expect(firstPage).toBeHidden();
  await expect(secondPage).toBeVisible();
  await expect(thirdPage).toBeHidden();
  await expect(fourthPage).toBeHidden();
  await expect(modal.locator('.ms-modal-header')).toHaveText('Multi-Factor Authentication (MFA)');
  await expect(secondPage.locator('.step-code-qrcode')).toBeVisible();
  await expect(secondPage.locator('.step-code-copy-text')).toHaveText(/^[A-Z0-9]{32}$/);
  await expect(secondPage.locator('.step-code-copy-button')).toHaveText('Copy key', { useInnerText: true });

  await setWriteClipboardPermission(myProfilePage.context(), true);
  await secondPage.locator('.step-code-copy-button').click();
  const totpSecret = await getClipboardText(myProfilePage);

  await nextButton.click();

  await expect(firstPage).toBeHidden();
  await expect(secondPage).toBeHidden();
  await expect(thirdPage).toBeVisible();
  await expect(fourthPage).toBeHidden();

  // Incorrect code first
  await expect(nextButton).toBeTrulyDisabled();
  await fillIonInput(thirdPage.locator('ion-input'), 'ABCDEF');
  await expect(nextButton).toBeTrulyEnabled();
  await nextButton.click();

  await expect(thirdPage).toBeVisible();
  await expect(modal.locator('.container-textinfo')).toBeVisible();
  await expect(modal.locator('.container-textinfo')).toHaveText('The code you entered is invalid. Please try again.');

  const totpCode = await generateTotpCode(totpSecret);

  expect(totpCode).toMatch(/^\d{6}$/);
  await fillIonInput(thirdPage.locator('ion-input'), totpCode);
  await expect(nextButton).toBeTrulyEnabled();
  await expect(modal.locator('.container-textinfo')).toBeHidden();
  await nextButton.click();
  await expect(firstPage).toBeHidden();
  await expect(secondPage).toBeHidden();
  await expect(thirdPage).toBeHidden();
  await expect(fourthPage).toBeVisible();
  await expect(nextButton).toHaveText('Done');
  await nextButton.click();
  await expect(modal).toBeHidden();
  await expect(myProfilePage).toShowToast(
    'Multi-factor authentication has been successfully enabled and will now be required to log in to this organization.',
    'Success',
  );
  await expect(totpStatus.locator('ion-toggle')).toBeVisible();
  await expect(totpStatus.locator('ion-toggle')).toHaveAttribute('aria-checked', 'true');

  await logout(myProfilePage);

  await myProfilePage.locator('.organization-card').first().click();
  await expect(myProfilePage.locator('#password-input')).toBeVisible();

  await expect(myProfilePage.locator('.login-button')).toHaveDisabledAttribute();

  await myProfilePage.locator('#password-input').locator('input').fill('P@ssw0rd.');
  await expect(myProfilePage.locator('.login-button')).toBeEnabled();
  await myProfilePage.locator('.login-button').click();

  await fillInputModal(myProfilePage, 'ABCDEF');
  await expect(myProfilePage).toShowToast('The code you entered is invalid. Please try again.', 'Error');
  await expect(myProfilePage).toBeHomePage();

  await expect(myProfilePage.locator('.login-button')).toBeEnabled();
  await myProfilePage.locator('.login-button').click();
  await fillInputModal(myProfilePage, await generateTotpCode(totpSecret));
  await expect(myProfilePage.locator('#connected-header')).toContainText('My workspaces');
  await expect(myProfilePage).toBeWorkspacePage();
});

msTest('Reset totp', async ({ myProfilePage }) => {
  await expect(myProfilePage.locator('.menu-list__item').nth(2)).toHaveText('Authentication');
  await myProfilePage.locator('.menu-list__item').nth(2).click();
  await expect(myProfilePage.locator('.profile-content-item').locator('.item-header__title')).toHaveText('Authentication');
  // Whoopsie, we forgot to save the secret
  await setupTotp(myProfilePage);
  const orgName = (await myProfilePage.locator('.large-display-menu-container').locator('.organization-text').textContent()) || '';
  expect(orgName).toBeTruthy();
  const email = (await myProfilePage.locator('.profile-info-card__email').textContent()) || '';
  expect(email).toBeTruthy();

  await logout(myProfilePage);
  const page = myProfilePage;
  const resetLink = await resetTotp(getServerHttpAddr(), orgName, email);
  await page.locator('#create-organization-button').click();

  await expect(page.locator('.homepage-popover')).toBeVisible();
  await page.locator('.homepage-popover').getByRole('listitem').nth(1).click();
  await fillInputModal(page, resetLink);
  const resetModal = page.locator('.mfa-activation-modal');
  await expect(resetModal).toBeVisible();

  const firstPage = resetModal.locator('.modal-content-information');
  const secondPage = resetModal.locator('.modal-content-enter-code');
  const thirdPage = resetModal.locator('.modal-content-finalization');
  const nextButton = resetModal.locator('#next-button');

  await expect(firstPage).toBeVisible();
  await expect(secondPage).toBeHidden();
  await expect(thirdPage).toBeHidden();

  await expect(resetModal.locator('.ms-modal-header')).toHaveText('Multi-Factor Authentication (MFA)');
  await expect(firstPage.locator('.step-code-qrcode')).toBeVisible();
  await expect(firstPage.locator('.step-code-copy-text')).toHaveText(/^[A-Z0-9]{32}$/);
  await expect(firstPage.locator('.step-code-copy-button')).toHaveText('Copy key', { useInnerText: true });

  await setWriteClipboardPermission(page.context(), true);
  await firstPage.locator('.step-code-copy-button').click();
  const totpSecret = await getClipboardText(page);

  await nextButton.click();

  await expect(firstPage).toBeHidden();
  await expect(secondPage).toBeVisible();
  await expect(thirdPage).toBeHidden();

  const totpCode = await generateTotpCode(totpSecret);

  expect(totpCode).toMatch(/^\d{6}$/);

  await expect(nextButton).toBeTrulyDisabled();
  await fillIonInput(secondPage.locator('ion-input'), totpCode);
  await expect(nextButton).toBeTrulyEnabled();
  await nextButton.click();
  await expect(firstPage).toBeHidden();
  await expect(secondPage).toBeHidden();
  await expect(thirdPage).toBeVisible();
  await expect(nextButton).toHaveText('Done');
  await nextButton.click();
  await expect(resetModal).toBeHidden();

  await page.locator('.organization-card').first().click();
  await expect(page.locator('#password-input')).toBeVisible();

  await expect(page.locator('.login-button')).toHaveDisabledAttribute();

  await page.locator('#password-input').locator('input').fill('P@ssw0rd.');
  await expect(page.locator('.login-button')).toBeEnabled();
  await page.locator('.login-button').click();
  await fillInputModal(page, await generateTotpCode(totpSecret));
  await expect(page.locator('#connected-header')).toContainText('My workspaces');
  await expect(page).toBeWorkspacePage();
});

msTest('Change auth with TOTP active', async ({ myProfilePage }) => {
  await expect(myProfilePage.locator('.menu-list__item').nth(2)).toHaveText('Authentication');
  await myProfilePage.locator('.menu-list__item').nth(2).click();
  await expect(myProfilePage.locator('.profile-content-item').locator('.item-header__title')).toHaveText('Authentication');
  const totpSecret = await setupTotp(myProfilePage);

  await myProfilePage.locator('#change-authentication-button').click();
  const changePasswordModal = myProfilePage.locator('.change-authentication-modal');
  await expect(changePasswordModal).toBeVisible();
  await expect(changePasswordModal.locator('.modal-header')).toHaveText('Enter your current password');
  await expect(changePasswordModal.locator('ion-footer').locator('#next-button')).toBeTrulyDisabled();
  const currentPasswordContainer = changePasswordModal.locator('.input-container').nth(0);
  const nextButton = changePasswordModal.locator('#next-button');

  await fillIonInput(currentPasswordContainer.locator('ion-input'), 'P@ssw0rd.');
  await nextButton.click();

  await expect(changePasswordModal.locator('.modal-header')).toHaveText('Multi-factor authentication code');
  await fillIonInput(changePasswordModal.locator('ion-input').nth(1), await generateTotpCode(totpSecret));
  await expect(nextButton).toBeTrulyEnabled();
  await nextButton.click();

  const authRadio = changePasswordModal.locator('.radio-list-item:visible');
  await expect(authRadio).toHaveAuthentication({ keyringDisabled: true, pkiDisabled: true });
  await authRadio.nth(3).click();

  await expect(nextButton).toHaveText('Update');
  await expect(nextButton).toHaveDisabledAttribute();
  await changePasswordModal.locator('.proconnect-button').click();
  await expect(nextButton).toNotHaveDisabledAttribute();
  await nextButton.click();
  await expect(changePasswordModal).toBeHidden();
  await expect(myProfilePage).toShowToast('Authentication has been updated.', 'Success');

  await logout(myProfilePage);

  // Alias for clarity
  const home = myProfilePage;

  await home.locator('.organization-list').locator('.organization-card').nth(0).click();
  await home.locator('.login-card').locator('.proconnect-button').click();
  await fillInputModal(home, await generateTotpCode(totpSecret));
  await expect(home).toBeWorkspacePage();
  await expect(home).toHaveHeader(['My workspaces'], false, false);
});

msTest('Remove and re-add TOTP', async ({ myProfilePage }) => {
  await expect(myProfilePage.locator('.menu-list__item').nth(2)).toHaveText('Authentication');
  await myProfilePage.locator('.menu-list__item').nth(2).click();
  await expect(myProfilePage.locator('.profile-content-item').locator('.item-header__title')).toHaveText('Authentication');
  const totp = myProfilePage.locator('.mfa');
  await expect(totp.locator('ion-toggle')).toBeHidden();

  const totpSecret = await setupTotp(myProfilePage);

  await expect(totp.locator('ion-toggle')).toBeVisible();
  await expect(totp.locator('ion-toggle')).toHaveAttribute('aria-checked', 'true');

  const removeTotpModal = myProfilePage.locator('.delete-totp');
  await expect(removeTotpModal).toBeHidden();
  await totp.locator('ion-toggle').click();
  await expect(removeTotpModal).toBeVisible();
  await expect(removeTotpModal.locator('.ms-modal-header__title')).toHaveText('Disable multi-factor authentication on this device');
  await expect(removeTotpModal.locator('.authentication-section')).toBeVisible();
  await expect(removeTotpModal.locator('.totp-section')).toBeHidden();
  await fillIonInput(removeTotpModal.locator('.authentication-section').locator('ion-input'), 'P@ssw0rd.');
  await removeTotpModal.locator('#next-button').click();
  await expect(removeTotpModal.locator('.authentication-section')).toBeHidden();
  await expect(removeTotpModal.locator('.totp-section')).toBeVisible();
  await fillIonInput(removeTotpModal.locator('.totp-section').locator('ion-input'), await generateTotpCode(totpSecret));
  await expect(removeTotpModal.locator('#next-button')).toHaveText('Delete MFA');
  await removeTotpModal.locator('#next-button').click();
  await expect(myProfilePage).toShowToast('Multi-factor authentication has been removed', 'Success');
  await expect(removeTotpModal).toBeHidden();
  await expect(totp.locator('ion-toggle')).toHaveAttribute('aria-checked', 'false');
  await logout(myProfilePage);
  // Alias for clarity
  const home = myProfilePage;

  await home.locator('.organization-card').first().click();
  await expect(home.locator('#password-input')).toBeVisible();

  await expect(home.locator('.login-button')).toHaveDisabledAttribute();

  await home.locator('#password-input').locator('input').fill('P@ssw0rd.');
  await expect(home.locator('.login-button')).toBeEnabled();
  await home.locator('.login-button').click();
  await expect(home.locator('#connected-header')).toContainText('My workspaces');
  await expect(home).toBeWorkspacePage();

  await myProfilePage.locator('.topbar').locator('.profile-header').click();
  const myProfileButton = myProfilePage.locator('.profile-header-organization-popover').locator('.main-list').getByRole('listitem').nth(0);
  await expect(myProfileButton).toHaveText('Settings');
  await myProfileButton.click();
  await expect(myProfilePage).toHavePageTitle('My profile');
  await expect(myProfilePage).toBeMyProfilePage();

  await expect(myProfilePage.locator('.menu-list__item').nth(2)).toHaveText('Authentication');
  await myProfilePage.locator('.menu-list__item').nth(2).click();
  await expect(myProfilePage.locator('.profile-content-item').locator('.item-header__title')).toHaveText('Authentication');
  await expect(totp.locator('ion-toggle')).toHaveAttribute('aria-checked', 'false');
  const activateTotpModal = myProfilePage.locator('.activate-totp-modal');
  await expect(activateTotpModal).toBeHidden();
  await totp.locator('ion-toggle').click();
  await expect(activateTotpModal).toBeVisible();
  await expect(activateTotpModal.locator('.prompt-authentication')).toBeVisible();
  await expect(activateTotpModal.locator('.enter-code')).toBeHidden();
  await fillIonInput(activateTotpModal.locator('.prompt-authentication').locator('ion-input'), 'P@ssw0rd.');
  await activateTotpModal.locator('#next-button').click();
  await expect(activateTotpModal.locator('.prompt-authentication')).toBeHidden();
  await expect(activateTotpModal.locator('.enter-code')).toBeVisible();
  await fillIonInput(activateTotpModal.locator('.enter-code').locator('ion-input'), await generateTotpCode(totpSecret));
  await expect(activateTotpModal.locator('#next-button')).toHaveText('Next');
  await activateTotpModal.locator('#next-button').click();
  await expect(activateTotpModal.locator('.modal-content-finalization')).toBeVisible();
  await expect(activateTotpModal.locator('.modal-content-finalization').locator('.container-textinfo')).toHaveText(
    'Your authentication method has been successfully updated.',
  );
  await activateTotpModal.locator('#next-button').click();
  await expect(myProfilePage).toShowToast(
    'Multi-factor authentication has been successfully enabled and will now be required to log in to this organization.',
    'Success',
  );
  await expect(activateTotpModal).toBeHidden();
  await expect(totp.locator('ion-toggle')).toHaveAttribute('aria-checked', 'true');
  await logout(myProfilePage);

  await home.locator('.organization-card').first().click();
  await expect(home.locator('#password-input')).toBeVisible();

  await expect(home.locator('.login-button')).toHaveDisabledAttribute();

  await home.locator('#password-input').locator('input').fill('P@ssw0rd.');
  await expect(home.locator('.login-button')).toBeEnabled();
  await home.locator('.login-button').click();
  await fillInputModal(home, await generateTotpCode(totpSecret));
  await expect(home.locator('#connected-header')).toContainText('My workspaces');
  await expect(home).toBeWorkspacePage();
});
