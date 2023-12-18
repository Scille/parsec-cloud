// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import {
  isValidDeviceName,
  isValidEmail,
  isValidEntryName,
  isValidOrganizationName,
  isValidUserName,
  isValidWorkspaceName,
  parseBackendAddr,
} from '@/parsec';
import { ParsedBackendAddrTag } from '@/plugins/libparsec';

export enum Validity {
  Invalid = 0,
  Intermediate = 1,
  Valid = 2,
}

export interface IValidator {
  (value: string): Promise<Validity>;
}

// Validators in this file are meant to be later replaced by using
// calls to the bindings.

export const emailValidator: IValidator = async function (value: string) {
  value = value.trim();
  if (!value.includes('@') || value.length === 0) {
    return Validity.Intermediate;
  }
  return (await isValidEmail(value)) ? Validity.Valid : Validity.Invalid;
};

export const deviceNameValidator: IValidator = async function (value: string) {
  value = value.trim();
  if (value.length === 0) {
    return Validity.Intermediate;
  }
  return (await isValidDeviceName(value)) ? Validity.Valid : Validity.Invalid;
};

export const userNameValidator: IValidator = async function (value: string) {
  value = value.trim();
  if (value.length === 0) {
    return Validity.Intermediate;
  }
  return (await isValidUserName(value)) ? Validity.Valid : Validity.Invalid;
};

export const workspaceNameValidator: IValidator = async function (value: string) {
  value = value.trim();
  if (value.length === 0) {
    return Validity.Intermediate;
  }
  return (await isValidWorkspaceName(value)) ? Validity.Valid : Validity.Invalid;
};

export const entryNameValidator: IValidator = async function (value: string) {
  value = value.trim();
  if (value.length === 0) {
    return Validity.Intermediate;
  }
  return (await isValidEntryName(value)) ? Validity.Valid : Validity.Invalid;
};

export const backendAddrValidator: IValidator = async function (value: string) {
  value = value.trim();
  if (value.length === 0) {
    return Validity.Intermediate;
  }
  const result = await parseBackendAddr(value);
  if (result.ok) {
    return result.value.tag === ParsedBackendAddrTag.Server ? Validity.Valid : Validity.Invalid;
  }
  return Validity.Invalid;
};

export const backendOrganizationAddrValidator: IValidator = async function (value: string) {
  value = value.trim();
  if (value.length === 0) {
    return Validity.Intermediate;
  }
  const result = await parseBackendAddr(value);
  if (result.ok) {
    return result.value.tag === ParsedBackendAddrTag.Organization ? Validity.Valid : Validity.Invalid;
  }
  return Validity.Invalid;
};

export const organizationValidator: IValidator = async function (value: string) {
  value = value.trim();
  if (value.length === 0) {
    return Validity.Intermediate;
  }
  return (await isValidOrganizationName(value)) ? Validity.Valid : Validity.Invalid;
};

export const claimLinkValidator: IValidator = async function (value: string) {
  value = value.trim();
  if (value.length === 0) {
    return Validity.Intermediate;
  }
  const result = await parseBackendAddr(value);
  if (result.ok) {
    return result.value.tag === ParsedBackendAddrTag.InvitationUser || result.value.tag === ParsedBackendAddrTag.InvitationDevice
      ? Validity.Valid
      : Validity.Invalid;
  }
  return Validity.Invalid;
};

export const fileLinkValidator: IValidator = async function (value: string) {
  value = value.trim();
  if (value.length === 0) {
    return Validity.Intermediate;
  }
  const result = await parseBackendAddr(value);
  if (result.ok) {
    return result.value.tag === ParsedBackendAddrTag.OrganizationFileLink ? Validity.Valid : Validity.Invalid;
  }
  return Validity.Invalid;
};

export const claimUserLinkValidator: IValidator = async function (value: string) {
  value = value.trim();
  if (value.length === 0) {
    return Validity.Intermediate;
  }
  const result = await parseBackendAddr(value);
  if (result.ok) {
    return result.value.tag === ParsedBackendAddrTag.InvitationUser ? Validity.Valid : Validity.Invalid;
  }
  return Validity.Invalid;
};

export const claimDeviceLinkValidator: IValidator = async function (value: string) {
  value = value.trim();
  if (value.length === 0) {
    return Validity.Intermediate;
  }
  const result = await parseBackendAddr(value);
  if (result.ok) {
    return result.value.tag === ParsedBackendAddrTag.InvitationDevice ? Validity.Valid : Validity.Invalid;
  }
  return Validity.Invalid;
};

export const secretKeyValidator: IValidator = async function (value: string) {
  value = value.trim();
  if (value.length === 0) {
    return Validity.Intermediate;
  }
  return /^([A-Z0-9]{4}-){12}[A-Z0-9]{4}$/.test(value) ? Validity.Valid : Validity.Invalid;
};
