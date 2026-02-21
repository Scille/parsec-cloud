// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import {
  isValidDeviceName,
  isValidEmail,
  isValidEntryName,
  isValidOrganizationName,
  isValidUserName,
  isValidWorkspaceName,
  parseParsecAddr,
} from '@/parsec';
import { ParsedParsecAddrTag } from '@/plugins/libparsec';
import { IValidator, Validity } from 'megashark-lib';

const ENTRY_NAME_LIMIT = 128;

export const emailValidator: IValidator = async function (value: string) {
  value = value.trim();
  if (value.length === 0) {
    return { validity: Validity.Intermediate };
  }
  return (await isValidEmail(value)) ? { validity: Validity.Valid } : { validity: Validity.Invalid, reason: 'validators.userInfo.email' };
};

export const deviceNameValidator: IValidator = async function (value: string) {
  value = value.trim();
  if (value.length === 0) {
    return { validity: Validity.Intermediate };
  }
  return (await isValidDeviceName(value)) ? { validity: Validity.Valid } : { validity: Validity.Invalid };
};

export const userNameValidator: IValidator = async function (value: string) {
  value = value.trim();
  if (value.length === 0) {
    return { validity: Validity.Intermediate };
  }
  return (await isValidUserName(value)) ? { validity: Validity.Valid } : { validity: Validity.Invalid, reason: 'validators.userInfo.name' };
};

export const workspaceNameValidator: IValidator = async function (value: string) {
  value = value.trim();
  if (value.length === 0) {
    return { validity: Validity.Intermediate };
  } else if (value.length > ENTRY_NAME_LIMIT) {
    return { validity: Validity.Invalid, reason: { key: 'validators.workspaceName.tooLong', data: { limit: ENTRY_NAME_LIMIT } } };
  }

  return (await isValidWorkspaceName(value)) ? { validity: Validity.Valid } : { validity: Validity.Invalid };
};

export const entryNameValidator: IValidator = async function (value: string) {
  value = value.trim();
  if (value.length === 0) {
    return { validity: Validity.Intermediate };
  } else if (value.length > ENTRY_NAME_LIMIT) {
    return { validity: Validity.Invalid, reason: { key: 'validators.fileName.tooLong', data: { limit: ENTRY_NAME_LIMIT } } };
  }
  return (await isValidEntryName(value)) ? { validity: Validity.Valid } : { validity: Validity.Invalid };
};

export const parsecAddrValidator: IValidator = async function (value: string) {
  value = value.trim();
  if (value.length === 0) {
    return { validity: Validity.Intermediate };
  }
  const result = await parseParsecAddr(value);
  if (result.ok && result.value.tag === ParsedParsecAddrTag.Server) {
    return { validity: Validity.Valid };
  }
  let reason = '';
  if (!value.startsWith('parsec3://')) {
    reason = 'validators.invalidProtocol';
  }

  return { validity: Validity.Invalid, reason: reason };
};

export const parsecOrganizationAddrValidator: IValidator = async function (value: string) {
  value = value.trim();
  if (value.length === 0) {
    return { validity: Validity.Intermediate };
  }
  const result = await parseParsecAddr(value);
  if (result.ok) {
    return result.value.tag === ParsedParsecAddrTag.Organization ? { validity: Validity.Valid } : { validity: Validity.Invalid };
  }
  return { validity: Validity.Invalid };
};

export const organizationValidator: IValidator = async function (value: string) {
  value = value.trim();
  if (value.length === 0) {
    return { validity: Validity.Intermediate };
  }
  if (await isValidOrganizationName(value)) {
    return { validity: Validity.Valid };
  }
  let reason = '';
  if (value.length > 32) {
    reason = 'validators.organizationName.tooLong';
  } else if (new RegExp(/^[\w_-]+$/u).test(value) === false) {
    reason = 'validators.organizationName.forbiddenCharacters';
  }
  return { validity: Validity.Invalid, reason: reason };
};

export const claimLinkValidator: IValidator = async function (value: string) {
  value = value.trim();
  if (value.length === 0) {
    return { validity: Validity.Intermediate };
  }
  const result = await parseParsecAddr(value);
  if (result.ok) {
    if (result.value.tag === ParsedParsecAddrTag.InvitationUser || result.value.tag === ParsedParsecAddrTag.InvitationDevice) {
      return { validity: Validity.Valid };
    } else {
      return { validity: Validity.Invalid, reason: 'validators.claimLink.invalidAction' };
    }
  }
  let reason = '';
  if (!value.startsWith('parsec3://')) {
    reason = 'validators.claimLink.invalidProtocol';
  } else if (!value.includes('a=')) {
    reason = 'validators.claimLink.missingAction';
  } else if (value.includes('a=') && !value.includes('a=claim_user') && !value.includes('a=claim_device')) {
    reason = 'validators.claimLink.invalidAction';
  } else if (!value.includes('p=')) {
    reason = 'validators.claimLink.missingToken';
  } else {
    reason = 'validators.claimLink.invalidToken';
  }
  return { validity: Validity.Invalid, reason: reason ? reason : '' };
};

export const claimAndBootstrapLinkValidator: IValidator = async function (value: string) {
  value = value.trim();
  if (value.length === 0) {
    return { validity: Validity.Intermediate };
  }
  const result = await parseParsecAddr(value);

  if (
    result.ok &&
    (result.value.tag === ParsedParsecAddrTag.OrganizationBootstrap ||
      result.value.tag === ParsedParsecAddrTag.InvitationUser ||
      result.value.tag === ParsedParsecAddrTag.InvitationDevice ||
      result.value.tag === ParsedParsecAddrTag.AsyncEnrollment ||
      // Not a join link but it we can use it as a backup
      result.value.tag === ParsedParsecAddrTag.TOTPReset)
  ) {
    return { validity: Validity.Valid };
  }
  return { validity: Validity.Invalid, reason: 'validators.link.invalid' };
};

export const fileLinkValidator: IValidator = async function (value: string) {
  value = value.trim();
  if (value.length === 0) {
    return { validity: Validity.Intermediate };
  }
  const result = await parseParsecAddr(value);
  if (result.ok) {
    return result.value.tag === ParsedParsecAddrTag.WorkspacePath ? { validity: Validity.Valid } : { validity: Validity.Invalid };
  }
  return { validity: Validity.Invalid };
};

export const claimUserLinkValidator: IValidator = async function (value: string) {
  value = value.trim();
  if (value.length === 0) {
    return { validity: Validity.Intermediate };
  }
  const result = await parseParsecAddr(value);
  if (result.ok) {
    return result.value.tag === ParsedParsecAddrTag.InvitationUser ? { validity: Validity.Valid } : { validity: Validity.Invalid };
  }
  let reason = '';
  if (!value.startsWith('parsec3://')) {
    reason = 'validators.claimUserLink.invalidProtocol';
  } else if (!value.includes('a=')) {
    reason = 'validators.claimUserLink.missingAction';
  } else if (value.includes('a=') && !value.includes('a=claim_user')) {
    reason = 'validators.claimUserLink.invalidAction';
  } else if (!value.includes('p=')) {
    reason = 'validators.claimUserLink.missingToken';
  } else {
    reason = 'validators.claimUserLink.invalidToken';
  }
  return { validity: Validity.Invalid, reason: reason ? reason : '' };
};

export const claimDeviceLinkValidator: IValidator = async function (value: string) {
  value = value.trim();
  if (value.length === 0) {
    return { validity: Validity.Intermediate };
  }
  const result = await parseParsecAddr(value);
  if (result.ok) {
    return result.value.tag === ParsedParsecAddrTag.InvitationDevice ? { validity: Validity.Valid } : { validity: Validity.Invalid };
  }
  let reason = '';
  if (!value.startsWith('parsec3://')) {
    reason = 'validators.claimDeviceLink.invalidProtocol';
  } else if (!value.includes('a=')) {
    reason = 'validators.claimDeviceLink.missingAction';
  } else if (value.includes('a=') && !value.includes('a=claim_device')) {
    reason = 'validators.claimDeviceLink.invalidAction';
  } else if (!value.includes('p=')) {
    reason = 'validators.claimDeviceLink.missingToken';
  } else {
    reason = 'validators.claimDeviceLink.invalidToken';
  }
  return { validity: Validity.Invalid, reason: reason ? reason : '' };
};

export const bootstrapLinkValidator: IValidator = async function (value: string) {
  value = value.trim();
  if (value.length === 0) {
    return { validity: Validity.Intermediate };
  }
  const result = await parseParsecAddr(value);
  if (result.ok) {
    return result.value.tag === ParsedParsecAddrTag.OrganizationBootstrap ? { validity: Validity.Valid } : { validity: Validity.Invalid };
  }
  let reason = '';
  if (!value.startsWith('parsec3://')) {
    reason = 'validators.bootstrapLink.invalidProtocol';
  } else if (!value.includes('a=')) {
    reason = 'validators.bootstrapLink.missingAction';
  } else if (value.includes('a=') && !value.includes('a=bootstrap_organization')) {
    reason = 'validators.bootstrapLink.invalidAction';
  } else if (!value.includes('p=')) {
    reason = 'validators.bootstrapLink.missingToken';
  } else {
    reason = 'validators.bootstrapLink.invalidToken';
  }
  return { validity: Validity.Invalid, reason: reason ? reason : '' };
};

export const secretKeyValidator: IValidator = async function (value: string) {
  value = value.trim();
  if (value.length === 0) {
    return { validity: Validity.Intermediate };
  }
  return { validity: /^([A-Z0-9]{4}-){12}[A-Z0-9]{4}$/.test(value) ? Validity.Valid : Validity.Invalid };
};

export const asyncEnrollmentLinkValidator: IValidator = async function (value: string) {
  value = value.trim();
  if (value.length === 0) {
    return { validity: Validity.Intermediate };
  }
  const result = await parseParsecAddr(value);
  if (result.ok) {
    return result.value.tag === ParsedParsecAddrTag.AsyncEnrollment ? { validity: Validity.Valid } : { validity: Validity.Invalid };
  }
  return { validity: Validity.Invalid, reason: '' };
};

export const totpResetLinkValidator: IValidator = async function (value: string) {
  value = value.trim();
  if (value.length === 0) {
    return { validity: Validity.Intermediate };
  }
  const result = await parseParsecAddr(value);
  if (result.ok) {
    return result.value.tag === ParsedParsecAddrTag.TOTPReset ? { validity: Validity.Valid } : { validity: Validity.Invalid };
  }
  return { validity: Validity.Invalid, reason: '' };
};
