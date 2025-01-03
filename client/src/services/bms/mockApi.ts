// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { wait } from '@/parsec/internals';
import { BmsApi } from '@/services/bms/api';
import {
  AuthenticationToken,
  BmsOrganization,
  BmsResponse,
  ClientQueryData,
  CreateCustomOrderRequestQueryData,
  CustomOrderQueryData,
  CustomOrderRequestStatus,
  CustomOrderStatus,
  DataType,
  InvoiceStatus,
  OrganizationQueryData,
} from '@/services/bms/types';
import { DateTime } from 'luxon';

const REQUEST_WAIT_TIME = 1500;

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

  if (params.isMocked) {
    if (params.shouldFail) {
      if (!mockError) {
        console.warn(`Function "${functionName}" does not have failure mock, defaulting to default API`);
      } else {
        console.info(`Mocking BMS calls to ${functionName}, set up to fail`);
        return async (...args: any[]): Promise<BmsResponse> => {
          await wait(REQUEST_WAIT_TIME);
          return await mockError(args);
        };
      }
    } else {
      if (!mockSuccess) {
        console.warn(`Function "${functionName}" does not have success mock, defaulting to default API`);
      } else {
        console.info(`Mocking BMS calls to ${functionName}, set up to succeed`);
        return async (...args: any[]): Promise<BmsResponse> => {
          await wait(REQUEST_WAIT_TIME);
          return await mockSuccess(args);
        };
      }
    }
  }

  return BmsApi[functionName as keyof typeof BmsApi];
}

function getMockParameters(functionName: string): MockParameters {
  const mockFunctionsVariable: string = import.meta.env.PARSEC_APP_BMS_MOCKED_FUNCTIONS ?? '';
  const failFunctionsVariable: string = import.meta.env.PARSEC_APP_BMS_FAIL_FUNCTIONS ?? '';
  const mockedFunctions = mockFunctionsVariable.split(';');
  const failFunctions = failFunctionsVariable.split(';');

  if (mockedFunctions.includes(functionName)) {
    console.debug(`Mock call to "${functionName}"`);
    return {
      isMocked: true,
      shouldFail: failFunctions.includes(functionName),
    };
  }
  return DefaultMockParameters;
}

export const MockedBmsApi = {
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
        dataSize: 4200000000,
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
        freeSliceSize: 42,
        payingSliceSize: 42,
        status: 'ok',
      },
    };
  }),
  getOrganizationStatus: createMockFunction('getOrganizationStatus'),
  getInvoices: createMockFunction('getInvoices'),
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
        status: CustomOrderStatus.EstimateLinked,
      },
    };
  }),
  getCustomOrderDetails: createMockFunction('getCustomOrderDetails', async (_token: AuthenticationToken, _query: CustomOrderQueryData) => {
    return {
      status: 200,
      isError: false,
      data: {
        type: DataType.CustomOrderDetails,
        id: 42,
        link: 'https://unknown/link.pdf',
        number: '21',
        amountWithTaxes: 62.0,
        amountWithoutTaxes: 16.0,
        amountDue: 62.0,
        currency: 'euro',
        created: DateTime.now().minus({ months: 2 }),
        dueDate: DateTime.now().plus({ days: 5 }),
        licenseStart: DateTime.now().minus({ months: 8 }),
        licenseEnd: DateTime.now().plus({ months: 2, days: 7 }),
        status: InvoiceStatus.Open,
        administrators: {
          quantityOrdered: 3,
          amountWithTaxes: 21.0,
        },
        standards: {
          quantityOrdered: 7,
          amountWithTaxes: 21.0,
        },
        outsiders: {
          quantityOrdered: 1000,
          amountWithTaxes: 0.0,
        },
        storage: {
          quantityOrdered: 1,
          amountWithTaxes: 22.0,
        },
      },
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
        type: DataType.GetCustomOrderRequests,
        requests: [
          {
            id: 'XX-1',
            describedNeeds: 'I wanna rock!',
            users: 50,
            storage: 1000,
            status: CustomOrderRequestStatus.Processing,
            comment: 'Turn it down you say, nut all I got to say to you is time and time again I say no (no), no no, no no, no.',
            orderDate: DateTime.now().minus({ months: 2 }),
          },
          {
            id: 'XX-2',
            describedNeeds: 'I want to hold your hand',
            users: 50,
            storage: 1000,
            organizationId: 'The Beatles',
            status: CustomOrderRequestStatus.Received,
            comment: "I think you'll understand",
            orderDate: DateTime.now().minus({ days: 4 }),
          },
          {
            id: 'XX-3',
            describedNeeds: 'I want to break free!',
            users: 300,
            storage: 9999,
            organizationId: 'Queen',
            status: CustomOrderRequestStatus.Finished,
            comment: "I've fallen in love",
            orderDate: DateTime.now().minus({ months: 6 }),
          },
          {
            id: 'XX-4',
            describedNeeds: 'I want you to want me',
            users: 9999,
            storage: 100,
            status: CustomOrderRequestStatus.Cancelled,
            comment: 'I need you to need me',
            orderDate: DateTime.now().minus({ days: 6 }),
          },
        ],
      };
    },
    async (_token: AuthenticationToken) => {
      return {
        status: 401,
        isError: true,
      };
    },
  ),
};
