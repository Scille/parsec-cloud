// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { Page } from '@playwright/test';
import { DEFAULT_ORGANIZATION_INFORMATION, DEFAULT_USER_INFORMATION } from '@tests/pw/helpers/data';

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

async function mockCreateOrganization(page: Page, bootstrapAddr: string): Promise<void> {
  await page.route(`**/users/${DEFAULT_USER_INFORMATION.id}/clients/1337/organizations`, async (route) => {
    route.fulfill({
      status: 201,
      json: {
        // eslint-disable-next-line camelcase
        bootstrap_link: bootstrapAddr,
      },
    });
  });
}

async function mockListOrganizations(page: Page): Promise<void> {
  await page.route(`**/users/${DEFAULT_USER_INFORMATION.id}/clients/${DEFAULT_USER_INFORMATION.clientId}/organizations`, async (route) => {
    route.fulfill({
      status: 200,
      json: {
        results: [
          {
            pk: DEFAULT_ORGANIZATION_INFORMATION.bmsId,
            // eslint-disable-next-line camelcase
            created_at: '2024-12-04T00:00:00.000',
            // eslint-disable-next-line camelcase
            expiration_date: null,
            name: DEFAULT_ORGANIZATION_INFORMATION.name,
            parsecId: DEFAULT_ORGANIZATION_INFORMATION.name,
            // eslint-disable-next-line camelcase
            stripe_subscription_id: 'stripe_id',
            bootstrapLink: '',
          },
        ],
      },
    });
  });
}

async function mockOrganizationStats(page: Page): Promise<void> {
  await page.route(
    `**/users/${DEFAULT_USER_INFORMATION.id}/clients/${DEFAULT_USER_INFORMATION.clientId}/organizations/stats`,
    async (route) => {
      route.fulfill({
        status: 200,
        json: {
          // eslint-disable-next-line camelcase
          data_size: 13374242,
          status: 'ok',
          users: 5,
        },
      });
    },
  );
}

interface StatusData {
  // eslint-disable-next-line camelcase
  active_users_limit?: number;
  // eslint-disable-next-line camelcase
  is_bootstrapped?: boolean;
  // eslint-disable-next-line camelcase
  is_frozen?: boolean;
  // eslint-disable-next-line camelcase
  is_initialized?: boolean;
  // eslint-disable-next-line camelcase
  user_profile_outsider_allowed?: boolean;
}

async function mockOrganizationStatus(page: Page, overload: StatusData = {}): Promise<void> {
  await page.route(
    `**/users/${DEFAULT_USER_INFORMATION.id}/clients/${DEFAULT_USER_INFORMATION.clientId}/organizations/status`,
    async (route) => {
      route.fulfill({
        status: 200,
        json: {
          // eslint-disable-next-line camelcase
          active_users_limit: overload.active_users_limit ?? 1000,
          // eslint-disable-next-line camelcase
          is_bootstrapped: overload.is_bootstrapped ?? true,
          // eslint-disable-next-line camelcase
          is_frozen: overload.is_frozen ?? false,
          // eslint-disable-next-line camelcase
          is_initialized: overload.is_initialized ?? true,
          // eslint-disable-next-line camelcase
          user_profile_outsider_allowed: overload.user_profile_outsider_allowed ?? true,
        },
      });
    },
  );
}

async function mockGetInvoices(page: Page, { fail, count }: { fail?: boolean; count?: number }): Promise<void> {
  await page.route(`**/users/${DEFAULT_USER_INFORMATION.id}/clients/${DEFAULT_USER_INFORMATION.clientId}/invoices`, async (route) => {
    if (fail) {
      await route.fulfill({
        status: 400,
        json: {
          type: 'error',
          errors: [{ code: 'invalid', attr: 'null', detail: 'An error occurred' }],
        },
      });
    } else {
      await route.fulfill({
        status: 200,
        json: {
          count: count,
          result: Array.from(Array(count).keys()).map((index) => {
            return {
              id: `Id${index}`,
              pdf: `https://fake/pdfs/${index}.pdf`,
              // eslint-disable-next-line camelcase
              period_start: '2024-07-01',
              // eslint-disable-next-line camelcase
              period_end: '2024-07-01',
              total: 13.37,
              status: ['paid', 'draft', 'open'][Math.floor(Math.random() * 3)],
              organization: `Org${index}`,
            };
          }),
        },
      });
    }
  });
}

export const MockBms = {
  mockLogin,
  mockUserInfo,
  mockCreateOrganization,
  mockListOrganizations,
  mockOrganizationStats,
  mockOrganizationStatus,
  mockGetInvoices,
};
