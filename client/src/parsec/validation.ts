// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { libparsec } from '@/plugins/libparsec';

export async function isValidWorkspaceName(name: string): Promise<boolean> {
  return await libparsec.validateEntryName(name);
}

export async function isValidPath(path: string): Promise<boolean> {
  return await libparsec.validatePath(path);
}

export async function isValidUserName(name: string): Promise<boolean> {
  return await libparsec.validateHumanHandleLabel(name);
}

export async function isValidEmail(email: string): Promise<boolean> {
  return await libparsec.validateEmail(email);
}

export async function isValidDeviceName(name: string): Promise<boolean> {
  return await libparsec.validateDeviceLabel(name);
}

export async function isValidInvitationToken(token: string): Promise<boolean> {
  return await libparsec.validateInvitationToken(token);
}

export async function isValidEntryName(name: string): Promise<boolean> {
  return await libparsec.validateEntryName(name);
}

export async function isValidOrganizationName(name: string): Promise<boolean> {
  return await libparsec.validateOrganizationId(name);
}
