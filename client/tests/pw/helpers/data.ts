// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

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

export const DEFAULT_ORGANIZATION_INFORMATION = {
  name: 'BlackMesa',
  // cspell:disable-next-line
  addr: 'parsec3://blackmesa.com/BlackMesa',
  // cspell:disable-next-line
  serverAddr: 'parsec3://blackmesa.com',
  bmsId: '42',
};

export const DEFAULT_ORGANIZATION_DATA_SLICE = {
  free: 1024 * 1024 * 1024 * 200, // 200 Gb
  paying: 1024 * 1024 * 1024 * 100, // 100 Gb
};

class _UserData {
  firstName: string;
  lastName: string;
  email: string;
  phone: string | undefined;
  job: string | undefined;
  company: string | undefined;

  constructor() {
    this.firstName = DEFAULT_USER_INFORMATION.firstName;
    this.lastName = DEFAULT_USER_INFORMATION.lastName;
    this.phone = DEFAULT_USER_INFORMATION.phone;
    this.email = DEFAULT_USER_INFORMATION.email;
    this.company = DEFAULT_USER_INFORMATION.company;
    this.job = DEFAULT_USER_INFORMATION.job;
  }

  reset(): void {
    this.firstName = DEFAULT_USER_INFORMATION.firstName;
    this.lastName = DEFAULT_USER_INFORMATION.lastName;
    this.phone = DEFAULT_USER_INFORMATION.phone;
    this.email = DEFAULT_USER_INFORMATION.email;
    this.company = DEFAULT_USER_INFORMATION.company;
    this.job = DEFAULT_USER_INFORMATION.job;
  }
}

export const UserData = new _UserData();
