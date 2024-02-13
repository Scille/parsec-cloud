// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { needsMocks } from '@/parsec/environment';
import { getClientInfo } from '@/parsec/login';
import { getParsecHandle } from '@/parsec/routing';
import {
  ClientListUserDevicesError,
  ClientListUserDevicesErrorTag,
  DeviceFileType,
  DeviceInfo,
  OwnDeviceInfo,
  Result,
  UserID,
} from '@/parsec/types';
import { AvailableDevice, libparsec } from '@/plugins/libparsec';
import { DateTime } from 'luxon';

const RECOVERY_DEVICE_PREFIX = 'recovery';

export type SecretKey = string;

export interface RecoveryDeviceData {
  code: string;
  file: string;
}

export enum RecoveryDeviceErrorTag {
  Internal = 'internal',
  Invalid = 'invalid',
}

export enum RecoveryImportErrorTag {
  Internal = 'internal',
  KeyError = 'keyError',
  RecoveryFileError = 'recoveryFileError',
}

export interface RecoveryDeviceError {
  tag: RecoveryDeviceErrorTag.Internal;
}

export interface WrongAuthenticationError {
  tag: RecoveryDeviceErrorTag.Invalid;
}

export interface RecoveryImportInternalError {
  tag: RecoveryImportErrorTag.Internal;
}

export interface RecoveryImportFileError {
  tag: RecoveryImportErrorTag.RecoveryFileError;
}

export interface RecoveryImportKeyError {
  tag: RecoveryImportErrorTag.KeyError;
}

export type RecoveryImportError = RecoveryImportInternalError | RecoveryImportFileError | RecoveryImportKeyError;

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

export async function importRecoveryDevice(
  deviceLabel: string,
  file: File,
  _passphrase: string,
): Promise<Result<DeviceInfo, RecoveryImportError>> {
  const handle = getParsecHandle();

  // cspell:disable-next-line
  if (_passphrase !== 'ABCD-EFGH-IJKL-MNOP-QRST-UVWX-YZ12-3456-7890-ABCD-EFGH-IJKL-MNOP') {
    return { ok: false, error: { tag: RecoveryImportErrorTag.KeyError } };
  }
  if (!file.name.endsWith('.psrk')) {
    return { ok: false, error: { tag: RecoveryImportErrorTag.RecoveryFileError } };
  }
  if (handle !== null && !needsMocks()) {
    return {
      ok: true,
      value: {
        id: 'fake_id',
        deviceLabel: deviceLabel,
        createdOn: DateTime.now(),
        createdBy: null,
      },
    };
  } else {
    return {
      ok: true,
      value: {
        id: 'fake_id',
        deviceLabel: deviceLabel,
        createdOn: DateTime.now(),
        createdBy: null,
      },
    };
  }
}

export async function saveDevice(deviceInfo: DeviceInfo, _password: string): Promise<Result<AvailableDevice, RecoveryImportError>> {
  // const _saveStrategy: DeviceSaveStrategyPassword = { tag: DeviceSaveStrategyTag.Password, password: password };
  return {
    ok: true,
    value: {
      keyFilePath: 'dummy',
      organizationId: 'dummy_org',
      deviceId: deviceInfo.id,
      humanHandle: {
        email: 'dummy_email@email.dum',
        label: 'dummy_label',
      },
      deviceLabel: deviceInfo.deviceLabel,
      slug: 'dummy_slug',
      ty: DeviceFileType.Password,
    },
  };
}

export async function deleteDevice(_device: AvailableDevice): Promise<Result<boolean>> {
  return { ok: true, value: true };
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
