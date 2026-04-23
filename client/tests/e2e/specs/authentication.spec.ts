// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import {
  AllowAllProtectionMethods,
  AllowedAuthenticationMethod,
  answerQuestion,
  AuthMethodDisabledReason,
  DEFAULT_USER_INFORMATION,
  expect,
  fillIonInput,
  generateTotpCode,
  getClipboardText,
  msTest,
  ProtectionMethod,
  setWriteClipboardPermission,
} from '@tests/e2e/helpers';
import { mockAllowedProtectionMethods } from '@tests/e2e/helpers/mock';

function getAuthAssert(methods: Array<ProtectionMethod>): [
  {
    passwordDisabled?: boolean;
    ssoDisabled?: boolean;
    pkiDisabled?: boolean;
    keyringDisabled?: boolean;
  },
  {
    password?: AuthMethodDisabledReason;
    sso?: AuthMethodDisabledReason;
    pki?: AuthMethodDisabledReason;
    keyring?: AuthMethodDisabledReason;
  },
] {
  return [
    {
      pkiDisabled: true,
      keyringDisabled: true,
      passwordDisabled: methods.find((m) => m.primary === AllowedAuthenticationMethod.Password) === undefined,
      ssoDisabled: methods.find((m) => m.primary === AllowedAuthenticationMethod.OpenBao) === undefined,
    },
    {
      password:
        methods.find((m) => m.primary === AllowedAuthenticationMethod.Password) === undefined
          ? AuthMethodDisabledReason.Forbidden
          : undefined,
      sso:
        methods.find((m) => m.primary === AllowedAuthenticationMethod.OpenBao) === undefined
          ? AuthMethodDisabledReason.Forbidden
          : undefined,
      pki:
        methods.find((m) => m.primary === AllowedAuthenticationMethod.PKI) === undefined ? AuthMethodDisabledReason.Forbidden : undefined,
      keyring:
        methods.find((m) => m.primary === AllowedAuthenticationMethod.Keyring) === undefined
          ? AuthMethodDisabledReason.Forbidden
          : undefined,
    },
  ];
}

for (const [index, authMethods] of [
  AllowAllProtectionMethods,
  [{ primary: AllowedAuthenticationMethod.OpenBao, withTotp: false }],
  [{ primary: AllowedAuthenticationMethod.PKI, withTotp: false }],
].entries()) {
  msTest(`Check allowed auth method at org creation (${index + 1})`, async ({ home }) => {
    await mockAllowedProtectionMethods(home, authMethods);

    await home.locator('#create-organization-button').click();
    await home.locator('.popover-viewport').getByRole('listitem').nth(0).click();
    const modal = home.locator('.create-organization-modal');
    await modal.locator('.server-page-footer').locator('ion-button').nth(0).click();

    const orgServerContainer = modal.locator('.organization-name-and-server-page');
    await expect(orgServerContainer.locator('.modal-header-title__text')).toHaveText('Create organization on my Parsec server');
    const orgPrevious = orgServerContainer.locator('.organization-name-and-server-page-footer').locator('ion-button').nth(0);
    const orgNext = orgServerContainer.locator('.organization-name-and-server-page-footer').locator('ion-button').nth(1);
    await expect(orgPrevious).toBeVisible();
    await expect(orgNext).toHaveDisabledAttribute();
    await fillIonInput(orgServerContainer.locator('ion-input').nth(0), 'BlackMesa');
    await expect(orgNext).toHaveDisabledAttribute();
    await fillIonInput(orgServerContainer.locator('ion-input').nth(1), home.orgInfo.serverAddr);
    await expect(orgNext).toNotHaveDisabledAttribute();
    await orgNext.click();

    const userInfoContainer = modal.locator('.user-information-page');
    const userNext = modal.locator('.user-information-page-footer').locator('ion-button').nth(1);
    await expect(userNext).toHaveDisabledAttribute();
    await expect(userInfoContainer.locator('.modal-header-title__text')).toHaveText('Enter your personal information');
    await fillIonInput(userInfoContainer.locator('ion-input').nth(0), DEFAULT_USER_INFORMATION.name);
    await expect(userNext).toHaveDisabledAttribute();
    await fillIonInput(userInfoContainer.locator('ion-input').nth(1), DEFAULT_USER_INFORMATION.email);
    await expect(userNext).toNotHaveDisabledAttribute();
    await userNext.click();

    const authContainer = modal.locator('.authentication-page');
    const authNext = modal.locator('.authentication-page-footer').locator('ion-button').nth(1);
    await expect(authContainer).toBeVisible();
    await expect(authNext).toHaveDisabledAttribute();

    const authRadio = authContainer.locator('.choose-auth-page').locator('.radio-list-item:visible');
    await expect(authRadio).toHaveCount(4);
    await expect(authRadio).toHaveAuthentication(...getAuthAssert(authMethods));
  });
}

for (const [index, authMethods] of [
  AllowAllProtectionMethods,
  [
    { primary: AllowedAuthenticationMethod.Password, withTotp: false },
    { primary: AllowedAuthenticationMethod.Keyring, withTotp: false },
  ],
  [
    { primary: AllowedAuthenticationMethod.OpenBao, withTotp: false },
    { primary: AllowedAuthenticationMethod.PKI, withTotp: false },
  ],
].entries()) {
  msTest(`Check allowed auth method when updating auth (${index + 1})`, async ({ myProfilePage }) => {
    await mockAllowedProtectionMethods(myProfilePage, authMethods);

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
    await expect(authRadio).toHaveCount(4);
    await expect(authRadio).toHaveAuthentication(...getAuthAssert(authMethods));
  });
}

msTest('Login with forbidden auth method and update', async ({ home }) => {
  msTest.setTimeout(45_000);
  await mockAllowedProtectionMethods(home, [{ primary: AllowedAuthenticationMethod.OpenBao, withTotp: false }]);

  await home.locator('.organization-card').first().click();
  await expect(home.locator('#password-input')).toBeVisible();

  await expect(home.locator('.login-button')).toHaveDisabledAttribute();

  await home.locator('#password-input').locator('input').fill('P@ssw0rd.');
  await expect(home.locator('.login-button')).toBeEnabled();

  await home.locator('.login-button').click();
  await expect(home.locator('#connected-header')).toContainText('My workspaces');
  await expect(home.locator('.topbar-right').locator('.text-content-name')).toHaveText('Alicey McAliceFace');
  await expect(home).toBeWorkspacePage();

  await answerQuestion(home, true, {
    expectedTitleText: 'Current authentication method not allowed',
    expectedQuestionText: 'The current authentication method is not allowed by the organization. Please choose a different method.',
    expectedPositiveText: 'Change authentication method',
    expectedNegativeText: 'Log out',
  });

  const authModal = home.locator('.change-authentication-modal');
  await expect(authModal).toBeVisible();
  await fillIonInput(authModal.locator('.current-authentication').locator('#ms-password-input'), 'P@ssw0rd.');
  await authModal.locator('#next-button').click();

  const authRadio = authModal.locator('.radio-list-item:visible');
  await expect(authRadio).toHaveCount(4);
  await expect(authRadio).toHaveAuthentication(
    { passwordDisabled: true, keyringDisabled: true, pkiDisabled: true },
    { password: AuthMethodDisabledReason.Forbidden, keyring: AuthMethodDisabledReason.Forbidden, pki: AuthMethodDisabledReason.Forbidden },
  );
  await authRadio.nth(3).click();
  await expect(authModal.locator('.proconnect-button')).toBeVisible();
  await authModal.locator('.proconnect-button').click();
  await expect(authModal.locator('.proconnect-button')).toBeHidden();
  await expect(authModal.locator('.proconnect-group--connected')).toBeVisible();
  await expect(authModal.locator('.proconnect-group--connected')).toHaveText('Connected');
  await expect(authModal.locator('#next-button')).toHaveText('Update');
  await expect(authModal.locator('#next-button')).toBeTrulyEnabled();
  await authModal.locator('#next-button').click();
  await expect(authModal).toBeHidden();
  await expect(home).toShowToast('Authentication has been updated.', 'Success');
  await expect(home).toBeWorkspacePage();
});

msTest('Login without totp and setup', async ({ home }) => {
  await mockAllowedProtectionMethods(home, [{ primary: AllowedAuthenticationMethod.Password, withTotp: true }]);

  await home.locator('.organization-card').first().click();
  await expect(home.locator('#password-input')).toBeVisible();

  await expect(home.locator('.login-button')).toHaveDisabledAttribute();

  await home.locator('#password-input').locator('input').fill('P@ssw0rd.');
  await expect(home.locator('.login-button')).toBeEnabled();

  await home.locator('.login-button').click();
  await expect(home.locator('#connected-header')).toContainText('My workspaces');
  await expect(home.locator('.topbar-right').locator('.text-content-name')).toHaveText('Alicey McAliceFace');
  await expect(home).toBeWorkspacePage();

  const requiredTotpModal = home.locator('.totp-required-modal');
  await expect(requiredTotpModal).toBeVisible();
  await expect(requiredTotpModal.locator('.totp-required__title')).toHaveText('Multi-factor authentication required');
  await expect(requiredTotpModal.locator('.totp-required-message__subtitle')).toHaveText(
    'Your organization requires multi-factor authentication. Please set it up to access this organization.',
  );
  await expect(requiredTotpModal.locator('.totp-required-footer__button').nth(0)).toHaveText('Configure MFA');
  await expect(requiredTotpModal.locator('.totp-required-footer__button').nth(1)).toHaveText('Log out');
  await requiredTotpModal.locator('.totp-required-footer__button').nth(0).click();
  await expect(requiredTotpModal).toBeHidden();

  const totpModal = home.locator('.mfa-activation-modal');
  await expect(totpModal).toBeVisible();
  await fillIonInput(totpModal.locator('.modal-current-authentication').locator('ion-input'), 'P@ssw0rd.');
  await totpModal.locator('#next-button').click();

  await setWriteClipboardPermission(home.context(), true);
  await totpModal.locator('.modal-content-information').locator('.input-action-button').click();
  const totpSecret = await getClipboardText(home);
  await totpModal.locator('#next-button').click();
  const totpCode = await generateTotpCode(totpSecret);
  await fillIonInput(totpModal.locator('.modal-content-enter-code').locator('ion-input'), totpCode);
  await totpModal.locator('#next-button').click();
  await expect(totpModal.locator('.modal-content-finalization')).toBeVisible();
  await expect(totpModal.locator('#next-button')).toHaveText('Done');
  await totpModal.locator('#next-button').click();
  await expect(totpModal).toBeHidden();
  await expect(home).toShowToast(
    'Multi-factor authentication has been successfully enabled and will now be required to log in to this organization.',
    'Success',
  );
  await expect(home).toBeWorkspacePage();
  await expect(home.locator('.information-modal')).toBeHidden();
});

msTest('Login update auth method and setup totp', async ({ home }) => {
  msTest.setTimeout(45_000);
  await mockAllowedProtectionMethods(home, [{ primary: AllowedAuthenticationMethod.OpenBao, withTotp: true }]);

  await home.locator('.organization-card').first().click();
  await expect(home.locator('#password-input')).toBeVisible();

  await expect(home.locator('.login-button')).toHaveDisabledAttribute();

  await home.locator('#password-input').locator('input').fill('P@ssw0rd.');
  await expect(home.locator('.login-button')).toBeEnabled();

  await home.locator('.login-button').click();
  await expect(home.locator('#connected-header')).toContainText('My workspaces');
  await expect(home.locator('.topbar-right').locator('.text-content-name')).toHaveText('Alicey McAliceFace');
  await expect(home).toBeWorkspacePage();

  await answerQuestion(home, true, {
    expectedTitleText: 'Current authentication method not allowed',
    expectedQuestionText: 'The current authentication method is not allowed by the organization. Please choose a different method.',
    expectedPositiveText: 'Change authentication method',
    expectedNegativeText: 'Log out',
  });

  const authModal = home.locator('.change-authentication-modal');
  await expect(authModal).toBeVisible();
  await fillIonInput(authModal.locator('.current-authentication').locator('#ms-password-input'), 'P@ssw0rd.');
  await authModal.locator('#next-button').click();

  const authRadio = authModal.locator('.radio-list-item:visible');
  await expect(authRadio).toHaveCount(4);
  await expect(authRadio).toHaveAuthentication(
    { passwordDisabled: true, keyringDisabled: true, pkiDisabled: true },
    { password: AuthMethodDisabledReason.Forbidden, keyring: AuthMethodDisabledReason.Forbidden, pki: AuthMethodDisabledReason.Forbidden },
  );
  await authRadio.nth(3).click();
  await expect(authModal.locator('.proconnect-button')).toBeVisible();
  await authModal.locator('.proconnect-button').click();
  await expect(authModal.locator('.proconnect-button')).toBeHidden();
  await expect(authModal.locator('.proconnect-group--connected')).toBeVisible();
  await expect(authModal.locator('.proconnect-group--connected')).toHaveText('Connected');
  await expect(authModal.locator('#next-button')).toHaveText('Update');
  await expect(authModal.locator('#next-button')).toBeTrulyEnabled();
  await authModal.locator('#next-button').click();
  await expect(authModal).toBeHidden();
  await expect(home).toShowToast('Authentication has been updated.', 'Success');

  const requiredTotpModal = home.locator('.totp-required-modal');
  await expect(requiredTotpModal).toBeVisible();
  await expect(requiredTotpModal.locator('.totp-required__title')).toHaveText('Multi-factor authentication required');
  await expect(requiredTotpModal.locator('.totp-required-message__subtitle')).toHaveText(
    'Your organization requires multi-factor authentication. Please set it up to access this organization.',
  );
  await expect(requiredTotpModal.locator('.totp-required-footer__button').nth(0)).toHaveText('Configure MFA');
  await expect(requiredTotpModal.locator('.totp-required-footer__button').nth(1)).toHaveText('Log out');
  await requiredTotpModal.locator('.totp-required-footer__button').nth(0).click();
  await expect(requiredTotpModal).toBeHidden();

  const totpModal = home.locator('.mfa-activation-modal');
  await expect(totpModal).toBeVisible();

  await setWriteClipboardPermission(home.context(), true);
  await totpModal.locator('.modal-content-information').locator('.input-action-button').click();
  const totpSecret = await getClipboardText(home);
  await totpModal.locator('#next-button').click();
  const totpCode = await generateTotpCode(totpSecret);
  await fillIonInput(totpModal.locator('.modal-content-enter-code').locator('ion-input'), totpCode);
  await totpModal.locator('#next-button').click();
  await expect(totpModal.locator('.modal-content-finalization')).toBeVisible();
  await expect(totpModal.locator('#next-button')).toHaveText('Done');
  await totpModal.locator('#next-button').click();
  await expect(totpModal).toBeHidden();
  await expect(home).toShowToast(
    'Multi-factor authentication has been successfully enabled and will now be required to log in to this organization.',
    'Success',
  );
  await expect(home).toBeWorkspacePage();
  await expect(home.locator('.information-modal')).toBeHidden();
});

msTest('Login with forbidden auth method cancel', async ({ home }) => {
  await mockAllowedProtectionMethods(home, [{ primary: AllowedAuthenticationMethod.OpenBao, withTotp: true }]);

  await home.locator('.organization-card').first().click();
  await expect(home.locator('#password-input')).toBeVisible();

  await expect(home.locator('.login-button')).toHaveDisabledAttribute();

  await home.locator('#password-input').locator('input').fill('P@ssw0rd.');
  await expect(home.locator('.login-button')).toBeEnabled();

  await home.locator('.login-button').click();
  await expect(home.locator('#connected-header')).toContainText('My workspaces');
  await expect(home.locator('.topbar-right').locator('.text-content-name')).toHaveText('Alicey McAliceFace');
  await expect(home).toBeWorkspacePage();

  await answerQuestion(home, false, {
    expectedTitleText: 'Current authentication method not allowed',
    expectedQuestionText: 'The current authentication method is not allowed by the organization. Please choose a different method.',
    expectedPositiveText: 'Change authentication method',
    expectedNegativeText: 'Log out',
  });
  await expect(home).toBeHomePage();
});
