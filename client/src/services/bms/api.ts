// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { AuthenticationToken, BmsError, BmsResponse, CreateOrganizationQueryData, DataType, LoginQueryData } from '@/services/bms/types';
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

async function login(query: LoginQueryData): Promise<BmsResponse> {
  try {
    const axiosResponse = await http.getInstance().post('/api/token', query);
    if (axiosResponse.status !== 200) {
      return {
        status: axiosResponse.status,
        isError: true,
        errors: parseBmsErrors(axiosResponse.data),
      };
    }
    return {
      status: axiosResponse.status,
      isError: false,
      data: {
        type: DataType.Login,
        token: axiosResponse.data.access,
      },
    };
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

async function getPersonalInformation(token: AuthenticationToken): Promise<BmsResponse> {
  try {
    const decodedToken = decodeToken(token);
    if (decodedToken === undefined) {
      throw new Error('Token is invalid');
    }
    const axiosResponse = await http.getInstance().get(`/users/${decodedToken.userId}`, { headers: { Authorization: `Bearer ${token}` } });
    if (axiosResponse.status !== 200) {
      return {
        status: axiosResponse.status,
        isError: true,
        errors: parseBmsErrors(axiosResponse.data),
      };
    }
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

async function createOrganization(token: AuthenticationToken, query: CreateOrganizationQueryData): Promise<BmsResponse> {
  try {
    const decodedToken = decodeToken(token);
    if (decodedToken === undefined) {
      throw new Error('Token is invalid');
    }
    const axiosResponse = await http.getInstance().post(
      `/users/${query.userId}/clients/${query.clientId}/organizations`,
      // eslint-disable-next-line camelcase
      { organization_id: query.organizationName },
      { headers: { Authorization: `Bearer ${token}` } },
    );
    if (axiosResponse.status !== 201) {
      return {
        status: axiosResponse.status,
        isError: true,
        errors: parseBmsErrors(axiosResponse.data),
      };
    }
    return {
      status: axiosResponse.status,
      isError: false,
      data: {
        type: DataType.CreateOrganization,
        bootstrapLink: axiosResponse.data.bootstrap_link,
      },
    };
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

export const BmsApi = {
  login,
  getPersonalInformation,
  createOrganization,
};
