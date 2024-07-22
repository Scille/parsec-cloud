// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { Locator, Page } from '@playwright/test';
import { expect } from '@tests/pw/helpers/assertions';
import { DEFAULT_ORGANIZATION_INFORMATION, DEFAULT_USER_INFORMATION } from '@tests/pw/helpers/data';
import { msTest } from '@tests/pw/helpers/fixtures';
import { fillInputModal, fillIonInput } from '@tests/pw/helpers/utils';

// cspell:disable-next-line
const _HOST = 'saas-demo-v3-mightyfairy.parsec.cloud';
// cspell:disable-next-line
const _PAYLOAD = 'xBCy2YVGB31DPzcxGZbGVUt7';
const BOOTSTRAP_ADDR = `parsec3://${_HOST}/BlackMesa?no_ssl=true&a=bootstrap_organization&p=${_PAYLOAD}`;

async function openCreateOrganizationModal(page: Page): Promise<Locator> {
  await page.locator('#create-organization-button').click();
  await page.locator('.popover-viewport').getByRole('listitem').nth(0).click();
  const modal = page.locator('.create-organization-modal');
  await modal.locator('.server-choice-item').nth(0).click();
  await modal.locator('.server-modal-footer').locator('ion-button').nth(1).click();
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

async function mockLogin(page: Page, success: boolean, timeout?: boolean): Promise<void> {
  const TOKEN = btoa(
    JSON.stringify({
      email: DEFAULT_USER_INFORMATION.email,
      // eslint-disable-next-line camelcase
      is_staff: true,
      // eslint-disable-next-line camelcase
      token_type: 'access',
      // eslint-disable-next-line camelcase
      user_id: DEFAULT_USER_INFORMATION.id,
      exp: 4242,
      iat: 0,
    }),
  );

  await page.route('**/api/token', async (route) => {
    if (success) {
      route.fulfill({
        status: 200,
        json: {
          access: TOKEN,
        },
      });
    } else {
      if (timeout) {
        route.abort('timedout');
      } else {
        route.fulfill({
          status: 401,
          json: {
            type: 'login_error',
            errors: [{ code: 'invalid', attr: 'email', detail: 'Cannot log in' }],
          },
        });
      }
    }
  });
}

async function mockUserInfo(page: Page): Promise<void> {
  await page.route(`**/users/${DEFAULT_USER_INFORMATION.id}`, async (route) => {
    route.fulfill({
      status: 200,
      json: {
        id: DEFAULT_USER_INFORMATION.id,
        // eslint-disable-next-line camelcase
        created_at: '2024-07-15T13:21:32.141317Z',
        email: DEFAULT_USER_INFORMATION.email,
        client: {
          firstname: DEFAULT_USER_INFORMATION.firstName,
          lastname: DEFAULT_USER_INFORMATION.lastName,
          id: '1337',
        },
      },
    });
  });
}

async function mockCreateOrganization(page: Page): Promise<void> {
  await page.route(`**/users/${DEFAULT_USER_INFORMATION.id}/clients/1337/organizations`, async (route) => {
    route.fulfill({
      status: 201,
      json: {
        // eslint-disable-next-line camelcase
        bootstrap_link: BOOTSTRAP_ADDR,
      },
    });
  });
}

msTest('Go through saas org creation process', async ({ home }) => {
  const modal = await openCreateOrganizationModal(home);

  await mockLogin(home, true);
  await mockUserInfo(home);
  await mockCreateOrganization(home);

  const bmsContainer = modal.locator('.saas-login-container');
  await expect(bmsContainer.locator('.modal-header__title')).toHaveText('Link you customer account to your new organization');
  const bmsNext = bmsContainer.locator('.saas-login-footer').locator('ion-button').nth(0);
  await expect(bmsNext).toHaveDisabledAttribute();
  await fillIonInput(bmsContainer.locator('ion-input').nth(0), DEFAULT_USER_INFORMATION.email);
  await expect(bmsNext).toHaveDisabledAttribute();
  await fillIonInput(bmsContainer.locator('ion-input').nth(1), DEFAULT_USER_INFORMATION.password);
  await expect(bmsNext).not.toHaveDisabledAttribute();

  await cancelAndResume(home, bmsContainer);
  await bmsNext.click();

  const orgNameContainer = modal.locator('.organization-name-page');
  await expect(bmsContainer).toBeHidden();
  await expect(orgNameContainer).toBeVisible();
  await expect(orgNameContainer.locator('.modal-header__title')).toHaveText('Create an organization');
  const orgNameNext = modal.locator('.organization-name-page-footer').locator('ion-button');
  await expect(orgNameNext).toHaveDisabledAttribute();

  await cancelAndResume(home, orgNameContainer);

  // Invalid org name
  await fillIonInput(orgNameContainer.locator('ion-input').nth(0), 'Invalid Org N@me');
  await expect(orgNameNext).toHaveDisabledAttribute();
  const orgNameError = orgNameContainer.locator('#org-name-input').locator('.form-error');
  await expect(orgNameError).toBeVisible();
  await expect(orgNameError).toHaveText('Only letters, digits, underscores and hyphens. No spaces.');

  // Back to good name
  await fillIonInput(orgNameContainer.locator('ion-input'), DEFAULT_ORGANIZATION_INFORMATION.name);
  await expect(orgNameError).toBeHidden();
  await expect(orgNameNext).not.toHaveDisabledAttribute();

  await orgNameNext.click();

  const authContainer = modal.locator('.authentication-page');
  const authPrevious = modal.locator('.authentication-page-footer').locator('ion-button').nth(0);
  const authNext = modal.locator('.authentication-page-footer').locator('ion-button').nth(1);
  await expect(orgNameContainer).toBeHidden();
  await expect(bmsContainer).toBeHidden();
  await expect(authContainer).toBeVisible();
  await expect(authContainer.locator('.modal-header__title')).toHaveText('Authentication');
  await expect(authPrevious).toBeVisible();
  await expect(authPrevious).not.toHaveDisabledAttribute();
  await expect(authNext).toBeVisible();
  await expect(authNext).toHaveDisabledAttribute();
  await fillIonInput(authContainer.locator('.choose-password').locator('ion-input').nth(0), DEFAULT_USER_INFORMATION.password);
  await expect(authNext).toHaveDisabledAttribute();
  await fillIonInput(authContainer.locator('.choose-password').locator('ion-input').nth(1), DEFAULT_USER_INFORMATION.password);
  await expect(authNext).not.toHaveDisabledAttribute();

  // Try cancelling
  await cancelAndResume(home, authContainer);

  // Password too simple
  await fillIonInput(authContainer.locator('.choose-password').locator('ion-input').nth(0), 'EasyP@ssw0rd');
  await expect(authContainer.locator('.password-level__text')).toHaveText('Low');
  await expect(authNext).toHaveDisabledAttribute();

  // Back to complicated password
  await fillIonInput(authContainer.locator('.choose-password').locator('ion-input').nth(0), DEFAULT_USER_INFORMATION.password);
  await expect(authContainer.locator('.password-level__text')).toHaveText('Strong');
  await expect(authNext).not.toHaveDisabledAttribute();

  // Check does not match
  await fillIonInput(authContainer.locator('.choose-password').locator('ion-input').nth(1), `${DEFAULT_USER_INFORMATION.password}-extra`);
  await expect(authNext).toHaveDisabledAttribute();
  const matchError = authContainer.locator('.choose-password').locator('.inputs-container-item').nth(1).locator('.form-helperText').nth(1);
  await expect(matchError).toBeVisible();
  await expect(matchError).toHaveText('Do not match');

  // Back to matching password
  await fillIonInput(authContainer.locator('.choose-password').locator('ion-input').nth(1), DEFAULT_USER_INFORMATION.password);
  await expect(authNext).not.toHaveDisabledAttribute();
  await expect(matchError).toBeHidden();

  await authNext.click();

  const summaryContainer = modal.locator('.summary-page');
  const summaryPrevious = modal.locator('.summary-page-footer').locator('ion-button').nth(0);
  const summaryNext = modal.locator('.summary-page-footer').locator('ion-button').nth(1);
  await expect(orgNameContainer).toBeHidden();
  await expect(bmsContainer).toBeHidden();
  await expect(authContainer).toBeHidden();
  await expect(summaryContainer).toBeVisible();
  await expect(summaryContainer.locator('.modal-header__title')).toHaveText('Overview of your organization');
  await expect(summaryPrevious).toBeVisible();
  await expect(summaryPrevious).not.toHaveDisabledAttribute();
  await expect(summaryNext).toBeVisible();
  await expect(summaryNext).not.toHaveDisabledAttribute();

  await cancelAndResume(home, summaryContainer);

  await expect(summaryContainer.locator('.summary-item__label')).toHaveText([
    'Organization',
    'Full name',
    'Email',
    'Server choice',
    'Authentication method',
  ]);
  await expect(summaryContainer.locator('.summary-item__text')).toHaveText([
    DEFAULT_ORGANIZATION_INFORMATION.name,
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

  await expect(modal.locator('.created-page')).toBeVisible();
  await expect(modal.locator('.creation-page')).toBeHidden();
  await expect(modal.locator('.created-page').locator('.closeBtn')).toBeHidden();
  await modal.locator('.created-page-footer').locator('ion-button').click();
  await expect(modal).toBeHidden();
});

msTest('Go through saas org creation process from bootstrap link', async ({ home }) => {
  await home.locator('#create-organization-button').click();
  await home.locator('.popover-viewport').getByRole('listitem').nth(1).click();
  await fillInputModal(home, BOOTSTRAP_ADDR);
  const modal = home.locator('.create-organization-modal');

  await mockLogin(home, true);
  await mockUserInfo(home);
  await mockCreateOrganization(home);

  const bmsContainer = modal.locator('.saas-login-container');
  await expect(bmsContainer.locator('.modal-header__title')).toHaveText('Link you customer account to your new organization');
  const bmsNext = bmsContainer.locator('.saas-login-footer').locator('ion-button').nth(0);
  await expect(bmsNext).toHaveDisabledAttribute();
  await fillIonInput(bmsContainer.locator('ion-input').nth(0), DEFAULT_USER_INFORMATION.email);
  await expect(bmsNext).toHaveDisabledAttribute();
  await fillIonInput(bmsContainer.locator('ion-input').nth(1), DEFAULT_USER_INFORMATION.password);
  await expect(bmsNext).not.toHaveDisabledAttribute();

  await bmsNext.click();

  const authContainer = modal.locator('.authentication-page');
  const authPrevious = modal.locator('.authentication-page-footer').locator('ion-button').nth(0);
  const authNext = modal.locator('.authentication-page-footer').locator('ion-button').nth(1);
  await expect(bmsContainer).toBeHidden();
  await expect(authContainer).toBeVisible();
  await expect(authContainer.locator('.modal-header__title')).toHaveText('Authentication');
  await expect(authPrevious).toBeVisible();
  await expect(authPrevious).not.toHaveDisabledAttribute();
  await expect(authNext).toBeVisible();
  await expect(authNext).toHaveDisabledAttribute();
  await fillIonInput(authContainer.locator('.choose-password').locator('ion-input').nth(0), DEFAULT_USER_INFORMATION.password);
  await expect(authNext).toHaveDisabledAttribute();
  await fillIonInput(authContainer.locator('.choose-password').locator('ion-input').nth(1), DEFAULT_USER_INFORMATION.password);
  await expect(authNext).not.toHaveDisabledAttribute();

  await authNext.click();

  const summaryContainer = modal.locator('.summary-page');
  const summaryPrevious = modal.locator('.summary-page-footer').locator('ion-button').nth(0);
  const summaryNext = modal.locator('.summary-page-footer').locator('ion-button').nth(1);
  await expect(bmsContainer).toBeHidden();
  await expect(authContainer).toBeHidden();
  await expect(summaryContainer).toBeVisible();
  await expect(summaryContainer.locator('.modal-header__title')).toHaveText('Overview of your organization');
  await expect(summaryPrevious).toBeVisible();
  await expect(summaryPrevious).not.toHaveDisabledAttribute();
  await expect(summaryNext).toBeVisible();
  await expect(summaryNext).not.toHaveDisabledAttribute();

  await cancelAndResume(home, summaryContainer);

  await expect(summaryContainer.locator('.summary-item__label')).toHaveText([
    'Organization',
    'Full name',
    'Email',
    'Server choice',
    'Authentication method',
  ]);
  await expect(summaryContainer.locator('.summary-item__text')).toHaveText([
    DEFAULT_ORGANIZATION_INFORMATION.name,
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
  await home.waitForTimeout(1000);

  await expect(modal.locator('.created-page')).toBeVisible();
  await expect(modal.locator('.creation-page')).toBeHidden();
  await expect(modal.locator('.created-page').locator('.closeBtn')).toBeHidden();
  await modal.locator('.created-page-footer').locator('ion-button').click();
  await expect(modal).toBeHidden();
});

msTest('Open account creation', async ({ home }) => {
  const modal = await openCreateOrganizationModal(home);

  const bmsContainer = modal.locator('.saas-login-container');
  await expect(bmsContainer.locator('.saas-login-footer').locator('ion-button').nth(1)).toHaveText('Create an account');
  const newTabPromise = home.waitForEvent('popup');
  await bmsContainer.locator('.saas-login-footer').locator('ion-button').nth(1).click();
  const newTab = await newTabPromise;
  await expect(newTab).toHaveURL(/^https:\/\/parsec\.cloud(?:\/en)\/tarification\/?$/);
});

msTest('Fail to login to BMS', async ({ home }) => {
  const modal = await openCreateOrganizationModal(home);

  await mockLogin(home, false);

  const bmsContainer = modal.locator('.saas-login-container');
  const bmsNext = bmsContainer.locator('.saas-login-footer').locator('ion-button').nth(0);
  await fillIonInput(bmsContainer.locator('ion-input').nth(0), DEFAULT_USER_INFORMATION.email);
  await fillIonInput(bmsContainer.locator('ion-input').nth(1), DEFAULT_USER_INFORMATION.password);
  await bmsNext.click();

  await expect(bmsContainer.locator('.login-button-error')).toBeVisible();
  await expect(bmsContainer.locator('.login-button-error')).toHaveText('Cannot log in. Please check your email and password.');
});

msTest('Cannot reach the BMS', async ({ home }) => {
  const modal = await openCreateOrganizationModal(home);

  await mockLogin(home, false, true);

  const bmsContainer = modal.locator('.saas-login-container');
  const bmsNext = bmsContainer.locator('.saas-login-footer').locator('ion-button').nth(0);
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

  await mockLogin(home, true);
  await mockUserInfo(home);
  await mockCreateOrganization(home);

  const bmsContainer = modal.locator('.saas-login-container');
  const bmsNext = bmsContainer.locator('.saas-login-footer').locator('ion-button').nth(0);
  await fillIonInput(bmsContainer.locator('ion-input').nth(0), DEFAULT_USER_INFORMATION.email);
  await fillIonInput(bmsContainer.locator('ion-input').nth(1), DEFAULT_USER_INFORMATION.password);
  await bmsNext.click();

  const orgNameContainer = modal.locator('.organization-name-page');
  const orgNameNext = modal.locator('.organization-name-page-footer').locator('ion-button');
  await fillIonInput(orgNameContainer.locator('ion-input'), DEFAULT_ORGANIZATION_INFORMATION.name);
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
    DEFAULT_ORGANIZATION_INFORMATION.name,
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
  await fillIonInput(orgNameContainer.locator('ion-input'), `${DEFAULT_ORGANIZATION_INFORMATION.name}2`);
  await orgNameNext.click();

  await authNext.click();

  await expect(summaryContainer).toBeVisible();
  await expect(orgNameContainer).toBeHidden();
  await expect(summaryContainer.locator('.summary-item__text')).toHaveText([
    `${DEFAULT_ORGANIZATION_INFORMATION.name}2`,
    DEFAULT_USER_INFORMATION.name,
    DEFAULT_USER_INFORMATION.email,
    'Parsec SaaS',
    'Password',
  ]);
  await expect(summaryNext).toBeTrulyEnabled();
});
