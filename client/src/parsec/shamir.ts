// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import {
  ClientDeleteShamirRecoveryError,
  ClientGetSelfShamirRecoveryError,
  ClientGetSelfShamirRecoveryErrorTag,
  ClientListShamirRecoveriesForOthersError,
  ClientListShamirRecoveriesForOthersErrorTag,
  ClientSetupShamirRecoveryError,
  OtherShamirRecoveryInfo,
  Result,
  SelfShamirRecoveryInfo,
  SelfShamirRecoveryInfoSetupAllValid,
  SelfShamirRecoveryInfoTag,
  UserID,
  UserInfo,
} from '@/parsec/types';
import { getUserInfo } from '@/parsec/user';
import { generateNoHandleError } from '@/parsec/utils';
import { libparsec } from '@/plugins/libparsec';
import { getConnectionHandle } from '@/router';
import { DateTime } from 'luxon';

export async function getRequiredShamirThreshold(): Promise<number> {
  return 2;
}

export async function getSelfShamirRecovery(): Promise<Result<SelfShamirRecoveryInfo, ClientGetSelfShamirRecoveryError>> {
  const handle = getConnectionHandle();

  if (handle === null) {
    return generateNoHandleError<ClientGetSelfShamirRecoveryError>();
  }
  const result = await libparsec.clientGetSelfShamirRecovery(handle);
  if (result.ok) {
    (result.value as SelfShamirRecoveryInfo).isUsable = () => false;
    if (result.value.tag !== SelfShamirRecoveryInfoTag.NeverSetup) {
      result.value.createdOn = DateTime.fromSeconds(result.value.createdOn as any as number);
    }
    if (
      result.value.tag === SelfShamirRecoveryInfoTag.SetupAllValid ||
      result.value.tag === SelfShamirRecoveryInfoTag.SetupWithRevokedRecipients
    ) {
      (result.value as SelfShamirRecoveryInfo).isUsable = () => true;
      try {
        const users = await Promise.all(
          result.value.perRecipientShares.keys().map(async (userId) => {
            const userResult = await getUserInfo(userId);
            if (userResult.ok) {
              return userResult.value;
            }
            throw new Error(userResult.error.tag);
          }),
        );
        (result.value as SelfShamirRecoveryInfoSetupAllValid).recipients = users;
      } catch (_err: any) {
        return { ok: false, error: { tag: ClientGetSelfShamirRecoveryErrorTag.Internal, error: 'Failed to get user info' } };
      }
    } else if (result.value.tag === SelfShamirRecoveryInfoTag.Deleted) {
      result.value.deletedOn = DateTime.fromSeconds(result.value.deletedOn as any as number);
    }
  }
  return result as Result<SelfShamirRecoveryInfo, ClientGetSelfShamirRecoveryError>;
}

export async function deleteSelfShamirRecovery(): Promise<Result<null, ClientDeleteShamirRecoveryError>> {
  const handle = getConnectionHandle();

  if (handle === null) {
    return generateNoHandleError<ClientDeleteShamirRecoveryError>();
  }
  return await libparsec.clientDeleteShamirRecovery(handle);
}

export async function getOthersShamirRecovery(): Promise<Result<Array<OtherShamirRecoveryInfo>, ClientListShamirRecoveriesForOthersError>> {
  const handle = getConnectionHandle();

  if (handle === null) {
    return generateNoHandleError<ClientListShamirRecoveriesForOthersError>();
  }
  const result = await libparsec.clientListShamirRecoveriesForOthers(handle);

  if (result.ok) {
    try {
      result.value = await Promise.all(
        result.value.map(async (shamirInfo) => {
          shamirInfo.createdOn = DateTime.fromSeconds(shamirInfo.createdOn as any as number);
          const userResult = await getUserInfo(shamirInfo.userId);
          if (userResult.ok) {
            (shamirInfo as OtherShamirRecoveryInfo).owner = userResult.value;
          } else {
            throw new Error(userResult.error.tag);
          }
          return shamirInfo;
        }),
      );
    } catch (_err: any) {
      return { ok: false, error: { tag: ClientListShamirRecoveriesForOthersErrorTag.Internal, error: 'Failed to get user info' } };
    }
  }
  return result as Result<Array<OtherShamirRecoveryInfo>, ClientListShamirRecoveriesForOthersError>;
}

export async function setupShamirRecovery(
  users: Array<UserInfo>,
  threshold: number,
  partsPerRecipient: number,
): Promise<Result<null, ClientSetupShamirRecoveryError>> {
  const handle = getConnectionHandle();

  if (handle === null) {
    return generateNoHandleError<ClientSetupShamirRecoveryError>();
  }
  const map = new Map(users.map((user) => [user.id, partsPerRecipient]));
  return await libparsec.clientSetupShamirRecovery(handle, map as any as Map<UserID, number>, threshold);
}
