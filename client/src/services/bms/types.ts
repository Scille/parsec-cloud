// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { OrganizationID } from '@/parsec';
import { DateTime } from 'luxon';

export type BmsLang = 'en' | 'fr';

export const CONNECTION_ERROR_STATUS = 0;

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
  ChangePassword = 'change-password',
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
  UnsubscribeOrganization = 'unsubscribe-organization',
  CustomOrderStatus = 'custom-order-status',
  CustomOrderDetails = 'custom-order-details',
  CreateCustomOrderRequest = 'create-custom-order-request',
  GetCustomOrderRequests = 'get-custom-order-requests',
  CustomOrderInvoices = 'custom-order-invoices',
}

enum PaymentMethod {
  Card = 'card',
  SepaTransfer = 'sepa-transfer',
}

enum BillingSystem {
  None = 'NONE',
  CustomOrder = 'CUSTOM_ORDER',
  Stripe = 'STRIPE',
  ExperimentalCandidate = 'EXPERIMENTAL_CANDIDATE',
}

enum CustomOrderStatus {
  Unknown = 'unknown',
  ContractEnded = 'contract_ended',
  NothingLinked = 'nothing_linked',
  EstimateLinked = 'estimate_linked',
  InvoiceToBePaid = 'invoice_to_be_paid',
  InvoicePaid = 'invoice_paid',
}

enum CustomOrderRequestStatus {
  Received = 'RECEIVED',
  Processing = 'PROCESSING',
  Standby = 'STANDBY',
  Finished = 'FINISHED',
  Cancelled = 'CANCELLED',
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
  billingSystem: BillingSystem;
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

interface CustomOrderStatusResultData {
  type: DataType.CustomOrderStatus;
  status: CustomOrderStatus;
}

interface CustomOrderRow {
  quantityOrdered: number;
  amountWithTaxes: number;
}

interface CustomOrderDetailsResultData {
  type: DataType.CustomOrderDetails;
  id: number;
  link: string;
  number: string;
  amountWithTaxes: number;
  amountWithoutTaxes: number;
  amountDue: number;
  currency: string;
  created: DateTime;
  dueDate: DateTime;
  licenseStart?: DateTime;
  licenseEnd?: DateTime;
  status: InvoiceStatus;
  administrators: CustomOrderRow;
  standards: CustomOrderRow;
  outsiders: CustomOrderRow;
  storage: CustomOrderRow;
}

interface CustomOrderInvoicesResultData {
  type: DataType.CustomOrderInvoices;
  invoices: Array<CustomOrderDetailsResultData>;
}

interface InvoicesResultData {
  type: DataType.Invoices;
  invoices: Array<BmsInvoice>;
}

interface RefreshTokenResultData {
  type: DataType.RefreshToken;
  token: AuthenticationToken;
}

interface GetCustomOrderRequestsResultData {
  type: DataType.GetCustomOrderRequests;
  requests: Array<{
    id: string;
    organizationId?: string;
    describedNeeds: string;
    users: number;
    storage: number;
    status: CustomOrderRequestStatus;
    comment: string;
    orderDate: DateTime;
  }>;
}

interface BmsAddress {
  line1: string;
  line2?: string;
  city: string;
  postalCode: string;
  country: string;
  state?: string;
}

interface BillingDetailsPaymentMethodCard {
  type: PaymentMethod.Card;
  id: string;
  lastDigits: string;
  isDefault: boolean;
  expirationDate: DateTime;
  brand: string;
  isExpired: () => boolean;
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
  address: BmsAddress;
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
  | BillingDetailsResultData
  | CustomOrderStatusResultData
  | CustomOrderDetailsResultData
  | GetCustomOrderRequestsResultData
  | CustomOrderInvoicesResultData;

// Misc data
interface BmsOrganization {
  bmsId: string;
  createdAt: DateTime;
  parsecId: OrganizationID;
  name: string;
  bootstrapLink: string;
  expirationDate?: DateTime;
  stripeSubscriptionId?: string;
  isSubscribed: () => boolean;
}

interface BmsInvoice {
  id: string;
  pdfLink: string;
  start: DateTime;
  end: DateTime;
  total: number;
  status: InvoiceStatus;
  organizationId: OrganizationID;
  number: string;
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

interface UserQueryData {
  userId: string;
}

interface ClientQueryData extends UserQueryData {
  clientId: string;
}

interface OrganizationQueryData extends ClientQueryData {
  organizationId: string;
}

interface UpdatePersonalInformationQueryData extends UserQueryData {
  client: {
    firstname?: string;
    lastname?: string;
    phone?: string;
    country?: string;
    company?: string;
    job?: string;
  };
}

interface UpdateEmailQueryData extends UserQueryData {
  email: string;
  password: string;
  code: string;
  lang: BmsLang;
}

interface UpdateEmailSendCodeQueryData {
  email: string;
  lang: BmsLang;
}

interface ChangePasswordQueryData {
  email: string;
  lang: BmsLang;
}

interface UpdateAuthenticationQueryData extends UserQueryData {
  password: string;
  newPassword: string;
}

interface CreateOrganizationQueryData extends ClientQueryData {
  organizationName: string;
}

interface AddPaymentMethodQueryData extends ClientQueryData {
  paymentMethod: string;
}

interface SetDefaultPaymentMethodQueryData extends ClientQueryData {
  paymentMethod: string;
}

interface DeletePaymentMethodQueryData extends ClientQueryData {
  paymentMethod: string;
}

interface UpdateBillingDetailsQueryData extends ClientQueryData {
  address: BmsAddress;
}

interface CustomOrderQueryData extends ClientQueryData {
  organization: BmsOrganization;
}

interface CreateCustomOrderRequestQueryData {
  describedNeeds: string;
  adminUsers?: number;
  standardUsers: number;
  outsiderUsers?: number;
  storage: number;
  formula?: string;
  organizationName?: string;
}

export {
  AddPaymentMethodQueryData,
  AuthenticationToken,
  BillingDetailsPaymentMethodCard,
  BillingDetailsPaymentMethodSepaTransfer,
  BillingDetailsResultData,
  BillingSystem,
  BmsAddress,
  BmsError,
  BmsInvoice,
  BmsOrganization,
  BmsResponse,
  ChangePasswordQueryData,
  ClientQueryData,
  CreateCustomOrderRequestQueryData,
  CreateOrganizationQueryData,
  CustomOrderDetailsResultData,
  CustomOrderInvoicesResultData,
  CustomOrderQueryData,
  CustomOrderRequestStatus,
  CustomOrderStatus,
  CustomOrderStatusResultData,
  DataType,
  DeletePaymentMethodQueryData,
  InvoiceStatus,
  ListOrganizationsResultData,
  LoginQueryData,
  LoginResultData,
  OrganizationQueryData,
  OrganizationStatsResultData,
  OrganizationStatusResultData,
  PaymentMethod,
  PersonalInformationResultData,
  SetDefaultPaymentMethodQueryData,
  UpdateAuthenticationQueryData,
  UpdateBillingDetailsQueryData,
  UpdateEmailQueryData,
  UpdateEmailSendCodeQueryData,
  UpdatePersonalInformationQueryData,
  UserQueryData,
};
