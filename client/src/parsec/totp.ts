// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { getClientConfig } from '@/parsec/internals';
import {
  ClientTotpCreateOpaqueKeyError,
  ClientTOTPSetupConfirmError,
  ClientTotpSetupStatusError,
  OrganizationID,
  ParsecAddr,
  ParsecTOTPResetAddr,
  Result,
  SecretKey,
  TotpFetchOpaqueKeyError,
  TOTPOpaqueKeyID,
  TotpSetupConfirmAnonymousError,
  TOTPSetupStatus,
  TotpSetupStatusAnonymousError,
  UserID,
} from '@/parsec/types';
import { generateNoHandleError } from '@/parsec/utils';
import { libparsec } from '@/plugins/libparsec';
import { getConnectionHandle } from '@/router';

export async function getTotpStatus(): Promise<Result<TOTPSetupStatus, ClientTotpSetupStatusError>> {
  const handle = getConnectionHandle();

  if (handle !== null) {
    return await libparsec.clientTotpSetupStatus(handle);
  }
  return generateNoHandleError<ClientTotpSetupStatusError>();
}

export async function verifyTotpSetup(verifyCode: string): Promise<Result<null, ClientTOTPSetupConfirmError>> {
  const handle = getConnectionHandle();

  if (handle !== null) {
    return await libparsec.clientTotpSetupConfirm(handle, verifyCode);
  }
  return generateNoHandleError<ClientTOTPSetupConfirmError>();
}

export async function getTotpOpaqueKey(): Promise<Result<[TOTPOpaqueKeyID, SecretKey], ClientTotpCreateOpaqueKeyError>> {
  const handle = getConnectionHandle();

  if (handle !== null) {
    return await libparsec.clientTotpCreateOpaqueKey(handle);
  }
  return generateNoHandleError<ClientTotpCreateOpaqueKeyError>();
}

export async function fetchTotpOpaqueKey(
  serverAddr: ParsecAddr,
  organizationId: OrganizationID,
  userId: UserID,
  opaqueKeyId: TOTPOpaqueKeyID,
  code: string,
): Promise<Result<SecretKey, TotpFetchOpaqueKeyError>> {
  return await libparsec.totpFetchOpaqueKey(getClientConfig(), serverAddr, organizationId, userId, opaqueKeyId, code);
}

export async function totpResetStatus(addr: ParsecTOTPResetAddr): Promise<Result<TOTPSetupStatus, TotpSetupStatusAnonymousError>> {
  return await libparsec.totpSetupStatusAnonymous(getClientConfig(), addr);
}

export async function totpConfirmReset(
  addr: ParsecTOTPResetAddr,
  verifyCode: string,
): Promise<Result<null, TotpSetupConfirmAnonymousError>> {
  return await libparsec.totpSetupConfirmAnonymous(getClientConfig(), addr, verifyCode);
}

export async function generateTotpUrl(secret: string, organization: OrganizationID, issuer = 'parsec.cloud'): Promise<string> {
  // cspell:disable-next-line
  return `otpauth://totp/${issuer}:${organization}?secret=${secret}&issuer=${issuer}`;
}
