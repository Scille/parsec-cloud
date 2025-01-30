// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { expect, fillIonInput, msTest } from '@tests/e2e/helpers';

msTest('Check devices list', async ({ myProfilePage }) => {
  await expect(myProfilePage.locator('.menu-list__item').nth(1)).toHaveText('My devices');
  await myProfilePage.locator('.menu-list__item').nth(1).click();
  await expect(myProfilePage.locator('#add-device-button')).toHaveText('Add a new device');
  const devices = myProfilePage.locator('#devices-list').getByRole('listitem');
  await expect(devices.locator('.device-name')).toHaveText([/^device\d$/, /^device\d$/]);
  await expect(devices.locator('.join-date')).toHaveText(['Joined: Today', 'Joined: Today']);
  await expect(devices.locator('.label-id')).toHaveText([/^Technical ID: device\d$/, /^Technical ID: device\d$/]);
  await expect(devices.nth(0).locator('.badge')).toBeVisible();
  await expect(devices.nth(1).locator('.badge')).toBeHidden();
});

msTest('Open authentication section', async ({ myProfilePage }) => {
  await expect(myProfilePage.locator('.menu-list__item').nth(2)).toHaveText('Authentication');
  await myProfilePage.locator('.menu-list__item').nth(2).click();
  await expect(myProfilePage.locator('.profile-content-item').locator('.item-header__title')).toHaveText('Authentication');
  await expect(myProfilePage.locator('#change-authentication-button')).toBeVisible();
});

msTest('Check if restore-password section is displayed', async ({ myProfilePage }) => {
  await expect(myProfilePage.locator('.menu-list__item').nth(3)).toHaveText('Organization recovery');
  await myProfilePage.locator('.menu-list__item').nth(3).click();
  const restorePassword = myProfilePage.locator('.recovery');
  await expect(restorePassword).toBeVisible();
  await expect(restorePassword.locator('.item-header__title')).toHaveText('Organization recovery files');
  await expect(restorePassword.locator('.item-header__description span').nth(0)).toHaveText(
    'A recovery file enables you to reclaim access to your data if you forget your password or lose your devices.',
  );
  await expect(restorePassword.locator('.item-header__description span').nth(1)).toHaveText(
    `Without a recovery file, you wouldn't be able to recover your account in such a case,
  and you would need to be re-invited to the organization to regain access to your data.`,
  );
  await expect(restorePassword.locator('.restore-password-button')).toHaveText('Create a recovery file');
});

msTest('Change password', async ({ home, myProfilePage }) => {
  await expect(myProfilePage.locator('.menu-list__item').nth(2)).toHaveText('Authentication');
  await myProfilePage.locator('.menu-list__item').nth(2).click();
  await expect(myProfilePage.locator('.profile-content-item').locator('.item-header__title')).toHaveText('Authentication');
  await myProfilePage.locator('#change-authentication-button').click();
  const changePasswordModal = home.locator('.change-authentication-modal');
  await expect(changePasswordModal).toBeVisible();
  await expect(changePasswordModal.locator('.modal-header')).toHaveText('Enter your current password');
  await expect(changePasswordModal.locator('ion-footer').locator('#next-button')).toBeTrulyDisabled();
  const currentPasswordContainer = changePasswordModal.locator('.input-container').nth(0);
  await fillIonInput(currentPasswordContainer.locator('ion-input'), 'InvalidP@ssw0rd.');
  await expect(changePasswordModal.locator('#next-button')).toBeTrulyEnabled();
  await changePasswordModal.locator('#next-button').click();
  await expect(currentPasswordContainer.locator('.form-error')).toBeVisible();
  await expect(currentPasswordContainer.locator('.form-error')).toHaveText('Wrong password. Please try again.');

  await fillIonInput(currentPasswordContainer.locator('ion-input'), 'P@ssw0rd.');
  await changePasswordModal.locator('#next-button').click();

  await expect(changePasswordModal.locator('.modal-header')).toHaveText('Choose your new authentication method');
  await expect(changePasswordModal.locator('#next-button')).toHaveDisabledAttribute();
  const passwordInputs = changePasswordModal.locator('.input-container').locator('ion-input');
  await fillIonInput(passwordInputs.nth(1), 'New-P@ssw0rd.6786?6786');
  await fillIonInput(passwordInputs.nth(2), 'New-P@ssw0rd');
  await expect(changePasswordModal.locator('.inputs-container-item').nth(1).getByText('Do not match')).toBeVisible();
  await expect(changePasswordModal.locator('#next-button')).toHaveDisabledAttribute();
  await fillIonInput(passwordInputs.nth(2), 'New-P@ssw0rd.6786?6786');
  await expect(changePasswordModal.locator('#next-button')).toNotHaveDisabledAttribute();
  await changePasswordModal.locator('#next-button').click();
  await expect(changePasswordModal).toBeHidden();
  await expect(myProfilePage).toShowToast('Authentication has been updated.', 'Success');
});
