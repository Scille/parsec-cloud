// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { Locator, Page } from '@playwright/test';
import {
  DEFAULT_USER_INFORMATION,
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

async function openCreateOrganizationModal(page: Page): Promise<Locator> {
  await page.locator('#create-organization-button').click();
  await page.locator('.popover-viewport').getByRole('listitem').nth(0).click();
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

msTest('Go through saas org creation process', async ({ home }) => {
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
  msTest(`Org creation error (${testInfo.status} - ${testInfo.code})`, async ({ home }) => {
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
    const authNext = modal.locator('.authentication-page-footer').locator('ion-button').nth(1);
    await fillIonInput(authContainer.locator('.choose-password').locator('ion-input').nth(0), DEFAULT_USER_INFORMATION.password);
    await fillIonInput(authContainer.locator('.choose-password').locator('ion-input').nth(1), DEFAULT_USER_INFORMATION.password);
    await authNext.click();

    const summaryContainer = modal.locator('.summary-page');
    const summaryNext = modal.locator('.summary-page-footer').locator('ion-button').nth(1);
    await summaryNext.click();

    await home.waitForTimeout(1000);

    await expect(summaryContainer.locator('.login-button-error')).toBeVisible();
    await expect(summaryContainer.locator('.login-button-error')).toHaveText(testInfo.expectedMessage);
  });

msTest('Go through saas org creation process from bootstrap link', async ({ context }) => {
  const page = (await context.newPage()) as MsPage;
  // Making sure that the testbed is recognized as the saas server
  const testbedUrl = new URL(process.env.TESTBED_SERVER ?? '');
  await setupNewPage(page, { trialServers: 'unknown.host', saasServers: testbedUrl.host });

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

  const authContainer = modal.locator('.authentication-page');
  const authPrevious = modal.locator('.authentication-page-footer').locator('ion-button').nth(0);
  const authNext = modal.locator('.authentication-page-footer').locator('ion-button').nth(1);
  await expect(bmsContainer).toBeHidden();
  await expect(authContainer).toBeVisible();
  await expect(authContainer.locator('.modal-header-title__text')).toHaveText('Authentication');
  await expect(authPrevious).not.toBeVisible();
  await expect(authNext).toBeVisible();
  await expect(authNext).toHaveDisabledAttribute();
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

  await page.release();
});

msTest('Open customer account creation', async ({ home }) => {
  const modal = await openCreateOrganizationModal(home);

  const bmsContainer = modal.locator('.saas-login');
  await expect(bmsContainer.locator('.saas-login-footer').locator('.create-account__link')).toHaveText('Create an account');
  await openExternalLink(
    home,
    bmsContainer.locator('.saas-login-footer').locator('.create-account__link'),
    /https:\/\/sign-dev\.parsec\.cloud.*/,
  );
});

msTest('Fail to login to BMS', async ({ home }) => {
  const modal = await openCreateOrganizationModal(home);

  await MockBms.mockLogin(home, { POST: { errors: { status: 401 } } });

  const bmsContainer = modal.locator('.saas-login');
  const bmsNext = bmsContainer.locator('.saas-login-button').locator('.saas-login-button__item').nth(1);
  await fillIonInput(bmsContainer.locator('ion-input').nth(0), DEFAULT_USER_INFORMATION.email);
  await fillIonInput(bmsContainer.locator('ion-input').nth(1), DEFAULT_USER_INFORMATION.password);
  await bmsNext.click();

  await expect(bmsContainer.locator('.login-button-error')).toBeVisible();
  await expect(bmsContainer.locator('.login-button-error')).toHaveText('Cannot log in. Please check your email and password.');
});

msTest('Cannot reach the BMS', async ({ home }) => {
  await MockBms.mockLogin(home, { POST: { timeout: true } });
  const modal = await openCreateOrganizationModal(home);

  const bmsContainer = modal.locator('.saas-login');
  const bmsNext = bmsContainer.locator('.saas-login-button').locator('.saas-login-button__item').nth(1);
  await fillIonInput(bmsContainer.locator('ion-input').nth(0), DEFAULT_USER_INFORMATION.email);
  await fillIonInput(bmsContainer.locator('ion-input').nth(1), DEFAULT_USER_INFORMATION.password);
  await bmsNext.click();

  await expect(bmsContainer.locator('.login-button-error')).toBeVisible();
  await expect(bmsContainer.locator('.login-button-error')).toHaveText(
    'Could not reach the server. Make sure that you are online and try again.',
  );
});

msTest('Edit from summary', async ({ home }) => {
  const modal = await openCreateOrganizationModal(home);

  await MockBms.mockLogin(home);
  await MockBms.mockUserRoute(home);
  await MockBms.mockCreateOrganization(home, getTestbedBootstrapAddr(home.orgInfo.name));

  const bmsContainer = modal.locator('.saas-login');
  const bmsNext = bmsContainer.locator('.saas-login-button').locator('.saas-login-button__item').nth(1);
  await fillIonInput(bmsContainer.locator('ion-input').nth(0), DEFAULT_USER_INFORMATION.email);
  await fillIonInput(bmsContainer.locator('ion-input').nth(1), DEFAULT_USER_INFORMATION.password);
  await bmsNext.click();

  const orgNameContainer = modal.locator('.organization-name-page');
  const orgNameNext = modal.locator('.organization-name-page-footer').locator('ion-button').nth(1);
  await fillIonInput(orgNameContainer.locator('ion-input'), home.orgInfo.name);
  await orgNameNext.click();

  const authContainer = modal.locator('.authentication-page');
  const authNext = modal.locator('.authentication-page-footer').locator('ion-button').nth(1);
  await fillIonInput(authContainer.locator('.choose-password').locator('ion-input').nth(0), DEFAULT_USER_INFORMATION.password);
  await fillIonInput(authContainer.locator('.choose-password').locator('ion-input').nth(1), DEFAULT_USER_INFORMATION.password);
  await authNext.click();

  const summaryContainer = modal.locator('.summary-page');
  const summaryNext = modal.locator('.summary-page-footer').locator('ion-button').nth(1);

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

msTest('Try to create an org with custom order', async ({ home }) => {
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

msTest('Try to create an org without being a client', async ({ home }) => {
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
