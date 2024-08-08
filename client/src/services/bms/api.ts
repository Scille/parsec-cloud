// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import {
  AddPaymentMethodQueryData,
  AuthenticationToken,
  BillingDetailsQueryData,
  BmsError,
  BmsResponse,
  CreateOrganizationQueryData,
  DataType,
  DeletePaymentMethodQueryData,
  InvoicesQueryData,
  ListOrganizationsQueryData,
  LoginQueryData,
  OrganizationStatsQueryData,
  OrganizationStatusQueryData,
  PaymentMethod,
  SetDefaultPaymentMethodQueryData,
  UpdateAuthenticationQueryData,
  UpdateEmailQueryData,
  UpdatePasswordQueryData,
  UpdatePersonalInformationQueryData,
} from '@/services/bms/types';
import axios, { AxiosError, AxiosInstance, AxiosResponse, isAxiosError } from 'axios';
import { DateTime } from 'luxon';
import { decodeToken } from 'megashark-lib';

const BMS_ENV_VARIABLE = 'VITE_BMS_API_URL';
const DEFAULT_BMS_URL = 'https://bms.parsec.cloud';
const DEFAULT_BMS_DEV_URL = 'https://bms-dev.parsec.cloud';

function getBmsUrl(): string {
  if (import.meta.env[BMS_ENV_VARIABLE]) {
    return import.meta.env[BMS_ENV_VARIABLE];
  }
  if (window.isDev()) {
    return DEFAULT_BMS_DEV_URL;
  }
  return DEFAULT_BMS_URL;
}

// Used to delay the creation of the AxiosInstance, since it requires
// variables on the window object that are only set when the app is
// fully initialized. Instead we initialize the instance only at the
// first request.
class HTTPRequest {
  private _instance: AxiosInstance | null;

  constructor() {
    this._instance = null;
  }

  getInstance(): AxiosInstance {
    if (!this._instance) {
      window.electronAPI.log('info', `Using ${getBmsUrl()} as Parsec BMS`);
      this._instance = axios.create({
        baseURL: getBmsUrl(),
        timeout: 3000,
      });
    }
    return this._instance;
  }
}

const http = new HTTPRequest();

function parseBmsErrors(data: object): Array<BmsError> {
  if ((data as { errors: [] }).errors) {
    return (data as { errors: [] }).errors as Array<BmsError>;
  }
  return [];
}

async function wrapQuery(queryFunc: () => Promise<BmsResponse>): Promise<BmsResponse> {
  try {
    return await queryFunc();
  } catch (error: any) {
    if (isAxiosError(error)) {
      if ((error as AxiosError).response) {
        const response = (error as AxiosError).response as AxiosResponse;
        return {
          status: response.status,
          isError: true,
          errors: parseBmsErrors(response.data),
        };
      } else {
        return {
          status: 0,
          isError: true,
        };
      }
    }
    throw new Error(error);
  }
}

async function login(query: LoginQueryData): Promise<BmsResponse> {
  return await wrapQuery(async () => {
    const axiosResponse = await http.getInstance().post('/api/token', query, { validateStatus: (status) => status === 200 });
    return {
      status: axiosResponse.status,
      isError: false,
      data: {
        type: DataType.Login,
        accessToken: axiosResponse.data.access,
        refreshToken: axiosResponse.data.refresh,
      },
    };
  });
}

async function getPersonalInformation(token: AuthenticationToken): Promise<BmsResponse> {
  return await wrapQuery(async () => {
    const decodedToken = decodeToken(token);
    if (decodedToken === undefined) {
      throw new Error('Token is invalid');
    }
    const axiosResponse = await http.getInstance().get(`/users/${decodedToken.userId}`, {
      headers: { Authorization: `Bearer ${token}` },
      validateStatus: (status) => status === 200,
    });
    return {
      status: axiosResponse.status,
      isError: false,
      data: {
        type: DataType.PersonalInformation,
        id: axiosResponse.data.id,
        createdAt: DateTime.fromISO(axiosResponse.data.created_at),
        email: axiosResponse.data.email,
        firstName: axiosResponse.data.client.firstname,
        lastName: axiosResponse.data.client.lastname,
        clientId: axiosResponse.data.client.id,
        phone: axiosResponse.data.client.phone || undefined,
        job: axiosResponse.data.client.job || undefined,
        company: axiosResponse.data.client.company || undefined,
      },
    };
  });
}

async function updatePersonalInformation(token: AuthenticationToken, data: UpdatePersonalInformationQueryData): Promise<BmsResponse> {
  return await wrapQuery(async () => {
    const axiosResponse = await http.getInstance().patch(`/users/${data.userId}`, data, {
      headers: { Authorization: `Bearer ${token}` },
    });
    return {
      status: axiosResponse.status,
      isError: false,
    };
  });
}

async function updateEmail(token: AuthenticationToken, data: UpdateEmailQueryData): Promise<BmsResponse> {
  return await wrapQuery(async () => {
    const axiosResponse = await http.getInstance().post(`/users/${data.userId}/update_email`, data, {
      headers: { Authorization: `Bearer ${token}` },
    });
    return {
      status: axiosResponse.status,
      isError: false,
    };
  });
}

async function updatePassword(token: AuthenticationToken, data: UpdatePasswordQueryData): Promise<BmsResponse> {
  return await wrapQuery(async () => {
    const axiosResponse = await http.getInstance().post('/users/change_password', data, {
      headers: { Authorization: `Bearer ${token}` },
    });
    return {
      status: axiosResponse.status,
      isError: false,
    };
  });
}

async function updateAuthentication(token: AuthenticationToken, data: UpdateAuthenticationQueryData): Promise<BmsResponse> {
  return wrapQuery(async () => {
    const axiosResponse = await http.getInstance().post(
      `/users/${data.userId}/update_authentication`,
      {
        password: data.password,
        // eslint-disable-next-line camelcase
        new_password: data.newPassword,
      },
      {
        headers: { Authorization: `Bearer ${token}` },
        validateStatus: (status) => status === 204,
      },
    );

    return {
      type: DataType.UpdateAuthentication,
      status: axiosResponse.status,
      isError: false,
    };
  });
}

async function createOrganization(token: AuthenticationToken, query: CreateOrganizationQueryData): Promise<BmsResponse> {
  return await wrapQuery(async () => {
    const axiosResponse = await http.getInstance().post(
      `/users/${query.userId}/clients/${query.clientId}/organizations`,
      // eslint-disable-next-line camelcase
      { organization_id: query.organizationName },
      { headers: { Authorization: `Bearer ${token}` }, validateStatus: (status) => status === 201 },
    );
    return {
      status: axiosResponse.status,
      isError: false,
      data: {
        type: DataType.CreateOrganization,
        bootstrapLink: axiosResponse.data.bootstrap_link,
      },
    };
  });
}

async function listOrganizations(token: AuthenticationToken, query: ListOrganizationsQueryData): Promise<BmsResponse> {
  return await wrapQuery(async () => {
    const axiosResponse = await http.getInstance().get(`/users/${query.userId}/clients/${query.clientId}/organizations`, {
      headers: { Authorization: `Bearer ${token}` },
      validateStatus: (status) => status === 200,
    });
    return {
      status: axiosResponse.status,
      isError: false,
      data: {
        type: DataType.ListOrganizations,
        organizations: (axiosResponse.data.results as Array<any>).map((org) => {
          return {
            bmsId: org.pk,
            createdAt: DateTime.fromISO(org.created_at, { zone: 'utc' }),
            expirationDate: org.created_at ? DateTime.fromISO(org.expiration_date, { zone: 'utc' }) : undefined,
            name: org.suffix,
            parsecId: org.parsec_id,
            stripeSubscriptionId: org.stripe_subscription_id,
            bootstrapLink: org.bootstrap_link,
          };
        }),
      },
    };
  });
}

async function getOrganizationStats(token: AuthenticationToken, query: OrganizationStatsQueryData): Promise<BmsResponse> {
  return await wrapQuery(async () => {
    const axiosResponse = await http
      .getInstance()
      .get(`/users/${query.userId}/clients/${query.clientId}/organizations/${query.organizationId}/stats`, {
        headers: { Authorization: `Bearer ${token}` },
        validateStatus: (status) => status === 200,
      });
    return {
      status: axiosResponse.status,
      isError: false,
      data: {
        type: DataType.OrganizationStats,
        dataSize: axiosResponse.data.data_size,
        metadataSize: axiosResponse.data.metadata_size,
        users: axiosResponse.data.users ?? 0,
        activeUsers: axiosResponse.data.active_users ?? 0,
        adminUsersDetail: axiosResponse.data.users_per_profile_detail.ADMIN,
        standardUsersDetail: axiosResponse.data.users_per_profile_detail.STANDARD,
        outsiderUsersDetail: axiosResponse.data.users_per_profile_detail.OUTSIDER,
        freeSliceSize: axiosResponse.data.free_slice_size ?? 1024 * 1024 * 1024 * 200, // arbitrary value
        payingSliceSize: axiosResponse.data.paying_slice_size ?? 1024 * 1024 * 1024 * 100, // arbitrary value
        status: axiosResponse.data.status,
      },
    };
  });
}

async function getOrganizationStatus(token: AuthenticationToken, query: OrganizationStatusQueryData): Promise<BmsResponse> {
  return await wrapQuery(async () => {
    const axiosResponse = await http
      .getInstance()
      .get(`/users/${query.userId}/clients/${query.clientId}/organizations/${query.organizationId}/stats`, {
        headers: { Authorization: `Bearer ${token}` },
        validateStatus: (status) => status === 200,
      });
    return {
      status: axiosResponse.status,
      isError: false,
      data: {
        type: DataType.OrganizationStatus,
        activeUsersLimit: axiosResponse.data.active_users_limit ?? undefined,
        isBootstrapped: axiosResponse.data.is_bootstrapped,
        isFrozen: axiosResponse.data.is_frozen,
        isInitialized: axiosResponse.data.is_initialized,
        outsidersAllowed: axiosResponse.data.user_profile_outsider_allowed,
      },
    };
  });
}

async function getInvoices(token: AuthenticationToken, query: InvoicesQueryData): Promise<BmsResponse> {
  return await wrapQuery(async () => {
    const axiosResponse = await http.getInstance().get(`/users/${query.userId}/clients/${query.clientId}/invoices`, {
      headers: { Authorization: `Bearer ${token}` },
      validateStatus: (status) => status === 200,
    });

    const invoices: Array<object> = [];

    for (let i = 2012; i < 2024; i++) {
      for (let j = 1; j < 13; j++) {
        invoices.push({
          id: `${i}-${j}`,
          pdfLink: 'https://www.youtube.com/watch?v=dQw4w9WgXcQ',
          start: DateTime.fromFormat(`${i}-${j}`, 'yyyy-L'),
          end: DateTime.fromFormat(`${i}-${j}`, 'yyyy-L').plus({ month: 1 }),
          total: 13.37,
          status: 'paid',
          number: `${i}-${j}`,
          receiptNumber: `${i}-${j}`,
        });
      }
    }

    return {
      status: 200,
      isError: false,
      data: {
        type: DataType.Invoices,
        invoices: invoices,
      },
    };

    // return {
    //   status: axiosResponse.status,
    //   isError: false,
    //   data: {
    //     type: DataType.Invoices,
    //     invoices: axiosResponse.data.results.map((invoice: any) => {
    //       return {
    //         id: invoice.id,
    //         pdfLink: invoice.pdf,
    //         start: DateTime.fromISO(invoice.period_start, { zone: 'utc' }),
    //         end: DateTime.fromISO(invoice.period_end, { zone: 'utc' }),
    //         total: invoice.total,
    //         status: invoice.status,
    //         organizationId: invoice.organization,
    //         number: invoice.number,
    //         receiptNumber: invoice.receipt_number,
    //       };
    //     }),
    //   },
    // };
  });
}

async function getBillingDetails(token: AuthenticationToken, query: BillingDetailsQueryData): Promise<BmsResponse> {
  return wrapQuery(async () => {
    const axiosResponse = await http.getInstance().get(`/users/${query.userId}/clients/${query.clientId}/billing_details`, {
      headers: { Authorization: `Bearer ${token}` },
      validateStatus: (status) => status === 200,
    });

    return {
      status: axiosResponse.status,
      isError: false,
      data: {
        type: DataType.BillingDetails,
        email: axiosResponse.data.email,
        name: axiosResponse.data.name,
        address: axiosResponse.data.address,
        paymentMethods: (axiosResponse.data.payment_methods ?? [])
          .map((method: any) => {
            if (method.type === 'card') {
              return {
                type: PaymentMethod.Card,
                id: method.id,
                brand: method.brand,
                expirationDate: DateTime.fromFormat(method.exp_date, 'LL/yy', { zone: 'utc' }),
                lastDigits: method.last_digits,
                isDefault: method.default,
              };
            } else if (method.type === 'debit') {
              return {
                type: PaymentMethod.SepaTransfer,
                id: method.id,
                bankName: method.bank_name,
                lastDigits: method.last_digits,
                isDefault: method.default,
              };
            } else {
              console.warn(`Unknown payment method type ${method.type}`);
              return undefined;
            }
          })
          .filter((method: any) => method !== undefined),
      },
    };
  });
}

async function addPaymentMethod(token: AuthenticationToken, query: AddPaymentMethodQueryData): Promise<BmsResponse> {
  return wrapQuery(async () => {
    const axiosResponse = await http.getInstance().put(
      `/users/${query.userId}/clients/${query.clientId}/add_payment_method`,
      // eslint-disable-next-line camelcase
      { payment_method: query.paymentMethod },
      {
        headers: { Authorization: `Bearer ${token}` },
        validateStatus: (status) => status === 200,
      },
    );

    return {
      type: DataType.AddPaymentMethod,
      status: axiosResponse.status,
      isError: false,
    };
  });
}

async function setDefaultPaymentMethod(token: AuthenticationToken, query: SetDefaultPaymentMethodQueryData): Promise<BmsResponse> {
  return wrapQuery(async () => {
    const axiosResponse = await http.getInstance().patch(
      `/users/${query.userId}/clients/${query.clientId}/default_payment_method`,
      // eslint-disable-next-line camelcase
      { payment_method: query.paymentMethod },
      {
        headers: { Authorization: `Bearer ${token}` },
        validateStatus: (status) => status === 200,
      },
    );

    return {
      type: DataType.SetDefaultPaymentMethod,
      status: axiosResponse.status,
      isError: false,
    };
  });
}

async function deletePaymentMethod(token: AuthenticationToken, query: DeletePaymentMethodQueryData): Promise<BmsResponse> {
  return wrapQuery(async () => {
    const axiosResponse = await http.getInstance().post(
      `/users/${query.userId}/clients/${query.clientId}/delete_payment_method`,
      // eslint-disable-next-line camelcase
      { payment_method: query.paymentMethod },
      {
        headers: { Authorization: `Bearer ${token}` },
        validateStatus: (status) => status === 200,
      },
    );

    return {
      type: DataType.DeletePaymentMethod,
      status: axiosResponse.status,
      isError: false,
    };
  });
}

async function refreshToken(refreshToken: AuthenticationToken): Promise<BmsResponse> {
  return await wrapQuery(async () => {
    const axiosResponse = await http.getInstance().post(
      '/api/token/refresh',
      {
        refresh: refreshToken,
      },
      {
        validateStatus: (status) => status === 200,
      },
    );
    return {
      status: axiosResponse.status,
      isError: false,
      data: {
        type: DataType.RefreshToken,
        token: axiosResponse.data.access,
      },
    };
  });
}

export const BmsApi = {
  login,
  getPersonalInformation,
  updatePersonalInformation,
  updateEmail,
  updatePassword,
  createOrganization,
  listOrganizations,
  getOrganizationStats,
  getOrganizationStatus,
  getInvoices,
  refreshToken,
  getBillingDetails,
  addPaymentMethod,
  setDefaultPaymentMethod,
  deletePaymentMethod,
  updateAuthentication,
};
