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
  MonthlySubscriptionInvoices = 'monthly-subscription-invoices',
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
  status: CustomOrderStatus;
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
  organizationId: OrganizationID;
}

interface CustomOrderInvoicesResultData {
  type: DataType.CustomOrderInvoices;
  invoices: Array<SellsyInvoice>;
}

interface MonthlySubscriptionInvoicesResultData {
  type: DataType.MonthlySubscriptionInvoices;
  invoices: Array<StripeInvoice>;
}

interface RefreshTokenResultData {
  type: DataType.RefreshToken;
  token: AuthenticationToken;
}

interface CustomOrderRequest {
  id: string;
  organizationId?: OrganizationID;
  describedNeeds: string;
  users: number;
  storage: number;
  status: CustomOrderRequestStatus;
  comment: string;
  orderDate: DateTime;
}

interface GetCustomOrderRequestsResultData {
  type: DataType.GetCustomOrderRequests;
  requests: Array<CustomOrderRequest>;
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
  | MonthlySubscriptionInvoicesResultData
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
  receiptNumber: string;
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

enum InvoiceType {
  Sellsy = 'sellsy',
  Stripe = 'stripe',
}

interface Invoice {
  getType(): InvoiceType;
  getId(): string;
  getDate(): DateTime;
  getNumber(): string;
  getAmount(): number;
  getOrganizationId(): OrganizationID;
  getStatus(): InvoiceStatus;
  getLink(): string;
}

class SellsyInvoice implements Invoice {
  invoice: CustomOrderDetailsResultData;

  constructor(invoice: CustomOrderDetailsResultData) {
    this.invoice = invoice;
  }

  getType(): InvoiceType {
    return InvoiceType.Sellsy;
  }

  getId(): string {
    return this.invoice.id.toString();
  }

  getDate(): DateTime {
    return this.invoice.created;
  }

  getNumber(): string {
    return this.invoice.number;
  }

  getAmount(): number {
    return this.invoice.amountWithTaxes;
  }

  // TODO: Replace with real organization ID
  getOrganizationId(): OrganizationID {
    return this.invoice.organizationId;
  }

  getStatus(): InvoiceStatus {
    return this.invoice.status;
  }

  getLink(): string {
    return this.invoice.link;
  }

  getLicenseStart(): DateTime | undefined {
    return this.invoice.licenseStart;
  }

  getLicenseEnd(): DateTime | undefined {
    return this.invoice.licenseEnd;
  }
}

class StripeInvoice implements Invoice {
  invoice: BmsInvoice;

  constructor(invoice: BmsInvoice) {
    this.invoice = invoice;
  }

  getType(): InvoiceType {
    return InvoiceType.Stripe;
  }

  getId(): string {
    return this.invoice.id;
  }

  getDate(): DateTime {
    return this.invoice.start;
  }

  getNumber(): string {
    return this.invoice.number;
  }

  getAmount(): number {
    return this.invoice.total;
  }

  getOrganizationId(): OrganizationID {
    return this.invoice.organizationId;
  }

  getStatus(): InvoiceStatus {
    return this.invoice.status;
  }

  getLink(): string {
    return this.invoice.pdfLink;
  }
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
  CustomOrderRequest,
  CustomOrderRequestStatus,
  CustomOrderStatus,
  CustomOrderStatusResultData,
  DataType,
  DeletePaymentMethodQueryData,
  GetCustomOrderRequestsResultData,
  Invoice,
  InvoiceStatus,
  InvoiceType,
  ListOrganizationsResultData,
  LoginQueryData,
  LoginResultData,
  OrganizationQueryData,
  OrganizationStatsResultData,
  OrganizationStatusResultData,
  PaymentMethod,
  PersonalInformationResultData,
  SellsyInvoice,
  SetDefaultPaymentMethodQueryData,
  StripeInvoice,
  UpdateAuthenticationQueryData,
  UpdateBillingDetailsQueryData,
  UpdateEmailQueryData,
  UpdateEmailSendCodeQueryData,
  UpdatePersonalInformationQueryData,
  UserQueryData,
};
