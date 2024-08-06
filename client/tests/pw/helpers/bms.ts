// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { Page, Route } from '@playwright/test';
import { DEFAULT_ORGANIZATION_DATA_SLICE, DEFAULT_ORGANIZATION_INFORMATION, DEFAULT_USER_INFORMATION } from '@tests/pw/helpers/data';
import { DateTime } from 'luxon';

async function mockRoute(
  page: Page,
  url: string,
  methods: Array<string>,
  options: MockRouteOptions | undefined,
  handler: (route: Route) => Promise<void>,
): Promise<void> {
  await page.route(url, async (route) => {
    if (!methods.some((method) => method.toLowerCase() === route.request().method().toLowerCase())) {
      await route.continue();
    }

    if (options) {
      if (options.timeout) {
        await route.abort('timedout');
        return;
      }
      if (options.errors) {
        await route.fulfill({
          status: options.errors.status ?? 400,
          json: {
            type: 'error',
            errors: [{ code: 'error', attr: options.errors.attribute ?? 'attr', detail: 'Default error' }],
          },
        });
        return;
      }
    }
    await handler(route);
  });
}

interface MockRouteOptions {
  timeout?: boolean;
  errors?: {
    attribute?: string;
    status?: number;
  };
}

async function mockLogin(page: Page, options?: MockRouteOptions): Promise<void> {
  const TOKEN_RAW = {
    email: DEFAULT_USER_INFORMATION.email,
    // eslint-disable-next-line camelcase
    is_staff: true,
    // eslint-disable-next-line camelcase
    token_type: 'access',
    // eslint-disable-next-line camelcase
    user_id: DEFAULT_USER_INFORMATION.id,
    exp: DateTime.utc().plus({ years: 42 }).toJSDate().valueOf(),
    iat: 0,
  };
  const TOKEN = btoa(JSON.stringify(TOKEN_RAW));

  await mockRoute(page, '**/api/token', ['POST'], options, async (route) => {
    await route.fulfill({
      status: 200,
      json: {
        access: TOKEN,
        refresh: TOKEN,
      },
    });
  });
}

async function mockUserInfo(page: Page, options?: MockRouteOptions): Promise<void> {
  await mockRoute(page, `**/users/${DEFAULT_USER_INFORMATION.id}`, ['GET'], options, async (route) => {
    await route.fulfill({
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

async function mockCreateOrganization(page: Page, bootstrapAddr: string, options?: MockRouteOptions): Promise<void> {
  await mockRoute(
    page,
    `**/users/${DEFAULT_USER_INFORMATION.id}/clients/${DEFAULT_USER_INFORMATION.clientId}/organizations`,
    ['POST'],
    options,
    async (route) => {
      await route.fulfill({
        status: 201,
        json: {
          // eslint-disable-next-line camelcase
          bootstrap_link: bootstrapAddr,
        },
      });
    },
  );
}

async function mockListOrganizations(page: Page, options?: MockRouteOptions): Promise<void> {
  await mockRoute(
    page,
    `**/users/${DEFAULT_USER_INFORMATION.id}/clients/${DEFAULT_USER_INFORMATION.clientId}/organizations`,
    ['GET'],
    options,
    async (route) => {
      await route.fulfill({
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
              // eslint-disable-next-line camelcase
              parsec_id: DEFAULT_ORGANIZATION_INFORMATION.name,
              suffix: DEFAULT_ORGANIZATION_INFORMATION.name,
              // eslint-disable-next-line camelcase
              stripe_subscription_id: 'stripe_id',
              // eslint-disable-next-line camelcase
              bootstrap_link: '',
            },
            {
              pk: `${DEFAULT_ORGANIZATION_INFORMATION.bmsId}-2`,
              // eslint-disable-next-line camelcase
              created_at: '2024-12-04T00:00:00.000',
              // eslint-disable-next-line camelcase
              expiration_date: null,
              name: DEFAULT_ORGANIZATION_INFORMATION.name,
              // eslint-disable-next-line camelcase
              parsec_id: `${DEFAULT_ORGANIZATION_INFORMATION.name}-2`,
              suffix: `${DEFAULT_ORGANIZATION_INFORMATION.name}-2`,
              // eslint-disable-next-line camelcase
              stripe_subscription_id: 'stripe_id2',
              // eslint-disable-next-line camelcase
              bootstrap_link: '',
            },
          ],
        },
      });
    },
  );
}

interface MockOrganizationStatsOverload {
  dataSize?: number;
  metadataSize?: number;
  freeSliceSize?: number;
  payingSliceSize?: number;
  users?: number;
  activeUsers?: number;
  status?: string;
  usersPerProfileDetails?: {
    [profile: string]: { active: number; revoked: number };
  };
}

async function mockOrganizationStats(page: Page, overload: MockOrganizationStatsOverload = {}, options?: MockRouteOptions): Promise<void> {
  const usersPerProfileDetail: { [profile: string]: { active: number; revoked: number } } = {};
  usersPerProfileDetail.ADMIN =
    overload.usersPerProfileDetails && overload.usersPerProfileDetails.ADMIN
      ? overload.usersPerProfileDetails.ADMIN
      : { active: 4, revoked: 1 };
  usersPerProfileDetail.STANDARD =
    overload.usersPerProfileDetails && overload.usersPerProfileDetails.STANDARD
      ? overload.usersPerProfileDetails.STANDARD
      : { active: 54, revoked: 1 };
  usersPerProfileDetail.OUTSIDER =
    overload.usersPerProfileDetails && overload.usersPerProfileDetails.OUTSIDER
      ? overload.usersPerProfileDetails.OUTSIDER
      : { active: 1, revoked: 142 };

  await mockRoute(
    page,
    // eslint-disable-next-line max-len
    `**/users/${DEFAULT_USER_INFORMATION.id}/clients/${DEFAULT_USER_INFORMATION.clientId}/organizations/${DEFAULT_ORGANIZATION_INFORMATION.bmsId}/stats`,
    ['GET'],
    options,
    async (route) => {
      await route.fulfill({
        status: 200,
        json: {
          // eslint-disable-next-line camelcase
          users_per_profile_detail: usersPerProfileDetail,
          // eslint-disable-next-line camelcase
          data_size: overload.dataSize ?? 400000000000,
          // eslint-disable-next-line camelcase
          metadata_size: overload.metadataSize ?? 400000000,
          // eslint-disable-next-line camelcase
          free_slice_size: overload.freeSliceSize ?? DEFAULT_ORGANIZATION_DATA_SLICE.free,
          // eslint-disable-next-line camelcase
          paying_slice_size: overload.payingSliceSize ?? DEFAULT_ORGANIZATION_DATA_SLICE.paying,
          users: overload.users ?? 203,
          // eslint-disable-next-line camelcase
          active_users: overload.activeUsers ?? 59,
          status: overload.status ?? 'ok',
        },
      });
    },
  );
}

interface MockOrganizationStatusOverload {
  activeUsersLimit?: number;
  isBootstrapped?: boolean;
  isFrozen?: boolean;
  isInitialized?: boolean;
  outsiderAllowed?: boolean;
}

async function mockOrganizationStatus(
  page: Page,
  overload: MockOrganizationStatusOverload = {},
  options?: MockRouteOptions,
): Promise<void> {
  await mockRoute(
    page,
    `**/users/${DEFAULT_USER_INFORMATION.id}/clients/${DEFAULT_USER_INFORMATION.clientId}/organizations/status`,
    ['GET'],
    options,
    async (route) => {
      await route.fulfill({
        status: 200,
        json: {
          // eslint-disable-next-line camelcase
          active_users_limit: overload.activeUsersLimit ?? 1000,
          // eslint-disable-next-line camelcase
          is_bootstrapped: overload.isBootstrapped ?? true,
          // eslint-disable-next-line camelcase
          is_frozen: overload.isFrozen ?? false,
          // eslint-disable-next-line camelcase
          is_initialized: overload.isInitialized ?? true,
          // eslint-disable-next-line camelcase
          user_profile_outsider_allowed: overload.outsiderAllowed ?? true,
        },
      });
    },
  );
}

async function mockGetInvoices(page: Page, count: number, options?: MockRouteOptions): Promise<void> {
  await mockRoute(
    page,
    `**/users/${DEFAULT_USER_INFORMATION.id}/clients/${DEFAULT_USER_INFORMATION.clientId}/invoices`,
    ['GET'],
    options,
    async (route) => {
      const BASE_DATE = DateTime.fromISO('1988-04-07');
      await route.fulfill({
        status: 200,
        json: {
          count: count,
          results: Array.from(Array(count).keys()).map((index) => {
            return {
              id: `Id${index}`,
              pdf: `https://fake/pdfs/${index}.pdf`,
              // eslint-disable-next-line camelcase
              period_start: BASE_DATE.plus({ months: index }).toFormat('yyyy-LL-dd'),
              // eslint-disable-next-line camelcase
              period_end: BASE_DATE.plus({ month: index + 1 }).toFormat('yyyy-LL-dd'),
              total: Math.round(Math.random() * 1000),
              status: ['paid', 'draft', 'open'][Math.floor(Math.random() * 3)],
              organization: DEFAULT_ORGANIZATION_INFORMATION.name,
            };
          }),
        },
      });
    },
  );
}

interface MockBillingDetailsOverload {
  cardsCount?: number;
  sepaCount?: number;
  email?: string;
  name?: string;
  address?: string;
}

async function mockBillingDetails(page: Page, overload: MockBillingDetailsOverload = {}, options?: MockRouteOptions): Promise<void> {
  await mockRoute(
    page,
    `**/users/${DEFAULT_USER_INFORMATION.id}/clients/${DEFAULT_USER_INFORMATION.clientId}/billing_details`,
    ['GET'],
    options,
    async (route) => {
      const paymentMethods = [];
      for (let i = 0; i < (overload.cardsCount ?? 1); i++) {
        paymentMethods.push({
          type: 'card',
          id: `card${i}`,
          brand: 'mastercard',
          // eslint-disable-next-line camelcase
          exp_date: '12/47',
          // eslint-disable-next-line camelcase
          last_digits: '4444',
          default: true,
        });
      }
      for (let i = 0; i < (overload.sepaCount ?? 1); i++) {
        paymentMethods.push({
          type: 'debit',
          id: `debit${i}`,
          // eslint-disable-next-line camelcase
          bank_name: 'Bank',
          // eslint-disable-next-line camelcase
          last_digits: '1234',
          default: overload.cardsCount === undefined || overload.cardsCount === 0 ? true : false,
        });
      }
      route.fulfill({
        status: 200,
        json: {
          email: overload.email ?? DEFAULT_USER_INFORMATION.email,
          name: overload.name ?? `${DEFAULT_USER_INFORMATION.firstName} ${DEFAULT_USER_INFORMATION.lastName}`,
          address: overload.address ?? DEFAULT_USER_INFORMATION.address.full,
          // eslint-disable-next-line camelcase
          payment_methods: paymentMethods,
        },
      });
    },
  );
}

async function mockAddPaymentMethod(page: Page, options?: MockRouteOptions): Promise<void> {
  await mockRoute(
    page,
    `**/users/${DEFAULT_USER_INFORMATION.id}/clients/${DEFAULT_USER_INFORMATION.clientId}/add_payment_method`,
    ['PUT'],
    options,
    async (route) => {
      await route.fulfill({
        status: 200,
        json: {
          // eslint-disable-next-line camelcase
          payment_method: '123456',
        },
      });
    },
  );
}

async function mockSetDefaultPaymentMethod(page: Page, options?: MockRouteOptions): Promise<void> {
  await mockRoute(
    page,
    `**/users/${DEFAULT_USER_INFORMATION.id}/clients/${DEFAULT_USER_INFORMATION.clientId}/default_payment_method`,
    ['PATCH'],
    options,
    async (route) => {
      await route.fulfill({
        status: 200,
        json: {
          // eslint-disable-next-line camelcase
          payment_method: '123456',
        },
      });
    },
  );
}

async function mockDeletePaymentMethod(page: Page, options?: MockRouteOptions): Promise<void> {
  await mockRoute(
    page,
    `**/users/${DEFAULT_USER_INFORMATION.id}/clients/${DEFAULT_USER_INFORMATION.clientId}/delete_payment_method`,
    ['POST'],
    options,
    async (route) => {
      await route.fulfill({
        status: 200,
        json: {
          // eslint-disable-next-line camelcase
          payment_method: '123456',
        },
      });
    },
  );
}

async function mockUpdateUser(page: Page, options?: MockRouteOptions): Promise<void> {
  await mockRoute(page, `**/users/${DEFAULT_USER_INFORMATION.id}`, ['PATCH'], options, async (route) => {
    await route.fulfill({
      status: 200,
    });
  });
}

async function mockUpdateEmail(page: Page, options?: MockRouteOptions): Promise<void> {
  await mockRoute(page, `**/users/${DEFAULT_USER_INFORMATION.id}/update_email`, ['POST'], options, async (route) => {
    await route.fulfill({
      status: 200,
    });
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
  mockBillingDetails,
  mockAddPaymentMethod,
  mockSetDefaultPaymentMethod,
  mockDeletePaymentMethod,
  mockUpdateEmail,
  mockUpdateUser,
};
