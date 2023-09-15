// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import * as Parsec from '@/common/parsec';

export enum Validity {
  Invalid = 0,
  Intermediate = 1,
  Valid = 2,
}

interface IValidator {
  (value: string): Promise<Validity>
}

// Validators in this file are meant to be later replaced by using
// calls to the bindings.

export const emailValidator: IValidator = async function(value: string) {
  value = value.trim();
  if (!value.includes('@') || value.length === 0) {
    return Validity.Intermediate;
  }
  return value.match(/^[^\s]+@[^\s]+(\.[^\s]+)?$/i) ? Validity.Valid : Validity.Invalid;
};

export const deviceNameValidator: IValidator = async function(value: string) {
  value = value.trim();
  if (value.length === 0) {
    return Validity.Intermediate;
  }
  return value.match(/^[a-z0-9_-]{1,32}$/i) ? Validity.Valid : Validity.Invalid;
};

export const userNameValidator: IValidator = async function(value: string) {
  value = value.trim();
  if (value.length === 0) {
    return Validity.Intermediate;
  } else if (value.length > 128) {
    return Validity.Invalid;
  }
  return Validity.Valid;
};

export const backendAddrValidator: IValidator = async function(value: string) {
  value = value.trim();
  if (value.length === 0) {
    return Validity.Intermediate;
  }
  const result = await Parsec.parseBackendAddr(value);
  if (result.ok) {
    return (result.value.tag === 'Base' ? Validity.Valid : Validity.Invalid);
  }
  return Validity.Invalid;
};

export const organizationValidator: IValidator = async function(value: string) {
  value = value.trim();
  if (value.length === 0) {
    return Validity.Intermediate;
  }
  return value.match(/^[a-z0-9_-]{1,32}$/i) ? Validity.Valid : Validity.Invalid;
};

export const claimLinkValidator: IValidator = async function(value: string) {
  value = value.trim();
  if (value.length === 0) {
    return Validity.Intermediate;
  }
  const result = await Parsec.parseBackendAddr(value);
  if (result.ok) {
    return (result.value.tag === 'InvitationUser' || result.value.tag === 'InvitationDevice' ? Validity.Valid : Validity.Invalid);
  }
  return Validity.Invalid;
};

export const claimUserLinkValidator: IValidator = async function(value: string) {
  value = value.trim();
  if (value.length === 0) {
    return Validity.Intermediate;
  }
  const result = await Parsec.parseBackendAddr(value);
  if (result.ok) {
    return (result.value.tag === 'InvitationUser' ? Validity.Valid : Validity.Invalid);
  }
  return Validity.Invalid;
};

export const claimDeviceLinkValidator: IValidator = async function(value: string) {
  value = value.trim();
  if (value.length === 0) {
    return Validity.Intermediate;
  }
  const result = await Parsec.parseBackendAddr(value);
  if (result.ok) {
    return (result.value.tag === 'InvitationDevice' ? Validity.Valid : Validity.Invalid);
  }
  return Validity.Invalid;
};
