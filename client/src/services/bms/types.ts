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
  UpdatePersonalInformation = 'update-personal-information',
  UpdateEmail = 'update-email',
  UpdatePassword = 'update-password',
  CreateOrganization = 'create-organization',
  ListOrganizations = 'list-organizations',
  OrganizationStats = 'organization-stats',
  OrganizationStatus = 'organization-status',
  Invoices = 'invoices',
  RefreshToken = 'refresh-token',
  BillingDetails = 'billing-details',
  AddPaymentMethod = 'add-payment-method',
  SetDefaultPaymentMethod = 'set-default-payment-method',
  DeletePaymentMethod = 'delete-payment-method',
  UpdateAuthentication = 'update-authentication',
}

enum PaymentMethod {
  Card = 'card',
  SepaTransfer = 'sepa-transfer',
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
  phone?: string;
  company?: string;
  job?: string;
}

interface CreateOrganizationResultData {
  type: DataType.CreateOrganization;
  bootstrapLink: string;
}

interface ListOrganizationsResultData {
  type: DataType.ListOrganizations;
  organizations: Array<BmsOrganization>;
}

interface UserPerProfileDetails {
  active: number;
  revoked: number;
}

interface OrganizationStatsResultData {
  type: DataType.OrganizationStats;
  realms?: number;
  dataSize: number;
  metadataSize: number;
  users: number;
  activeUsers: number;
  adminUsersDetail: UserPerProfileDetails;
  standardUsersDetail: UserPerProfileDetails;
  outsiderUsersDetail: UserPerProfileDetails;
  freeSliceSize: number;
  payingSliceSize: number;
  // Add more status
  status: 'ok';
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

interface BillingDetailsPaymentMethodCard {
  type: PaymentMethod.Card;
  id: string;
  lastDigits: string;
  isDefault: boolean;
  expirationDate: DateTime;
  brand: string;
}

interface BillingDetailsPaymentMethodSepaTransfer {
  type: PaymentMethod.SepaTransfer;
  id: string;
  lastDigits: string;
  isDefault: boolean;
  bankName: string;
}

interface BillingDetailsResultData {
  type: DataType.BillingDetails;
  email: string;
  name: string;
  address: string;
  paymentMethods: Array<BillingDetailsPaymentMethodCard | BillingDetailsPaymentMethodSepaTransfer>;
}

type ResultData =
  | LoginResultData
  | PersonalInformationResultData
  | CreateOrganizationResultData
  | ListOrganizationsResultData
  | OrganizationStatsResultData
  | OrganizationStatusResultData
  | InvoicesResultData
  | RefreshTokenResultData
  | BillingDetailsResultData;

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
  pdfLink: string;
  start: DateTime;
  end: DateTime;
  total: number;
  status: InvoiceStatus;
  organizationId: OrganizationID;
}

enum InvoiceStatus {
  Paid = 'paid',
  Draft = 'draft',
  Open = 'open',
  Uncollectible = 'uncollectible',
  Void = 'void',
}

// Data used for queries
interface LoginQueryData {
  email: string;
  password: string;
}

interface _UserQueryData {
  userId: string;
}

interface _ClientQueryData extends _UserQueryData {
  clientId: string;
}

interface UpdatePersonalInformationQueryData extends _UserQueryData {
  client: {
    firstname?: string;
    lastname?: string;
    phone?: string;
    country?: string;
    company?: string;
    job?: string;
  };
}

interface UpdateEmailQueryData extends _UserQueryData {
  email: string;
  password: string;
  lang: string;
}

interface UpdatePasswordQueryData extends _UserQueryData {
  email: string;
  lang: string;
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

interface BillingDetailsQueryData extends _ClientQueryData {}

interface AddPaymentMethodQueryData extends _ClientQueryData {
  paymentMethod: string;
}

interface SetDefaultPaymentMethodQueryData extends _ClientQueryData {
  paymentMethod: string;
}

interface DeletePaymentMethodQueryData extends _ClientQueryData {
  paymentMethod: string;
}

interface UpdateAuthenticationQueryData {
  userId: string;
  password: string;
  newPassword: string;
}

export {
  AddPaymentMethodQueryData,
  AuthenticationToken,
  BillingDetailsPaymentMethodCard,
  BillingDetailsPaymentMethodSepaTransfer,
  BillingDetailsQueryData,
  BillingDetailsResultData,
  BmsError,
  BmsInvoice,
  BmsOrganization,
  BmsResponse,
  CreateOrganizationQueryData,
  DataType,
  DeletePaymentMethodQueryData,
  InvoicesQueryData,
  ListOrganizationsQueryData,
  ListOrganizationsResultData,
  LoginQueryData,
  LoginResultData,
  OrganizationStatsQueryData,
  OrganizationStatsResultData,
  OrganizationStatusQueryData,
  OrganizationStatusResultData,
  PaymentMethod,
  PersonalInformationResultData,
  SetDefaultPaymentMethodQueryData,
  UpdateAuthenticationQueryData,
  UpdateEmailQueryData,
  UpdatePasswordQueryData,
  UpdatePersonalInformationQueryData,
};
