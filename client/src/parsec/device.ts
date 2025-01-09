// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { needsMocks } from '@/parsec/environment';
import { getClientConfig, wait } from '@/parsec/internals';
import { getClientInfo } from '@/parsec/login';
import { getParsecHandle } from '@/parsec/routing';
import {
  ArchiveDeviceError,
  AvailableDevice,
  ClientExportRecoveryDeviceError,
  ClientListUserDevicesError,
  ClientListUserDevicesErrorTag,
  ClientNewDeviceInvitationError,
  DeviceFileType,
  DeviceInfo,
  DevicePurpose,
  DeviceSaveStrategy,
  ImportRecoveryDeviceError,
  ImportRecoveryDeviceErrorTag,
  InvitationEmailSentStatus,
  NewInvitationInfo,
  OwnDeviceInfo,
  Result,
  UserID,
} from '@/parsec/types';
import { libparsec } from '@/plugins/libparsec';
import { DateTime } from 'luxon';

const RECOVERY_DEVICE_PREFIX = 'recovery';

function generateRecoveryDeviceLabel(): string {
  return `${RECOVERY_DEVICE_PREFIX}_${DateTime.utc().toMillis()}`;
}

export async function exportRecoveryDevice(): Promise<Result<[string, Uint8Array], ClientExportRecoveryDeviceError>> {
  const handle = getParsecHandle();

  if (handle !== null && !needsMocks()) {
    return await libparsec.clientExportRecoveryDevice(handle, generateRecoveryDeviceLabel());
  } else {
    await wait(300);
    return {
      ok: true,
      value: ['ABCDEF', new Uint8Array([0x6d, 0x65, 0x6f, 0x77])],
    };
  }
}

function areArraysEqual(a: Uint8Array, b: Uint8Array): boolean {
  return a.every((val, index) => {
    return val === b[index];
  });
}

export async function importRecoveryDevice(
  deviceLabel: string,
  recoveryData: Uint8Array,
  passphrase: string,
  saveStrategy: DeviceSaveStrategy,
): Promise<Result<AvailableDevice, ImportRecoveryDeviceError>> {
  if (!needsMocks()) {
    const result = await libparsec.importRecoveryDevice(getClientConfig(), recoveryData, passphrase, deviceLabel, saveStrategy);
    if (result.ok) {
      result.value.createdOn = DateTime.fromSeconds(result.value.createdOn as any as number);
      result.value.protectedOn = DateTime.fromSeconds(result.value.protectedOn as any as number);
    }
    return result;
  }

  // cspell:disable-next-line
  if (passphrase !== 'ABCD-EFGH-IJKL-MNOP-QRST-UVWX-YZ12-3456-7890-ABCD-EFGH-IJKL-MNOP') {
    return { ok: false, error: { tag: ImportRecoveryDeviceErrorTag.InvalidPassphrase, error: 'Wrong passphrase' } };
  }
  if (areArraysEqual(recoveryData, new Uint8Array([78, 79, 80, 10]))) {
    return { ok: false, error: { tag: ImportRecoveryDeviceErrorTag.InvalidData, error: 'Wrong data' } };
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
          (device as OwnDeviceInfo).isRecovery =
            device.deviceLabel.startsWith(RECOVERY_DEVICE_PREFIX) || device.purpose === DevicePurpose.PassphraseRecovery;
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
      value: [1, 2, 3].map((n) => {
        return {
          id: `device${n}`,
          deviceLabel: n === 3 ? `${RECOVERY_DEVICE_PREFIX}_device${n}` : `device${n}`,
          createdOn: DateTime.now(),
          createdBy: 'some_device',
          isCurrent: n === 1,
          isRecovery: n === 3,
          purpose: n === 3 ? DevicePurpose.PassphraseRecovery : DevicePurpose.Standard,
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
          purpose: DevicePurpose.Standard,
          createdOn: DateTime.now(),
          createdBy: 'some_device',
        },
        {
          id: 'device2',
          deviceLabel: 'My Second Device',
          purpose: DevicePurpose.Standard,
          createdOn: DateTime.now(),
          createdBy: 'device1',
        },
        {
          id: `${RECOVERY_DEVICE_PREFIX}_device1`,
          deviceLabel: 'Recovery First Device',
          purpose: DevicePurpose.PassphraseRecovery,
          createdOn: DateTime.now(),
          createdBy: 'device1',
        },
      ],
    };
  }
}

export async function archiveDevice(device: AvailableDevice): Promise<Result<null, ArchiveDeviceError>> {
  return await libparsec.archiveDevice(device.keyFilePath);
}

export async function createDeviceInvitation(sendEmail: boolean): Promise<Result<NewInvitationInfo, ClientNewDeviceInvitationError>> {
  const clientHandle = getParsecHandle();

  if (clientHandle !== null && !needsMocks()) {
    const result = await libparsec.clientNewDeviceInvitation(clientHandle, sendEmail);
    return result;
  } else {
    return {
      ok: true,
      value: {
        // cspell:disable-next-line
        addr: 'parsec3://example.parsec.cloud/Org?a=claim_device&p=xBj1p7vXl_j1tzTjrx5pzbXV7XTbx_Xnnb0',
        // cspell:disable-next-line
        token: '9ae715f49bc0468eac211e1028f15529',
        emailSentStatus: InvitationEmailSentStatus.Success,
      },
    };
  }
}
