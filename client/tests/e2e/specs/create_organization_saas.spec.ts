// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { Locator, Page, TestInfo } from '@playwright/test';
import {
  DEFAULT_USER_INFORMATION,
  DisplaySize,
  MockBms,
  MsPage,
  expect,
  fillInputModal,
  fillIonInput,
  getTestbedBootstrapAddr,
  msTest,
  openExternalLink,
  setupNewPage,
} from '@tests/e2e/helpers';
import { randomInt } from 'crypto';
import path from 'path';

async function openCreateOrganizationModal(page: MsPage): Promise<Locator> {
  await page.locator('#create-organization-button').click();
  const smallDisplay = (await page.getDisplaySize()) === DisplaySize.Small;

  if (smallDisplay) {
    await page.locator('.create-join-modal-list__item').nth(0).click();
  } else {
    await page.locator('.popover-viewport').getByRole('listitem').nth(0).click();
  }
  const modal = page.locator('.create-organization-modal');
  await modal.locator('.server-choice-item').nth(0).click();
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

msTest('Go through saas org creation process', { tag: '@important' }, async ({ context }) => {
  const home = (await context.newPage()) as MsPage;
  await setupNewPage(home, { enableStripe: true });

  const modal = await openCreateOrganizationModal(home);

  const uniqueOrgName = `${home.orgInfo.name}-${randomInt(2 ** 47)}`;

  await MockBms.mockLogin(home);
  await MockBms.mockUserRoute(home);
  await MockBms.mockCreateOrganization(home, getTestbedBootstrapAddr(uniqueOrgName));

  const bmsContainer = modal.locator('.saas-login');
  await expect(bmsContainer.locator('.modal-header-title__text')).toHaveText('Link your customer account to your new organization');
  const bmsNext = bmsContainer.locator('.saas-login-button').locator('.saas-login-button__item').nth(1);
  await expect(bmsNext).toHaveDisabledAttribute();
  await fillIonInput(bmsContainer.locator('ion-input').nth(0), DEFAULT_USER_INFORMATION.email);
  await expect(bmsNext).toHaveDisabledAttribute();
  await fillIonInput(bmsContainer.locator('ion-input').nth(1), DEFAULT_USER_INFORMATION.password);
  await expect(bmsNext).toNotHaveDisabledAttribute();

  await cancelAndResume(home, bmsContainer);
  await bmsNext.click();

  const orgNameContainer = modal.locator('.organization-name-page');
  await expect(bmsContainer).toBeHidden();
  await expect(orgNameContainer).toBeVisible();
  await expect(orgNameContainer.locator('.modal-header-title__text')).toHaveText('Create an organization');
  const orgNameNext = modal.locator('.organization-name-page-footer').locator('ion-button').nth(1);
  await expect(orgNameNext).toHaveDisabledAttribute();

  await cancelAndResume(home, orgNameContainer);

  // Invalid org name
  await fillIonInput(orgNameContainer.locator('ion-input').nth(0), 'Invalid Org N@me');
  await expect(orgNameNext).toHaveDisabledAttribute();
  const orgNameError = orgNameContainer.locator('#org-name-input').locator('.form-error');
  await expect(orgNameError).toBeVisible();
  await expect(orgNameError).toHaveText('Only letters, digits, underscores and hyphens. No spaces.');

  // Back to good name
  await fillIonInput(orgNameContainer.locator('ion-input'), uniqueOrgName);
  await expect(orgNameError).toBeHidden();
  await expect(orgNameNext).toNotHaveDisabledAttribute();

  await orgNameNext.click();

  const authContainer = modal.locator('.authentication-page');
  const authPrevious = modal.locator('.authentication-page-footer').locator('ion-button').nth(0);
  const authNext = modal.locator('.authentication-page-footer').locator('ion-button').nth(1);
  await expect(orgNameContainer).toBeHidden();
  await expect(bmsContainer).toBeHidden();
  await expect(authContainer).toBeVisible();
  await expect(authContainer.locator('.modal-header-title__text')).toHaveText('Authentication');
  await expect(authPrevious).toBeVisible();
  await expect(authPrevious).toNotHaveDisabledAttribute();
  await expect(authNext).toBeVisible();
  await expect(authNext).toHaveDisabledAttribute();

  const authRadio = authContainer.locator('.choose-auth-page').locator('.radio-list-item:visible');
  await expect(authRadio).toHaveAuthentication({ pkiDisabled: true, keyringDisabled: true, ssoDisabled: true });
  await authRadio.nth(0).click();

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

  const summaryContainer = modal.locator('.summary-page');
  const summaryPrevious = modal.locator('.summary-page-footer').locator('ion-button').nth(0);
  const summaryNext = modal.locator('.summary-page-footer').locator('ion-button').nth(1);
  const summaryEditButtons = modal.locator('.summary-item__button');
  await expect(orgNameContainer).toBeHidden();
  await expect(bmsContainer).toBeHidden();
  await expect(authContainer).toBeHidden();
  await expect(summaryContainer).toBeVisible();
  await expect(summaryContainer.locator('.modal-header-title__text')).toHaveText('Overview of your organization');
  await expect(summaryPrevious).toBeVisible();
  await expect(summaryPrevious).toNotHaveDisabledAttribute();
  await expect(summaryNext).toBeVisible();
  await expect(summaryNext).toNotHaveDisabledAttribute();
  await expect(summaryContainer.locator('.tos')).toHaveText('By using Parsec, I accept the Terms and Conditions and Privacy Policy');

  // Only the authentication and org name fields can be updated
  await expect(summaryEditButtons.nth(0)).toBeVisible();
  await expect(summaryEditButtons.nth(1)).not.toBeVisible();
  await expect(summaryEditButtons.nth(2)).not.toBeVisible();
  await expect(summaryEditButtons.nth(3)).not.toBeVisible();
  await expect(summaryEditButtons.nth(4)).toBeVisible();

  await cancelAndResume(home, summaryContainer);

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
    'Password',
  ]);
  await summaryNext.click();

  await expect(orgNameContainer).toBeHidden();
  await expect(bmsContainer).toBeHidden();
  await expect(authContainer).toBeHidden();
  await expect(summaryContainer).toBeHidden();
  await expect(modal.locator('.creation-page')).toBeVisible();
  await expect(modal.locator('.creation-page').locator('.closeBtn')).toBeHidden();
  await home.waitForTimeout(1000);

  await expect(modal.locator('.creation-page')).toBeHidden();
  await expect(modal.locator('.created-page')).toBeVisible();
  await expect(modal.locator('.created-page').locator('.closeBtn')).toBeHidden();
  await modal.locator('.created-page-footer').locator('ion-button').click();
  await expect(modal).toBeHidden();
  await home.waitForTimeout(1000);
  await expect(home).toBeWorkspacePage();
});

for (const testInfo of [
  {
    expectedMessage: 'This organization name is not available, please choose another one.',
    status: 500,
    code: 'parsec_bad_status',
  },
  {
    expectedMessage: 'Failed to create the organization (reason: Unknown).',
    status: 500,
    code: 'unknown',
  },
  {
    expectedMessage: 'Failed to contact server. Please make sure you are online.',
    status: 0,
  },
])
  msTest(`Org creation error (${testInfo.status} - ${testInfo.code})`, async ({ context }) => {
    const home = (await context.newPage()) as MsPage;
    await setupNewPage(home, { enableStripe: true });

    const modal = await openCreateOrganizationModal(home);

    const uniqueOrgName = `${home.orgInfo.name}-${randomInt(2 ** 47)}`;

    await MockBms.mockLogin(home);
    await MockBms.mockUserRoute(home);

    const routeOptions: any = {};

    if (testInfo.status === 0) {
      routeOptions.timeout = true;
    } else {
      routeOptions.errors = {
        code: testInfo.code,
        status: testInfo.status,
      };
    }

    await MockBms.mockCreateOrganization(home, getTestbedBootstrapAddr(uniqueOrgName), { POST: routeOptions });

    const bmsContainer = modal.locator('.saas-login');
    const bmsNext = bmsContainer.locator('.saas-login-button').locator('.saas-login-button__item').nth(1);
    await fillIonInput(bmsContainer.locator('ion-input').nth(0), DEFAULT_USER_INFORMATION.email);
    await fillIonInput(bmsContainer.locator('ion-input').nth(1), DEFAULT_USER_INFORMATION.password);
    await bmsNext.click();

    const orgNameContainer = modal.locator('.organization-name-page');
    const orgNameNext = modal.locator('.organization-name-page-footer').locator('ion-button').nth(1);
    await fillIonInput(orgNameContainer.locator('ion-input'), uniqueOrgName);
    await orgNameNext.click();

    const authContainer = modal.locator('.authentication-page');

    const authRadio = authContainer.locator('.choose-auth-page').locator('.radio-list-item:visible');
    await expect(authRadio).toHaveAuthentication({ pkiDisabled: true, keyringDisabled: true, ssoDisabled: true });
    await authRadio.nth(0).click();
    const authNext = modal.locator('.authentication-page-footer').locator('ion-button').nth(1);
    await fillIonInput(authContainer.locator('.choose-password').locator('ion-input').nth(0), DEFAULT_USER_INFORMATION.password);
    await fillIonInput(authContainer.locator('.choose-password').locator('ion-input').nth(1), DEFAULT_USER_INFORMATION.password);
    await authNext.click();

    const summaryContainer = modal.locator('.summary-page');
    const summaryNext = modal.locator('.summary-page-footer').locator('ion-button').nth(1);
    await summaryNext.click();

    await home.waitForTimeout(1000);

    await expect(summaryContainer.locator('.form-error')).toBeVisible();
    await expect(summaryContainer.locator('.form-error')).toHaveText(testInfo.expectedMessage);
  });

msTest('Go through saas org creation process from bootstrap link', async ({ context }) => {
  const page = (await context.newPage()) as MsPage;
  // Making sure that the testbed is recognized as the saas server
  const testbedUrl = new URL(process.env.TESTBED_SERVER ?? '');
  await setupNewPage(page, { trialServers: 'unknown.host', saasServers: testbedUrl.host, enableStripe: true });

  const uniqueOrgName = `${page.orgInfo.name}-${randomInt(2 ** 47)}`;

  await page.locator('#create-organization-button').click();
  await page.locator('.popover-viewport').getByRole('listitem').nth(1).click();
  await fillInputModal(page, getTestbedBootstrapAddr(uniqueOrgName));
  const modal = page.locator('.create-organization-modal');

  // Mock the BMS
  await MockBms.mockLogin(page);
  await MockBms.mockUserRoute(page);
  await MockBms.mockCreateOrganization(page, getTestbedBootstrapAddr(uniqueOrgName));

  const bmsContainer = modal.locator('.saas-login');
  await expect(bmsContainer.locator('.modal-header-title__text')).toHaveText('Link your customer account to your new organization');
  const bmsNext = bmsContainer.locator('.saas-login-button').locator('.saas-login-button__item').nth(1);
  await expect(bmsNext).toHaveDisabledAttribute();
  await fillIonInput(bmsContainer.locator('ion-input').nth(0), DEFAULT_USER_INFORMATION.email);
  await expect(bmsNext).toHaveDisabledAttribute();
  await fillIonInput(bmsContainer.locator('ion-input').nth(1), DEFAULT_USER_INFORMATION.password);
  await expect(bmsNext).toNotHaveDisabledAttribute();

  await bmsNext.click();

  const orgNameContainer = modal.locator('.organization-name-page');
  await expect(bmsContainer).toBeHidden();
  await expect(orgNameContainer).toBeVisible();
  await expect(orgNameContainer.locator('.modal-header-title__text')).toHaveText('Create an organization');
  const orgNameNext = modal.locator('.organization-name-page-footer').locator('ion-button').nth(1);
  await expect(orgNameNext).toBeTrulyEnabled();
  await expect(orgNameContainer.locator('ion-input').nth(0).locator('input')).toHaveValue(uniqueOrgName);
  await orgNameNext.click();

  const authContainer = modal.locator('.authentication-page');
  const authPrevious = modal.locator('.authentication-page-footer').locator('ion-button').nth(0);
  const authNext = modal.locator('.authentication-page-footer').locator('ion-button').nth(1);
  await expect(bmsContainer).toBeHidden();
  await expect(authContainer).toBeVisible();
  await expect(authContainer.locator('.modal-header-title__text')).toHaveText('Authentication');
  await expect(authPrevious).not.toBeVisible();
  await expect(authNext).toBeVisible();
  await expect(authNext).toHaveDisabledAttribute();

  const authRadio = authContainer.locator('.choose-auth-page').locator('.radio-list-item:visible');
  await expect(authRadio).toHaveAuthentication({ pkiDisabled: true, keyringDisabled: true });
  await authRadio.nth(0).click();
  await fillIonInput(authContainer.locator('.choose-password').locator('ion-input').nth(0), DEFAULT_USER_INFORMATION.password);
  await expect(authNext).toHaveDisabledAttribute();
  await fillIonInput(authContainer.locator('.choose-password').locator('ion-input').nth(1), DEFAULT_USER_INFORMATION.password);
  await expect(authNext).toNotHaveDisabledAttribute();

  await authNext.click();

  const summaryContainer = modal.locator('.summary-page');
  const summaryPrevious = modal.locator('.summary-page-footer').locator('ion-button').nth(0);
  const summaryNext = modal.locator('.summary-page-footer').locator('ion-button').nth(1);
  const summaryEditButtons = modal.locator('.summary-item__button');
  await expect(bmsContainer).toBeHidden();
  await expect(authContainer).toBeHidden();
  await expect(summaryContainer).toBeVisible();
  await expect(summaryContainer.locator('.modal-header-title__text')).toHaveText('Overview of your organization');
  await expect(summaryPrevious).toBeVisible();
  await expect(summaryPrevious).toNotHaveDisabledAttribute();
  await expect(summaryNext).toBeVisible();
  await expect(summaryNext).toNotHaveDisabledAttribute();

  // Only the authentication field can be updated
  await expect(summaryEditButtons.nth(0)).not.toBeVisible();
  await expect(summaryEditButtons.nth(1)).not.toBeVisible();
  await expect(summaryEditButtons.nth(2)).not.toBeVisible();
  await expect(summaryEditButtons.nth(3)).not.toBeVisible();
  await expect(summaryEditButtons.nth(4)).toBeVisible();

  await cancelAndResume(page, summaryContainer);

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
    'Password',
  ]);
  await summaryNext.click();

  await expect(bmsContainer).toBeHidden();
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

msTest('Open customer account creation', async ({ context }) => {
  const home = (await context.newPage()) as MsPage;
  await setupNewPage(home, { enableStripe: true });

  const modal = await openCreateOrganizationModal(home);

  const bmsContainer = modal.locator('.saas-login');
  await expect(bmsContainer.locator('.saas-login-footer').locator('.create-account__link')).toHaveText('Create an account');
  await openExternalLink(
    home,
    bmsContainer.locator('.saas-login-footer').locator('.create-account__link'),
    /https:\/\/sign-dev\.parsec\.cloud.*/,
  );
});

msTest('Fail to login to BMS', async ({ context }) => {
  const home = (await context.newPage()) as MsPage;
  await setupNewPage(home, { enableStripe: true });

  const modal = await openCreateOrganizationModal(home);

  await MockBms.mockLogin(home, { POST: { errors: { status: 401 } } });

  const bmsContainer = modal.locator('.saas-login');
  await expect(bmsContainer.locator('.modal-header-title__text')).toHaveText('Link your customer account to your new organization');
  const bmsNext = bmsContainer.locator('.saas-login-button').locator('.saas-login-button__item').nth(1);
  await expect(bmsNext).toBeTrulyDisabled();
  await fillIonInput(bmsContainer.locator('ion-input').nth(0), DEFAULT_USER_INFORMATION.email);
  await expect(bmsNext).toBeTrulyDisabled();
  await fillIonInput(bmsContainer.locator('ion-input').nth(1), DEFAULT_USER_INFORMATION.password);
  await expect(bmsNext).toBeTrulyEnabled();
  await expect(bmsNext).toHaveText('Log in');
  await bmsNext.click();

  await expect(bmsContainer.locator('.login-button-error')).toBeVisible();
  await expect(bmsContainer.locator('.login-button-error')).toHaveText('Cannot log in. Please check your email and password.');
});

msTest('Cannot reach the BMS', async ({ context }) => {
  const home = (await context.newPage()) as MsPage;
  await setupNewPage(home, { enableStripe: true });

  await MockBms.mockLogin(home, { POST: { timeout: true } });
  const modal = await openCreateOrganizationModal(home);

  const bmsContainer = modal.locator('.saas-login');
  await expect(bmsContainer.locator('.modal-header-title__text')).toHaveText('Link your customer account to your new organization');
  const bmsNext = bmsContainer.locator('.saas-login-button').locator('.saas-login-button__item').nth(1);
  await expect(bmsNext).toBeTrulyDisabled();
  await fillIonInput(bmsContainer.locator('ion-input').nth(0), DEFAULT_USER_INFORMATION.email);
  await expect(bmsNext).toBeTrulyDisabled();
  await fillIonInput(bmsContainer.locator('ion-input').nth(1), DEFAULT_USER_INFORMATION.password);
  await expect(bmsNext).toBeTrulyEnabled();
  await expect(bmsNext).toHaveText('Log in');
  await bmsNext.click();

  await expect(bmsContainer.locator('.login-button-error')).toBeVisible();
  await expect(bmsContainer.locator('.login-button-error')).toHaveText(
    'Could not reach the server. Make sure that you are online and try again.',
  );
});

msTest('Edit from summary', async ({ context }) => {
  const home = (await context.newPage()) as MsPage;
  await setupNewPage(home, { enableStripe: true });

  const modal = await openCreateOrganizationModal(home);

  await MockBms.mockLogin(home);
  await MockBms.mockUserRoute(home);
  await MockBms.mockCreateOrganization(home, getTestbedBootstrapAddr(home.orgInfo.name));

  const bmsContainer = modal.locator('.saas-login');
  await expect(bmsContainer.locator('.modal-header-title__text')).toHaveText('Link your customer account to your new organization');
  const bmsNext = bmsContainer.locator('.saas-login-button').locator('.saas-login-button__item').nth(1);
  await expect(bmsNext).toBeTrulyDisabled();
  await fillIonInput(bmsContainer.locator('ion-input').nth(0), DEFAULT_USER_INFORMATION.email);
  await expect(bmsNext).toBeTrulyDisabled();
  await fillIonInput(bmsContainer.locator('ion-input').nth(1), DEFAULT_USER_INFORMATION.password);
  await expect(bmsNext).toBeTrulyEnabled();
  await bmsNext.click();

  const orgNameContainer = modal.locator('.organization-name-page');
  await expect(orgNameContainer).toBeVisible();
  const orgNameNext = modal.locator('.organization-name-page-footer').locator('ion-button').nth(1);
  await fillIonInput(orgNameContainer.locator('ion-input'), home.orgInfo.name);
  await expect(orgNameNext).toBeTrulyEnabled();
  await orgNameNext.click();

  const authContainer = modal.locator('.authentication-page');
  const authNext = modal.locator('.authentication-page-footer').locator('ion-button').nth(1);

  await expect(authContainer).toBeVisible();
  const authRadio = authContainer.locator('.choose-auth-page').locator('.radio-list-item:visible');
  await expect(authRadio).toHaveAuthentication({ pkiDisabled: true, keyringDisabled: true, ssoDisabled: true });
  await authRadio.nth(0).click();

  await expect(authNext).toBeTrulyDisabled();
  await fillIonInput(authContainer.locator('.choose-password').locator('ion-input').nth(0), DEFAULT_USER_INFORMATION.password);
  await fillIonInput(authContainer.locator('.choose-password').locator('ion-input').nth(1), DEFAULT_USER_INFORMATION.password);
  await expect(authNext).toBeTrulyEnabled();
  await authNext.click();

  const summaryContainer = modal.locator('.summary-page');
  const summaryNext = modal.locator('.summary-page-footer').locator('ion-button').nth(1);
  await expect(summaryContainer).toBeVisible();

  await expect(summaryContainer.locator('.summary-item__label')).toHaveText([
    'Organization',
    'Full name',
    'Email',
    'Server choice',
    'Authentication method',
  ]);
  await expect(summaryContainer.locator('.summary-item__text')).toHaveText([
    home.orgInfo.name,
    DEFAULT_USER_INFORMATION.name,
    DEFAULT_USER_INFORMATION.email,
    'Parsec SaaS',
    'Password',
  ]);
  const editButton = summaryContainer.locator('.summary-item-container').nth(0).locator('.summary-item__button');
  await expect(editButton).toBeVisible();
  await expect(editButton).toHaveText('Edit');
  await editButton.click();

  await expect(summaryContainer).toBeHidden();
  await expect(orgNameContainer).toBeVisible();
  await fillIonInput(orgNameContainer.locator('ion-input'), `${home.orgInfo.name}-2`);
  await orgNameNext.click();

  await authNext.click();

  await expect(summaryContainer).toBeVisible();
  await expect(orgNameContainer).toBeHidden();
  await expect(summaryContainer.locator('.summary-item__text')).toHaveText([
    `${home.orgInfo.name}-2`,
    DEFAULT_USER_INFORMATION.name,
    DEFAULT_USER_INFORMATION.email,
    'Parsec SaaS',
    'Password',
  ]);
  await expect(summaryNext).toBeTrulyEnabled();
});

msTest('Try to create an org with custom order', async ({ context }) => {
  const home = (await context.newPage()) as MsPage;
  await setupNewPage(home, { enableStripe: true });

  const modal = await openCreateOrganizationModal(home);

  await MockBms.mockLogin(home);
  await MockBms.mockUserRoute(home, { billingSystem: 'CUSTOM_ORDER' });

  const bmsContainer = modal.locator('.saas-login');
  await fillIonInput(bmsContainer.locator('ion-input').nth(0), DEFAULT_USER_INFORMATION.email);
  await fillIonInput(bmsContainer.locator('ion-input').nth(1), DEFAULT_USER_INFORMATION.password);
  const button = bmsContainer.locator('.saas-login-button').locator('.saas-login-button__item').nth(1);
  await expect(button).toHaveText('Log in');
  await expect(button).toBeTrulyEnabled();
  await button.click();
  await expect(modal).toBeHidden();
  await expect(home).toShowInformationModal(
    // eslint-disable-next-line max-len
    'Clients with a custom contract are not allowed to directly create organizations. If you need another organization, please make a request from your customer area.',
    'Error',
  );
});

msTest('Try to create an org without being a client', async ({ context }) => {
  const home = (await context.newPage()) as MsPage;
  await setupNewPage(home, { enableStripe: true });

  const modal = await openCreateOrganizationModal(home);

  await MockBms.mockLogin(home);
  await MockBms.mockUserRoute(home, { noClient: true });

  const bmsContainer = modal.locator('.saas-login');
  await fillIonInput(bmsContainer.locator('ion-input').nth(0), DEFAULT_USER_INFORMATION.email);
  await fillIonInput(bmsContainer.locator('ion-input').nth(1), DEFAULT_USER_INFORMATION.password);
  const button = bmsContainer.locator('.saas-login-button').locator('.saas-login-button__item').nth(1);
  await expect(button).toHaveText('Log in');
  await expect(button).toBeTrulyEnabled();
  await button.click();
  await expect(modal).toBeHidden();
  await expect(home).toShowInformationModal('Your account is not a client account.', 'Error');
});

for (const displaySize of ['small', 'large']) {
  msTest(`Go through saas org creation process with authority key on ${displaySize} display`, async ({ context }, testInfo: TestInfo) => {
    const home = (await context.newPage()) as MsPage;
    await setupNewPage(home, { enableStripe: true });

    if (displaySize === DisplaySize.Small) {
      await home.setDisplaySize(DisplaySize.Small);
    }

    const modal = await openCreateOrganizationModal(home);

    const uniqueOrgName = `${home.orgInfo.name}-${randomInt(2 ** 47)}`;

    await MockBms.mockLogin(home);
    await MockBms.mockUserRoute(home);
    await MockBms.mockCreateOrganization(home, getTestbedBootstrapAddr(uniqueOrgName));

    const bmsContainer = modal.locator('.saas-login');
    await expect(bmsContainer.locator('.modal-header-title__text')).toHaveText('Link your customer account to your new organization');
    const bmsNext = bmsContainer.locator('.saas-login-button').locator('.saas-login-button__item').nth(1);
    await expect(bmsNext).toHaveDisabledAttribute();
    await fillIonInput(bmsContainer.locator('ion-input').nth(0), DEFAULT_USER_INFORMATION.email);
    await expect(bmsNext).toHaveDisabledAttribute();
    await fillIonInput(bmsContainer.locator('ion-input').nth(1), DEFAULT_USER_INFORMATION.password);
    await expect(bmsNext).toNotHaveDisabledAttribute();

    await bmsNext.click();

    const orgNameContainer = modal.locator('.organization-name-page');
    await expect(bmsContainer).toBeHidden();
    await expect(orgNameContainer).toBeVisible();
    await expect(orgNameContainer.locator('.modal-header-title__text')).toHaveText('Create an organization');
    const orgNameNext = modal.locator('.organization-name-page-footer').locator('ion-button').nth(1);
    await expect(orgNameNext).toHaveDisabledAttribute();

    await fillIonInput(orgNameContainer.locator('ion-input'), uniqueOrgName);
    await expect(orgNameNext).toNotHaveDisabledAttribute();

    await expect(orgNameContainer.locator('.advanced-settings')).toHaveText('Advanced Settings');
    await expect(orgNameContainer.locator('.sequester-container')).toBeHidden();
    await orgNameContainer.locator('.advanced-settings').click();
    await expect(orgNameContainer.locator('.sequester-container')).toBeVisible();

    await orgNameContainer.locator('.sequester-container').locator('ion-toggle').click();
    await expect(orgNameNext).toBeTrulyDisabled();
    await expect(orgNameContainer.locator('.sequester-container').locator('.upload-key__button')).toBeVisible();
    await expect(orgNameContainer.locator('.sequester-container').locator('.upload-key__button')).toHaveText('Add authority key');
    await expect(orgNameContainer.locator('.sequester-container').locator('.upload-key-update')).toBeHidden();

    const fileChooserPromise = home.waitForEvent('filechooser');
    await orgNameContainer.locator('.sequester-container').locator('.upload-key__button').click();
    const fileChooser = await fileChooserPromise;
    expect(fileChooser.isMultiple()).toBe(false);
    const importPath = path.join(testInfo.config.rootDir, 'data', 'public_key.pem');
    await fileChooser.setFiles([importPath]);

    await expect(orgNameContainer.locator('.sequester-container').locator('.upload-key__button')).toBeHidden();
    await expect(orgNameContainer.locator('.sequester-container').locator('.upload-key-update')).toBeVisible();
    await expect(orgNameContainer.locator('.sequester-container').locator('.upload-key-update').locator('ion-text')).toHaveText(
      'public_key.pem',
    );
    await expect(
      orgNameContainer.locator('.sequester-container').locator('.upload-key-update').locator('.upload-key-update__button'),
    ).toHaveText('Update');

    await expect(orgNameNext).toBeTrulyEnabled();

    await orgNameNext.click();

    const authContainer = modal.locator('.authentication-page');
    const authPrevious = modal.locator('.authentication-page-footer').locator('ion-button').nth(0);
    const authNext = modal.locator('.authentication-page-footer').locator('ion-button').nth(1);
    await expect(orgNameContainer).toBeHidden();
    await expect(bmsContainer).toBeHidden();
    await expect(authContainer).toBeVisible();
    await expect(authContainer.locator('.modal-header-title__text')).toHaveText('Authentication');
    await expect(authPrevious).toBeVisible();
    await expect(authPrevious).toNotHaveDisabledAttribute();
    await expect(authNext).toBeVisible();
    await expect(authNext).toHaveDisabledAttribute();

    const authRadio = authContainer.locator('.choose-auth-page').locator('.radio-list-item:visible');
    await expect(authRadio).toHaveAuthentication({ pkiDisabled: true, keyringDisabled: true, ssoDisabled: true });
    await authRadio.nth(0).click();
    await fillIonInput(authContainer.locator('.choose-password').locator('ion-input').nth(0), DEFAULT_USER_INFORMATION.password);
    await expect(authNext).toHaveDisabledAttribute();
    await fillIonInput(authContainer.locator('.choose-password').locator('ion-input').nth(1), DEFAULT_USER_INFORMATION.password);
    await expect(authNext).toNotHaveDisabledAttribute();

    await authNext.click();

    const summaryContainer = modal.locator('.summary-page');
    const summaryPrevious = modal.locator('.summary-page-footer').locator('ion-button').nth(0);
    const summaryNext = modal.locator('.summary-page-footer').locator('ion-button').nth(1);
    const summaryEditButtons = modal.locator('.summary-item__button');
    await expect(orgNameContainer).toBeHidden();
    await expect(bmsContainer).toBeHidden();
    await expect(authContainer).toBeHidden();
    await expect(summaryContainer).toBeVisible();
    await expect(summaryContainer.locator('.modal-header-title__text')).toHaveText('Overview of your organization');
    await expect(summaryPrevious).toBeVisible();
    await expect(summaryPrevious).toNotHaveDisabledAttribute();
    await expect(summaryNext).toBeVisible();
    await expect(summaryNext).toNotHaveDisabledAttribute();
    await expect(summaryContainer.locator('.tos')).toHaveText('By using Parsec, I accept the Terms and Conditions and Privacy Policy');

    // Only the authentication and org name/authority key fields can be updated
    await expect(summaryEditButtons.nth(0)).toBeVisible();
    await expect(summaryEditButtons.nth(1)).toBeVisible();
    await expect(summaryEditButtons.nth(2)).not.toBeVisible();
    await expect(summaryEditButtons.nth(3)).not.toBeVisible();
    await expect(summaryEditButtons.nth(4)).not.toBeVisible();
    await expect(summaryEditButtons.nth(5)).toBeVisible();

    await expect(summaryContainer.locator('.summary-item__label')).toHaveText([
      'Organization',
      'Data sequester',
      'Full name',
      'Email',
      'Server choice',
      'Authentication method',
    ]);
    await expect(summaryContainer.locator('.summary-item__text')).toHaveText([
      uniqueOrgName,
      'Authority key added',
      DEFAULT_USER_INFORMATION.name,
      DEFAULT_USER_INFORMATION.email,
      'Parsec SaaS',
      'Password',
    ]);
    await summaryNext.click();

    await expect(orgNameContainer).toBeHidden();
    await expect(bmsContainer).toBeHidden();
    await expect(authContainer).toBeHidden();
    await expect(summaryContainer).toBeHidden();
    await expect(modal.locator('.creation-page')).toBeVisible();
    await expect(modal.locator('.creation-page').locator('.closeBtn')).toBeHidden();
    await home.waitForTimeout(1000);

    await expect(modal.locator('.creation-page')).toBeHidden();
    await expect(modal.locator('.created-page')).toBeVisible();
    await expect(modal.locator('.created-page').locator('.closeBtn')).toBeHidden();
    await modal.locator('.created-page-footer').locator('ion-button').click();
    await expect(modal).toBeHidden();
    await home.waitForTimeout(1000);
    await expect(home).toBeWorkspacePage();
  });

  msTest(
    `Go through saas org creation process with invalid authority key on ${displaySize} display`,
    async ({ context }, testInfo: TestInfo) => {
      const home = (await context.newPage()) as MsPage;
      await setupNewPage(home, { enableStripe: true });

      if (displaySize === DisplaySize.Small) {
        await home.setDisplaySize(DisplaySize.Small);
      }

      const modal = await openCreateOrganizationModal(home);

      const uniqueOrgName = `${home.orgInfo.name}-${randomInt(2 ** 47)}`;

      await MockBms.mockLogin(home);
      await MockBms.mockUserRoute(home);
      await MockBms.mockCreateOrganization(home, getTestbedBootstrapAddr(uniqueOrgName));

      const bmsContainer = modal.locator('.saas-login');
      await expect(bmsContainer.locator('.modal-header-title__text')).toHaveText('Link your customer account to your new organization');
      const bmsNext = bmsContainer.locator('.saas-login-button').locator('.saas-login-button__item').nth(1);
      await expect(bmsNext).toHaveDisabledAttribute();
      await fillIonInput(bmsContainer.locator('ion-input').nth(0), DEFAULT_USER_INFORMATION.email);
      await expect(bmsNext).toHaveDisabledAttribute();
      await fillIonInput(bmsContainer.locator('ion-input').nth(1), DEFAULT_USER_INFORMATION.password);
      await expect(bmsNext).toNotHaveDisabledAttribute();

      await bmsNext.click();

      const orgNameContainer = modal.locator('.organization-name-page');
      await expect(bmsContainer).toBeHidden();
      await expect(orgNameContainer).toBeVisible();
      await expect(orgNameContainer.locator('.modal-header-title__text')).toHaveText('Create an organization');
      const orgNameNext = modal.locator('.organization-name-page-footer').locator('ion-button').nth(1);
      await expect(orgNameNext).toHaveDisabledAttribute();

      await fillIonInput(orgNameContainer.locator('ion-input'), uniqueOrgName);
      await expect(orgNameNext).toNotHaveDisabledAttribute();

      await orgNameContainer.locator('.advanced-settings').click();
      await orgNameContainer.locator('.sequester-container').locator('ion-toggle').click();
      await expect(orgNameNext).toBeTrulyDisabled();

      const fileChooserPromise = home.waitForEvent('filechooser');
      await orgNameContainer.locator('.sequester-container').locator('.upload-key__button').click();
      const fileChooser = await fileChooserPromise;
      expect(fileChooser.isMultiple()).toBe(false);
      const importPath = path.join(testInfo.config.rootDir, 'data', 'imports', 'text.txt');
      await fileChooser.setFiles([importPath]);

      await expect(orgNameContainer.locator('.sequester-container').locator('.upload-key-update').locator('ion-text')).toHaveText(
        'text.txt',
      );

      await expect(orgNameNext).toBeTrulyEnabled();

      await orgNameNext.click();

      const authContainer = modal.locator('.authentication-page');
      const authPrevious = modal.locator('.authentication-page-footer').locator('ion-button').nth(0);
      const authNext = modal.locator('.authentication-page-footer').locator('ion-button').nth(1);
      await expect(orgNameContainer).toBeHidden();
      await expect(bmsContainer).toBeHidden();
      await expect(authContainer).toBeVisible();
      await expect(authContainer.locator('.modal-header-title__text')).toHaveText('Authentication');
      await expect(authPrevious).toBeVisible();
      await expect(authPrevious).toNotHaveDisabledAttribute();
      await expect(authNext).toBeVisible();
      await expect(authNext).toHaveDisabledAttribute();

      const authRadio = authContainer.locator('.choose-auth-page').locator('.radio-list-item:visible');
      await expect(authRadio).toHaveAuthentication({ pkiDisabled: true, keyringDisabled: true, ssoDisabled: true });
      await authRadio.nth(0).click();
      await fillIonInput(authContainer.locator('.choose-password').locator('ion-input').nth(0), DEFAULT_USER_INFORMATION.password);
      await expect(authNext).toHaveDisabledAttribute();
      await fillIonInput(authContainer.locator('.choose-password').locator('ion-input').nth(1), DEFAULT_USER_INFORMATION.password);
      await expect(authNext).toNotHaveDisabledAttribute();

      await authNext.click();

      const summaryContainer = modal.locator('.summary-page');
      await modal.locator('.summary-page-footer').locator('ion-button').nth(1).click();

      await home.waitForTimeout(1000);

      await expect(summaryContainer.locator('.form-error')).toBeVisible();
      await expect(summaryContainer.locator('.form-error')).toHaveText('The provided authority key is not valid.');
    },
  );

  msTest(
    `Go through saas org creation process from bootstrap link and authority key on ${displaySize} display`,
    async ({ context }, testInfo: TestInfo) => {
      const page = (await context.newPage()) as MsPage;
      // Making sure that the testbed is recognized as the saas server
      const testbedUrl = new URL(process.env.TESTBED_SERVER ?? '');
      await setupNewPage(page, { trialServers: 'unknown.host', saasServers: testbedUrl.host, enableStripe: true });

      if (displaySize === DisplaySize.Small) {
        await page.setDisplaySize(DisplaySize.Small);
      }

      const uniqueOrgName = `${page.orgInfo.name}-${randomInt(2 ** 47)}`;

      await page.locator('#create-organization-button').click();
      if (displaySize === DisplaySize.Small) {
        await page.locator('.create-join-modal-list__item').nth(1).click();
      } else {
        await page.locator('.popover-viewport').getByRole('listitem').nth(1).click();
      }
      await fillInputModal(page, getTestbedBootstrapAddr(uniqueOrgName));
      const modal = page.locator('.create-organization-modal');

      // Mock the BMS
      await MockBms.mockLogin(page);
      await MockBms.mockUserRoute(page);
      await MockBms.mockCreateOrganization(page, getTestbedBootstrapAddr(uniqueOrgName));

      const bmsContainer = modal.locator('.saas-login');
      await expect(bmsContainer.locator('.modal-header-title__text')).toHaveText('Link your customer account to your new organization');
      const bmsNext = bmsContainer.locator('.saas-login-button').locator('.saas-login-button__item').nth(1);
      await expect(bmsNext).toHaveDisabledAttribute();
      await fillIonInput(bmsContainer.locator('ion-input').nth(0), DEFAULT_USER_INFORMATION.email);
      await expect(bmsNext).toHaveDisabledAttribute();
      await fillIonInput(bmsContainer.locator('ion-input').nth(1), DEFAULT_USER_INFORMATION.password);
      await expect(bmsNext).toNotHaveDisabledAttribute();

      await bmsNext.click();

      const orgNameContainer = modal.locator('.organization-name-page');
      await expect(bmsContainer).toBeHidden();
      await expect(orgNameContainer).toBeVisible();
      await expect(orgNameContainer.locator('.modal-header-title__text')).toHaveText('Create an organization');
      const orgNameNext = modal.locator('.organization-name-page-footer').locator('ion-button').nth(1);
      await expect(orgNameNext).toBeTrulyEnabled();
      await expect(orgNameContainer.locator('ion-input').nth(0).locator('input')).toHaveValue(uniqueOrgName);

      await orgNameContainer.locator('.advanced-settings').click();
      await orgNameContainer.locator('.sequester-container').locator('ion-toggle').click();
      await expect(orgNameNext).toBeTrulyDisabled();

      const fileChooserPromise = page.waitForEvent('filechooser');
      await orgNameContainer.locator('.sequester-container').locator('.upload-key__button').click();
      const fileChooser = await fileChooserPromise;
      expect(fileChooser.isMultiple()).toBe(false);
      const importPath = path.join(testInfo.config.rootDir, 'data', 'public_key.pem');
      await fileChooser.setFiles([importPath]);

      await expect(orgNameContainer.locator('.sequester-container').locator('.upload-key-update').locator('ion-text')).toHaveText(
        'public_key.pem',
      );

      await expect(orgNameNext).toBeTrulyEnabled();

      await orgNameNext.click();

      const authContainer = modal.locator('.authentication-page');
      const authPrevious = modal.locator('.authentication-page-footer').locator('ion-button').nth(0);
      const authNext = modal.locator('.authentication-page-footer').locator('ion-button').nth(1);
      await expect(bmsContainer).toBeHidden();
      await expect(authContainer).toBeVisible();
      await expect(authContainer.locator('.modal-header-title__text')).toHaveText('Authentication');
      await expect(authPrevious).not.toBeVisible();
      await expect(authNext).toBeVisible();
      await expect(authNext).toHaveDisabledAttribute();

      const authRadio = authContainer.locator('.choose-auth-page').locator('.radio-list-item:visible');
      await expect(authRadio).toHaveAuthentication({ pkiDisabled: true, keyringDisabled: true });
      await authRadio.nth(0).click();
      await fillIonInput(authContainer.locator('.choose-password').locator('ion-input').nth(0), DEFAULT_USER_INFORMATION.password);
      await expect(authNext).toHaveDisabledAttribute();
      await fillIonInput(authContainer.locator('.choose-password').locator('ion-input').nth(1), DEFAULT_USER_INFORMATION.password);
      await expect(authNext).toNotHaveDisabledAttribute();

      await authNext.click();

      const summaryContainer = modal.locator('.summary-page');
      const summaryPrevious = modal.locator('.summary-page-footer').locator('ion-button').nth(0);
      const summaryNext = modal.locator('.summary-page-footer').locator('ion-button').nth(1);
      const summaryEditButtons = modal.locator('.summary-item__button');
      await expect(bmsContainer).toBeHidden();
      await expect(authContainer).toBeHidden();
      await expect(summaryContainer).toBeVisible();
      await expect(summaryContainer.locator('.modal-header-title__text')).toHaveText('Overview of your organization');
      await expect(summaryPrevious).toBeVisible();
      await expect(summaryPrevious).toNotHaveDisabledAttribute();
      await expect(summaryNext).toBeVisible();
      await expect(summaryNext).toNotHaveDisabledAttribute();

      // Only the authentication and authority key fields can be updated
      await expect(summaryEditButtons.nth(0)).not.toBeVisible();
      await expect(summaryEditButtons.nth(1)).toBeVisible();
      await expect(summaryEditButtons.nth(2)).not.toBeVisible();
      await expect(summaryEditButtons.nth(3)).not.toBeVisible();
      await expect(summaryEditButtons.nth(4)).not.toBeVisible();
      await expect(summaryEditButtons.nth(5)).toBeVisible();

      await cancelAndResume(page, summaryContainer);

      await expect(summaryContainer.locator('.summary-item__label')).toHaveText([
        'Organization',
        'Data sequester',
        'Full name',
        'Email',
        'Server choice',
        'Authentication method',
      ]);
      await expect(summaryContainer.locator('.summary-item__text')).toHaveText([
        uniqueOrgName,
        'Authority key added',
        DEFAULT_USER_INFORMATION.name,
        DEFAULT_USER_INFORMATION.email,
        'Parsec SaaS',
        'Password',
      ]);
      await summaryNext.click();

      await expect(bmsContainer).toBeHidden();
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
    },
  );
}
