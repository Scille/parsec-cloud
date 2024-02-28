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
import { ParsedParsecAddrTag } from '@/plugins/libparsec';
import { translate } from '@/services/translation';

export enum Validity {
  Invalid = 0,
  Intermediate = 1,
  Valid = 2,
}

export interface ValidationResult {
  validity: Validity;
  reason?: string;
}

export interface IValidator {
  (value: string): Promise<ValidationResult>;
}

export const emailValidator: IValidator = async function (value: string) {
  value = value.trim();
  if (!value.includes('@') || value.length === 0) {
    return { validity: Validity.Intermediate };
  }
  return (await isValidEmail(value)) ? { validity: Validity.Valid } : { validity: Validity.Invalid };
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
  return (await isValidUserName(value)) ? { validity: Validity.Valid } : { validity: Validity.Invalid };
};

export const workspaceNameValidator: IValidator = async function (value: string) {
  value = value.trim();
  if (value.length === 0) {
    return { validity: Validity.Intermediate };
  }
  return (await isValidWorkspaceName(value)) ? { validity: Validity.Valid } : { validity: Validity.Invalid };
};

export const entryNameValidator: IValidator = async function (value: string) {
  value = value.trim();
  if (value.length === 0) {
    return { validity: Validity.Intermediate };
  }
  return (await isValidEntryName(value)) ? { validity: Validity.Valid } : { validity: Validity.Invalid };
};

export const backendAddrValidator: IValidator = async function (value: string) {
  value = value.trim();
  if (value.length === 0) {
    return { validity: Validity.Intermediate };
  }
  const result = await parseBackendAddr(value);
  if (result.ok) {
    return result.value.tag === ParsedParsecAddrTag.Server ? { validity: Validity.Valid } : { validity: Validity.Invalid };
  }
  return { validity: Validity.Invalid };
};

export const backendOrganizationAddrValidator: IValidator = async function (value: string) {
  value = value.trim();
  if (value.length === 0) {
    return { validity: Validity.Intermediate };
  }
  const result = await parseBackendAddr(value);
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
  return { validity: Validity.Invalid, reason: reason ? translate(reason) : '' };
};

export const claimLinkValidator: IValidator = async function (value: string) {
  value = value.trim();
  if (value.length === 0) {
    return { validity: Validity.Intermediate };
  }
  const result = await parseBackendAddr(value);
  if (result.ok) {
    if (result.value.tag === ParsedParsecAddrTag.InvitationUser || result.value.tag === ParsedParsecAddrTag.InvitationDevice) {
      return { validity: Validity.Valid };
    } else {
      return { validity: Validity.Invalid, reason: translate('validators.claimLink.invalidAction') };
    }
  }
  let reason = '';
  if (!value.startsWith('parsec://')) {
    reason = 'validators.claimLink.invalidProtocol';
  } else if (!value.includes('action=')) {
    reason = 'validators.claimLink.missingAction';
  } else if (value.includes('action=') && !value.includes('action=claim_user') && !value.includes('action=claim_device')) {
    reason = 'validators.claimLink.invalidAction';
  } else if (!value.includes('token=')) {
    reason = 'validators.claimLink.missingToken';
  } else if (!/^.+token=[a-f90-9]{32}&?$/.test(value)) {
    reason = 'validators.claimLink.invalidToken';
  }
  return { validity: Validity.Invalid, reason: reason ? translate(reason) : '' };
};

export const fileLinkValidator: IValidator = async function (value: string) {
  value = value.trim();
  if (value.length === 0) {
    return { validity: Validity.Intermediate };
  }
  const result = await parseBackendAddr(value);
  if (result.ok) {
    return result.value.tag === ParsedParsecAddrTag.OrganizationFileLink ? { validity: Validity.Valid } : { validity: Validity.Invalid };
  }
  return { validity: Validity.Invalid };
};

export const claimUserLinkValidator: IValidator = async function (value: string) {
  value = value.trim();
  if (value.length === 0) {
    return { validity: Validity.Intermediate };
  }
  const result = await parseBackendAddr(value);
  if (result.ok) {
    return result.value.tag === ParsedParsecAddrTag.InvitationUser ? { validity: Validity.Valid } : { validity: Validity.Invalid };
  }
  let reason = '';
  if (!value.startsWith('parsec://')) {
    reason = 'validators.claimUserLink.invalidProtocol';
  } else if (!value.includes('action=')) {
    reason = 'validators.claimUserLink.missingAction';
  } else if (value.includes('action=') && !value.includes('action=claim_user')) {
    reason = 'validators.claimUserLink.invalidAction';
  } else if (!value.includes('token=')) {
    reason = 'validators.claimUserLink.missingToken';
  } else if (!/^.+token=[a-f90-9]{32}&?$/.test(value)) {
    reason = 'validators.claimUserLink.invalidToken';
  }
  return { validity: Validity.Invalid, reason: reason ? translate(reason) : '' };
};

export const claimDeviceLinkValidator: IValidator = async function (value: string) {
  value = value.trim();
  if (value.length === 0) {
    return { validity: Validity.Intermediate };
  }
  const result = await parseBackendAddr(value);
  if (result.ok) {
    return result.value.tag === ParsedParsecAddrTag.InvitationDevice ? { validity: Validity.Valid } : { validity: Validity.Invalid };
  }
  let reason = '';
  if (!value.startsWith('parsec://')) {
    reason = 'validators.claimDeviceLink.invalidProtocol';
  } else if (!value.includes('action=')) {
    reason = 'validators.claimDeviceLink.missingAction';
  } else if (value.includes('action=') && !value.includes('action=claim_device')) {
    reason = 'validators.claimDeviceLink.invalidAction';
  } else if (!value.includes('token=')) {
    reason = 'validators.claimDeviceLink.missingToken';
  } else if (!/^.+token=[a-f90-9]{32}&?$/.test(value)) {
    reason = 'validators.claimDeviceLink.invalidToken';
  }
  return { validity: Validity.Invalid, reason: reason ? translate(reason) : '' };
};

export const secretKeyValidator: IValidator = async function (value: string) {
  value = value.trim();
  if (value.length === 0) {
    return { validity: Validity.Intermediate };
  }
  return { validity: /^([A-Z0-9]{4}-){12}[A-Z0-9]{4}$/.test(value) ? Validity.Valid : Validity.Invalid };
};
