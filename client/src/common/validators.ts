// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

export enum Validity {
  Invalid = 0,
  Intermediate = 1,
  Valid = 2
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
  return value.match(/^[^\s@]+@[^\s@]+(\.[^\s@]+)?$/i) ? Validity.Valid : Validity.Invalid;
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
  } else if (value.length >= 128) {
    return Validity.Invalid;
  }
  return Validity.Valid;
};

export const backendAddrValidator: IValidator = function(value: string) {
  value = value.trim();
  if (value.length === 0) {
    return Validity.Intermediate;
  }
  return value.match(/^parsec:\/\/[a-z0-9-_#./?]+$/i) ? Validity.Valid : Validity.Invalid;
};

export const organizationValidator: IValidator = function(value: string) {
  value = value.trim();
  if (value.length === 0) {
    return Validity.Intermediate;
  }
  return value.match(/^[a-z0-9_-]{1,32}$/i) ? Validity.Valid : Validity.Invalid;
};
