// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { BmsApi } from '@/services/bms/api';
import { AuthenticationToken, BmsError, BmsResponse, DataType, PersonalInformationResultData } from '@/services/bms/types';
import { storageManagerInstance } from '@/services/storageManager';

function assertLoggedIn<T>(value: T): asserts value is NonNullable<T> {
  if (value === null) {
    throw new Error('Not logged in');
  }
}

class BmsAccess {
  private token: AuthenticationToken | null = null;
  private customerInformation: PersonalInformationResultData | null = null;
  private storeCredentials: boolean = true;
  public reloadKey: number = 0;

  constructor() {}

  async tryAutoLogin(): Promise<boolean> {
    this.reloadKey += 1;
    if (this.storeCredentials) {
      await this.restoreAccess();
    }
    if (!this.token) {
      return false;
    }
    const infoResponse = await BmsApi.getPersonalInformation(this.token);
    if (!infoResponse.isError && infoResponse.data && infoResponse.data.type === DataType.PersonalInformation) {
      this.customerInformation = infoResponse.data;
      return true;
    }
    this.token = null;
    return false;
  }

  async login(email: string, password: string): Promise<{ ok: boolean; errors?: Array<BmsError> }> {
    this.reloadKey += 1;
    const response = await BmsApi.login({ email: email, password: password });
    if (!response.isError && response.data && response.data.type === DataType.Login) {
      this.token = response.data.token;
    } else {
      this.token = null;
      return { ok: false, errors: response.errors };
    }
    const infoResponse = await BmsApi.getPersonalInformation(this.token);
    if (!infoResponse.isError && infoResponse.data && infoResponse.data.type === DataType.PersonalInformation) {
      this.customerInformation = infoResponse.data;
    } else {
      this.token = null;
    }
    if (this.storeCredentials) {
      await this.storeAccess();
    }
    return { ok: true };
  }

  async logout(): Promise<void> {
    this.token = null;
    this.customerInformation = null;
    await this.clearStoredAccess();
  }

  async getPersonalInformation(): Promise<PersonalInformationResultData> {
    assertLoggedIn(this.customerInformation);
    return this.customerInformation;
  }

  async getToken(): Promise<AuthenticationToken> {
    assertLoggedIn(this.token);
    return this.token;
  }

  async listOrganizations(): Promise<BmsResponse> {
    assertLoggedIn(this.token);
    assertLoggedIn(this.customerInformation);
    return await BmsApi.listOrganizations(this.token, {
      userId: this.customerInformation.id,
      clientId: this.customerInformation.clientId,
    });
  }

  async getOrganizationStatus(organizationId: string): Promise<BmsResponse> {
    assertLoggedIn(this.token);
    assertLoggedIn(this.customerInformation);
    return await BmsApi.getOrganizationStatus(this.token, {
      userId: this.customerInformation.id,
      clientId: this.customerInformation.clientId,
      organizationId: organizationId,
    });
  }

  async getOrganizationStats(organizationId: string): Promise<BmsResponse> {
    assertLoggedIn(this.token);
    assertLoggedIn(this.customerInformation);
    return await BmsApi.getOrganizationStats(this.token, {
      userId: this.customerInformation.id,
      clientId: this.customerInformation.clientId,
      organizationId: organizationId,
    });
  }

  async getInvoices(): Promise<BmsResponse> {
    assertLoggedIn(this.token);
    assertLoggedIn(this.customerInformation);
    return await BmsApi.getInvoices(this.token, {
      userId: this.customerInformation.id,
      clientId: this.customerInformation.clientId,
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
    if (this.token) {
      await storageManagerInstance.get().storeBmsAccess({ token: this.token });
    }
  }

  async restoreAccess(): Promise<void> {
    const bmsAccess = await storageManagerInstance.get().retrieveBmsAccess();

    if (bmsAccess) {
      this.token = bmsAccess.token;
    }
  }

  isLoggedIn(): boolean {
    return this.token !== null;
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
