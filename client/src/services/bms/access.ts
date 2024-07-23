// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { BmsApi } from '@/services/bms/api';
import { AuthenticationToken, BmsResponse, DataType, PersonalInformationResultData } from '@/services/bms/types';

function getDevBmsCredentials(): string | undefined {
  return import.meta.env.VITE_DEV_BMS_CREDENTIALS;
}

function assertLoggedIn<T>(value: T): asserts value is NonNullable<T> {
  if (value === null) {
    throw new Error('Not logged in');
  }
}

interface Connection {
  token: AuthenticationToken;
  customerInformation: PersonalInformationResultData;
}

class BmsAccess {
  private connection: Connection | null = null;

  constructor() {}

  async login(email: string, password: string): Promise<BmsResponse> {
    if (this.connection) {
      this.logout();
    }
    const response = await BmsApi.login({ email: email, password: password });
    if (!response.isError && response.data && response.data.type === DataType.Login) {
      const infoResponse = await BmsApi.getPersonalInformation(response.data.token);
      if (!infoResponse.isError && infoResponse.data && infoResponse.data.type === DataType.PersonalInformation) {
        this.connection = {
          token: response.data.token,
          customerInformation: infoResponse.data,
        };
      }
    }
    return response;
  }

  async logout(): Promise<void> {
    this.connection = null;
  }

  async getPersonalInformation(): Promise<PersonalInformationResultData> {
    if (this._useDevLogin()) {
      await this._devLogin();
    }
    assertLoggedIn(this.connection);
    return this.connection.customerInformation;
  }

  async listOrganizations(): Promise<BmsResponse> {
    if (this._useDevLogin()) {
      await this._devLogin();
    }
    assertLoggedIn(this.connection);
    return await BmsApi.listOrganizations(this.connection.token, {
      userId: this.connection.customerInformation.id,
      clientId: this.connection.customerInformation.clientId,
    });
  }

  async getOrganizationStatus(organizationId: string): Promise<BmsResponse> {
    if (this._useDevLogin()) {
      await this._devLogin();
    }
    assertLoggedIn(this.connection);
    return await BmsApi.getOrganizationStatus(this.connection.token, {
      userId: this.connection.customerInformation.id,
      clientId: this.connection.customerInformation.clientId,
      organizationId: organizationId,
    });
  }

  async getOrganizationStats(organizationId: string): Promise<BmsResponse> {
    if (this._useDevLogin()) {
      await this._devLogin();
    }
    assertLoggedIn(this.connection);
    return await BmsApi.getOrganizationStats(this.connection.token, {
      userId: this.connection.customerInformation.id,
      clientId: this.connection.customerInformation.clientId,
      organizationId: organizationId,
    });
  }

  async getInvoices(): Promise<BmsResponse> {
    if (this._useDevLogin()) {
      await this._devLogin();
    }
    assertLoggedIn(this.connection);
    return await BmsApi.getInvoices(this.connection.token, {
      userId: this.connection.customerInformation.id,
      clientId: this.connection.customerInformation.clientId,
    });
  }

  isLoggedIn(): boolean {
    if (this._useDevLogin()) {
      this._devLogin();
      return true;
    }
    return this.connection !== null;
  }

  private async _devLogin(): Promise<void> {
    if (this.connection) {
      return;
    }
    console.debug('Using dev credentials for BMS requests');
    const parts = getDevBmsCredentials()?.split(':') as Array<string>;
    await this.login(parts[0], parts[1]);
  }

  private _useDevLogin(): boolean {
    return getDevBmsCredentials() !== undefined;
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
