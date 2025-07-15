// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { getOrganizationAddr, getServerAddr } from '@tests/e2e/helpers/utils';
import { randomUUID } from 'crypto';

export interface UserInformation {
  id: string;
  clientId: string;
  name: string;
  firstName: string;
  lastName: string;
  email: string;
  password: string;
  job: string;
  company: string;
  phone?: string;
  address: {
    line1: string;
    postalCode: string;
    city: string;
    state: string;
    country: string;
    full: string;
  };
}

export const DEFAULT_USER_INFORMATION = {
  id: '4242',
  clientId: '1337',
  name: 'Gordon Freeman',
  firstName: 'Gordon',
  lastName: 'Freeman',
  email: 'gordon.freeman@blackmesa.nm',
  password: 'D3@th2N1h1l@nth',
  job: 'Researcher',
  company: 'Black Mesa',
  phone: undefined,
  address: {
    line1: 'Black Mesa Research Facility',
    postalCode: '88201',
    city: 'Roswell',
    state: 'New Mexico',
    country: 'United States of America',
    full: 'Black Mesa Research Facility, 88201 Roswell, New Mexico, United States of America',
  },
};

export interface OrganizationInformation {
  name: string;
  addr: string;
  serverAddr: string;
  bmsId: string;
}

export function generateDefaultOrganizationInformation(): OrganizationInformation {
  const name = 'BlackMesa';
  return {
    name: name,
    addr: getOrganizationAddr(name),
    serverAddr: getServerAddr(),
    bmsId: '42',
  };
}

export function generateUniqueEmail(): string {
  return `${randomUUID()}@host.com`;
}

export const DEFAULT_ORGANIZATION_DATA_SLICE = {
  free: 1024 * 1024 * 1024 * 100, // 100 Gb
  paying: 1024 * 1024 * 1024 * 100, // 100 Gb
};

export class UserData {
  firstName: string;
  lastName: string;
  email: string;
  phone?: string;
  job?: string;
  company?: string;
  address: {
    line1: string;
    line2?: string;
    postalCode: string;
    city: string;
    country: string;
  };

  constructor() {
    this.firstName = DEFAULT_USER_INFORMATION.firstName;
    this.lastName = DEFAULT_USER_INFORMATION.lastName;
    this.phone = DEFAULT_USER_INFORMATION.phone;
    this.email = DEFAULT_USER_INFORMATION.email;
    this.company = DEFAULT_USER_INFORMATION.company;
    this.job = DEFAULT_USER_INFORMATION.job;
    this.address = {
      line1: DEFAULT_USER_INFORMATION.address.line1,
      postalCode: DEFAULT_USER_INFORMATION.address.postalCode,
      city: DEFAULT_USER_INFORMATION.address.city,
      country: DEFAULT_USER_INFORMATION.address.country,
    };
  }

  reset(): void {
    this.firstName = DEFAULT_USER_INFORMATION.firstName;
    this.lastName = DEFAULT_USER_INFORMATION.lastName;
    this.phone = DEFAULT_USER_INFORMATION.phone;
    this.email = DEFAULT_USER_INFORMATION.email;
    this.company = DEFAULT_USER_INFORMATION.company;
    this.job = DEFAULT_USER_INFORMATION.job;
    this.address = {
      line1: DEFAULT_USER_INFORMATION.address.line1,
      postalCode: DEFAULT_USER_INFORMATION.address.postalCode,
      city: DEFAULT_USER_INFORMATION.address.city,
      country: DEFAULT_USER_INFORMATION.address.country,
    };
  }
}

export function generateDefaultUserData(): UserData {
  return new UserData();
}
