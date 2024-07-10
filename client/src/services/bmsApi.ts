// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { needsMocks } from '@/parsec';
import axios, { AxiosError, AxiosResponse, isAxiosError } from 'axios';

const BMS_ENV_VARIABLE = 'VITE_BMS_URL';
const DEFAULT_BMS_URL = 'https://bms-dev.parsec.cloud';

function getBmsUrl(): string {
  return import.meta.env[BMS_ENV_VARIABLE] ?? DEFAULT_BMS_URL;
}

interface BmsError {
  code: string;
  attr: string;
  detail: string;
}

interface BmsResponse {
  status: number;
  isError: () => boolean;
  errors?: Array<BmsError>;
  data?: ResultData;
}

enum DataType {
  Login = 'login',
  PersonalInformation = 'personal-information',
}

type AuthenticationToken = string;

// Data used for result
interface LoginResultData {
  type: DataType.Login;
  token: AuthenticationToken;
}

interface PersonalInformationResultData {
  type: DataType.PersonalInformation;
  firstName: string;
  lastName: string;
  email: string;
}

type ResultData = LoginResultData | PersonalInformationResultData;

// Data used for queries
interface LoginQueryData {
  email: string;
  password: string;
}

interface PersonalInformationQueryData {
  token: AuthenticationToken;
}

const axiosInstance = axios.create({
  baseURL: getBmsUrl(),
  timeout: 10000,
});

const Routes = {
  login: '/login',
  getPersonalInformation: '/info',
};

function parseBmsErrors(data: object): Array<BmsError> {
  if ((data as { errors: [] }).errors) {
    return (data as { errors: [] }).errors as Array<BmsError>;
  }
  return [];
}

async function login(query: LoginQueryData): Promise<BmsResponse> {
  if (needsMocks()) {
    return {
      status: 200,
      isError: (): boolean => false,
      data: {
        type: DataType.Login,
        token: 't0k3n',
      },
    };
  }
  try {
    const axiosResponse = await axiosInstance.post(Routes.login, query);
    return {
      status: axiosResponse.status,
      isError: (): boolean => axiosResponse.status !== 200,
      data: {
        type: DataType.Login,
        token: axiosResponse.data.token,
      },
    };
  } catch (error: any) {
    if (isAxiosError(error) && (error as AxiosError).response) {
      const response = (error as AxiosError).response as AxiosResponse;
      return {
        status: response.status,
        isError: (): boolean => true,
        errors: parseBmsErrors(response.data),
      };
    }
    throw new Error(error);
  }
}

async function getPersonalInformation(query: PersonalInformationQueryData): Promise<BmsResponse> {
  if (needsMocks()) {
    return {
      status: 200,
      isError: (): boolean => false,
      data: {
        type: DataType.PersonalInformation,
        email: 'gordon.freeman@blackmesa.nm',
        firstName: 'Gordon',
        lastName: 'Freeman',
      },
    };
  }
  try {
    const axiosResponse = await axiosInstance.post(Routes.login, query, { headers: { Authorization: `Token ${query.token}` } });
    return {
      status: axiosResponse.status,
      isError: (): boolean => axiosResponse.status !== 200,
      data: {
        type: DataType.PersonalInformation,
        email: axiosResponse.data.email,
        firstName: axiosResponse.data.firstname,
        lastName: axiosResponse.data.lastname,
      },
    };
  } catch (error: any) {
    if (isAxiosError(error) && (error as AxiosError).response) {
      const response = (error as AxiosError).response as AxiosResponse;
      return {
        status: response.status,
        isError: (): boolean => true,
        errors: parseBmsErrors(response.data),
      };
    }
    throw new Error(error);
  }
}

export const BmsApi = {
  login,
  getPersonalInformation,
};

export {
  AuthenticationToken,
  BmsError,
  BmsResponse,
  DataType,
  LoginQueryData,
  LoginResultData,
  PersonalInformationQueryData,
  PersonalInformationResultData,
};
