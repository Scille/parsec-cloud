// Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS

import { HumanHandle, libparsec, UserInfo as ParsecUserInfo } from '@/plugins/libparsec';

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
  UserTimelineEntry,
  UserTimelineEntryTag,
} from '@/parsec/types';
import { generateNoHandleError } from '@/parsec/utils';
import { getConnectionHandle } from '@/router';
import { InformationManager } from '@/services/informationManager';
import { useSidePopoverMenu } from '@/services/sidePopoverMenu';
import UserTimeline from '@/views/users/UserTimeline.vue';
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
  (item as UserInfo).isAnonymous = (): boolean =>
    item.humanHandle.label === item.id && item.humanHandle.email === `${item.id}@redacted.invalid`;
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

  if (handle === null) {
    return generateNoHandleError<ClientGetUserInfoError>();
  }
  const result = await libparsec.clientGetUserInfo(handle, userId);
  if (result.ok) {
    result.value = patchUserInfo(result.value);
  }
  return result as Result<UserInfo, ClientGetUserInfoError>;
}

export async function updateProfile(userId: UserID, profile: UserProfile): Promise<Result<null, ClientUserUpdateProfileError>> {
  const handle = getConnectionHandle();

  if (handle !== null) {
    return await libparsec.clientUpdateUserProfile(handle, userId, profile);
  }
  return generateNoHandleError<ClientUserUpdateProfileError>();
}

export async function getUserInfoFromDeviceID(deviceId: DeviceID): Promise<Result<UserInfo, ClientGetUserDeviceError>> {
  const handle = getConnectionHandle();
  if (handle === null) {
    return generateNoHandleError<ClientGetUserDeviceError>();
  }
  const result = await libparsec.clientGetUserDevice(handle, deviceId);

  if (result.ok) {
    result.value[0] = patchUserInfo(result.value[0]);
    return { ok: true, value: result.value[0] as UserInfo };
  }
  return result;
}

export async function getUserTimeline(): Promise<Result<Array<UserTimelineEntry>, ClientListUsersError>> {
  const users = await listUsers(false);
  if (!users.ok) {
    return users;
  }
  const entries: Array<UserTimelineEntry> = [];

  for (const user of users.value) {
    let inviter: HumanHandle | undefined = undefined;
    if (user.createdBy) {
      const inviterResult = await getUserInfoFromDeviceID(user.createdBy);
      if (inviterResult.ok) {
        inviter = inviterResult.value.humanHandle;
      }
    }
    entries.push({
      tag: UserTimelineEntryTag.Joined,
      timestamp: user.createdOn,
      user: user.humanHandle,
      inviter: inviter,
    });

    if (user.revokedOn) {
      let revoker: HumanHandle | undefined = undefined;
      if (user.revokedBy) {
        const revokerResult = await getUserInfoFromDeviceID(user.revokedBy);
        if (revokerResult.ok) {
          revoker = revokerResult.value.humanHandle;
        }
      }
      entries.push({
        tag: UserTimelineEntryTag.Revoked,
        timestamp: user.revokedOn,
        user: user.humanHandle,
        revoker: revoker,
      });
    }
  }
  return { ok: true, value: entries.sort((a, b) => (a.timestamp.toMillis() > b.timestamp.toMillis() ? 1 : -1)) };
}

export async function showUserTimeline(informationManager: InformationManager): Promise<void> {
  const sidePopoverMenu = useSidePopoverMenu();
  await sidePopoverMenu.open({
    component: UserTimeline,
    props: {
      informationManager: informationManager,
    },
  });
}
