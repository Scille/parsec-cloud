// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { expect, fillIonInput, msTest } from '@tests/e2e/helpers';

msTest('Check devices list', async ({ myProfilePage }) => {
  await expect(myProfilePage.locator('#add-device-button')).toHaveText('Add');
  const devices = myProfilePage.locator('#devices-list').getByRole('listitem');
  await expect(devices.locator('.device-name')).toHaveText([/^device\d$/, /^device\d$/]);
  await expect(devices.locator('.join-date')).toHaveText(['Joined: Today', 'Joined: Today']);
  await expect(devices.locator('.label-id')).toHaveText([/^Technical ID: device\d$/, /^Technical ID: device\d$/]);
  await expect(devices.nth(0).locator('.badge')).toBeVisible();
  await expect(devices.nth(1).locator('.badge')).toBeHidden();
});

msTest('Check if restore-password section is displayed', async ({ myProfilePage }) => {
  const restorePassword = myProfilePage.locator('.restore-password');
  await expect(restorePassword).toBeVisible();
  await expect(restorePassword.locator('.restore-password-header__title')).toHaveText('Create a recovery file');
  await expect(restorePassword.locator('.restore-password-subtitles')).toHaveText(
    `A recovery file allows you to get back access to your data in case your forgot your password or lose your
 devices.Without a recovery file, your account cannot be recovered and you will need to be re-invited to join the organization.`,
  );
  await expect(restorePassword.locator('.restore-password-button')).toHaveText('Create a recovery file');
});

msTest('Open authentication section', async ({ myProfilePage }) => {
  await myProfilePage.locator('ion-radio').nth(1).click();
  await expect(myProfilePage.locator('.user-info').locator('.title')).toHaveText('Password');
  await expect(myProfilePage.locator('.user-info').locator('.input-container').locator('.user-info__input')).toHaveTheClass(
    'input-disabled',
  );
  await expect(myProfilePage.locator('#change-authentication-button')).toBeVisible();
});

msTest('Change password', async ({ home, myProfilePage }) => {
  await myProfilePage.locator('ion-radio').nth(1).click();
  await myProfilePage.locator('.user-info').locator('#change-authentication-button').click();
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
  await expect(changePasswordModal.locator('#next-button')).not.toHaveDisabledAttribute();
  await changePasswordModal.locator('#next-button').click();
  await expect(changePasswordModal).toBeHidden();
  await expect(myProfilePage).toShowToast('Authentication has been updated.', 'Success');
});
