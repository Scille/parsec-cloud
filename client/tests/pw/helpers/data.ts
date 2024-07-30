// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

export const DEFAULT_USER_INFORMATION = {
  id: '4242',
  clientId: '1337',
  name: 'Gordon Freeman',
  firstName: 'Gordon',
  lastName: 'Freeman',
  email: 'gordon.freeman@blackmesa.nm',
  password: 'D3@th2N1h1l@nth',
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
