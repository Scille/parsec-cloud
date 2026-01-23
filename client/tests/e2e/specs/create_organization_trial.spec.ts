// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { Locator, Page } from '@playwright/test';
import {
  DEFAULT_USER_INFORMATION,
  expect,
  fillInputModal,
  fillIonInput,
  getTestbedBootstrapAddr,
  MsPage,
  msTest,
  setupNewPage,
} from '@tests/e2e/helpers';
import { randomInt } from 'crypto';

async function openCreateOrganizationModal(page: Page): Promise<Locator> {
  await page.locator('#create-organization-button').click();
  await page.locator('.popover-viewport').getByRole('listitem').nth(0).click();
  const modal = page.locator('.create-organization-modal');
  await modal.locator('.server-choice-item').nth(1).click();
  await modal.locator('.server-page-footer').locator('ion-button').nth(1).click();
  return modal;
}

async function cancelAndResume(page: Page, currentContainer: Locator): Promise<void> {
  await expect(currentContainer.locator('.closeBtn')).toBeVisible();
  await currentContainer.locator('.closeBtn').click();
  await expect(page.locator('.create-organization-modal')).toBeHidden();
  await expect(page.locator('.question-modal')).toBeVisible();
  await page.locator('.question-modal').locator('#cancel-button').click();
  await expect(page.locator('.question-modal')).toBeHidden();
  await expect(page.locator('.create-organization-modal')).toBeVisible();
}

msTest('Go through trial org creation process', async ({ context }) => {
  const home = (await context.newPage()) as MsPage;
  // Making sure that the testbed is recognized as the trial server
  await setupNewPage(home, { trialServers: process.env.TESTBED_SERVER?.replace('parsec3://', ''), saasServers: 'unknown.host' });

  const modal = await openCreateOrganizationModal(home);

  const userInfoContainer = modal.locator('.user-information-page');
  const userPrevious = modal.locator('.user-information-page-footer').locator('ion-button').nth(0);
  const userNext = modal.locator('.user-information-page-footer').locator('ion-button').nth(1);
  await expect(userPrevious).toBeVisible();
  await expect(userNext).toBeVisible();
  await expect(userNext).toHaveDisabledAttribute();
  await expect(userInfoContainer.locator('.modal-header-title__text')).toHaveText('Enter your personal information');
  await fillIonInput(userInfoContainer.locator('ion-input').nth(0), DEFAULT_USER_INFORMATION.name);
  await expect(userNext).toHaveDisabledAttribute();
  await fillIonInput(userInfoContainer.locator('ion-input').nth(1), DEFAULT_USER_INFORMATION.email);
  await expect(userNext).toHaveDisabledAttribute();
  await userInfoContainer.locator('.ms-checkbox').check();
  await expect(userNext).toNotHaveDisabledAttribute();

  // Try cancelling
  await cancelAndResume(home, userInfoContainer);

  // Try with a bad name
  await fillIonInput(userInfoContainer.locator('ion-input').nth(0), '"#"');
  const userNameError = userInfoContainer.locator('.input-container').nth(0).locator('.form-error');
  await expect(userNameError).toBeVisible();
  await expect(userNameError).toHaveText('Name contains invalid characters.');
  await expect(userNext).toHaveDisabledAttribute();

  // And correct name again
  await fillIonInput(userInfoContainer.locator('ion-input').nth(0), DEFAULT_USER_INFORMATION.name);
  await expect(userNext).toNotHaveDisabledAttribute();
  await expect(userNameError).toBeHidden();

  // Now with bad email
  await fillIonInput(userInfoContainer.locator('ion-input').nth(1), 'invalid email');
  const userEmailError = userInfoContainer.locator('.input-container').nth(1).locator('.form-error');
  await expect(userEmailError).toBeVisible();
  await expect(userEmailError).toHaveText('Wrong email address format.');
  await expect(userNext).toHaveDisabledAttribute();

  // Correct email again
  await fillIonInput(userInfoContainer.locator('ion-input').nth(1), DEFAULT_USER_INFORMATION.email);
  await expect(userNext).toNotHaveDisabledAttribute();
  await expect(userNameError).toBeHidden();

  await userNext.click();

  const authContainer = modal.locator('.authentication-page');
  const authPrevious = modal.locator('.authentication-page-footer').locator('ion-button').nth(0);
  const authNext = modal.locator('.authentication-page-footer').locator('ion-button').nth(1);
  await expect(userInfoContainer).toBeHidden();
  await expect(authContainer).toBeVisible();
  await expect(authPrevious).toBeVisible();
  await expect(authPrevious).toNotHaveDisabledAttribute();
  await expect(authNext).toBeVisible();
  await expect(authNext).toHaveDisabledAttribute();
  await expect(authContainer.locator('.modal-header-title__text')).toHaveText('Authentication');

  const authRadio = authContainer.locator('.choose-auth-page').locator('.radio-list-item:visible');
  await expect(authRadio).toHaveCount(3);
  await expect(authRadio.nth(0)).toHaveTheClass('radio-disabled');
  await expect(authRadio.nth(0).locator('.authentication-card-text__title')).toHaveText('System authentication');
  await expect(authRadio.nth(1)).toHaveText('Password');
  await authRadio.nth(1).click();

  await fillIonInput(authContainer.locator('.choose-password').locator('ion-input').nth(0), DEFAULT_USER_INFORMATION.password);
  await expect(authNext).toHaveDisabledAttribute();
  await fillIonInput(authContainer.locator('.choose-password').locator('ion-input').nth(1), DEFAULT_USER_INFORMATION.password);
  await expect(authNext).toNotHaveDisabledAttribute();

  // Try cancelling
  await cancelAndResume(home, authContainer);

  // Password too simple
  await fillIonInput(authContainer.locator('.choose-password').locator('ion-input').nth(0), 'EasyP@ssw0rd');
  await expect(authNext).toHaveDisabledAttribute();

  // Back to complicated password
  await fillIonInput(authContainer.locator('.choose-password').locator('ion-input').nth(0), DEFAULT_USER_INFORMATION.password);
  await expect(authNext).toNotHaveDisabledAttribute();

  // Check does not match
  await fillIonInput(authContainer.locator('.choose-password').locator('ion-input').nth(1), `${DEFAULT_USER_INFORMATION.password}-extra`);
  await expect(authNext).toHaveDisabledAttribute();
  const matchError = authContainer.locator('.choose-password').locator('.inputs-container-item').nth(1).locator('.form-helperText').nth(1);
  await expect(matchError).toBeVisible();
  await expect(matchError).toHaveText('Do not match');

  // Back to matching password
  await fillIonInput(authContainer.locator('.choose-password').locator('ion-input').nth(1), DEFAULT_USER_INFORMATION.password);
  await expect(authNext).toNotHaveDisabledAttribute();
  await expect(matchError).toBeHidden();

  await authNext.click();

  await expect(userInfoContainer).toBeHidden();
  await expect(authContainer).toBeHidden();
  await expect(modal.locator('.creation-page')).toBeVisible();
  await home.waitForTimeout(1000);

  await expect(modal.locator('.created-page')).toBeVisible();
  await expect(modal.locator('.creation-page')).toBeHidden();
  await expect(modal.locator('.created-page').locator('.closeBtn')).toBeHidden();
  await modal.locator('.created-page-footer').locator('ion-button').click();
  await expect(modal).toBeHidden();
  await home.waitForTimeout(1000);
  await expect(home).toBeWorkspacePage();
});

msTest('Go through trial org creation process from bootstrap link', async ({ context }) => {
  const page = (await context.newPage()) as MsPage;
  // Making sure that the testbed is recognized as the trial server
  const testbedUrl = new URL(process.env.TESTBED_SERVER ?? '');
  await setupNewPage(page, { trialServers: testbedUrl.host, saasServers: 'unknown.host' });

  await page.locator('#create-organization-button').click();
  await page.locator('.popover-viewport').getByRole('listitem').nth(1).click();
  const uniqueOrgName = `${page.orgInfo.name}-${randomInt(2 ** 47)}`;
  const bootstrapAddr = getTestbedBootstrapAddr(uniqueOrgName);
  await fillInputModal(page, bootstrapAddr);
  const modal = page.locator('.create-organization-modal');

  const userInfoContainer = modal.locator('.user-information-page');
  const userPrevious = modal.locator('.user-information-page-footer').locator('ion-button').nth(0);
  const userNext = modal.locator('.user-information-page-footer').locator('ion-button').nth(1);
  await expect(userPrevious).toBeHidden();
  await expect(userNext).toBeVisible();
  await expect(userNext).toHaveDisabledAttribute();
  await expect(userInfoContainer.locator('.modal-header-title__text')).toHaveText('Enter your personal information');
  await fillIonInput(userInfoContainer.locator('ion-input').nth(0), DEFAULT_USER_INFORMATION.name);
  await expect(userNext).toHaveDisabledAttribute();
  await fillIonInput(userInfoContainer.locator('ion-input').nth(1), DEFAULT_USER_INFORMATION.email);
  await expect(userNext).toHaveDisabledAttribute();
  await userInfoContainer.locator('.ms-checkbox').check();
  await expect(userNext).toNotHaveDisabledAttribute();
  await userNext.click();

  const authContainer = modal.locator('.authentication-page');
  const authPrevious = modal.locator('.authentication-page-footer').locator('ion-button').nth(0);
  const authNext = modal.locator('.authentication-page-footer').locator('ion-button').nth(1);
  await expect(userInfoContainer).toBeHidden();
  await expect(authContainer).toBeVisible();
  await expect(authPrevious).toBeVisible();
  await expect(authPrevious).toNotHaveDisabledAttribute();
  await expect(authNext).toBeVisible();
  await expect(authNext).toHaveDisabledAttribute();
  await expect(authContainer.locator('.modal-header-title__text')).toHaveText('Authentication');

  const authRadio = authContainer.locator('.choose-auth-page').locator('.radio-list-item:visible');
  await expect(authRadio).toHaveCount(3);
  await expect(authRadio.nth(0)).toHaveTheClass('radio-disabled');
  await expect(authRadio.nth(0).locator('.authentication-card-text__title')).toHaveText('System authentication');
  await expect(authRadio.nth(1)).toHaveText('Password');
  await authRadio.nth(1).click();
  await fillIonInput(authContainer.locator('.choose-password').locator('ion-input').nth(0), DEFAULT_USER_INFORMATION.password);
  await expect(authNext).toHaveDisabledAttribute();
  await fillIonInput(authContainer.locator('.choose-password').locator('ion-input').nth(1), DEFAULT_USER_INFORMATION.password);
  await authNext.scrollIntoViewIfNeeded();
  await expect(authNext).toNotHaveDisabledAttribute();
  await authNext.click();

  await expect(userInfoContainer).toBeHidden();
  await expect(authContainer).toBeHidden();
  await expect(modal.locator('.creation-page')).toBeVisible();
  await page.waitForTimeout(1000);

  await expect(modal.locator('.created-page')).toBeVisible();
  await expect(modal.locator('.creation-page')).toBeHidden();
  await modal.locator('.created-page-footer').locator('ion-button').click();
  await expect(modal).toBeHidden();
  await page.waitForTimeout(1000);
  await expect(page).toBeWorkspacePage();
  await page.release();
});
