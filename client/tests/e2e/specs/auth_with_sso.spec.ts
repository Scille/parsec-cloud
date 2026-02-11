// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { Locator, Page } from '@playwright/test';
import { DEFAULT_USER_INFORMATION, expect, fillIonInput, logout, MsPage, msTest, openExternalLink } from '@tests/e2e/helpers';
import { randomInt } from 'crypto';

async function openCreateOrganizationModal(page: Page): Promise<Locator> {
  await page.locator('#create-organization-button').click();
  await page.locator('.popover-viewport').getByRole('listitem').nth(0).click();
  const modal = page.locator('.create-organization-modal');
  await modal.locator('.server-page-footer').locator('ion-button').nth(0).click();
  return modal;
}

msTest('Go through custom org creation process, auth SSO', async ({ home }) => {
  const modal = await openCreateOrganizationModal(home);

  const uniqueOrgName = `${home.orgInfo.name}-${randomInt(2 ** 47)}`;
  const orgServerContainer = modal.locator('.organization-name-and-server-page');
  const orgNext = orgServerContainer.locator('.organization-name-and-server-page-footer').locator('ion-button').nth(1);
  await fillIonInput(orgServerContainer.locator('ion-input').nth(0), uniqueOrgName);
  await fillIonInput(orgServerContainer.locator('ion-input').nth(1), home.orgInfo.serverAddr);
  await expect(orgNext).toNotHaveDisabledAttribute();
  await orgNext.click();
  await expect(orgServerContainer).toBeHidden();

  const userInfoContainer = modal.locator('.user-information-page');
  const userNext = modal.locator('.user-information-page-footer').locator('ion-button').nth(1);
  await expect(userInfoContainer).toBeVisible();
  await expect(userInfoContainer.locator('.modal-header-title__text')).toHaveText('Enter your personal information');
  await fillIonInput(userInfoContainer.locator('ion-input').nth(0), DEFAULT_USER_INFORMATION.name);
  await fillIonInput(userInfoContainer.locator('ion-input').nth(1), DEFAULT_USER_INFORMATION.email);
  await expect(userNext).toNotHaveDisabledAttribute();
  await userNext.click();

  const authContainer = modal.locator('.authentication-page');
  const authNext = modal.locator('.authentication-page-footer').locator('ion-button').nth(1);
  await expect(userInfoContainer).toBeHidden();
  await expect(authContainer).toBeVisible();
  await expect(authContainer.locator('.modal-header-title__text')).toHaveText('Authentication');

  const authRadio = authContainer.locator('.choose-auth-page').locator('.radio-list-item:visible');
  await expect(authRadio).toHaveAuthentication({ pkiDisabled: true, keyringDisabled: true });
  await authRadio.nth(3).click();
  await expect(authContainer.locator('.proconnect-button')).toBeVisible();
  await expect(modal.locator('.proconnect-group--connected')).toBeHidden();
  await expect(authNext).toBeTrulyDisabled();

  // Fun fact: this event is never triggered if the URL given when opening
  // the pop-up doesn't point to anything, ie:
  // window.open('https://doesnotexist', 'Login');
  // will open the popup (visible in debug mode) but will not trigger that event.
  const popupPromise = home.context().waitForEvent('page');
  await authContainer.locator('.proconnect-button').click();
  const popup = await popupPromise;
  await popup.waitForLoadState();
  await popup.waitForEvent('close');
  expect(popup.isClosed()).toBeTruthy();
  await expect(authNext).toBeTrulyEnabled();

  await expect(modal.locator('.proconnect-group--connected')).toBeVisible();
  await expect(modal.locator('.connected-text')).toHaveText('Connected');

  await authNext.click();

  await expect(authContainer).toBeHidden();
  const summaryContainer = modal.locator('.summary-page');
  const summaryNext = modal.locator('.summary-page-footer').locator('ion-button').nth(1);
  await expect(summaryContainer).toBeVisible();
  await expect(summaryContainer.locator('.modal-header-title__text')).toHaveText('Overview of your organization');

  await expect(summaryContainer.locator('.summary-item__label')).toHaveText([
    'Organization',
    'Full name',
    'Email',
    'Server choice',
    'Authentication method',
  ]);
  await expect(summaryContainer.locator('.summary-item__text')).toHaveText([
    uniqueOrgName,
    DEFAULT_USER_INFORMATION.name,
    DEFAULT_USER_INFORMATION.email,
    'Custom Server',
    'Single Sign-On',
  ]);
  await summaryNext.click();

  await expect(modal.locator('.creation-page')).toBeVisible();
  await home.waitForTimeout(1000);

  await expect(modal.locator('.created-page')).toBeVisible();
  await expect(modal.locator('.creation-page')).toBeHidden();
  await expect(modal.locator('.created-page').locator('.closeBtn')).toBeHidden();
  await modal.locator('.created-page-footer').locator('ion-button').click();
  await expect(modal).toBeHidden();
  await home.waitForTimeout(1000);
  await expect(home).toBeWorkspacePage();

  await logout(home);
  const cards = home.locator('.organization-list').locator('.organization-card');
  await expect(cards).toHaveCount(4);
  await expect(cards.nth(0).locator('.organization-card-content').locator('.organization-name')).toHaveText(uniqueOrgName);
  await cards.nth(0).click();
  await expect(home.locator('.login-card-footer').locator('.sso-provider-card')).toBeVisible();
  await home.locator('.sso-provider-card').locator('.proconnect-button').click();
  await home.waitForTimeout(1000);
  await expect(home).toBeWorkspacePage();

  await home.release();
});

async function mockSSOServer(page: MsPage, error: 'timeout' | '400'): Promise<void> {
  await page.route('**/testbed/mock/openbao/v1/auth/pro_connect/oidc/auth_url', async (route) => {
    if (error === 'timeout') {
      await page.waitForTimeout(500);
      await route.abort('timedout');
    } else if (error === '400') {
      await route.fulfill({ status: 400 });
    } else {
      await route.continue();
    }
  });
}

for (const error of ['timeout', '400', 'popup']) {
  msTest(`Custom org creation with SSO error ${error}`, async ({ home }) => {
    const modal = await openCreateOrganizationModal(home);

    const uniqueOrgName = `${home.orgInfo.name}-${randomInt(2 ** 47)}`;
    const orgServerContainer = modal.locator('.organization-name-and-server-page');
    const orgNext = orgServerContainer.locator('.organization-name-and-server-page-footer').locator('ion-button').nth(1);
    await fillIonInput(orgServerContainer.locator('ion-input').nth(0), uniqueOrgName);
    await fillIonInput(orgServerContainer.locator('ion-input').nth(1), home.orgInfo.serverAddr);
    await expect(orgNext).toNotHaveDisabledAttribute();
    await orgNext.click();
    await expect(orgServerContainer).toBeHidden();

    const userInfoContainer = modal.locator('.user-information-page');
    const userNext = modal.locator('.user-information-page-footer').locator('ion-button').nth(1);
    await expect(userInfoContainer).toBeVisible();
    await expect(userInfoContainer.locator('.modal-header-title__text')).toHaveText('Enter your personal information');
    await fillIonInput(userInfoContainer.locator('ion-input').nth(0), DEFAULT_USER_INFORMATION.name);
    await fillIonInput(userInfoContainer.locator('ion-input').nth(1), DEFAULT_USER_INFORMATION.email);
    await expect(userNext).toNotHaveDisabledAttribute();
    await userNext.click();

    const authContainer = modal.locator('.authentication-page');
    const authNext = modal.locator('.authentication-page-footer').locator('ion-button').nth(1);
    await expect(userInfoContainer).toBeHidden();
    await expect(authContainer).toBeVisible();
    await expect(authContainer.locator('.modal-header-title__text')).toHaveText('Authentication');

    const authRadio = authContainer.locator('.choose-auth-page').locator('.radio-list-item:visible');
    await expect(authRadio).toHaveAuthentication({ pkiDisabled: true, keyringDisabled: true });
    await authRadio.nth(3).click();
    await expect(authContainer.locator('.proconnect-button')).toBeVisible();
    await expect(modal.locator('.proconnect-group--connected')).toBeHidden();
    await expect(authNext).toBeTrulyDisabled();

    const formError = authContainer.locator('.choose-auth-choice').locator('.form-error');
    await expect(formError).toBeHidden();

    if (error === 'popup') {
      // Cannot set the permissions. Instead we simulate window.open returning null.
      home.evaluate(() => {
        window.open = () => null;
      });
    } else {
      await mockSSOServer(home, error as any);
    }

    await authContainer.locator('.proconnect-button').click();

    if (error === 'timeout') {
      await expect(authContainer.locator('.authentication-card')).toHaveTheClass('authentication-card--disabled');
      await expect(authContainer.locator('.authentication-card').locator('.authentication-card__update-button')).toBeTrulyDisabled();
    }

    await expect(authNext).toBeTrulyDisabled();
    await expect(formError).toBeVisible();
    if (error === 'popup') {
      await expect(formError).toHaveText('Please make sure that your browser allows pop-ups for this website.');
    } else {
      await expect(formError).toHaveText('The SSO configuration provided by the server is invalid. Please contact an administrator.');
    }
  });
}

msTest('Check ProConnect link', async ({ home }) => {
  const modal = await openCreateOrganizationModal(home);

  const uniqueOrgName = `${home.orgInfo.name}-${randomInt(2 ** 47)}`;
  const orgServerContainer = modal.locator('.organization-name-and-server-page');
  const orgNext = orgServerContainer.locator('.organization-name-and-server-page-footer').locator('ion-button').nth(1);
  await fillIonInput(orgServerContainer.locator('ion-input').nth(0), uniqueOrgName);
  await fillIonInput(orgServerContainer.locator('ion-input').nth(1), home.orgInfo.serverAddr);
  await expect(orgNext).toNotHaveDisabledAttribute();
  await orgNext.click();
  await expect(orgServerContainer).toBeHidden();

  const userInfoContainer = modal.locator('.user-information-page');
  const userNext = modal.locator('.user-information-page-footer').locator('ion-button').nth(1);
  await expect(userInfoContainer).toBeVisible();
  await expect(userInfoContainer.locator('.modal-header-title__text')).toHaveText('Enter your personal information');
  await fillIonInput(userInfoContainer.locator('ion-input').nth(0), DEFAULT_USER_INFORMATION.name);
  await fillIonInput(userInfoContainer.locator('ion-input').nth(1), DEFAULT_USER_INFORMATION.email);
  await expect(userNext).toNotHaveDisabledAttribute();
  await userNext.click();

  const authContainer = modal.locator('.authentication-page');
  await expect(userInfoContainer).toBeHidden();
  await expect(authContainer).toBeVisible();

  const authRadio = authContainer.locator('.choose-auth-page').locator('.radio-list-item:visible');
  await expect(authRadio).toHaveAuthentication({ pkiDisabled: true, keyringDisabled: true });
  await authRadio.nth(3).click();
  const card = authContainer.locator('.sso-provider-card');
  await expect(card).toHaveCount(1);
  await expect(card.locator('a')).toHaveText("What's ProConnect?");
  await openExternalLink(home, card.locator('a'), /^https:\/\/www\.proconnect\.gouv\.fr\/?.*$/);
});
