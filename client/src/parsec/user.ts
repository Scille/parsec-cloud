// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { libparsec } from '@/plugins/libparsec';

import { getParsecHandle } from '@/parsec/routing';
import { ClientListUsersError, ClientRevokeUserError, Result, UserID, UserInfo } from '@/parsec/types';
import { generateNoHandleError } from '@/parsec/utils';
import { DateTime } from 'luxon';

function filterUserList(list: Array<UserInfo>, pattern: string): Array<UserInfo> {
  pattern = pattern.toLocaleLowerCase();
  return list.filter((item) => {
    return item.humanHandle.label.toLocaleLowerCase().includes(pattern) || item.humanHandle.email.toLocaleLowerCase().includes(pattern);
  });
}

export async function listUsers(skipRevoked = true, pattern = ''): Promise<Result<Array<UserInfo>, ClientListUsersError>> {
  const handle = getParsecHandle();

  if (handle !== null) {
    const result = await libparsec.clientListUsers(handle, skipRevoked);
    if (result.ok) {
      const frozenResult = await libparsec.clientListFrozenUsers(handle);
      const frozen: Array<UserID> = frozenResult.ok ? frozenResult.value : [];
      if (pattern.length > 0) {
        // Won't be using dates or `isRevoked` so the cast is fine
        result.value = filterUserList(result.value as Array<UserInfo>, pattern);
      }
      result.value.map((item) => {
        item.createdOn = DateTime.fromSeconds(item.createdOn as any as number);
        if (item.revokedOn) {
          item.revokedOn = DateTime.fromSeconds(item.revokedOn as any as number);
        }
        (item as UserInfo).isRevoked = (): boolean => item.revokedOn !== null;
        (item as UserInfo).isFrozen = (): boolean =>
          !(item as UserInfo).isRevoked() && frozen.find((userId) => userId === item.id) !== undefined;
        (item as UserInfo).isActive = (): boolean => !(item as UserInfo).isRevoked() && !(item as UserInfo).isFrozen();
        return item;
      });
    }
    return result as any as Promise<Result<Array<UserInfo>, ClientListUsersError>>;
  }
  return generateNoHandleError<ClientListUsersError>();
}

export async function revokeUser(userId: UserID): Promise<Result<null, ClientRevokeUserError>> {
  const handle = getParsecHandle();

  if (handle !== null) {
    return await libparsec.clientRevokeUser(handle, userId);
  }
  return generateNoHandleError<ClientRevokeUserError>();
}

export enum UserInfoErrorTag {
  NotFound = 'NotFound',
  Internal = 'Internal',
}

interface UserInfoNotFoundError {
  tag: UserInfoErrorTag.NotFound;
}

interface UserInfoInternalError {
  tag: UserInfoErrorTag.Internal;
}

export type UserInfoError = UserInfoInternalError | UserInfoNotFoundError;

export async function getUserInfo(userId: UserID): Promise<Result<UserInfo, UserInfoError>> {
  const listResult = await listUsers(false);

  if (!listResult.ok) {
    return { ok: false, error: { tag: UserInfoErrorTag.Internal } };
  }

  const userInfo = listResult.value.find((item) => item.id === userId);
  if (!userInfo) {
    return { ok: false, error: { tag: UserInfoErrorTag.NotFound } };
  }
  return { ok: true, value: userInfo };
}
