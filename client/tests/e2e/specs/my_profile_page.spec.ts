// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { Page } from '@playwright/test';
import { DisplaySize, expect, fillIonInput, logout, msTest, openExternalLink, selectDropdown } from '@tests/e2e/helpers';

msTest('Check devices list', async ({ myProfilePage }) => {
  await expect(myProfilePage.locator('.menu-list__item').nth(1)).toHaveText('My devices');
  await myProfilePage.locator('.menu-list__item').nth(1).click();
  await expect(myProfilePage.locator('#add-device-button')).toHaveText('Add a new device');
  const devices = myProfilePage.locator('#devices-list').getByRole('listitem');
  await expect(devices.locator('.device-name')).toHaveText(['My alice@dev2 machine', 'My alice@dev1 machine']);
  await expect(devices.locator('.join-date')).toHaveText(['Joined: Jan 4, 2000', 'Joined: Jan 2, 2000']);
  await expect(devices.locator('.label-id')).toHaveText([/^Internal ID: [a-f0-9]+$/, /^Internal ID: [a-f0-9]+$/]);
  await expect(devices.nth(1).locator('.badge')).toBeHidden();
  await expect(devices.nth(0).locator('.badge')).toBeVisible();
  await expect(devices.nth(0).locator('.badge')).toHaveText('Current');
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
  await expect(myProfilePage.locator('.menu-content__title').nth(0)).toHaveText('My account');
  await expect(myProfilePage.locator('.menu-content__title').nth(1)).toHaveText('Support');

  await checkMenuItem(myProfilePage, 0, 'Settings');
  await checkMenuItem(myProfilePage, 1, 'My devices', 'My devices', 'devices');
  await checkMenuItem(myProfilePage, 2, 'Authentication', 'Authentication', 'authentication');
  await checkMenuItem(myProfilePage, 3, 'Recovery files', 'Organization recovery files', 'recovery');
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
  await expect(myProfilePage.locator('.menu-list__item').nth(3)).toHaveText('Recovery files');
  await myProfilePage.locator('.menu-list__item').nth(3).click();
  const restorePassword = myProfilePage.locator('.recovery');
  await expect(restorePassword).toBeVisible();
  await expect(restorePassword.locator('.item-header__title')).toHaveText('Organization recovery files');
  await expect(restorePassword.locator('.restore-password__description span').nth(0)).toHaveText(
    'Recovery files allow you to regain access to this organization if you forgot your password or lose your devices.',
  );
  await expect(restorePassword.locator('.restore-password__description span').nth(1)).toHaveText(
    `Without recovery files, you will not be able to recover your account in such a case,
  and you will need to be re-invited to the organization.`,
  );
  await expect(restorePassword.locator('.restore-password__description span').nth(2)).toHaveText(
    `They consist of a recovery file and a recovery key. You need to save both in a secure place,
    if possible in separate places, as both are needed to recover a device's access to an organization.`,
  );
  await expect(restorePassword.locator('.restore-password-button')).toHaveText('Create recovery files');
});

msTest('Change password', async ({ myProfilePage }) => {
  const newPassword = 'New-P@ssw0rd.6786?6786';
  await expect(myProfilePage.locator('.menu-list__item').nth(2)).toHaveText('Authentication');
  await myProfilePage.locator('.menu-list__item').nth(2).click();
  await expect(myProfilePage.locator('.profile-content-item').locator('.item-header__title')).toHaveText('Authentication');
  await myProfilePage.locator('#change-authentication-button').click();
  const changePasswordModal = myProfilePage.locator('.change-authentication-modal');
  await expect(changePasswordModal).toBeVisible();
  await expect(changePasswordModal.locator('.modal-header')).toHaveText('Enter your current password');
  await expect(changePasswordModal.locator('ion-footer').locator('#next-button')).toBeTrulyDisabled();
  const currentPasswordContainer = changePasswordModal.locator('.input-container').nth(0);
  await fillIonInput(currentPasswordContainer.locator('ion-input'), 'InvalidP@ssw0rd.');
  await expect(changePasswordModal.locator('#next-button')).toBeTrulyEnabled();
  await changePasswordModal.locator('#next-button').click();
  await expect(currentPasswordContainer.locator('.form-error')).toBeVisible();
  await expect(currentPasswordContainer.locator('.form-error')).toHaveText('Invalid authentication.');

  await fillIonInput(currentPasswordContainer.locator('ion-input'), 'P@ssw0rd.');
  await changePasswordModal.locator('#next-button').click();

  await expect(changePasswordModal.locator('.modal-header')).toHaveText('Change authentication method');
  await expect(changePasswordModal.locator('#next-button')).toHaveDisabledAttribute();

  const authRadio = changePasswordModal.locator('.radio-list-item:visible');
  await expect(authRadio).toHaveAuthentication({ pkiDisabled: true, keyringDisabled: true });
  await authRadio.nth(0).click();

  await expect(changePasswordModal.locator('.method-chosen').locator('.authentication-card__update-button')).toHaveText('Update');
  await changePasswordModal.locator('.method-chosen').locator('.authentication-card__update-button').click();
  await expect(changePasswordModal.locator('.method-chosen')).toBeHidden();
  await changePasswordModal.locator('.authentication-card').nth(0).click();
  const passwordInputs = changePasswordModal.locator('.input-container').locator('ion-input');
  await fillIonInput(passwordInputs.nth(1), newPassword);
  await fillIonInput(passwordInputs.nth(2), 'no match');
  await expect(changePasswordModal.locator('.inputs-container-item').nth(1).getByText('Do not match')).toBeVisible();
  await expect(changePasswordModal.locator('#next-button')).toHaveDisabledAttribute();
  await fillIonInput(passwordInputs.nth(2), newPassword);
  await expect(changePasswordModal.locator('#next-button')).toNotHaveDisabledAttribute();
  await changePasswordModal.locator('#next-button').click();
  await expect(changePasswordModal).toBeHidden();
  await expect(myProfilePage).toShowToast('Authentication has been updated.', 'Success');

  await logout(myProfilePage);

  // Alias for clarity
  const home = myProfilePage;

  await home.locator('.organization-list').locator('.organization-card').nth(0).click();
  const loginButton = home.locator('.login-card-footer').locator('.login-button');
  await expect(loginButton).toHaveDisabledAttribute();
  await expect(home.locator('#password-input').locator('.form-error')).toBeHidden();
  const input = home.locator('#password-input').locator('ion-input');
  // Original password should fail
  await fillIonInput(input, 'P@ssw0rd.');
  await expect(loginButton).toNotHaveDisabledAttribute();
  await loginButton.click();
  const error = home.locator('#password-input').locator('.form-error');
  await expect(error).toBeVisible();
  await expect(error).toHaveText('Incorrect password.');
  await expect(home).toBeHomePage();

  // Now with the new password
  await fillIonInput(input, newPassword);
  await expect(loginButton).toNotHaveDisabledAttribute();
  await loginButton.click();
  await expect(home).toBeWorkspacePage();
  await expect(home).toHaveHeader(['My workspaces'], false, false);
  const profile = home.locator('.topbar').locator('.profile-header');
  await expect(profile.locator('.text-content-name')).toHaveText('Alicey McAliceFace');
});

msTest('Change auth to/from openbao', async ({ myProfilePage }) => {
  await expect(myProfilePage.locator('.menu-list__item').nth(2)).toHaveText('Authentication');
  await myProfilePage.locator('.menu-list__item').nth(2).click();
  await expect(myProfilePage.locator('.profile-content-item').locator('.item-header__title')).toHaveText('Authentication');
  await myProfilePage.locator('#change-authentication-button').click();
  const changePasswordModal = myProfilePage.locator('.change-authentication-modal');
  await expect(changePasswordModal).toBeVisible();
  await expect(changePasswordModal.locator('.modal-header')).toHaveText('Enter your current password');
  await expect(changePasswordModal.locator('ion-footer').locator('#next-button')).toBeTrulyDisabled();
  const currentPasswordContainer = changePasswordModal.locator('.input-container').nth(0);

  await fillIonInput(currentPasswordContainer.locator('ion-input'), 'P@ssw0rd.');
  await changePasswordModal.locator('#next-button').click();

  await expect(changePasswordModal.locator('.modal-header')).toHaveText('Change authentication method');
  await expect(changePasswordModal.locator('#next-button')).toHaveDisabledAttribute();

  const authRadio = changePasswordModal.locator('.radio-list-item:visible');
  await expect(authRadio).toHaveAuthentication({ keyringDisabled: true, pkiDisabled: true });
  await authRadio.nth(3).click();

  await expect(changePasswordModal.locator('#next-button')).toHaveText('Update');
  await expect(changePasswordModal.locator('#next-button')).toHaveDisabledAttribute();
  await changePasswordModal.locator('.proconnect-button').click();
  await expect(changePasswordModal.locator('#next-button')).toNotHaveDisabledAttribute();
  await changePasswordModal.locator('#next-button').click();
  await expect(changePasswordModal).toBeHidden();
  await expect(myProfilePage).toShowToast('Authentication has been updated.', 'Success');

  await logout(myProfilePage);

  // Alias for clarity
  const home = myProfilePage;

  await home.locator('.organization-list').locator('.organization-card').nth(0).click();
  await home.locator('.login-card').locator('.proconnect-button').click();
  await expect(home).toBeWorkspacePage();
  await expect(home).toHaveHeader(['My workspaces'], false, false);
  const profile = home.locator('.topbar').locator('.profile-header');
  await expect(profile.locator('.text-content-name')).toHaveText('Alicey McAliceFace');
  await profile.click();
  const myProfileButton = home.locator('.profile-header-organization-popover').locator('.main-list').getByRole('listitem').nth(2);
  await expect(myProfileButton).toHaveText('Authentication');
  await myProfileButton.click();
  await expect(myProfilePage.locator('.profile-content-item').locator('.item-header__title')).toHaveText('Authentication');

  await myProfilePage.locator('#change-authentication-button').click();
});

msTest('Check settings section', async ({ myProfilePage }) => {
  await checkMenuItem(myProfilePage, 0, 'Settings');
  await expect(myProfilePage.locator('.profile-content-item').nth(0).locator('.settings-list-group__title')).toHaveText([
    'Display',
    'Configuration',
  ]);
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
  await expect(myProfilePage.locator('.profile-content-item').nth(0).locator('.settings-list-group__title')).toHaveText([
    'Affichage',
    'Configuration',
  ]);
  await expect(theme.locator('.settings-option__content').locator('.title')).toHaveText('Thème');
  await expect(theme.locator('.settings-option__content').locator('.description')).toHaveText("Choisir l'apparence de l'application");
  await expect(theme.locator('.dropdown-container')).toHaveText('Clair');

  const telemetry = options.nth(2);
  await expect(telemetry.locator('.settings-option__content').locator('.title')).toHaveText("Envoyer les rapports d'erreur");
  await expect(telemetry.locator('.settings-option__content').locator('.description')).toHaveText(
    "Les rapports d'erreur aident à améliorer Parsec",
  );

  const logs = options.nth(3);
  await expect(logs.locator('.settings-option__content').locator('.title')).toHaveText('Logs');
  await expect(logs.locator('.settings-option__content').locator('.description')).toHaveText("Afficher les logs de l'application");
});

msTest('Open documentation', async ({ myProfilePage }) => {
  await expect(myProfilePage.locator('.item-container__text').nth(4)).toHaveText('Documentation');
  await openExternalLink(
    myProfilePage,
    myProfilePage.locator('.item-container__text').nth(4),
    new RegExp('https://docs.parsec.cloud/(en|fr)/[a-z0-9-+.]+'),
  );
  // Re-check, just to for it to wait
  await expect(myProfilePage.locator('.item-container__text').nth(4)).toHaveText('Documentation');
});

msTest('Open feedback', async ({ myProfilePage }) => {
  await expect(myProfilePage.locator('.item-container__text').nth(5)).toHaveText('Feedback');

  await openExternalLink(
    myProfilePage,
    myProfilePage.locator('.item-container__text').nth(5),
    new RegExp('https://sign(-dev)?.parsec.cloud/contact'),
  );
  await expect(myProfilePage.locator('.item-container__text').nth(5)).toHaveText('Feedback');
});

msTest('Logout from my profile page', async ({ myProfilePage }) => {
  // Verify the "Logout" button is present
  const logoutButton = myProfilePage.locator('.logout');
  await expect(logoutButton).toBeVisible();
  await expect(logoutButton.locator('.logout-text')).toHaveText('Log out');

  await expect(logoutButton).toHaveText('Log out');
  await logoutButton.click();
});

msTest('Profile page back button', async ({ workspaces }) => {
  await workspaces.locator('.topbar').locator('.profile-header').click();
  const myProfileButton = workspaces.locator('.profile-header-organization-popover').locator('.main-list').getByRole('listitem').nth(0);
  await myProfileButton.click();
  await expect(workspaces).toHavePageTitle('My profile');
  await expect(workspaces).toBeMyProfilePage();
  // Navigating into sub-menus, should not add anything to the history
  const profile = workspaces.locator('.profile-page-container');
  await profile.locator('.menu-list__item').nth(1).click();
  await profile.locator('.menu-list__item').nth(2).click();
  await profile.locator('.menu-list__item').nth(3).click();
  await profile.locator('.menu-list__item').nth(1).click();

  await expect(workspaces).toHavePageTitle('My profile');
  await expect(workspaces).toBeMyProfilePage();

  const backButton = workspaces.locator('.topbar-left').locator('.back-button');
  await expect(backButton).toBeVisible();
  await backButton.click();
  await expect(workspaces.locator('#connected-header')).toContainText('My workspaces');
  await expect(workspaces).toBeWorkspacePage();
});

msTest('Open modal to greet user from ProfilePage in small display', async ({ usersPage }) => {
  await usersPage.setDisplaySize(DisplaySize.Small);

  await usersPage.locator('.tab-bar-menu').locator('.tab-bar-menu-button').nth(3).click();
  await usersPage.locator('.tab-bar-menu').locator('#add-menu-fab-button').click();
  await usersPage.locator('.list-group-item').nth(0).click();
  await expect(usersPage.locator('.modal-sheet')).toBeVisible();
});
