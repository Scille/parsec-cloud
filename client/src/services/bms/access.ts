// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { BmsApi } from '@/services/bms/api';
import { AuthenticationToken, BmsError, BmsResponse, DataType, PersonalInformationResultData } from '@/services/bms/types';
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
  private storeCredentials: boolean = true;
  public reloadKey: number = 0;

  constructor() {}

  async tryAutoLogin(): Promise<boolean> {
    this.reloadKey += 1;
    if (this.storeCredentials) {
      await this.restoreAccess();
    }
    if (!this.tokens) {
      return false;
    }

    await this.ensureFreshToken();
    const infoResponse = await BmsApi.getPersonalInformation(this.tokens.access);
    if (!infoResponse.isError && infoResponse.data && infoResponse.data.type === DataType.PersonalInformation) {
      this.customerInformation = infoResponse.data;
      return true;
    }

    this.tokens = null;
    return false;
  }

  async login(email: string, password: string): Promise<{ ok: boolean; errors?: Array<BmsError> }> {
    this.reloadKey += 1;
    const response = await BmsApi.login({ email: email, password: password });
    if (!response.isError && response.data && response.data.type === DataType.Login) {
      this.tokens = { access: response.data.accessToken, refresh: response.data.refreshToken };
    } else {
      return { ok: false, errors: response.errors };
    }
    const infoResponse = await BmsApi.getPersonalInformation(this.tokens.access);
    if (!infoResponse.isError && infoResponse.data && infoResponse.data.type === DataType.PersonalInformation) {
      this.customerInformation = infoResponse.data;
    } else {
      this.tokens = null;
    }
    if (this.storeCredentials) {
      await this.storeAccess();
    }
    return { ok: true };
  }

  async logout(): Promise<void> {
    this.tokens = null;
    this.customerInformation = null;
    await this.clearStoredAccess();
  }

  async getPersonalInformation(): Promise<PersonalInformationResultData> {
    assertLoggedIn(this.customerInformation);
    return this.customerInformation;
  }

  async getToken(): Promise<AuthenticationToken> {
    assertLoggedIn(this.tokens);
    return this.tokens.access;
  }

  async listOrganizations(): Promise<BmsResponse> {
    assertLoggedIn(this.tokens);
    assertLoggedIn(this.customerInformation);
    await this.ensureFreshToken();
    return await BmsApi.listOrganizations(this.tokens.access, {
      userId: this.customerInformation.id,
      clientId: this.customerInformation.clientId,
    });
  }

  async getOrganizationStatus(organizationId: string): Promise<BmsResponse> {
    assertLoggedIn(this.tokens);
    assertLoggedIn(this.customerInformation);
    await this.ensureFreshToken();
    return await BmsApi.getOrganizationStatus(this.tokens.access, {
      userId: this.customerInformation.id,
      clientId: this.customerInformation.clientId,
      organizationId: organizationId,
    });
  }

  async getOrganizationStats(organizationId: string): Promise<BmsResponse> {
    assertLoggedIn(this.tokens);
    assertLoggedIn(this.customerInformation);
    await this.ensureFreshToken();
    return await BmsApi.getOrganizationStats(this.tokens.access, {
      userId: this.customerInformation.id,
      clientId: this.customerInformation.clientId,
      organizationId: organizationId,
    });
  }

  async getInvoices(): Promise<BmsResponse> {
    assertLoggedIn(this.tokens);
    assertLoggedIn(this.customerInformation);
    await this.ensureFreshToken();
    return await BmsApi.getInvoices(this.tokens.access, {
      userId: this.customerInformation.id,
      clientId: this.customerInformation.clientId,
    });
  }

  async getBillingDetails(): Promise<BmsResponse> {
    assertLoggedIn(this.tokens);
    assertLoggedIn(this.customerInformation);
    await this.ensureFreshToken();
    return await BmsApi.getBillingDetails(this.tokens.access, {
      userId: this.customerInformation.id,
      clientId: this.customerInformation.clientId,
    });
  }

  async addPaymentMethod(paymentMethod: string): Promise<BmsResponse> {
    assertLoggedIn(this.tokens);
    assertLoggedIn(this.customerInformation);
    await this.ensureFreshToken();
    return await BmsApi.addPaymentMethod(this.tokens.access, {
      userId: this.customerInformation.id,
      clientId: this.customerInformation.clientId,
      paymentMethod: paymentMethod,
    });
  }

  async setDefaultPaymentMethod(paymentMethod: string): Promise<BmsResponse> {
    assertLoggedIn(this.tokens);
    assertLoggedIn(this.customerInformation);
    await this.ensureFreshToken();
    return await BmsApi.setDefaultPaymentMethod(this.tokens.access, {
      userId: this.customerInformation.id,
      clientId: this.customerInformation.clientId,
      paymentMethod: paymentMethod,
    });
  }

  async deletePaymentMethod(paymentMethod: string): Promise<BmsResponse> {
    assertLoggedIn(this.tokens);
    assertLoggedIn(this.customerInformation);
    await this.ensureFreshToken();
    return await BmsApi.deletePaymentMethod(this.tokens.access, {
      userId: this.customerInformation.id,
      clientId: this.customerInformation.clientId,
      paymentMethod: paymentMethod,
    });
  }

  async updateAuthentication(password: string, newPassword: string): Promise<BmsResponse> {
    assertLoggedIn(this.tokens);
    assertLoggedIn(this.customerInformation);
    await this.ensureFreshToken();
    return await BmsApi.updateAuthentication(this.tokens.access, {
      userId: this.customerInformation.id,
      password: password,
      newPassword: newPassword,
    });
  }

  async rememberCredentials(): Promise<void> {
    this.storeCredentials = true;
    await this.storeAccess();
  }

  async forgetCredentials(): Promise<void> {
    this.storeCredentials = false;
    await this.clearStoredAccess();
  }

  async clearStoredAccess(): Promise<void> {
    await storageManagerInstance.get().clearBmsAccess();
  }

  async storeAccess(): Promise<void> {
    if (this.tokens) {
      await storageManagerInstance.get().storeBmsAccess({ access: this.tokens.access, refresh: this.tokens.refresh });
    }
  }

  async restoreAccess(): Promise<void> {
    const bmsAccess = await storageManagerInstance.get().retrieveBmsAccess();

    if (bmsAccess) {
      this.tokens = {
        access: bmsAccess.access,
        refresh: bmsAccess.refresh,
      };
    }
  }

  isLoggedIn(): boolean {
    return this.tokens !== null && !this.tokenIsExpired(this.tokens.access);
  }

  private async ensureFreshToken(): Promise<boolean> {
    if (!this.tokens || this.tokenIsExpired(this.tokens.refresh)) {
      return false;
    }
    if (this.tokenIsExpired(this.tokens.access)) {
      const refreshResponse = await BmsApi.refreshToken(this.tokens.refresh);
      if (!refreshResponse.isError && refreshResponse.data && refreshResponse.data.type === DataType.RefreshToken) {
        this.tokens.access = refreshResponse.data.token;
      } else {
        return false;
      }
    }
    return true;
  }

  private tokenIsExpired(token: AuthenticationToken): boolean {
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
