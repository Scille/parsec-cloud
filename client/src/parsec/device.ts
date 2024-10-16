// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { needsMocks } from '@/parsec/environment';
import { getClientConfig } from '@/parsec/internals';
import { getClientInfo } from '@/parsec/login';
import { getParsecHandle } from '@/parsec/routing';
import {
  ClientListUserDevicesError,
  ClientListUserDevicesErrorTag,
  DeviceFileType,
  DeviceInfo,
  DeviceSaveStrategy,
  ExportRecoveryDeviceError,
  ImportRecoveryDeviceError,
  ImportRecoveryDeviceErrorTag,
  OwnDeviceInfo,
  Result,
  UserID,
} from '@/parsec/types';
import { AvailableDevice, libparsec } from '@/plugins/libparsec';
import { DateTime } from 'luxon';

const RECOVERY_DEVICE_PREFIX = 'recovery';

function generateRecoveryDeviceLabel(): string {
  return `${RECOVERY_DEVICE_PREFIX}_${DateTime.utc().toMillis()}`;
}

export async function exportRecoveryDevice(): Promise<Result<[string, Uint8Array], ExportRecoveryDeviceError>> {
  const handle = getParsecHandle();

  if (handle !== null && !needsMocks()) {
    return await libparsec.exportRecoveryDevice(handle, generateRecoveryDeviceLabel());
  } else {
    return {
      ok: true,
      value: ['ABCDEF', new Uint8Array([0x6d, 0x65, 0x6f, 0x77])],
    };
  }
}

export async function importRecoveryDevice(
  deviceLabel: string,
  recoveryData: Uint8Array,
  passphrase: string,
  saveStrategy: DeviceSaveStrategy,
): Promise<Result<AvailableDevice, ImportRecoveryDeviceError>> {
  if (!needsMocks()) {
    console.log(getClientConfig());
    return await libparsec.importRecoveryDevice(getClientConfig(), recoveryData, passphrase, deviceLabel, saveStrategy);
  }

  // cspell:disable-next-line
  if (passphrase !== 'ABCD-EFGH-IJKL-MNOP-QRST-UVWX-YZ12-3456-7890-ABCD-EFGH-IJKL-MNOP') {
    return { ok: false, error: { tag: ImportRecoveryDeviceErrorTag.InvalidPassphrase, error: 'Wrong passphrase' } };
  }
  return {
    ok: true,
    value: {
      keyFilePath: 'dummy',
      serverUrl: 'https://parsec.invalid',
      createdOn: DateTime.utc(),
      protectedOn: DateTime.utc(),
      organizationId: 'dummy_org',
      userId: 'dummy_user_id',
      deviceId: 'device_id',
      humanHandle: {
        email: 'dummy_email@email.dum',
        label: 'dummy_label',
      },
      deviceLabel: deviceLabel,
      ty: DeviceFileType.Password,
    },
  };
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
          (device as OwnDeviceInfo).isRecovery = device.deviceLabel.startsWith(RECOVERY_DEVICE_PREFIX);
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
      value: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10].map((n) => {
        return {
          id: n < 10 ? `device{n}` : `${RECOVERY_DEVICE_PREFIX}_device{n}`,
          deviceLabel: 'Web',
          createdOn: DateTime.now(),
          createdBy: 'some_device',
          isCurrent: n === 1,
          isRecovery: n === 10,
        };
      }),
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
    return result;
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
