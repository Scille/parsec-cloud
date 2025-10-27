// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

/* eslint-disable camelcase */

import { Page, Route } from '@playwright/test';
import { DEFAULT_ORGANIZATION_DATA_SLICE, DEFAULT_USER_INFORMATION } from '@tests/e2e/helpers/data';
import { MsPage } from '@tests/e2e/helpers/types';
import { DateTime } from 'luxon';

type RouteHandler = (route: Route) => Promise<void>;

interface MethodHandlers {
  GET?: RouteHandler;
  POST?: RouteHandler;
  PUT?: RouteHandler;
  PATCH?: RouteHandler;
}

async function mockRoute(page: Page, url: string | RegExp, options: MockRouteOptions | undefined, handlers: MethodHandlers): Promise<void> {
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
          errors: [{ code: options.errors.code ?? 'error', attr: options.errors.attribute ?? 'attr', detail: 'Default error' }],
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

    const handler = handlers[method as keyof RouteHandler] as RouteHandler;
    if (!handler) {
      console.warn(`${method} ${url}: Method not defined`);
      await route.fulfill({
        status: 405,
      });
      return;
    }

    await handler(route);
  });
}

interface MockMethodOptions {
  timeout?: boolean;
  errors?: {
    code?: string;
    attribute?: string;
    status?: number;
  };
}

export interface MockRouteOptions {
  PATCH?: MockMethodOptions;
  GET?: MockMethodOptions;
  PUT?: MockMethodOptions;
  POST?: MockMethodOptions;
}

interface MockCustomOrderDetailsOverload {
  empty?: boolean;
  single?: boolean;
  created?: DateTime;
  amountWithTaxes?: number;
  amountWithoutTaxes?: number;
  amountDue?: number;
  status?: 'paid' | 'draft' | 'open' | 'uncollectible' | 'void';
  licenseStart?: DateTime;
  licenseEnd?: DateTime;
  adminAmountWithTaxes?: number;
  outsiderAmountWithTaxes?: number;
  standardAmountWithTaxes?: number;
  storageAmountWithTaxes?: number;
  adminOrdered?: number;
  standardOrdered?: number;
  outsiderOrdered?: number;
  storageOrdered?: number;
}

export type MockClientAreaOverload =
  | MockBillingDetailsOverload
  | MockCustomOrderDetailsOverload
  | MockCustomOrderRequestOverload
  | MockCustomOrderStatusOverload
  | MockGetInvoicesOverload
  | MockListOrganizationOverload
  | MockOrganizationStatsOverload
  | MockOrganizationStatusOverload;

function createCustomOrderInvoices(overload: MockCustomOrderDetailsOverload = {}): Array<any> {
  const invoices: Array<any> = [];

  const STATUSES = ['paid', 'draft', 'open', 'uncollectible', 'void'];

  if (overload.empty) {
    return invoices;
  }
  for (let i = 0; i < (overload.single !== undefined ? 1 : 15); i++) {
    const licenseStart = overload.licenseStart
      ? overload.licenseStart
      : DateTime.fromObject({ year: 2024 + Math.floor(i / 12), month: (i % 12) + 1, day: 3 });
    const licenseEnd = overload.licenseEnd
      ? overload.licenseEnd
      : DateTime.fromObject({ year: 2024 + Math.floor((i + 1) / 12), month: ((i + 1) % 12) + 1, day: 3 });

    invoices.push({
      id: `custom_order_id${i + 1}`,
      created: overload.created ? overload.created.toISO() : licenseStart.toISO(),
      due_date: licenseEnd.toISO(),
      number: `FACT00${i + 1}`,
      status: overload.status ? overload.status : STATUSES[Math.floor(Math.random() * STATUSES.length)],
      amounts: {
        total_excl_tax: overload.amountWithoutTaxes ? overload.amountWithoutTaxes.toString() : '42.00',
        // x10, damn government
        total_incl_tax: overload.amountWithTaxes ? overload.amountWithTaxes.toString() : '420.00',
        total_remaining_due_incl_tax: overload.amountDue ? overload.amountDue.toString() : '420.00',
      },
      pdf_link: `https://parsec.cloud/invoices/${i}`,
      rows: [
        {
          reference: 'Psc_D0_Adm_M',
          amount_tax_inc: overload.adminAmountWithTaxes ? overload.adminAmountWithTaxes.toString() : '160.00',
        },
        {
          reference: 'Psc_D0_Std_User_M',
          amount_tax_inc: overload.standardAmountWithTaxes ? overload.standardAmountWithTaxes.toString() : '200.00',
        },
        {
          reference: 'Psc_D0_Ext_User_M',
          amount_tax_inc: overload.outsiderAmountWithTaxes ? overload.outsiderAmountWithTaxes.toString() : '80.00',
        },
        {
          // cspell:disable-next-line
          reference: 'Psc_Stck_100_Go_M',
          amount_tax_inc: overload.storageAmountWithTaxes ? overload.storageAmountWithTaxes.toString() : '120.00',
        },
      ],
      _embed: {
        custom_fields: [
          {
            code: 'parsec-saas-custom-order-start-date',
            value: licenseStart.toISO(),
          },
          {
            code: 'parsec-saas-custom-order-end-date',
            value: licenseEnd.toISO(),
          },
          {
            code: 'parsec-saas-custom-order-admin-license-count',
            value: overload.adminOrdered ? overload.adminOrdered.toString() : '32',
          },
          {
            code: 'parsec-saas-custom-order-outsider-license-count',
            value: overload.outsiderOrdered ? overload.outsiderOrdered.toString() : '100',
          },
          {
            code: 'parsec-saas-custom-order-standard-license-count',
            value: overload.standardOrdered ? overload.standardOrdered.toString() : '50',
          },
          {
            code: 'parsec-saas-custom-order-storage-license-count',
            value: overload.storageOrdered ? overload.storageOrdered.toString() : '2',
          },
        ],
      },
    });
  }

  return invoices;
}

async function mockLogin(page: Page, options?: MockRouteOptions): Promise<void> {
  const TOKEN_RAW = {
    email: DEFAULT_USER_INFORMATION.email,
    is_staff: true,
    token_type: 'access',
    user_id: DEFAULT_USER_INFORMATION.id,
    exp: DateTime.utc().plus({ years: 42 }).toJSDate().valueOf(),
    iat: 0,
  };
  const TOKEN = btoa(JSON.stringify(TOKEN_RAW));

  await mockRoute(page, '**/api/token', options, {
    POST: async (route) => {
      await route.fulfill({
        status: 200,
        json: {
          access: TOKEN,
          refresh: TOKEN,
        },
      });
    },
  });
}

interface MockUserOverload {
  billingSystem?: 'STRIPE' | 'CUSTOM_ORDER' | 'NONE' | 'EXPERIMENTAL_CANDIDATE';
  noClient?: boolean;
}

async function mockUserRoute(page: MsPage, overload: MockUserOverload = {}, options?: MockRouteOptions): Promise<void> {
  await mockRoute(page, `**/users/${DEFAULT_USER_INFORMATION.id}`, options, {
    GET: async (route) => {
      let client = null;
      if (!overload.noClient) {
        client = {
          firstname: page.userData.firstName,
          lastname: page.userData.lastName,
          id: '1337',
          job: page.userData.job,
          company: page.userData.company,
          phone: page.userData.phone,
          billing_system: overload.billingSystem ?? 'STRIPE',
        };
      }

      await route.fulfill({
        status: 200,
        json: {
          id: DEFAULT_USER_INFORMATION.id,
          created_at: '2024-07-15T13:21:32.141317Z',
          email: page.userData.email,
          client: client,
        },
      });
    },
    PATCH: async (route) => {
      const data = await route.request().postDataJSON();
      if (data.client) {
        if (data.client.firstname) {
          page.userData.firstName = data.client.firstname;
        }
        if (data.client.lastname) {
          page.userData.lastName = data.client.lastname;
        }
        if (data.client.phone) {
          page.userData.phone = data.client.phone;
        }
        if (data.client.job || data.client.job === null) {
          page.userData.job = data.client.job;
        }
        if (data.client.company || data.client.job === null) {
          page.userData.company = data.client.company;
        }
      }
      await route.fulfill({
        status: 200,
      });
    },
  });
}

async function mockCreateOrganization(page: Page, bootstrapAddr: string, options?: MockRouteOptions): Promise<void> {
  await mockRoute(page, `**/users/${DEFAULT_USER_INFORMATION.id}/clients/${DEFAULT_USER_INFORMATION.clientId}/organizations`, options, {
    POST: async (route) => {
      await route.fulfill({
        status: 201,
        json: {
          bootstrap_link: bootstrapAddr,
        },
      });
    },
  });
}

interface MockListOrganizationOverload {
  noOrg?: boolean;
}

async function mockListOrganizations(page: MsPage, overload?: MockListOrganizationOverload, options?: MockRouteOptions): Promise<void> {
  await mockRoute(page, `**/users/${DEFAULT_USER_INFORMATION.id}/clients/${DEFAULT_USER_INFORMATION.clientId}/organizations`, options, {
    GET: async (route) => {
      const orgs = [];
      if (!overload?.noOrg) {
        orgs.push({
          pk: page.orgInfo.bmsId,
          created_at: '2024-12-04T00:00:00.000',
          expiration_date: null,
          name: page.orgInfo.name,
          parsec_id: page.orgInfo.name,
          suffix: page.orgInfo.name,
          stripe_subscription_id: 'stripe_id',
          bootstrap_link: '',
        });
        orgs.push({
          pk: `${page.orgInfo.bmsId}-2`,
          created_at: '2024-12-04T00:00:00.000',
          expiration_date: null,
          name: page.orgInfo.name,
          parsec_id: `${page.orgInfo.name}-2`,
          suffix: `${page.orgInfo.name}-2`,
          stripe_subscription_id: 'stripe_id2',
          bootstrap_link: '',
        });
      }
      await route.fulfill({
        status: 200,
        json: {
          results: orgs,
        },
      });
    },
  });
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
    `*/**/users/${DEFAULT_USER_INFORMATION.id}/clients/${DEFAULT_USER_INFORMATION.clientId}/organizations/*/stats`,
    options,
    {
      GET: async (route) => {
        await route.fulfill({
          status: 200,
          json: {
            users_per_profile_detail: usersPerProfileDetail,
            data_size: overload.dataSize ?? 400000000000,
            metadata_size: overload.metadataSize ?? 400000000,
            free_slice_size: overload.freeSliceSize ?? DEFAULT_ORGANIZATION_DATA_SLICE.free,
            paying_slice_size: overload.payingSliceSize ?? DEFAULT_ORGANIZATION_DATA_SLICE.paying,
            users: overload.users ?? 203,
            active_users: overload.activeUsers ?? 59,
            status: overload.status ?? 'ok',
          },
        });
      },
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
    {
      GET: async (route) => {
        await route.fulfill({
          status: 200,
          json: {
            active_users_limit: overload.activeUsersLimit ?? 1000,
            is_bootstrapped: overload.isBootstrapped ?? true,
            is_frozen: overload.isFrozen ?? false,
            is_initialized: overload.isInitialized ?? true,
            user_profile_outsider_allowed: overload.outsiderAllowed ?? true,
          },
        });
      },
    },
  );
}

interface MockGetInvoicesOverload {
  count?: number;
}

async function mockGetInvoices(page: MsPage, overload: MockGetInvoicesOverload = {}, options?: MockRouteOptions): Promise<void> {
  await mockRoute(page, `**/users/${DEFAULT_USER_INFORMATION.id}/clients/${DEFAULT_USER_INFORMATION.clientId}/invoices`, options, {
    GET: async (route) => {
      let invoices = [];
      for (let year = 2019; year < 2022; year++) {
        for (let month = 1; month < 13; month++) {
          invoices.push({
            id: `Id${year}-${month}`,
            pdf: `https://fake/pdfs/${year}-${month}.pdf`,
            period_start: DateTime.fromObject({ year: year, month: month }).toFormat('yyyy-LL-dd'),
            period_end: DateTime.fromObject({ year: year, month: month }).endOf('month').toFormat('yyyy-LL-dd'),
            total: Math.round(Math.random() * 1000000),
            status: ['paid', 'draft', 'open'][Math.floor(Math.random() * 3)],
            organization: page.orgInfo.name,
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
  });
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

async function mockBillingDetails(page: MsPage, overload: MockBillingDetailsOverload = {}, options?: MockRouteOptions): Promise<void> {
  await mockRoute(page, `**/users/${DEFAULT_USER_INFORMATION.id}/clients/${DEFAULT_USER_INFORMATION.clientId}/billing_details`, options, {
    GET: async (route) => {
      const paymentMethods = [];
      for (let i = 0; i < (overload.cardsCount ?? 1); i++) {
        paymentMethods.push({
          type: 'card',
          id: `card${i}`,
          brand: 'mastercard',
          exp_date: '12/47',
          last_digits: '4444',
          default: true,
        });
      }
      for (let i = 0; i < (overload.sepaCount ?? 1); i++) {
        paymentMethods.push({
          type: 'debit',
          id: `debit${i}`,
          bank_name: 'Bank',
          last_digits: '1234',
          default: overload.cardsCount === undefined || overload.cardsCount === 0 ? true : false,
        });
      }
      await route.fulfill({
        status: 200,
        json: {
          email: overload.email ?? page.userData.email,
          name: overload.name ?? `${page.userData.firstName} ${page.userData.lastName}`,
          address: {
            line1: (overload.address && overload.address.line1) ?? page.userData.address.line1,
            line2: (overload.address && overload.address.line2) ?? '',
            city: (overload.address && overload.address.city) ?? page.userData.address.city,
            postal_code: (overload.address && overload.address.postalCode) ?? page.userData.address.postalCode,
            country: (overload.address && overload.address.country) ?? page.userData.address.country,
          },
          payment_methods: paymentMethods,
        },
      });
    },
    PATCH: async (route) => {
      const data = await route.request().postDataJSON();
      if (data.address) {
        if (data.address.line1) {
          page.userData.address.line1 = data.address.line1;
        }
        if (data.address.line2) {
          page.userData.address.line2 = data.address.line2;
        }
        if (data.address.postal_code) {
          page.userData.address.postalCode = data.address.postal_code;
        }
        if (data.address.city) {
          page.userData.address.line1 = data.address.city;
        }
        if (data.address.country) {
          page.userData.address.line1 = data.address.country;
        }
      }
      await route.fulfill({
        status: 200,
      });
    },
  });
}

async function mockAddPaymentMethod(page: Page, options?: MockRouteOptions): Promise<void> {
  await mockRoute(
    page,
    `**/users/${DEFAULT_USER_INFORMATION.id}/clients/${DEFAULT_USER_INFORMATION.clientId}/add_payment_method`,
    options,
    {
      PUT: async (route) => {
        await route.fulfill({
          status: 200,
          json: {
            payment_method: '123456',
          },
        });
      },
    },
  );
}

async function mockSetDefaultPaymentMethod(page: Page, options?: MockRouteOptions): Promise<void> {
  await mockRoute(
    page,
    `**/users/${DEFAULT_USER_INFORMATION.id}/clients/${DEFAULT_USER_INFORMATION.clientId}/default_payment_method`,
    options,
    {
      PATCH: async (route) => {
        await route.fulfill({
          status: 200,
          json: {
            payment_method: '123456',
          },
        });
      },
    },
  );
}

async function mockDeletePaymentMethod(page: Page, options?: MockRouteOptions): Promise<void> {
  await mockRoute(
    page,
    `**/users/${DEFAULT_USER_INFORMATION.id}/clients/${DEFAULT_USER_INFORMATION.clientId}/delete_payment_method`,
    options,
    {
      POST: async (route) => {
        await route.fulfill({
          status: 200,
          json: {
            payment_method: '123456',
          },
        });
      },
    },
  );
}

async function mockUpdateEmailSendCode(page: Page, options?: MockRouteOptions): Promise<void> {
  await mockRoute(page, '**/email_validation/send_code', options, {
    POST: async (route) => {
      await route.fulfill({
        status: 200,
      });
    },
  });
}

async function mockUpdateEmail(page: MsPage, options?: MockRouteOptions): Promise<void> {
  await mockRoute(page, `**/users/${DEFAULT_USER_INFORMATION.id}/update_email`, options, {
    POST: async (route) => {
      const data = await route.request().postDataJSON();
      if (data.email) {
        page.userData.email = data.email;
      }
      await route.fulfill({
        status: 200,
      });
    },
  });
}

async function mockUpdateAuthentication(page: Page, options?: MockRouteOptions): Promise<void> {
  await mockRoute(page, `**/users/${DEFAULT_USER_INFORMATION.id}/update_authentication`, options, {
    POST: async (route) => {
      await route.fulfill({
        status: 204,
      });
    },
  });
}

async function mockChangePassword(page: Page, options?: MockRouteOptions): Promise<void> {
  await mockRoute(page, '**/users/change_password', options, {
    POST: async (route) => {
      await route.fulfill({
        status: 200,
      });
    },
  });
}

async function mockCustomOrderDetails(
  page: MsPage,
  overload: MockCustomOrderDetailsOverload = {},
  options?: MockRouteOptions,
): Promise<void> {
  overload.single = true;
  await mockRoute(
    page,
    `**/users/${DEFAULT_USER_INFORMATION.id}/clients/${DEFAULT_USER_INFORMATION.clientId}/organizations/custom_order_details`,
    options,
    {
      POST: async (route) => {
        const postJSON = route.request().postDataJSON();
        const orgId = postJSON.organization_ids[0] as string;
        const data: { [key: string]: string } = {};
        data[orgId.endsWith('-2') ? `${page.orgInfo.name}-2` : page.orgInfo.name] = createCustomOrderInvoices(overload)[0];
        await route.fulfill({
          status: 200,
          json: data,
        });
      },
    },
  );
}

interface MockCustomOrderStatusOverload {
  status: 'invoice_paid' | 'nothing_linked' | 'unknown' | 'contract_ended' | 'invoice_to_be_paid' | 'estimate_linked';
}

async function mockCustomOrderStatus(page: MsPage, overload?: MockCustomOrderStatusOverload, options?: MockRouteOptions): Promise<void> {
  await mockRoute(
    page,
    `**/users/${DEFAULT_USER_INFORMATION.id}/clients/${DEFAULT_USER_INFORMATION.clientId}/organizations/custom_order_status`,
    options,
    {
      POST: async (route) => {
        const postJSON = route.request().postDataJSON();
        const orgId = postJSON.organization_ids[0] as string;
        const data: { [key: string]: string } = {};
        data[orgId.endsWith('-2') ? `${page.orgInfo.name}-2` : page.orgInfo.name] = overload ? overload.status : 'invoice_paid';

        await route.fulfill({
          status: 200,
          json: data,
        });
      },
    },
  );
}

interface MockCustomOrderRequestOverload {
  noRequest?: boolean;
}

async function mockCustomOrderRequest(page: MsPage, overload?: MockCustomOrderRequestOverload, options?: MockRouteOptions): Promise<void> {
  await mockRoute(page, '**/custom_order_requests', options, {
    POST: async (route) => {
      await route.fulfill({
        status: 204,
      });
    },
    GET: async (route) => {
      const requests = [];
      if (!overload?.noRequest) {
        requests.push({
          id: 'YY-00001',
          described_need: 'I need a hero!',
          standard_users: 300,
          admin_users: 10,
          outsider_users: 100,
          storage: 1000,
          status: 'FINISHED',
          label: 'ORD-YY-00001',
          formula: "I'm holding out for a hero 'till the end of the night",
          created_at: DateTime.fromObject({ year: 1988, month: 4, day: 7 }).toISO(),
        });
        requests.push({
          id: 'YY-00002',
          described_need: 'I need your love!',
          standard_users: 9999,
          admin_users: 3,
          outsider_users: 0,
          storage: 9999,
          organization_name: 'BlackMesa',
          status: 'FINISHED',
          label: 'ORD-YY-00002',
          formula: 'I want you every way',
          created_at: DateTime.fromObject({ year: 1990, month: 3, day: 30 }).toISO(),
        });
      }
      await route.fulfill({
        status: 200,
        json: requests,
      });
    },
  });
}

async function mockGetCustomOrderInvoices(
  page: MsPage,
  overload: MockCustomOrderDetailsOverload = {},
  options?: MockRouteOptions,
): Promise<void> {
  function formatResponse(postData: any): any {
    const ret: any = {};
    for (const orgId of postData.organization_ids ?? []) {
      const parsecId = orgId === '42' ? page.orgInfo.name : `${page.orgInfo.name}-2`;
      ret[parsecId] = createCustomOrderInvoices(overload);
    }
    return ret;
  }

  await mockRoute(
    page,
    `**/users/${DEFAULT_USER_INFORMATION.id}/clients/${DEFAULT_USER_INFORMATION.clientId}/organizations/custom_order_invoices`,
    options,
    {
      POST: async (route) => {
        await route.fulfill({
          status: 200,
          json: formatResponse(route.request().postDataJSON()),
        });
      },
    },
  );
}

async function mockReportBug(page: MsPage, options?: MockRouteOptions): Promise<void> {
  await mockRoute(page, '**/api/bug-report', options, {
    POST: async (route) => {
      await route.fulfill({ status: 200 });
    },
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
  mockChangePassword,
  mockCustomOrderStatus,
  mockCustomOrderDetails,
  mockUpdateEmailSendCode,
  mockCustomOrderRequest,
  mockGetCustomOrderInvoices,
  mockReportBug,
};
