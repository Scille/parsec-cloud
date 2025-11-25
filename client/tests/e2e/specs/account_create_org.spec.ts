// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import {
  DEFAULT_USER_INFORMATION,
  expect,
  fillIonInput,
  getTestbedBootstrapAddr,
  MockBms,
  MsPage,
  msTest,
  setupNewPage,
} from '@tests/e2e/helpers';
import { randomInt } from 'crypto';

msTest('Account create org custom server with server matching', async ({ context }) => {
  const page = (await context.newPage()) as MsPage;

  await setupNewPage(page, { withParsecAccount: true, parsecAccountAutoLogin: true, location: '/page' });
  await expect(page).toHaveURL(/.+\/home$/);

  await page.locator('#create-organization-button').click();
  await page.locator('.popover-viewport').getByRole('listitem').nth(0).click();
  const modal = page.locator('.create-organization-modal');
  await modal.locator('.server-page-footer').locator('ion-button').nth(0).click();

  const uniqueOrgName = `${page.orgInfo.name}-${randomInt(2 ** 47)}`;

  const orgServerContainer = modal.locator('.organization-name-and-server-page');
  const orgPrevious = orgServerContainer.locator('.organization-name-and-server-page-footer').locator('ion-button').nth(0);
  const orgNext = orgServerContainer.locator('.organization-name-and-server-page-footer').locator('ion-button').nth(1);
  await expect(orgPrevious).toBeVisible();
  await fillIonInput(orgServerContainer.locator('ion-input').nth(0), uniqueOrgName);
  await fillIonInput(orgServerContainer.locator('ion-input').nth(1), page.orgInfo.serverAddr);
  await expect(orgNext).toNotHaveDisabledAttribute();
  await orgNext.click();

  // Auto-filled by Parsec Account
  const userInfoContainer = modal.locator('.user-information-page');
  await expect(userInfoContainer).toBeHidden();

  // Taken care of by Parsec Account
  const authContainer = modal.locator('.authentication-page');
  await expect(authContainer).toBeHidden();

  const summaryContainer = modal.locator('.summary-page');
  const summaryNext = modal.locator('.summary-page-footer').locator('ion-button').nth(1);
  const summaryEditButtons = modal.locator('.summary-item__button');
  await expect(summaryContainer).toBeVisible();
  await expect(summaryNext).toBeVisible();

  await expect(summaryEditButtons.nth(0)).toBeVisible();
  await expect(summaryEditButtons.nth(1)).toBeHidden();
  await expect(summaryEditButtons.nth(2)).toBeHidden();
  await expect(summaryEditButtons.nth(3)).toBeVisible();
  await expect(summaryEditButtons.nth(4)).toBeHidden();

  await expect(summaryContainer.locator('.summary-item__label')).toHaveText([
    'Organization',
    'Full name',
    'Email',
    'Server choice',
    'Authentication method',
  ]);
  await expect(summaryContainer.locator('.summary-item__text')).toHaveText([
    uniqueOrgName,
    /^Agent\d+$/,
    /^agent\d+@example\.com$/,
    'Custom Server',
    'Parsec Account',
  ]);
  await summaryNext.click();

  await expect(userInfoContainer).toBeHidden();
  await expect(authContainer).toBeHidden();
  await expect(summaryContainer).toBeHidden();
  await expect(modal.locator('.creation-page')).toBeVisible();
  await expect(modal.locator('.creation-page').locator('.closeBtn')).toBeHidden();
  await page.waitForTimeout(1000);

  await expect(modal.locator('.created-page')).toBeVisible();
  await expect(modal.locator('.creation-page')).toBeHidden();
  await expect(modal.locator('.created-page').locator('.closeBtn')).toBeHidden();
  await modal.locator('.created-page-footer').locator('ion-button').click();
  await expect(modal).toBeHidden();
  await page.waitForTimeout(1000);
  await expect(page).toBeWorkspacePage();
  await page.release();
});

msTest('Account create org saas with server matching', async ({ context }) => {
  const page = (await context.newPage()) as MsPage;

  await setupNewPage(page, {
    withParsecAccount: true,
    parsecAccountAutoLogin: true,
    location: '/page',
    saasServers: process.env.TESTBED_SERVER?.replace('parsec3://', ''),
    enableStripe: true,
  });
  await expect(page).toHaveURL(/.+\/home$/);

  await page.locator('#create-organization-button').click();
  await page.locator('.popover-viewport').getByRole('listitem').nth(0).click();
  const modal = page.locator('.create-organization-modal');
  await modal.locator('.server-choice-item').nth(0).click();
  await modal.locator('.server-page-footer').locator('ion-button').nth(1).click();

  const uniqueOrgName = `${page.orgInfo.name}-${randomInt(2 ** 47)}`;

  await MockBms.mockLogin(page);
  await MockBms.mockUserRoute(page);
  await MockBms.mockCreateOrganization(page, getTestbedBootstrapAddr(uniqueOrgName));

  const bmsContainer = modal.locator('.saas-login');
  await expect(bmsContainer.locator('.modal-header-title__text')).toHaveText('Link your customer account to your new organization');
  const bmsNext = bmsContainer.locator('.saas-login-button').locator('.saas-login-button__item').nth(1);
  await fillIonInput(bmsContainer.locator('ion-input').nth(0), DEFAULT_USER_INFORMATION.email);
  await fillIonInput(bmsContainer.locator('ion-input').nth(1), DEFAULT_USER_INFORMATION.password);
  await bmsNext.click();

  const orgNameContainer = modal.locator('.organization-name-page');
  await expect(orgNameContainer).toBeVisible();
  const orgNameNext = modal.locator('.organization-name-page-footer').locator('ion-button').nth(1);
  await fillIonInput(orgNameContainer.locator('ion-input'), uniqueOrgName);
  await orgNameNext.click();

  // Auth is skipped, we're using Parsec Account
  const authContainer = modal.locator('.authentication-page');
  await expect(orgNameContainer).toBeHidden();
  await expect(authContainer).toBeHidden();

  const summaryContainer = modal.locator('.summary-page');
  const summaryNext = modal.locator('.summary-page-footer').locator('ion-button').nth(1);
  const summaryEditButtons = modal.locator('.summary-item__button');
  await expect(summaryContainer).toBeVisible();

  // Only the org name field can be updated
  await expect(summaryEditButtons.nth(0)).toBeVisible();
  await expect(summaryEditButtons.nth(1)).not.toBeVisible();
  await expect(summaryEditButtons.nth(2)).not.toBeVisible();
  await expect(summaryEditButtons.nth(3)).not.toBeVisible();
  await expect(summaryEditButtons.nth(4)).not.toBeVisible();

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
    'Parsec SaaS',
    // Using Parsec Account as auth
    'Parsec Account',
  ]);
  await summaryNext.click();

  await expect(summaryContainer).toBeHidden();
  await expect(modal.locator('.creation-page')).toBeVisible();
  await page.waitForTimeout(1000);

  await expect(modal.locator('.creation-page')).toBeHidden();
  await expect(modal.locator('.created-page')).toBeVisible();
  await modal.locator('.created-page-footer').locator('ion-button').click();
  await expect(modal).toBeHidden();
  await page.waitForTimeout(1000);
  await expect(page).toBeWorkspacePage();
  await page.release();
});
