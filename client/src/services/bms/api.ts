// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import {
  AddPaymentMethodQueryData,
  AuthenticationToken,
  BillingSystem,
  BmsError,
  BmsOrganization,
  BmsResponse,
  CONNECTION_ERROR_STATUS,
  ChangePasswordQueryData,
  ClientQueryData,
  CreateCustomOrderRequestQueryData,
  CreateOrganizationQueryData,
  CustomOrderQueryData,
  DataType,
  DeletePaymentMethodQueryData,
  LoginQueryData,
  OrganizationQueryData,
  PaymentMethod,
  SetDefaultPaymentMethodQueryData,
  UpdateAuthenticationQueryData,
  UpdateBillingDetailsQueryData,
  UpdateEmailQueryData,
  UpdateEmailSendCodeQueryData,
  UpdatePersonalInformationQueryData,
} from '@/services/bms/types';
import { Env } from '@/services/environment';
import axios, { AxiosError, AxiosInstance, AxiosResponse, isAxiosError } from 'axios';
import { DateTime } from 'luxon';
import { decodeToken } from 'megashark-lib';

class HTTPRequest {
  private _instance: AxiosInstance;

  constructor() {
    console.log(`Using ${Env.getBmsUrl()} as Parsec BMS`);
    this._instance = axios.create({
      baseURL: Env.getBmsUrl(),
      timeout: 15000,
    });
  }

  getInstance(): AxiosInstance {
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
          status: CONNECTION_ERROR_STATUS,
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
        firstName: axiosResponse.data.client ? axiosResponse.data.client.firstname : '',
        lastName: axiosResponse.data.client ? axiosResponse.data.client.lastname : '',
        clientId: axiosResponse.data.client ? axiosResponse.data.client.id : '',
        phone: axiosResponse.data.client ? axiosResponse.data.client.phone : undefined,
        job: axiosResponse.data.client ? axiosResponse.data.client.job : undefined,
        company: axiosResponse.data.client ? axiosResponse.data.client.company : undefined,
        billingSystem: axiosResponse.data.client ? (axiosResponse.data.client.billing_system as BillingSystem) : BillingSystem.None,
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

async function updateEmailSendCode(token: AuthenticationToken, data: UpdateEmailSendCodeQueryData): Promise<BmsResponse> {
  return await wrapQuery(async () => {
    const axiosResponse = await http.getInstance().post('/email_validation/send_code', data, {
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

async function changePassword(data: ChangePasswordQueryData): Promise<BmsResponse> {
  return await wrapQuery(async () => {
    const axiosResponse = await http.getInstance().post('/users/change_password', data);
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

async function listOrganizations(token: AuthenticationToken, query: ClientQueryData): Promise<BmsResponse> {
  return await wrapQuery(async () => {
    let fetchedAll = false;
    let currentUrl = `/users/${query.userId}/clients/${query.clientId}/organizations`;
    const orgs: Array<BmsOrganization> = [];

    while (!fetchedAll) {
      const axiosResponse = await http.getInstance().get(currentUrl, {
        headers: { Authorization: `Bearer ${token}` },
        validateStatus: (status) => status === 200,
      });
      orgs.push(
        ...(axiosResponse.data.results as Array<any>).map((org) => {
          return {
            bmsId: org.pk,
            createdAt: DateTime.fromISO(org.created_at, { zone: 'utc' }),
            expirationDate: org.created_at ? DateTime.fromISO(org.expiration_date, { zone: 'utc' }) : undefined,
            name: org.suffix,
            parsecId: org.parsec_id,
            stripeSubscriptionId: org.stripe_subscription_id ?? undefined,
            bootstrapLink: org.bootstrap_link,
            isSubscribed: () => Boolean(org.stripe_subscription_id),
          };
        }),
      );
      if (axiosResponse.data.next) {
        currentUrl = axiosResponse.data.next;
      } else {
        fetchedAll = true;
      }
    }
    return {
      status: 200,
      isError: false,
      data: {
        type: DataType.ListOrganizations,
        organizations: orgs,
      },
    };
  });
}

async function getOrganizationStats(token: AuthenticationToken, query: OrganizationQueryData): Promise<BmsResponse> {
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
        dataSize: axiosResponse.data.data_size ?? 0,
        metadataSize: axiosResponse.data.metadata_size ?? 0,
        users: axiosResponse.data.users ?? 0,
        activeUsers: axiosResponse.data.active_users ?? 0,
        adminUsersDetail: axiosResponse.data.users_per_profile_detail?.ADMIN ?? { active: 0, revoked: 0 },
        standardUsersDetail: axiosResponse.data.users_per_profile_detail?.STANDARD ?? { active: 0, revoked: 0 },
        outsiderUsersDetail: axiosResponse.data.users_per_profile_detail?.OUTSIDER ?? { active: 0, revoked: 0 },
        freeSliceSize: axiosResponse.data.free_slice_size ?? 1024 * 1024 * 1024 * 200, // arbitrary value
        payingSliceSize: axiosResponse.data.paying_slice_size ?? 1024 * 1024 * 1024 * 100, // arbitrary value
        status: axiosResponse.data.status,
      },
    };
  });
}

async function getOrganizationStatus(token: AuthenticationToken, query: OrganizationQueryData): Promise<BmsResponse> {
  return await wrapQuery(async () => {
    const axiosResponse = await http
      .getInstance()
      .get(`/users/${query.userId}/clients/${query.clientId}/organizations/${query.organizationId}/status`, {
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

async function getInvoices(token: AuthenticationToken, query: ClientQueryData): Promise<BmsResponse> {
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
            total: invoice.total / 100.0,
            status: invoice.status,
            organizationId: invoice.organization,
            number: invoice.number,
            receiptNumber: invoice.receipt_number,
          };
        }),
      },
    };
  });
}

async function getBillingDetails(token: AuthenticationToken, query: ClientQueryData): Promise<BmsResponse> {
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
        address: {
          line1: axiosResponse.data.address.line1,
          line2: axiosResponse.data.address.line2 || undefined,
          city: axiosResponse.data.address.city,
          postalCode: axiosResponse.data.address.postal_code,
          state: axiosResponse.data.address.state || undefined,
          country: axiosResponse.data.address.country,
        },
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
                isExpired: () => DateTime.fromFormat(method.exp_date, 'LL/yy', { zone: 'utc' }).plus({ month: 1 }) < DateTime.utc(),
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

async function updateBillingDetails(token: AuthenticationToken, query: UpdateBillingDetailsQueryData): Promise<BmsResponse> {
  return wrapQuery(async () => {
    const axiosResponse = await http.getInstance().patch(
      `/users/${query.userId}/clients/${query.clientId}/billing_details`,
      {
        address: {
          line1: query.address.line1,
          line2: query.address.line2,
          city: query.address.city,
          // eslint-disable-next-line camelcase
          postal_code: query.address.postalCode,
          country: query.address.country,
        },
      },
      {
        headers: { Authorization: `Bearer ${token}` },
        validateStatus: (status) => status === 200,
      },
    );
    return {
      status: axiosResponse.status,
      isError: false,
    };
  });
}

async function getCustomOrderStatus(token: AuthenticationToken, query: CustomOrderQueryData): Promise<BmsResponse> {
  return wrapQuery(async () => {
    const axiosResponse = await http.getInstance().post(
      `/users/${query.userId}/clients/${query.clientId}/organizations/custom_order_status`,
      {
        // eslint-disable-next-line camelcase
        organization_ids: [query.organization.bmsId],
      },
      {
        headers: { Authorization: `Bearer ${token}` },
        validateStatus: (status) => status === 200,
        timeout: 15000,
      },
    );
    const orgData = axiosResponse.data[query.organization.parsecId];
    if (!orgData) {
      return {
        status: 404,
        isError: true,
      };
    }
    return {
      status: axiosResponse.status,
      isError: false,
      data: {
        type: DataType.CustomOrderStatus,
        status: orgData,
      },
    };
  });
}

async function getCustomOrderDetails(token: AuthenticationToken, query: CustomOrderQueryData): Promise<BmsResponse> {
  enum CustomOrderRowName {
    Administrator = 'Psc_D0_Adm_M',
    Standard = 'Psc_D0_Std_User_M',
    Outsider = 'Psc_D0_Ext_User_M',
    // cspell:disable-next-line
    Storage = 'Psc_Stck_100_Go_M',
  }

  enum CustomOrderFieldName {
    LicenseStart = 'parsec-saas-custom-order-start-date',
    LicenseEnd = 'parsec-saas-custom-order-end-date',
    StandardCount = 'parsec-saas-custom-order-standard-license-count',
    OutsiderCount = 'parsec-saas-custom-order-outsider-license-count',
    AdministratorCount = 'parsec-saas-custom-order-admin-license-count',
    StorageCount = 'parsec-saas-custom-order-storage-license-count',
  }

  return wrapQuery(async () => {
    const axiosResponse = await http.getInstance().post(
      `/users/${query.userId}/clients/${query.clientId}/organizations/custom_order_details`,
      {
        // eslint-disable-next-line camelcase
        organization_ids: [query.organization.bmsId],
      },
      {
        headers: { Authorization: `Bearer ${token}` },
        validateStatus: (status) => status === 200,
        timeout: 15000,
      },
    );
    const orgData = axiosResponse.data[query.organization.parsecId];
    if (!orgData || Object.keys(orgData).length === 0) {
      return {
        status: 404,
        isError: true,
      };
    }
    const adminRow = orgData.rows?.find((row: any) => row.reference === CustomOrderRowName.Administrator);
    const standardRow = orgData.rows?.find((row: any) => row.reference === CustomOrderRowName.Standard);
    const outsiderRow = orgData.rows?.find((row: any) => row.reference === CustomOrderRowName.Outsider);
    const storageRow = orgData.rows?.find((row: any) => row.reference === CustomOrderRowName.Storage);

    const fields = orgData._embed?.custom_fields || [];
    const adminField = fields.find((field: any) => field.code === CustomOrderFieldName.AdministratorCount);
    const standardField = fields.find((field: any) => field.code === CustomOrderFieldName.StandardCount);
    const outsiderField = fields.find((field: any) => field.code === CustomOrderFieldName.OutsiderCount);
    const storageField = fields.find((field: any) => field.code === CustomOrderFieldName.StorageCount);
    const licenseStartField = fields.find((field: any) => field.code === CustomOrderFieldName.LicenseStart);
    const licenseEndField = fields.find((field: any) => field.code === CustomOrderFieldName.LicenseEnd);

    return {
      status: axiosResponse.status,
      isError: false,
      data: {
        type: DataType.CustomOrderDetails,
        id: orgData.id,
        link: orgData.pdf_link,
        number: orgData.number,
        amountWithTaxes: parseFloat(orgData.amounts?.total_incl_tax),
        amountWithoutTaxes: parseFloat(orgData.amounts?.total_excl_tax),
        amountDue: parseFloat(orgData.amounts?.total_remaining_due_incl_tax),
        currency: orgData.currency,
        created: DateTime.fromISO(orgData.created, { zone: 'utc' }),
        dueDate: DateTime.fromISO(orgData.due_date, { zone: 'utc' }),
        licenseStart: licenseStartField ? DateTime.fromISO(licenseStartField.value, { zone: 'utc' }) : undefined,
        licenseEnd: licenseEndField ? DateTime.fromISO(licenseEndField.value, { zone: 'utc' }) : undefined,
        status: orgData.status,
        administrators: {
          quantityOrdered: adminField ? parseInt(adminField.value) : 0,
          amountWithTaxes: adminRow ? parseInt(adminRow.amount_tax_inc) : 0,
        },
        standards: {
          quantityOrdered: standardField ? parseInt(standardField.value) : 0,
          amountWithTaxes: standardRow ? parseInt(standardRow.amount_tax_inc) : 0,
        },
        outsiders: {
          quantityOrdered: outsiderField ? parseInt(outsiderField.value) : 0,
          amountWithTaxes: outsiderRow ? parseInt(outsiderRow.amount_tax_inc) : 0,
        },
        storage: {
          quantityOrdered: storageField ? parseInt(storageField.value) : 0,
          amountWithTaxes: storageRow ? parseInt(storageRow.amount_tax_inc) : 0,
        },
      },
    };
  });
}

async function unsubscribeOrganization(token: AuthenticationToken, query: OrganizationQueryData): Promise<BmsResponse> {
  return wrapQuery(async () => {
    const axiosResponse = await http.getInstance().post(
      `/users/${query.userId}/clients/${query.clientId}/organizations/${query.organizationId}/unsubscribe`,
      {},
      {
        headers: { Authorization: `Bearer ${token}` },
        validateStatus: (status) => status === 204,
      },
    );

    return {
      type: DataType.UnsubscribeOrganization,
      status: axiosResponse.status,
      isError: false,
    };
  });
}

async function subscribeOrganization(token: AuthenticationToken, query: OrganizationQueryData): Promise<BmsResponse> {
  return wrapQuery(async () => {
    const axiosResponse = await http.getInstance().post(
      `/users/${query.userId}/clients/${query.clientId}/organizations/${query.organizationId}/subscribe`,
      {},
      {
        headers: { Authorization: `Bearer ${token}` },
        validateStatus: (status) => status === 204,
      },
    );

    return {
      type: DataType.UnsubscribeOrganization,
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

async function createCustomOrderRequest(token: AuthenticationToken, query: CreateCustomOrderRequestQueryData): Promise<BmsResponse> {
  return await wrapQuery(async () => {
    const axiosResponse = await http.getInstance().post(
      '/custom_order_requests',
      {
        firstname: 'IGNORED',
        lastname: 'IGNORED',
        email: 'IGNORED@IGNORED.IG',
        password: 'ThisP@ssw0rd1sIGN0ReD#',
        address: {
          line1: 'IGNORED',
          city: 'IGNORED',
          // eslint-disable-next-line camelcase
          postal_code: '00000',
          country: 'IGNORED',
        },
        // eslint-disable-next-line camelcase
        described_need: query.describedNeeds,
        phone: '+33600000000',
        // eslint-disable-next-line camelcase
        admin_users: query.adminUsers,
        // eslint-disable-next-line camelcase
        standard_users: query.standardUsers,
        // eslint-disable-next-line camelcase
        outsider_users: query.outsiderUsers,
        storage: query.storage,
        formula: query.formula,
        // eslint-disable-next-line camelcase
        organization_name: query.organizationName,
      },
      {
        headers: { Authorization: `Bearer ${token}` },
        validateStatus: (status) => status === 204,
        timeout: 10000,
      },
    );
    return {
      status: axiosResponse.status,
      isError: false,
    };
  });
}

async function getCustomOrderRequests(token: AuthenticationToken): Promise<BmsResponse> {
  return wrapQuery(async () => {
    const axiosResponse = await http.getInstance().get('/custom_order_requests', {
      headers: { Authorization: `Bearer ${token}` },
      validateStatus: (status) => status === 200,
    });

    return {
      status: axiosResponse.status,
      isError: false,
      data: {
        type: DataType.GetCustomOrderRequests,
        requests: axiosResponse.data.requests.map((req: any) => {
          return {
            id: req.id,
            organizationId: req.parsec_id,
            describedNeeds: req.described_need,
            users: req.standard_users,
            storage: req.storage,
            status: req.status,
            comment: req.comment,
            orderDate: DateTime.fromISO(axiosResponse.data.created_at),
          };
        }),
      },
    };
  });
}

export const BmsApi = {
  login,
  getPersonalInformation,
  updatePersonalInformation,
  updateEmail,
  changePassword,
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
  updateBillingDetails,
  getCustomOrderStatus,
  getCustomOrderDetails,
  unsubscribeOrganization,
  subscribeOrganization,
  updateEmailSendCode,
  createCustomOrderRequest,
  getCustomOrderRequests,
};
