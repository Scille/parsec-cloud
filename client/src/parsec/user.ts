// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { libparsec, UserInfo as ParsecUserInfo } from '@/plugins/libparsec';

import {
  ClientGetUserDeviceError,
  ClientGetUserInfoError,
  ClientListUsersError,
  ClientRevokeUserError,
  ClientUserUpdateProfileError,
  DeviceID,
  Result,
  UserID,
  UserInfo,
  UserProfile,
} from '@/parsec/types';
import { generateNoHandleError } from '@/parsec/utils';
import { getConnectionHandle } from '@/router';
import { DateTime } from 'luxon';

function filterUserList(list: Array<UserInfo>, pattern: string): Array<UserInfo> {
  pattern = pattern.toLocaleLowerCase();
  return list.filter((item) => {
    return item.humanHandle.label.toLocaleLowerCase().includes(pattern) || item.humanHandle.email.toLocaleLowerCase().includes(pattern);
  });
}

function patchUserInfo(item: ParsecUserInfo, frozenList: Array<UserID> = []): UserInfo {
  item.createdOn = DateTime.fromSeconds(item.createdOn as any as number);
  if (item.revokedOn) {
    item.revokedOn = DateTime.fromSeconds(item.revokedOn as any as number);
  }
  (item as UserInfo).isRevoked = (): boolean => item.revokedOn !== null;
  (item as UserInfo).isFrozen = (): boolean =>
    !(item as UserInfo).isRevoked() && frozenList.find((userId) => userId === item.id) !== undefined;
  (item as UserInfo).isActive = (): boolean => !(item as UserInfo).isRevoked() && !(item as UserInfo).isFrozen();
  return item as UserInfo;
}

export async function listUsers(skipRevoked = true, pattern = ''): Promise<Result<Array<UserInfo>, ClientListUsersError>> {
  const handle = getConnectionHandle();

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
        return patchUserInfo(item, frozen);
      });
    }
    return result as any as Promise<Result<Array<UserInfo>, ClientListUsersError>>;
  }
  return generateNoHandleError<ClientListUsersError>();
}

export async function revokeUser(userId: UserID): Promise<Result<null, ClientRevokeUserError>> {
  const handle = getConnectionHandle();

  if (handle !== null) {
    return await libparsec.clientRevokeUser(handle, userId);
  }
  return generateNoHandleError<ClientRevokeUserError>();
}

export async function getUserInfo(userId: UserID): Promise<Result<UserInfo, ClientGetUserInfoError>> {
  const handle = getConnectionHandle();

  if (handle !== null) {
    await libparsec.clientGetUserInfo(handle, userId);
  }
  return generateNoHandleError<ClientGetUserInfoError>();
}

export async function updateProfile(userId: UserID, profile: UserProfile): Promise<Result<null, ClientUserUpdateProfileError>> {
  const handle = getConnectionHandle();

  if (handle !== null) {
    return await libparsec.clientUpdateUserProfile(handle, userId, profile);
  }
  return generateNoHandleError<ClientUserUpdateProfileError>();
}

const DeviceUserMapping = new Map<DeviceID, UserInfo>();

export async function getUserInfoFromDeviceID(deviceId: DeviceID): Promise<Result<UserInfo, ClientGetUserDeviceError>> {
  const handle = getConnectionHandle();
  if (handle !== null) {
    let userInfo: UserInfo | undefined = DeviceUserMapping.get(deviceId);

    if (!userInfo) {
      const result = await libparsec.clientGetUserDevice(handle, deviceId);

      if (!result.ok) {
        return result;
      }
      userInfo = patchUserInfo(result.value[0]);
      DeviceUserMapping.set(deviceId, userInfo);
    }
    return { ok: true, value: userInfo };
  }
  return generateNoHandleError<ClientGetUserDeviceError>();
}
