// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { Page } from '@playwright/test';
import { expect, fillIonInput, msTest, selectDropdown } from '@tests/e2e/helpers';

msTest('Check devices list', async ({ myProfilePage }) => {
  await expect(myProfilePage.locator('.menu-list__item').nth(1)).toHaveText('My devices');
  await myProfilePage.locator('.menu-list__item').nth(1).click();
  await expect(myProfilePage.locator('#add-device-button')).toHaveText('Add a new device');
  const devices = myProfilePage.locator('#devices-list').getByRole('listitem');
  await expect(devices.locator('.device-name')).toHaveText(['My alice@dev1 machine', 'My alice@dev2 machine']);
  await expect(devices.locator('.join-date')).toHaveText(['Joined: Jan 2, 2000', 'Joined: Jan 4, 2000']);
  await expect(devices.locator('.label-id')).toHaveText([/^(Technical ID: )[a-f0-9-]+$/, /^(Technical ID: )[a-f0-9-]+$/]);
  await expect(devices.nth(0).locator('.badge')).toBeHidden();
  await expect(devices.nth(1).locator('.badge')).toBeVisible();
});

async function checkMenuItem(
  myProfilePage: Page,
  index: number,
  expectedText: string,
  expectedHeaderText?: string,
  expectedUrlParam?: string,
): Promise<void> {
  await expect(myProfilePage.locator('.menu-list__item').nth(index)).toHaveText(expectedText);
  await myProfilePage.locator('.menu-list__item').nth(index).click();
  if (expectedHeaderText) {
    await expect(myProfilePage.locator('.item-header__title')).toHaveText(expectedHeaderText);
  }
  if (expectedUrlParam) {
    await expect(myProfilePage).toHaveURL(new RegExp(`\\?profilePage=${expectedUrlParam}`));
  }
}

msTest('Check if each tab is displayed', async ({ myProfilePage }) => {
  await expect(myProfilePage.locator('.profile-menu').locator('.menu-list__item')).toHaveCount(7);
  await expect(myProfilePage.locator('.menu-list__title').nth(0)).toHaveText('My account');
  await expect(myProfilePage.locator('.menu-list__title').nth(1)).toHaveText('Support');

  await checkMenuItem(myProfilePage, 0, 'Settings', 'Settings', 'settings');
  await checkMenuItem(myProfilePage, 1, 'My devices', 'My devices', 'devices');
  await checkMenuItem(myProfilePage, 2, 'Authentication', 'Authentication', 'authentication');
  await checkMenuItem(myProfilePage, 3, 'Organization recovery', 'Organization recovery files', 'recovery');
  await expect(myProfilePage.locator('.item-container__text').nth(5)).toHaveText('Feedback');
  await expect(myProfilePage.locator('.item-container__text').nth(6)).toHaveText('About');
  await checkMenuItem(myProfilePage, 6, 'About', 'About', 'about');
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

  await myProfilePage.locator('#change-authentication-button').click();
  await fillIonInput(currentPasswordContainer.locator('ion-input'), 'P@ssw0rd.');
  await expect(changePasswordModal.locator('#next-button')).toBeTrulyEnabled();
  await changePasswordModal.locator('#next-button').click();
  await expect(currentPasswordContainer.locator('.form-error')).toBeVisible();
  await expect(currentPasswordContainer.locator('.form-error')).toHaveText('Wrong password. Please try again.');
});

msTest('Create recovery files', async ({ myProfilePage }) => {
  await expect(myProfilePage.locator('.menu-list__item').nth(3)).toHaveText('Organization recovery');
  await myProfilePage.locator('.menu-list__item').nth(3).click();
  const restorePassword = myProfilePage.locator('.recovery');
  await expect(restorePassword).toBeVisible();
  await expect(restorePassword.locator('.item-header__title')).toHaveText('Organization recovery files');
  await expect(restorePassword.locator('.done')).toBeHidden();
  await expect(restorePassword.locator('.danger')).toBeVisible();
  await expect(restorePassword.locator('.danger')).toHaveText('Action required');
  await expect(restorePassword.locator('.restore-password-button')).toHaveText('Create a recovery file');
  await restorePassword.locator('.restore-password-button').locator('ion-button').click();
  await expect(restorePassword.locator('.recovery-item-download').nth(0)).toBeVisible();
  await expect(restorePassword.locator('.recovery-item-download').nth(1)).toBeVisible();

  // Download recovery file
  await expect(restorePassword.locator('.recovery-item-download__button').nth(0)).toHaveText('Download');
  await expect(restorePassword.locator('.recovery-item-download__downloaded').nth(0)).toBeHidden();
  await restorePassword.locator('.recovery-item-download__button').nth(0).click();
  await expect(restorePassword.locator('.recovery-item-download__button').nth(0)).toHaveText('Download again');
  await expect(restorePassword.locator('.recovery-item-download__downloaded').nth(0)).toBeVisible();

  // Download secret key
  await expect(restorePassword.locator('.recovery-item-download__button').nth(1)).toHaveText('Download');
  await expect(restorePassword.locator('.recovery-item-download__downloaded').nth(1)).toBeHidden();
  await restorePassword.locator('.recovery-item-download__button').nth(1).click();
  await expect(restorePassword.locator('.recovery-item-download__button').nth(1)).toHaveText('Download again');
  await expect(restorePassword.locator('.recovery-item-download__downloaded').nth(1)).toBeVisible();

  // Switch page to reload and check chip text
  await myProfilePage.locator('.menu-list__item').nth(2).click();
  await myProfilePage.locator('.menu-list__item').nth(3).click();
  await expect(restorePassword.locator('.danger')).toBeHidden();
  await expect(restorePassword.locator('.done')).toBeVisible();
  await expect(restorePassword.locator('.done')).toHaveText('Recovery file already created');
});

msTest('Check settings section', async ({ myProfilePage }) => {
  await checkMenuItem(myProfilePage, 0, 'Settings', 'Settings');
  const settings = myProfilePage.locator('.settings-list-container');
  const options = settings.locator('.settings-option');
  await expect(options).toHaveCount(4);
  const lang = options.nth(0);
  await expect(lang.locator('.settings-option__content').locator('.title')).toHaveText('Language');
  await expect(lang.locator('.settings-option__content').locator('.description')).toHaveText('Choose application language');
  await expect(lang.locator('.dropdown-container')).toHaveText('English');
  await expect(lang.locator('.settings-option__content').locator('.title')).toHaveText('Language');
  await expect(lang.locator('.settings-option__content').locator('.description')).toHaveText('Choose application language');
  await expect(lang.locator('.dropdown-container').locator('.input-text')).toHaveText('English');
  await selectDropdown(lang.locator('ion-button'), 'Français', 'English');
  await expect(lang.locator('.dropdown-container').locator('.input-text')).toHaveText('Français');
  const theme = options.nth(1);
  // Now we're in French
  await expect(theme.locator('.settings-option__content').locator('.title')).toHaveText('Thème');
  await expect(theme.locator('.settings-option__content').locator('.description')).toHaveText("Choisir l'apparence de l'application");
  await expect(theme.locator('.dropdown-container')).toHaveText('Clair');

  const telemetry = options.nth(2);
  await expect(telemetry.locator('.settings-option__content').locator('.title')).toHaveText("Envoyer les rapports d'erreur");
  await expect(telemetry.locator('.settings-option__content').locator('.description')).toHaveText(
    "Les rapports d'erreur aident à améliorer Parsec",
  );
});

msTest('Open documentation', async ({ myProfilePage }) => {
  await expect(myProfilePage.locator('.item-container__text').nth(4)).toHaveText('Documentation');
  await myProfilePage.locator('.item-container__text').nth(4).click();
  const newTab = await myProfilePage.waitForEvent('popup');
  await newTab.waitForLoadState();
  await expect(newTab).toHaveURL(new RegExp('https://docs.parsec.cloud/(en|fr)/[a-z0-9-+.]+'));
});

msTest('Open feedback', async ({ myProfilePage }) => {
  await expect(myProfilePage.locator('.item-container__text').nth(5)).toHaveText('Feedback');
  await myProfilePage.locator('.item-container__text').nth(5).click();
  const newTab = await myProfilePage.waitForEvent('popup');
  await newTab.waitForLoadState();
  await expect(newTab).toHaveURL(new RegExp('https://sign(-dev)?.parsec.cloud/contact'));
});
