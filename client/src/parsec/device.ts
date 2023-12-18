// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { needsMocks } from '@/parsec/environment';
import { getClientInfo } from '@/parsec/login';
import { getParsecHandle } from '@/parsec/routing';
import { ClientListUserDevicesError, ClientListUserDevicesErrorTag, DeviceInfo, OwnDeviceInfo, Result, UserID } from '@/parsec/types';
import { libparsec } from '@/plugins/libparsec';
import { DateTime } from 'luxon';

const RECOVERY_DEVICE_PREFIX = 'recovery';

export interface RecoveryDeviceData {
  code: string;
  file: string;
}

export enum RecoveryDeviceErrorTag {
  Internal = 'internal',
  Invalid = 'invalid',
}

export interface RecoveryDeviceError {
  tag: RecoveryDeviceErrorTag.Internal;
}

export interface WrongAuthenticationError {
  tag: RecoveryDeviceErrorTag.Invalid;
}

export async function exportRecoveryDevice(_password: string): Promise<Result<RecoveryDeviceData, WrongAuthenticationError>> {
  const handle = getParsecHandle();

  if (_password !== 'P@ssw0rd.') {
    return { ok: false, error: { tag: RecoveryDeviceErrorTag.Invalid } };
  }

  if (handle !== null && !needsMocks()) {
    return {
      ok: true,
      value: {
        code: 'ABCDEF',
        file: 'Q2lnYXJlQU1vdXN0YWNoZQ==',
      },
    };
  } else {
    return {
      ok: true,
      value: {
        code: 'ABCDEF',
        file: 'Q2lnYXJlQU1vdXN0YWNoZQ==',
      },
    };
  }
}

export async function hasRecoveryDevice(): Promise<boolean> {
  const result = await listOwnDevices();
  if (!result.ok) {
    return false;
  }
  return result.value.some((deviceInfo: OwnDeviceInfo) => deviceInfo.id.startsWith(RECOVERY_DEVICE_PREFIX));
}

export async function listOwnDevices(): Promise<Result<Array<OwnDeviceInfo>, ClientListUserDevicesError>> {
  const handle = getParsecHandle();

  if (handle !== null && !needsMocks()) {
    const clientResult = await getClientInfo();

    if (clientResult.ok) {
      const result = await listUserDevices(clientResult.value.userId);
      if (result.ok) {
        result.value.map((device) => {
          (device as OwnDeviceInfo).isCurrent = device.id === clientResult.value.deviceId;
          return device;
        });
      }
      return result as Result<Array<OwnDeviceInfo>, ClientListUserDevicesError>;
    } else {
      return {
        ok: false,
        error: { tag: ClientListUserDevicesErrorTag.Internal, error: '' },
      };
    }
  } else {
    return {
      ok: true,
      value: [
        {
          id: 'device1',
          deviceLabel: 'My First Device',
          createdOn: DateTime.now(),
          createdBy: 'some_device',
          isCurrent: true,
        },
        {
          id: 'device2',
          deviceLabel: 'My Second Device',
          createdOn: DateTime.now(),
          createdBy: 'device1',
          isCurrent: false,
        },
        {
          id: `${RECOVERY_DEVICE_PREFIX}_device1`,
          deviceLabel: 'Recovery First Device',
          createdOn: DateTime.now(),
          createdBy: 'device1',
          isCurrent: false,
        },
      ],
    };
  }
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
          id: `${RECOVERY_DEVICE_PREFIX}_device1`,
          deviceLabel: 'Recovery First Device',
          createdOn: DateTime.now(),
          createdBy: 'device1',
        },
      ],
    };
  }
}
