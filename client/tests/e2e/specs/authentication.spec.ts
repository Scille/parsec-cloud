// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import {
  AllowAllProtectionMethods,
  AllowedAuthenticationMethod,
  AuthMethodDisabledReason,
  DEFAULT_USER_INFORMATION,
  expect,
  fillIonInput,
  msTest,
  ProtectionMethod,
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
