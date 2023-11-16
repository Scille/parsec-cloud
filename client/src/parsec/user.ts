// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { libparsec } from '@/plugins/libparsec';

import { Result, UserInfo, UserProfile, ClientListUsersError, UserID, DeviceInfo, ClientListUserDevicesError } from '@/parsec/types';
import { getParsecHandle } from '@/parsec/routing';
import { DateTime } from 'luxon';
import { needsMocks } from '@/parsec/environment';

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

export async function listRevokedUsers(): Promise<Result<Array<UserInfo>, ClientListUsersError>> {
  const result = await listUsers(false);

  if (result.ok) {
    result.value = result.value.filter((user) => user.isRevoked());
  }
  return result;
}

export async function listUserDevices(user: UserID): Promise<Result<Array<DeviceInfo>, ClientListUserDevicesError>> {
  const handle = getParsecHandle();

  if (handle !== null && !needsMocks()) {
    const result = await libparsec.clientListUserDevices(handle, user);
    if (result.ok) {
      result.value.map((item) => {
        item.createdOn = DateTime.fromSeconds(item.createdOn as any as number);
        return item;
      });
    }
    return result as any as Promise<Result<Array<DeviceInfo>, ClientListUserDevicesError>>;
  } else {
    return {
      ok: true,
      value: [
        {
          id: 'device1',
          deviceLabel: 'My First Device',
          createdOn: DateTime.now(),
          createdBy: 'some_device',
        },
        {
          id: 'device2',
          deviceLabel: 'My Second Device',
          createdOn: DateTime.now(),
          createdBy: 'device1',
        },
        {
          id: 'recovery_device1',
          deviceLabel: 'Recovery First Device',
          createdOn: DateTime.now(),
          createdBy: 'device1',
        },
      ],
    };
  }
}

export enum RevokeUserTag {
  Internal = 'Internal',
}

export interface RevokeUserError {
  tag: RevokeUserTag.Internal;
}

export async function revokeUser(userId: UserID): Promise<Result<null, RevokeUserError>> {
  const handle = getParsecHandle();

  if (handle !== null && !needsMocks()) {
    // Will call the bindings
    return { ok: true, value: null };
  } else {
    return { ok: true, value: null };
  }
}
