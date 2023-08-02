// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

export enum Validity {
  Invalid = 0,
  Intermediate = 1,
  Valid = 2,
}

interface IValidator {
  (value: string): Validity
}

// Validators in this file are meant to be later replaced by using
// calls to the bindings.

export const emailValidator: IValidator = function(value: string) {
  value = value.trim();
  if (!value.includes('@') || value.length === 0) {
    return Validity.Intermediate;
  }
  return value.match(/^[^\s]+@[^\s]+(\.[^\s]+)?$/i) ? Validity.Valid : Validity.Invalid;
};

export const deviceNameValidator: IValidator = function(value: string) {
  value = value.trim();
  if (value.length === 0) {
    return Validity.Intermediate;
  }
  return value.match(/^[a-z0-9_-]{1,32}$/i) ? Validity.Valid : Validity.Invalid;
};

export const userNameValidator: IValidator = function(value: string) {
  value = value.trim();
  if (value.length === 0) {
    return Validity.Intermediate;
  } else if (value.length > 128) {
    return Validity.Invalid;
  }
  return Validity.Valid;
};

export const backendAddrValidator: IValidator = function(value: string) {
  value = value.trim();
  if (value.length === 0) {
    return Validity.Intermediate;
  }
  try {
    const url = new URL(value);

    if (url.protocol !== 'parsec:') {
      return Validity.Invalid;
    }
    if (url.pathname !== '') {
      return Validity.Invalid;
    }
  } catch (e) {
    return Validity.Invalid;
  }
  return Validity.Valid;
};

export const organizationValidator: IValidator = function(value: string) {
  value = value.trim();
  if (value.length === 0) {
    return Validity.Intermediate;
  }
  return value.match(/^[a-z0-9_-]{1,32}$/i) ? Validity.Valid : Validity.Invalid;
};

export const claimLinkValidator: IValidator = function(value: string) {
  value = value.trim();
  if (value.length === 0) {
    return Validity.Intermediate;
  }
  try {
    if (!value.startsWith('parsec://')) {
      return Validity.Invalid;
    }
    // URL does not parse other protocols correctly
    value = value.replace('parsec://', 'http://');
    const url = new URL(value);
    if (!url.pathname || organizationValidator(url.pathname.slice(1)) !== Validity.Valid) {
      return Validity.Invalid;
    }
    if (!url.searchParams.get('token') || !url.searchParams.get('token')?.match(/^[a-f\d]{32}$/)) {
      return Validity.Invalid;
    }
    if (
      !url.searchParams.get('action')
      || (url.searchParams.get('action') !== 'claim_user'
      && url.searchParams.get('action') !== 'claim_device')
    ) {
      return Validity.Invalid;
    }
  } catch (e) {
    return Validity.Invalid;
  }

  return Validity.Valid;
};

export const claimUserLinkValidator: IValidator = function(value: string) {
  const validity = claimLinkValidator(value);

  if (validity === Validity.Intermediate) {
    return Validity.Intermediate;
  } else if (validity === Validity.Valid && value.includes('claim_user')) {
    return Validity.Valid;
  }
  return Validity.Invalid;
};

export const claimDeviceLinkValidator: IValidator = function(value: string) {
  const validity = claimLinkValidator(value);

  if (validity === Validity.Intermediate) {
    return Validity.Intermediate;
  } else if (validity === Validity.Valid && value.includes('claim_device')) {
    return Validity.Valid;
  }
  return Validity.Invalid;
};
