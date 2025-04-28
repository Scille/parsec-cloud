// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { wait } from '@/parsec/internals';
import { BmsApi } from '@/services/bms/api';
import {
  AuthenticationToken,
  BmsOrganization,
  BmsResponse,
  ClientQueryData,
  CreateCustomOrderRequestQueryData,
  CustomOrderDetailsResultData,
  CustomOrderInvoicesResultData,
  CustomOrderQueryData,
  CustomOrderRequestStatus,
  CustomOrderStatus,
  DataType,
  InvoiceStatus,
  OrganizationQueryData,
  SellsyInvoice,
} from '@/services/bms/types';
import { DateTime } from 'luxon';

const REQUEST_WAIT_TIME = Number(import.meta.env.PARSEC_APP_BMS_MOCK_WAIT_DURATION) ?? 1000;

interface MockParameters {
  isMocked: boolean;
  shouldFail: boolean;
}

const DefaultMockParameters = {
  isMocked: false,
  shouldFail: false,
};

type MockFunction = (...args: any[]) => Promise<BmsResponse>;

// Automates a few things:
// - if we ask to mock a function but this function does not have mocks, prints a warning
// - automatically dispatch to the correct success or failure function
// - automatically defaults to the regular API
// - adds some waiting time to simulate the network
function createMockFunction(functionName: string, mockSuccess?: MockFunction, mockError?: MockFunction): MockFunction {
  const params = getMockParameters(functionName);

  if (import.meta.env.PARSEC_APP_BMS_USE_MOCK === 'true' && params.isMocked) {
    if (params.shouldFail) {
      if (!mockError) {
        console.warn(`Function "${functionName}" does not have failure mock, defaulting to default API`);
      } else {
        console.info(`Mocking BMS calls to ${functionName}, set up to fail`);
        return async (...args: any[]): Promise<BmsResponse> => {
          await wait(REQUEST_WAIT_TIME);
          return await mockError(...args);
        };
      }
    } else {
      if (!mockSuccess) {
        console.warn(`Function "${functionName}" does not have success mock, defaulting to default API`);
      } else {
        console.info(`Mocking BMS calls to ${functionName}, set up to succeed`);
        return async (...args: any[]): Promise<BmsResponse> => {
          await wait(REQUEST_WAIT_TIME);
          return await mockSuccess(...args);
        };
      }
    }
  }

  return BmsApi[functionName as keyof typeof BmsApi];
}

function getMockParameters(functionName: string): MockParameters {
  const mocksEnabled = import.meta.env.PARSEC_APP_BMS_USE_MOCK === 'true';
  const mockFunctionsVariable: string = import.meta.env.PARSEC_APP_BMS_MOCKED_FUNCTIONS ?? '';
  const failFunctionsVariable: string = import.meta.env.PARSEC_APP_BMS_FAIL_FUNCTIONS ?? '';
  const mockedFunctions = mockFunctionsVariable.split(';');
  const failFunctions = failFunctionsVariable.split(';');

  if (mocksEnabled && mockedFunctions.includes(functionName)) {
    console.debug(`Mock call to "${functionName}"`);
    return {
      isMocked: true,
      shouldFail: failFunctions.includes(functionName),
    };
  }
  return DefaultMockParameters;
}

function createCustomOrderInvoices(
  count: number = 1,
  organizations: Array<BmsOrganization> = [],
): CustomOrderDetailsResultData | CustomOrderInvoicesResultData {
  const invoices: Array<SellsyInvoice> = [];

  for (let i = 1; i < count + 1; i++) {
    const customOrderInvoice: CustomOrderDetailsResultData = {
      type: DataType.CustomOrderDetails,
      id: i,
      link: `https://unknown/link${i}.pdf`,
      number: `Invoice_${i}`,
      amountWithTaxes: 62.0 + (i % 2) * 16,
      amountWithoutTaxes: 16.0 + (i % 2) * 4,
      amountDue: 62.0 + (i % 2) * 16,
      currency: 'euro',
      created: DateTime.now().minus({ months: count + 1 - i }),
      dueDate: DateTime.now()
        .minus({ months: count + 1 - i })
        .plus({ days: 5 }),
      licenseStart: DateTime.now().minus({ months: count + 2 }),
      licenseEnd: DateTime.now().plus({ months: 2, days: 7 }),
      status: i === count ? InvoiceStatus.Open : InvoiceStatus.Paid,
      administrators: {
        quantityOrdered: 3 + (i % 2),
        amountWithTaxes: 21.0 + (i % 2) * 4,
      },
      standards: {
        quantityOrdered: 7 + (i % 2) * 3,
        amountWithTaxes: 21.0 + (i % 2) * 12,
      },
      outsiders: {
        quantityOrdered: 1000 + (i % 2) * 50,
        amountWithTaxes: 0.0,
      },
      storage: {
        quantityOrdered: 1 + (i % 2),
        amountWithTaxes: 22.0 + (i % 2) * 5,
      },
      organizationId: organizations.length > 0 ? organizations[i % organizations.length].parsecId : 'CustomOrderOrg',
    };
    invoices.push(new SellsyInvoice(customOrderInvoice));
  }

  if (count === 1) {
    return invoices[0].invoice;
  }
  return {
    type: DataType.CustomOrderInvoices,
    invoices: invoices,
  };
}

export const MockedBmsApi = {
  reportBug: createMockFunction('reportBug'),
  login: createMockFunction('login'),
  getPersonalInformation: createMockFunction('getPersonalInformation'),
  updatePersonalInformation: createMockFunction('updatePersonalInformation'),
  updateEmail: createMockFunction('updateEmail'),
  changePassword: createMockFunction('changePassword'),
  createOrganization: createMockFunction('createOrganization'),
  listOrganizations: createMockFunction('listOrganizations', async (_token: AuthenticationToken, _query: ClientQueryData) => {
    const orgs: Array<BmsOrganization> = [1, 2, 3].map((index) => {
      return {
        bmsId: `BmsId${index}`,
        createdAt: DateTime.now().minus({ months: index }),
        expirationDate: DateTime.now().plus({ months: index }),
        name: `org${index}`,
        parsecId: `MyOrg${index}`,
        stripeSubscriptionId: undefined,
        bootstrapLink: '',
        isSubscribed: () => false,
      };
    });
    return {
      status: 200,
      isError: false,
      data: {
        type: DataType.ListOrganizations,
        organizations: orgs,
      },
    };
  }),
  getOrganizationStats: createMockFunction('getOrganizationStats', async (_token: AuthenticationToken, _query: OrganizationQueryData) => {
    return {
      status: 200,
      isError: false,
      data: {
        type: DataType.OrganizationStats,
        dataSize: 127374182400,
        metadataSize: 42,
        users: 42,
        activeUsers: 10,
        adminUsersDetail: {
          active: 2,
          revoked: 2,
        },
        standardUsersDetail: {
          active: 1,
          revoked: 6,
        },
        outsiderUsersDetail: {
          active: 1010,
          revoked: 3,
        },
        freeSliceSize: 107374182400,
        payingSliceSize: 20000000000,
        status: CustomOrderStatus.InvoicePaid,
      },
    };
  }),
  getOrganizationStatus: createMockFunction('getOrganizationStatus', async (_token: AuthenticationToken, _query: OrganizationQueryData) => {
    return {
      status: 200,
      isError: false,
      data: {
        type: DataType.OrganizationStatus,
        activeUsersLimit: 100,
        isBootstrapped: true,
        isFrozen: false,
        isInitialized: true,
        outsidersAllowed: true,
      },
    };
  }),
  getMonthlySubscriptionInvoices: createMockFunction('getMonthlySubscriptionInvoices'),
  refreshToken: createMockFunction('refreshToken'),
  getBillingDetails: createMockFunction('getBillingDetails'),
  addPaymentMethod: createMockFunction('addPaymentMethod'),
  setDefaultPaymentMethod: createMockFunction('setDefaultPaymentMethod'),
  deletePaymentMethod: createMockFunction('deletePaymentMethod'),
  updateAuthentication: createMockFunction('updateAuthentication'),
  updateBillingDetails: createMockFunction('updateBillingDetails'),
  getCustomOrderStatus: createMockFunction('getCustomOrderStatus', async (_token: AuthenticationToken, _query: CustomOrderQueryData) => {
    return {
      status: 200,
      isError: false,
      data: {
        type: DataType.CustomOrderStatus,
        status: CustomOrderStatus.InvoicePaid,
      },
    };
  }),
  getCustomOrderDetails: createMockFunction('getCustomOrderDetails', async (_token: AuthenticationToken, _query: CustomOrderQueryData) => {
    return {
      status: 200,
      isError: false,
      data: createCustomOrderInvoices(1),
    };
  }),
  unsubscribeOrganization: createMockFunction('unsubscribeOrganization'),
  subscribeOrganization: createMockFunction('subscribeOrganization'),
  updateEmailSendCode: createMockFunction('updateEmailSendCode'),
  createCustomOrderRequest: createMockFunction(
    'createCustomOrderRequest',
    async (_token: AuthenticationToken, _query: CreateCustomOrderRequestQueryData) => {
      return {
        status: 204,
        isError: false,
      };
    },
    async (_token: AuthenticationToken, _query: CreateCustomOrderRequestQueryData) => {
      return {
        status: 401,
        isError: true,
      };
    },
  ),
  getCustomOrderRequests: createMockFunction(
    'getCustomOrderRequests',
    async (_token: AuthenticationToken) => {
      return {
        status: 200,
        isError: false,
        data: {
          type: DataType.GetCustomOrderRequests,
          requests: [
            {
              id: 'XX-1',
              describedNeeds: 'I wanna rock!',
              label: 'ORD-XX-1',
              adminUsers: 3,
              standardUsers: 50,
              outsiderUsers: 0,
              storage: 1000,
              status: CustomOrderRequestStatus.Processing,
              formula: 'Turn it down you say, nut all I got to say to you is time and time again I say no (no), no no, no no, no.',
              orderDate: DateTime.now().minus({ months: 2 }),
            },
            {
              id: 'XX-2',
              describedNeeds: 'I want to hold your hand',
              label: 'ORD-XX-2',
              adminUsers: 5,
              standardUsers: 50,
              outsiderUsers: 100,
              storage: 1000,
              status: CustomOrderRequestStatus.Received,
              formula: "I think you'll understand",
              orderDate: DateTime.now().minus({ days: 4 }),
            },
            {
              id: 'XX-3',
              describedNeeds: 'I want to break free!',
              label: 'ORD-XX-3',
              organizationId: 'Queen',
              adminUsers: 10,
              standardUsers: 300,
              outsiderUsers: 1000,
              storage: 9999,
              status: CustomOrderRequestStatus.Finished,
              formula: "I've fallen in love",
              orderDate: DateTime.now().minus({ months: 6 }),
            },
            {
              id: 'XX-4',
              describedNeeds: 'I want you to want me',
              label: 'ORD-XX-4',
              adminUsers: 1000,
              standardUsers: 9999,
              outsiderUsers: 10000,
              storage: 100,
              status: CustomOrderRequestStatus.Cancelled,
              orderDate: DateTime.now().minus({ days: 6 }),
            },
          ],
        },
      };
    },
    async (_token: AuthenticationToken) => {
      return {
        status: 401,
        isError: true,
      };
    },
  ),
  getCustomOrderInvoices: createMockFunction(
    'getCustomOrderInvoices',
    async (_token: AuthenticationToken, _query: CustomOrderQueryData, ...organizations: Array<BmsOrganization>) => {
      return {
        status: 200,
        isError: false,
        data: createCustomOrderInvoices(12, organizations),
      };
    },
  ),
};
