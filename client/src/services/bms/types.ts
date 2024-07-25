// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { OrganizationID } from '@/parsec';
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
  ListOrganizations = 'list-organizations',
  OrganizationStats = 'organization-stats',
  OrganizationStatus = 'organization-status',
  Invoices = 'invoices',
  RefreshToken = 'refresh-token',
}

type AuthenticationToken = string;

// Data used for result
interface LoginResultData {
  type: DataType.Login;
  accessToken: AuthenticationToken;
  refreshToken: AuthenticationToken;
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

interface ListOrganizationsResultData {
  type: DataType.ListOrganizations;
  organizations: Array<BmsOrganization>;
}

interface OrganizationStatsResultData {
  type: DataType.OrganizationStats;
  dataSize: number;
  // Add more status
  status: 'ok';
  users: number;
}

interface OrganizationStatusResultData {
  type: DataType.OrganizationStatus;
  activeUsersLimit?: number;
  isBootstrapped: boolean;
  isFrozen: boolean;
  isInitialized: boolean;
  outsidersAllowed: boolean;
}

interface InvoicesResultData {
  type: DataType.Invoices;
  invoices: Array<BmsInvoice>;
}

interface RefreshTokenResultData {
  type: DataType.RefreshToken;
  token: AuthenticationToken;
}

type ResultData =
  | LoginResultData
  | PersonalInformationResultData
  | CreateOrganizationResultData
  | ListOrganizationsResultData
  | OrganizationStatsResultData
  | OrganizationStatusResultData
  | InvoicesResultData
  | RefreshTokenResultData;

// Misc data
interface BmsOrganization {
  bmsId: string;
  createdAt: DateTime;
  parsecId: OrganizationID;
  name: string;
  bootstrapLink: string;
  expirationDate?: DateTime;
  stripeSubscriptionId?: string;
}

interface BmsInvoice {
  id: string;
  pdf: string;
  start: DateTime;
  end: DateTime;
  total: number;
  status: string;
  organizationId: OrganizationID;
}

// Data used for queries
interface LoginQueryData {
  email: string;
  password: string;
}

interface _ClientQueryData {
  userId: string;
  clientId: string;
}

interface CreateOrganizationQueryData extends _ClientQueryData {
  organizationName: string;
}

interface ListOrganizationsQueryData extends _ClientQueryData {}

interface OrganizationStatsQueryData extends _ClientQueryData {
  organizationId: string;
}

interface OrganizationStatusQueryData extends _ClientQueryData {
  organizationId: string;
}

interface InvoicesQueryData extends _ClientQueryData {}

export {
  AuthenticationToken,
  BmsError,
  BmsInvoice,
  BmsOrganization,
  BmsResponse,
  CreateOrganizationQueryData,
  DataType,
  InvoicesQueryData,
  ListOrganizationsQueryData,
  ListOrganizationsResultData,
  LoginQueryData,
  LoginResultData,
  OrganizationStatsQueryData,
  OrganizationStatsResultData,
  OrganizationStatusQueryData,
  OrganizationStatusResultData,
  PersonalInformationResultData,
};
