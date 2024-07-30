// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import {
  AuthenticationToken,
  BmsError,
  BmsResponse,
  CreateOrganizationQueryData,
  DataType,
  InvoicesQueryData,
  ListOrganizationsQueryData,
  LoginQueryData,
  OrganizationStatsQueryData,
  OrganizationStatusQueryData,
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
    if (isAxiosError(error) && (error as AxiosError).response) {
      const response = (error as AxiosError).response as AxiosResponse;
      return {
        status: response.status,
        isError: true,
        errors: parseBmsErrors(response.data),
      };
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
      },
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
        status: axiosResponse.data.status,
        users: axiosResponse.data.users,
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
    return {
      status: axiosResponse.status,
      isError: false,
      data: {
        type: DataType.Invoices,
        invoices: axiosResponse.data.results.map((invoice: any) => {
          return {
            id: invoice.id,
            pdfLink: invoice.pdf,
            start: DateTime.fromISO(invoice.period_start, { zone: 'utc' }),
            end: DateTime.fromISO(invoice.period_end, { zone: 'utc' }),
            total: invoice.total,
            status: invoice.status,
            organizationId: invoice.organization,
          };
        }),
      },
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
  createOrganization,
  listOrganizations,
  getOrganizationStats,
  getOrganizationStatus,
  getInvoices,
  refreshToken,
};
