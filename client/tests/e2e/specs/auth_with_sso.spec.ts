// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { Locator, Page } from '@playwright/test';
import { DEFAULT_USER_INFORMATION, expect, fillIonInput, logout, mockSso, MsPage, msTest, setupNewPage } from '@tests/e2e/helpers';
import { OPEN_BAO_SERVER } from '@tests/e2e/helpers/sso';

async function openCreateOrganizationModal(page: Page): Promise<Locator> {
  await page.locator('#create-organization-button').click();
  await page.locator('.popover-viewport').getByRole('listitem').nth(0).click();
  const modal = page.locator('.create-organization-modal');
  await modal.locator('.server-choice-item').nth(1).click();
  await modal.locator('.server-page-footer').locator('ion-button').nth(1).click();
  return modal;
}

msTest('Go through trial org creation process, auth SSO', async ({ context }) => {
  const page = (await context.newPage()) as MsPage;
  await setupNewPage(page, { openBaoServer: OPEN_BAO_SERVER });

  // Trust me, mock on the BrowserContext, not on the page
  await mockSso(context);
  const modal = await openCreateOrganizationModal(page);

  const userInfoContainer = modal.locator('.user-information-page');
  const userNext = modal.locator('.user-information-page-footer').locator('ion-button').nth(1);
  await expect(userInfoContainer).toBeVisible();
  await expect(userInfoContainer.locator('.modal-header-title__text')).toHaveText('Enter your personal information');
  await fillIonInput(userInfoContainer.locator('ion-input').nth(0), DEFAULT_USER_INFORMATION.name);
  await fillIonInput(userInfoContainer.locator('ion-input').nth(1), DEFAULT_USER_INFORMATION.email);
  await userInfoContainer.locator('.checkbox').locator('.native-wrapper').click();
  await expect(userNext).toNotHaveDisabledAttribute();
  await userNext.click();

  const authContainer = modal.locator('.authentication-page');
  const authNext = modal.locator('.authentication-page-footer').locator('ion-button').nth(1);
  await expect(userInfoContainer).toBeHidden();
  await expect(authContainer).toBeVisible();
  await expect(authContainer.locator('.modal-header-title__text')).toHaveText('Authentication');

  const authRadio = authContainer.locator('.choose-auth-page').locator('.radio-list-item:visible');
  await expect(authRadio).toHaveCount(3);
  await expect(authRadio.nth(0)).toHaveTheClass('radio-disabled');
  await expect(authRadio.nth(0).locator('.authentication-card-text__title')).toHaveText('System authentication');
  await expect(authRadio.nth(1)).toHaveText('Password');
  await expect(authRadio.nth(2).locator('.authentication-card-text__title')).toHaveText('Single Sign-On');
  await expect(authRadio.nth(2).locator('.authentication-card-text__description')).toHaveText('Login with an external account');
  await authRadio.nth(2).click();
  await expect(authContainer.locator('.proconnect-button')).toBeVisible();
  await expect(authNext).toBeTrulyDisabled();

  // Fun fact: this event is never triggered if the URL given when opening
  // the pop-up doesn't point to anything, ie:
  // window.open('https://doesnotexist', 'Login');
  // will open the popup (visible in debug mode) but will not trigger that event.
  // Since we're mocking the routes and we can't mock the route for that page since
  // it's not opened yet, we have to mock the routes on the BrowserContext instead.
  const popupPromise = context.waitForEvent('page');
  await authContainer.locator('.proconnect-button').click();
  const popup = await popupPromise;
  await popup.waitForLoadState();
  await popup.waitForEvent('close');
  expect(popup.isClosed()).toBeTruthy();
  await expect(authNext).toBeTrulyEnabled();

  await authNext.click();

  await expect(userInfoContainer).toBeHidden();
  await expect(authContainer).toBeHidden();
  await expect(modal.locator('.creation-page')).toBeVisible();
  await page.waitForTimeout(1000);

  await expect(modal.locator('.created-page')).toBeVisible();
  await expect(modal.locator('.creation-page')).toBeHidden();
  await expect(modal.locator('.created-page').locator('.closeBtn')).toBeHidden();
  await modal.locator('.created-page-footer').locator('ion-button').click();
  await expect(modal).toBeHidden();
  await page.waitForTimeout(1000);
  await expect(page).toBeWorkspacePage();

  await logout(page);
  const cards = page.locator('.organization-list').locator('.organization-card');
  await expect(cards).toHaveCount(4);
  await expect(cards.nth(3).locator('.organization-card-content').locator('.organization-name')).toHaveText(/^trial-.+$/);
  await cards.nth(3).click();
  await expect(page.locator('.login-card-footer').locator('.sso-provider-card')).toBeVisible();
  await page.locator('.sso-provider-card').locator('.proconnect-button').click();
  await page.waitForTimeout(1000);
  await expect(page).toBeWorkspacePage();

  await page.release();
});
