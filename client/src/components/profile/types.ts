// Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS

export enum AuthenticationCardState {
  Default = 'default',
  Selected = 'selected',
  Current = 'current',
  Active = 'active',
  Update = 'update',
  Disabled = 'disabled',
  Unavailable = 'unavailable',
  Forbidden = 'forbidden',
}

export enum ShamirTab {
  Self = 'shamir-tab-self',
  Others = 'shamir-tab-others',
}
