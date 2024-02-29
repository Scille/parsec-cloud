// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { libparsec } from '@/plugins/libparsec';

import { needsMocks } from '@/parsec/environment';
import { getParsecHandle } from '@/parsec/routing';
import { ClientListUsersError, ClientRevokeUserError, Result, UserID, UserInfo, UserProfile } from '@/parsec/types';
import { DateTime } from 'luxon';

export async function listUsers(skipRevoked = true): Promise<Result<Array<UserInfo>, ClientListUsersError>> {
  const handle = getParsecHandle();

  if (handle !== null && !needsMocks()) {
    const result = await libparsec.clientListUsers(handle, skipRevoked);
    if (result.ok) {
      result.value.map((item) => {
        item.createdOn = DateTime.fromSeconds(item.createdOn as any as number);
        if (item.revokedOn) {
          item.revokedOn = DateTime.fromSeconds(item.revokedOn as any as number);
        }
        (item as UserInfo).isRevoked = (): boolean => item.revokedOn !== null;
        return item;
      });
    }
    return result as any as Promise<Result<Array<UserInfo>, ClientListUsersError>>;
  } else {
    const value: Array<UserInfo> = [
      {
        id: 'id1',
        // cspell:disable-next-line
        humanHandle: { label: 'Cernd', email: 'cernd@gmail.com' },
        currentProfile: UserProfile.Standard,
        createdOn: DateTime.now(),
        createdBy: 'device',
        revokedOn: null,
        revokedBy: null,
        isRevoked: (): boolean => false,
      },
      {
        id: 'id2',
        // cspell:disable-next-line
        humanHandle: { label: 'Jaheira', email: 'jaheira@gmail.com' },
        currentProfile: UserProfile.Admin,
        createdOn: DateTime.now(),
        createdBy: 'device',
        revokedOn: null,
        revokedBy: null,
        isRevoked: (): boolean => false,
      },
      {
        id: 'me',
        humanHandle: {
          email: 'user@host.com',
          label: 'Gordon Freeman',
        },
        currentProfile: UserProfile.Admin,
        createdOn: DateTime.now(),
        createdBy: 'device',
        revokedOn: null,
        revokedBy: null,
        isRevoked: (): boolean => false,
      },
    ];
    if (!skipRevoked) {
      value.push({
        id: 'id3',
        // cspell:disable-next-line
        humanHandle: { label: 'Valygar Corthala', email: 'val@gmail.com' },
        currentProfile: UserProfile.Standard,
        createdOn: DateTime.now(),
        createdBy: 'device',
        revokedOn: DateTime.now(),
        revokedBy: 'device',
        isRevoked: (): boolean => true,
      });
    }
    return { ok: true, value: value };
  }
}

export async function revokeUser(userId: UserID): Promise<Result<null, ClientRevokeUserError>> {
  const handle = getParsecHandle();

  if (handle !== null && !needsMocks()) {
    return await libparsec.clientRevokeUser(handle, userId);
  } else {
    return { ok: true, value: null };
  }
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
