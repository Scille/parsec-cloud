// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { OrganizationID } from '@/parsec';
import { BmsApi } from '@/services/bms/api';
import { MockedBmsApi } from '@/services/bms/mockApi';
import {
  AuthenticationToken,
  BmsAddress,
  BmsError,
  BmsLang,
  BmsOrganization,
  BmsResponse,
  DataType,
  PersonalInformationResultData,
} from '@/services/bms/types';
import { storageManagerInstance } from '@/services/storageManager';
import { decodeToken } from 'megashark-lib';

function assertLoggedIn<T>(value: T): asserts value is NonNullable<T> {
  if (value === null) {
    throw new Error('Not logged in');
  }
}

interface Token {
  access: AuthenticationToken;
  refresh: AuthenticationToken;
}

class BmsAccess {
  private tokens: Token | null = null;
  private customerInformation: PersonalInformationResultData | null = null;
  public reloadKey: number = 0;
  private api: typeof BmsApi;

  constructor() {
    if (import.meta.env.PARSEC_APP_BMS_USE_MOCK === 'true') {
      console.info('Using Mocked BMS API');
      this.api = MockedBmsApi;
    } else {
      this.api = BmsApi;
    }
  }

  async tryAutoLogin(): Promise<boolean> {
    this.reloadKey += 1;
    await this.restoreAccess();
    if (!this.tokens) {
      return false;
    }

    if (await this.ensureFreshToken()) {
      const infoResponse = await this.api.getPersonalInformation(this.tokens.access);
      if (!infoResponse.isError && infoResponse.data && infoResponse.data.type === DataType.PersonalInformation) {
        this.customerInformation = infoResponse.data;
        return true;
      }
    }

    this.tokens = null;
    return false;
  }

  async login(email: string, password: string, storeCredentials = false): Promise<{ ok: boolean; errors?: Array<BmsError> }> {
    this.reloadKey += 1;
    const response = await this.api.login({ email: email, password: password });
    if (!response.isError && response.data && response.data.type === DataType.Login) {
      this.tokens = { access: response.data.accessToken, refresh: response.data.refreshToken };
    } else {
      return { ok: false, errors: response.errors };
    }
    const infoResponse = await this.api.getPersonalInformation(this.tokens.access);
    if (!infoResponse.isError && infoResponse.data && infoResponse.data.type === DataType.PersonalInformation) {
      this.customerInformation = infoResponse.data;
    } else {
      this.tokens = null;
    }
    if (storeCredentials) {
      await this.storeAccess();
    }
    return { ok: true };
  }

  async logout(): Promise<void> {
    this.tokens = null;
    this.customerInformation = null;
    await this.clearStoredAccess();
  }

  getPersonalInformation(): PersonalInformationResultData {
    assertLoggedIn(this.customerInformation);
    return this.customerInformation;
  }

  async getToken(): Promise<AuthenticationToken> {
    assertLoggedIn(this.tokens);
    return this.tokens.access;
  }

  async updatePersonalInformation(data: {
    firstname?: string;
    lastname?: string;
    phone?: string;
    country?: string;
    company?: string;
    job?: string;
  }): Promise<BmsResponse> {
    assertLoggedIn(this.tokens);
    assertLoggedIn(this.customerInformation);
    await this.ensureFreshToken();

    const response = await this.api.updatePersonalInformation(this.tokens.access, {
      userId: this.customerInformation.id,
      client: {
        firstname: data.firstname,
        lastname: data.lastname,
        phone: data.phone,
        country: data.country,
        company: data.company,
        job: data.job,
      },
    });
    if (!response.isError) {
      // Keep our local information up-to-date
      const infoResponse = await this.api.getPersonalInformation(this.tokens.access);
      if (!infoResponse.isError && infoResponse.data && infoResponse.data.type === DataType.PersonalInformation) {
        this.customerInformation = infoResponse.data;
      }
    }
    return response;
  }

  async createOrganization(organizationName: OrganizationID): Promise<BmsResponse> {
    assertLoggedIn(this.tokens);
    assertLoggedIn(this.customerInformation);
    await this.ensureFreshToken();

    return await this.api.createOrganization(this.tokens.access, {
      userId: this.customerInformation.id,
      clientId: this.customerInformation.clientId,
      organizationName: organizationName,
    });
  }

  async updateEmailSendCode(email: string, lang: BmsLang): Promise<BmsResponse> {
    assertLoggedIn(this.tokens);
    assertLoggedIn(this.customerInformation);
    await this.ensureFreshToken();
    const response = await this.api.updateEmailSendCode(this.tokens.access, {
      email: email,
      lang: lang,
    });
    return response;
  }

  async updateEmail(email: string, password: string, code: string, lang: BmsLang): Promise<BmsResponse> {
    assertLoggedIn(this.tokens);
    assertLoggedIn(this.customerInformation);
    await this.ensureFreshToken();
    const response = await this.api.updateEmail(this.tokens.access, {
      userId: this.customerInformation.id,
      email,
      password,
      code: code,
      lang,
    });

    const infoResponse = await this.api.getPersonalInformation(this.tokens.access);
    if (!infoResponse.isError && infoResponse.data && infoResponse.data.type === DataType.PersonalInformation) {
      this.customerInformation = infoResponse.data;
    }
    return response;
  }

  async updateAuthentication(password: string, newPassword: string): Promise<BmsResponse> {
    assertLoggedIn(this.tokens);
    assertLoggedIn(this.customerInformation);
    await this.ensureFreshToken();
    return await this.api.updateAuthentication(this.tokens.access, {
      userId: this.customerInformation.id,
      password: password,
      newPassword: newPassword,
    });
  }

  async listOrganizations(): Promise<BmsResponse> {
    assertLoggedIn(this.tokens);
    assertLoggedIn(this.customerInformation);
    await this.ensureFreshToken();
    return await this.api.listOrganizations(this.tokens.access, {
      userId: this.customerInformation.id,
      clientId: this.customerInformation.clientId,
    });
  }

  async getOrganizationStatus(organizationId: string): Promise<BmsResponse> {
    assertLoggedIn(this.tokens);
    assertLoggedIn(this.customerInformation);
    await this.ensureFreshToken();
    return await this.api.getOrganizationStatus(this.tokens.access, {
      userId: this.customerInformation.id,
      clientId: this.customerInformation.clientId,
      organizationId: organizationId,
    });
  }

  async getOrganizationStats(organizationId: string): Promise<BmsResponse> {
    assertLoggedIn(this.tokens);
    assertLoggedIn(this.customerInformation);
    await this.ensureFreshToken();
    return await this.api.getOrganizationStats(this.tokens.access, {
      userId: this.customerInformation.id,
      clientId: this.customerInformation.clientId,
      organizationId: organizationId,
    });
  }

  async getInvoices(): Promise<BmsResponse> {
    assertLoggedIn(this.tokens);
    assertLoggedIn(this.customerInformation);
    await this.ensureFreshToken();
    return await this.api.getInvoices(this.tokens.access, {
      userId: this.customerInformation.id,
      clientId: this.customerInformation.clientId,
    });
  }

  async getBillingDetails(): Promise<BmsResponse> {
    assertLoggedIn(this.tokens);
    assertLoggedIn(this.customerInformation);
    await this.ensureFreshToken();
    return await this.api.getBillingDetails(this.tokens.access, {
      userId: this.customerInformation.id,
      clientId: this.customerInformation.clientId,
    });
  }

  async addPaymentMethod(paymentMethod: string): Promise<BmsResponse> {
    assertLoggedIn(this.tokens);
    assertLoggedIn(this.customerInformation);
    await this.ensureFreshToken();
    return await this.api.addPaymentMethod(this.tokens.access, {
      userId: this.customerInformation.id,
      clientId: this.customerInformation.clientId,
      paymentMethod: paymentMethod,
    });
  }

  async setDefaultPaymentMethod(paymentMethod: string): Promise<BmsResponse> {
    assertLoggedIn(this.tokens);
    assertLoggedIn(this.customerInformation);
    await this.ensureFreshToken();
    return await this.api.setDefaultPaymentMethod(this.tokens.access, {
      userId: this.customerInformation.id,
      clientId: this.customerInformation.clientId,
      paymentMethod: paymentMethod,
    });
  }

  async deletePaymentMethod(paymentMethod: string): Promise<BmsResponse> {
    assertLoggedIn(this.tokens);
    assertLoggedIn(this.customerInformation);
    await this.ensureFreshToken();
    return await this.api.deletePaymentMethod(this.tokens.access, {
      userId: this.customerInformation.id,
      clientId: this.customerInformation.clientId,
      paymentMethod: paymentMethod,
    });
  }

  async updateBillingAddress(address: BmsAddress): Promise<BmsResponse> {
    assertLoggedIn(this.tokens);
    assertLoggedIn(this.customerInformation);
    await this.ensureFreshToken();
    return await this.api.updateBillingDetails(this.tokens.access, {
      userId: this.customerInformation.id,
      clientId: this.customerInformation.clientId,
      address: address,
    });
  }

  async getCustomOrderStatus(organization: BmsOrganization): Promise<BmsResponse> {
    assertLoggedIn(this.tokens);
    assertLoggedIn(this.customerInformation);
    await this.ensureFreshToken();
    return await this.api.getCustomOrderStatus(this.tokens.access, {
      userId: this.customerInformation.id,
      clientId: this.customerInformation.clientId,
      organization: organization,
    });
  }

  async getCustomOrderDetails(organization: BmsOrganization): Promise<BmsResponse> {
    assertLoggedIn(this.tokens);
    assertLoggedIn(this.customerInformation);
    await this.ensureFreshToken();
    return await this.api.getCustomOrderDetails(this.tokens.access, {
      userId: this.customerInformation.id,
      clientId: this.customerInformation.clientId,
      organization: organization,
    });
  }

  async unsubscribeOrganization(organizationId: string): Promise<BmsResponse> {
    assertLoggedIn(this.tokens);
    assertLoggedIn(this.customerInformation);
    await this.ensureFreshToken();
    return await this.api.unsubscribeOrganization(this.tokens.access, {
      userId: this.customerInformation.id,
      clientId: this.customerInformation.clientId,
      organizationId: organizationId,
    });
  }

  async subscribeOrganization(organizationId: string): Promise<BmsResponse> {
    assertLoggedIn(this.tokens);
    assertLoggedIn(this.customerInformation);
    await this.ensureFreshToken();
    return await this.api.subscribeOrganization(this.tokens.access, {
      userId: this.customerInformation.id,
      clientId: this.customerInformation.clientId,
      organizationId: organizationId,
    });
  }

  async createCustomOrderRequest(data: {
    needs: string;
    adminUsers?: number;
    standardUsers: number;
    outsiderUsers?: number;
    storage: number;
    formula?: string;
    organizationName?: string;
  }): Promise<BmsResponse> {
    assertLoggedIn(this.tokens);
    assertLoggedIn(this.customerInformation);
    await this.ensureFreshToken();

    const response = await this.api.createCustomOrderRequest(this.tokens.access, {
      describedNeeds: data.needs,
      adminUsers: data.adminUsers,
      standardUsers: data.standardUsers,
      outsiderUsers: data.outsiderUsers,
      storage: data.storage,
      formula: data.formula,
      organizationName: data.organizationName,
    });
    return response;
  }

  async getCustomOrderInvoices(organization: BmsOrganization): Promise<BmsResponse> {
    assertLoggedIn(this.tokens);
    assertLoggedIn(this.customerInformation);
    await this.ensureFreshToken();
    return await this.api.getCustomOrderInvoices(this.tokens.access, {
      userId: this.customerInformation.id,
      clientId: this.customerInformation.clientId,
      organization: organization,
    });
  }

  async clearStoredAccess(): Promise<void> {
    await storageManagerInstance.get().clearBmsAccess();
  }

  private async storeAccess(): Promise<void> {
    if (this.tokens) {
      await storageManagerInstance.get().storeBmsAccess({ access: this.tokens.access, refresh: this.tokens.refresh });
    }
  }

  private async restoreAccess(): Promise<void> {
    const bmsAccess = await storageManagerInstance.get().retrieveBmsAccess();

    if (bmsAccess) {
      this.tokens = {
        access: bmsAccess.access,
        refresh: bmsAccess.refresh,
      };
    }
  }

  isLoggedIn(): boolean {
    return this.tokens !== null && !this.tokenIsExpired(this.tokens.access) && this.customerInformation !== null;
  }

  private async ensureFreshToken(): Promise<boolean> {
    if (!this.tokens || this.tokenIsExpired(this.tokens.refresh)) {
      return false;
    }
    if (this.tokenIsExpired(this.tokens.access)) {
      const refreshResponse = await this.api.refreshToken(this.tokens.refresh);
      if (!refreshResponse.isError && refreshResponse.data && refreshResponse.data.type === DataType.RefreshToken) {
        this.tokens.access = refreshResponse.data.token;
      } else {
        return false;
      }
    }
    return true;
  }

  private tokenIsExpired(token: AuthenticationToken): boolean {
    if (token === undefined) {
      return true;
    }
    const tokenData = decodeToken(token);
    if (!tokenData) {
      return true;
    }
    return Date.now().valueOf() >= tokenData.expiresAt.toJSDate().valueOf();
  }
}

class AccessInstance {
  private instance: BmsAccess | null = null;

  constructor() {}

  get(): BmsAccess {
    if (!this.instance) {
      this.instance = new BmsAccess();
    }
    return this.instance;
  }
}

export const BmsAccessInstance = new AccessInstance();
