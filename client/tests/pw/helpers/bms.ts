// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { Page, Route } from '@playwright/test';
import {
  DEFAULT_ORGANIZATION_DATA_SLICE,
  DEFAULT_ORGANIZATION_INFORMATION,
  DEFAULT_USER_INFORMATION,
  UserData,
} from '@tests/pw/helpers/data';
import { DateTime } from 'luxon';

async function mockRoute(
  page: Page,
  url: string | RegExp,
  options: MockRouteOptions | undefined,
  handler: (route: Route) => Promise<void>,
): Promise<void> {
  async function _handleError(route: Route, options: MockMethodOptions): Promise<boolean> {
    if (options.timeout) {
      await route.abort('timedout');
      return true;
    }
    if (options.errors) {
      await route.fulfill({
        status: options.errors.status ?? 400,
        json: {
          type: 'error',
          errors: [{ code: 'error', attr: options.errors.attribute ?? 'attr', detail: 'Default error' }],
        },
      });
      return true;
    }
    return false;
  }

  await page.route(url, async (route) => {
    const method = route.request().method().toUpperCase();

    if (method === 'GET' && options && options.GET) {
      if (await _handleError(route, options.GET)) {
        return;
      }
    } else if (method === 'POST' && options && options.POST) {
      if (await _handleError(route, options.POST)) {
        return;
      }
    } else if (method === 'PUT' && options && options.PUT) {
      if (await _handleError(route, options.PUT)) {
        return;
      }
    } else if (method === 'PATCH' && options && options.PATCH) {
      if (await _handleError(route, options.PATCH)) {
        return;
      }
    }

    await handler(route);
  });
}

interface MockMethodOptions {
  timeout?: boolean;
  errors?: {
    attribute?: string;
    status?: number;
  };
}

interface MockRouteOptions {
  PATCH?: MockMethodOptions;
  GET?: MockMethodOptions;
  PUT?: MockMethodOptions;
  POST?: MockMethodOptions;
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

  await mockRoute(page, '**/api/token', options, async (route) => {
    await route.fulfill({
      status: 200,
      json: {
        access: TOKEN,
        refresh: TOKEN,
      },
    });
  });
}

async function mockUserRoute(page: Page, options?: MockRouteOptions): Promise<void> {
  await mockRoute(page, `**/users/${DEFAULT_USER_INFORMATION.id}`, options, async (route) => {
    if (route.request().method() === 'GET') {
      await route.fulfill({
        status: 200,
        json: {
          id: DEFAULT_USER_INFORMATION.id,
          // eslint-disable-next-line camelcase
          created_at: '2024-07-15T13:21:32.141317Z',
          email: UserData.email,
          client: {
            firstname: UserData.firstName,
            lastname: UserData.lastName,
            id: '1337',
            job: UserData.job,
            company: UserData.company,
            phone: UserData.phone,
          },
        },
      });
    } else if (route.request().method() === 'PATCH') {
      const data = await route.request().postDataJSON();
      if (data.client) {
        if (data.client.firstname) {
          UserData.firstName = data.client.firstname;
        }
        if (data.client.lastname) {
          UserData.lastName = data.client.lastname;
        }
        if (data.client.phone) {
          UserData.phone = data.client.phone;
        }
        if (data.client.job || data.client.job === null) {
          UserData.job = data.client.job;
        }
        if (data.client.company || data.client.job === null) {
          UserData.company = data.client.company;
        }
      }
      await route.fulfill({
        status: 200,
      });
    }
  });
}

async function mockCreateOrganization(page: Page, bootstrapAddr: string, options?: MockRouteOptions): Promise<void> {
  await mockRoute(
    page,
    `**/users/${DEFAULT_USER_INFORMATION.id}/clients/${DEFAULT_USER_INFORMATION.clientId}/organizations`,
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
    `*/**/users/${DEFAULT_USER_INFORMATION.id}/clients/${DEFAULT_USER_INFORMATION.clientId}/organizations/*/stats`,
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
    `**/users/${DEFAULT_USER_INFORMATION.id}/clients/${DEFAULT_USER_INFORMATION.clientId}/organizations/*/status`,
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

interface MockGetInvoicesOverload {
  count?: number;
}

async function mockGetInvoices(page: Page, overload: MockGetInvoicesOverload = {}, options?: MockRouteOptions): Promise<void> {
  await mockRoute(
    page,
    `**/users/${DEFAULT_USER_INFORMATION.id}/clients/${DEFAULT_USER_INFORMATION.clientId}/invoices`,
    options,
    async (route) => {
      let invoices = [];
      for (let year = 2019; year < 2022; year++) {
        for (let month = 1; month < 13; month++) {
          invoices.push({
            id: `Id${year}-${month}`,
            pdf: `https://fake/pdfs/${year}-${month}.pdf`,
            // eslint-disable-next-line camelcase
            period_start: DateTime.fromObject({ year: year, month: month }).toFormat('yyyy-LL-dd'),
            // eslint-disable-next-line camelcase
            period_end: DateTime.fromObject({ year: year, month: month }).endOf('month').toFormat('yyyy-LL-dd'),
            total: Math.round(Math.random() * 1000),
            status: ['paid', 'draft', 'open'][Math.floor(Math.random() * 3)],
            organization: DEFAULT_ORGANIZATION_INFORMATION.name,
            number: `${year}-${month}`,
            receiptNumber: `${year}-${month}`,
          });
        }
      }
      if (overload.count !== undefined) {
        invoices = invoices.slice(0, overload.count);
      }

      await route.fulfill({
        status: 200,
        json: {
          count: invoices.length,
          results: invoices,
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
  address?: {
    line1?: string;
    line2?: string;
    city?: string;
    postalCode?: string;
    country?: string;
  };
}

async function mockBillingDetails(page: Page, overload: MockBillingDetailsOverload = {}, options?: MockRouteOptions): Promise<void> {
  await mockRoute(
    page,
    `**/users/${DEFAULT_USER_INFORMATION.id}/clients/${DEFAULT_USER_INFORMATION.clientId}/billing_details`,
    options,
    async (route) => {
      if (route.request().method() === 'GET') {
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
        await route.fulfill({
          status: 200,
          json: {
            email: overload.email ?? UserData.email,
            name: overload.name ?? `${UserData.firstName} ${UserData.lastName}`,
            address: {
              line1: (overload.address && overload.address.line1) ?? UserData.address.line1,
              line2: (overload.address && overload.address.line2) ?? '',
              city: (overload.address && overload.address.city) ?? UserData.address.city,
              // eslint-disable-next-line camelcase
              postal_code: (overload.address && overload.address.postalCode) ?? UserData.address.postalCode,
              country: (overload.address && overload.address.country) ?? UserData.address.country,
            },
            // eslint-disable-next-line camelcase
            payment_methods: paymentMethods,
          },
        });
      } else if (route.request().method() === 'PATCH') {
        const data = await route.request().postDataJSON();
        if (data.address) {
          if (data.address.line1) {
            UserData.address.line1 = data.address.line1;
          }
          if (data.address.line2) {
            UserData.address.line2 = data.address.line2;
          }
          if (data.address.postal_code) {
            UserData.address.postalCode = data.address.postal_code;
          }
          if (data.address.city) {
            UserData.address.line1 = data.address.city;
          }
          if (data.address.country) {
            UserData.address.line1 = data.address.country;
          }
        }
        await route.fulfill({
          status: 200,
        });
      }
    },
  );
}

async function mockAddPaymentMethod(page: Page, options?: MockRouteOptions): Promise<void> {
  await mockRoute(
    page,
    `**/users/${DEFAULT_USER_INFORMATION.id}/clients/${DEFAULT_USER_INFORMATION.clientId}/add_payment_method`,
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

async function mockUpdateEmail(page: Page, options?: MockRouteOptions): Promise<void> {
  await mockRoute(page, `**/users/${DEFAULT_USER_INFORMATION.id}/update_email`, options, async (route) => {
    const data = await route.request().postDataJSON();
    if (data.email) {
      UserData.email = data.email;
    }
    await route.fulfill({
      status: 200,
    });
  });
}

async function mockUpdateAuthentication(page: Page, options?: MockRouteOptions): Promise<void> {
  await mockRoute(page, `**/users/${DEFAULT_USER_INFORMATION.id}/update_authentication`, options, async (route) => {
    await route.fulfill({
      status: 204,
    });
  });
}

export const MockBms = {
  mockLogin,
  mockUserRoute,
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
  mockUpdateAuthentication,
};
