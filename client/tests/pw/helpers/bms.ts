// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { Page } from '@playwright/test';
import { DEFAULT_ORGANIZATION_DATA_SLICE, DEFAULT_ORGANIZATION_INFORMATION, DEFAULT_USER_INFORMATION } from '@tests/pw/helpers/data';
import { DateTime } from 'luxon';

async function mockLogin(page: Page, success: boolean, timeout?: boolean): Promise<void> {
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

  await page.route('**/api/token', async (route) => {
    if (success) {
      await route.fulfill({
        status: 200,
        json: {
          access: TOKEN,
          refresh: TOKEN,
        },
      });
    } else {
      if (timeout) {
        route.abort('timedout');
      } else {
        await route.fulfill({
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

async function mockCreateOrganization(page: Page, bootstrapAddr: string): Promise<void> {
  await page.route(`**/users/${DEFAULT_USER_INFORMATION.id}/clients/${DEFAULT_USER_INFORMATION.clientId}/organizations`, async (route) => {
    await route.fulfill({
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
  });
}

async function mockOrganizationStats(page: Page, data?: number): Promise<void> {
  const usersPerProfileDetail: { [profile: string]: { active: number; revoked: number } } = {};
  usersPerProfileDetail.ADMIN = { active: 4, revoked: 1 };
  usersPerProfileDetail.STANDARD = { active: 54, revoked: 1 };
  usersPerProfileDetail.OUTSIDER = { active: 1, revoked: 142 };

  const dataSize = data ? data * 0.999 : 400000000000;
  const metadataSize = data ? data - dataSize : 400000000;

  await page.route(
    // eslint-disable-next-line max-len
    `**/users/${DEFAULT_USER_INFORMATION.id}/clients/${DEFAULT_USER_INFORMATION.clientId}/organizations/${DEFAULT_ORGANIZATION_INFORMATION.bmsId}/stats`,
    async (route) => {
      await route.fulfill({
        status: 200,
        json: {
          // eslint-disable-next-line camelcase
          users_per_profile_detail: usersPerProfileDetail,
          // eslint-disable-next-line camelcase
          data_size: dataSize,
          // eslint-disable-next-line camelcase
          metadata_size: metadataSize,
          // eslint-disable-next-line camelcase
          free_slice_size: DEFAULT_ORGANIZATION_DATA_SLICE.free,
          // eslint-disable-next-line camelcase
          paying_slice_size: DEFAULT_ORGANIZATION_DATA_SLICE.paying,
          users: 203,
          // eslint-disable-next-line camelcase
          active_users: 59,
          status: 'ok',
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
      await route.fulfill({
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
    }
  });
}

async function mockBillingDetails(
  page: Page,
  { includeCard, includeSepa, fail }: { includeCard?: boolean; includeSepa?: boolean; fail?: boolean },
): Promise<void> {
  await page.route(
    `**/users/${DEFAULT_USER_INFORMATION.id}/clients/${DEFAULT_USER_INFORMATION.clientId}/billing_details`,
    async (route) => {
      if (fail) {
        await route.fulfill({
          status: 401,
          json: {
            type: 'error',
            errors: [{ code: 'invalid', attr: 'null', detail: 'An error occured' }],
          },
        });
      } else {
        const paymentMethods = [];
        if (includeCard) {
          paymentMethods.push({
            type: 'card',
            id: 'card1',
            brand: 'mastercard',
            // eslint-disable-next-line camelcase
            exp_date: '12/47',
            // eslint-disable-next-line camelcase
            last_digits: '4444',
            default: true,
          });
        }
        if (includeSepa) {
          paymentMethods.push({
            type: 'debit',
            id: 'debit1',
            // eslint-disable-next-line camelcase
            bank_name: 'Bank',
            // eslint-disable-next-line camelcase
            last_digits: '1234',
            default: includeCard ? false : true,
          });
        }
        route.fulfill({
          status: 200,
          json: {
            email: DEFAULT_USER_INFORMATION.email,
            name: `${DEFAULT_USER_INFORMATION.firstName} ${DEFAULT_USER_INFORMATION.lastName}`,
            address: DEFAULT_USER_INFORMATION.address.full,
            // eslint-disable-next-line camelcase
            payment_methods: paymentMethods,
          },
        });
      }
    },
  );
}

async function mockAddPaymentMethod(page: Page, fail?: boolean): Promise<void> {
  await page.route(
    `**/users/${DEFAULT_USER_INFORMATION.id}/clients/${DEFAULT_USER_INFORMATION.clientId}/add_payment_method`,
    async (route) => {
      if (!fail) {
        await route.fulfill({
          status: 200,
          json: {
            // eslint-disable-next-line camelcase
            payment_method: '123456',
          },
        });
      } else {
        await route.fulfill({
          status: 401,
          json: {
            type: 'payment',
            errors: [{ code: 'invalid', attr: 'payment_method', detail: 'Invalid payment method' }],
          },
        });
      }
    },
  );
}

async function mockSetDefaultPaymentMethod(page: Page, fail?: boolean): Promise<void> {
  await page.route(
    `**/users/${DEFAULT_USER_INFORMATION.id}/clients/${DEFAULT_USER_INFORMATION.clientId}/default_payment_method`,
    async (route) => {
      if (!fail) {
        await route.fulfill({
          status: 200,
          json: {
            // eslint-disable-next-line camelcase
            payment_method: '123456',
          },
        });
      } else {
        await route.fulfill({
          status: 401,
          json: {
            type: 'payment',
            errors: [{ code: 'invalid', attr: 'payment_method', detail: 'Invalid payment method' }],
          },
        });
      }
    },
  );
}

async function mockDeletePaymentMethod(page: Page, fail?: boolean): Promise<void> {
  await page.route(
    `**/users/${DEFAULT_USER_INFORMATION.id}/clients/${DEFAULT_USER_INFORMATION.clientId}/delete_payment_method`,
    async (route) => {
      if (!fail) {
        await route.fulfill({
          status: 200,
          json: {
            // eslint-disable-next-line camelcase
            payment_method: '123456',
          },
        });
      } else {
        await route.fulfill({
          status: 401,
          json: {
            type: 'payment',
            errors: [{ code: 'invalid', attr: 'payment_method', detail: 'Invalid payment method' }],
          },
        });
      }
    },
  );
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
};
