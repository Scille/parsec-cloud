// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { DateTime } from 'luxon';

interface BmsError {
  code: string;
  attr: string;
  detail: string;
}

interface BmsResponse {
  status: number;
  isError: boolean;
  errors?: Array<BmsError>;
  data?: ResultData;
}

enum DataType {
  Login = 'login',
  PersonalInformation = 'personal-information',
  CreateOrganization = 'create-organization',
}

type AuthenticationToken = string;

// Data used for result
interface LoginResultData {
  type: DataType.Login;
  token: AuthenticationToken;
}

// BMS does return additional infos, ignoring them
// for now
interface PersonalInformationResultData {
  type: DataType.PersonalInformation;
  id: string;
  createdAt: DateTime;
  firstName: string;
  lastName: string;
  email: string;
  clientId: string;
}

interface CreateOrganizationResultData {
  type: DataType.CreateOrganization;
  bootstrapLink: string;
}

type ResultData = LoginResultData | PersonalInformationResultData | CreateOrganizationResultData;

// Data used for queries
interface LoginQueryData {
  email: string;
  password: string;
}

interface CreateOrganizationQueryData {
  userId: string;
  clientId: string;
  organizationName: string;
}

export {
  AuthenticationToken,
  BmsError,
  BmsResponse,
  CreateOrganizationQueryData,
  DataType,
  LoginQueryData,
  LoginResultData,
  PersonalInformationResultData,
};
