// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { getClientConfig } from '@/parsec/internals';
import { getClientInfo } from '@/parsec/login';
import {
  ArchiveDeviceError,
  AvailableDevice,
  ClientExportRecoveryDeviceError,
  ClientListUserDevicesError,
  ClientListUserDevicesErrorTag,
  ClientNewDeviceInvitationError,
  DeviceInfo,
  DevicePurpose,
  DeviceSaveStrategy,
  ImportRecoveryDeviceError,
  NewInvitationInfo,
  OwnDeviceInfo,
  Result,
  UserID,
} from '@/parsec/types';
import { generateNoHandleError } from '@/parsec/utils';
import { libparsec } from '@/plugins/libparsec';
import { getConnectionHandle } from '@/router';
import { DateTime } from 'luxon';

const RECOVERY_DEVICE_PREFIX = 'recovery';

function generateRecoveryDeviceLabel(): string {
  return `${RECOVERY_DEVICE_PREFIX}_${DateTime.utc().toMillis()}`;
}

export async function exportRecoveryDevice(): Promise<Result<[string, Uint8Array], ClientExportRecoveryDeviceError>> {
  const handle = getConnectionHandle();

  if (handle !== null) {
    return await libparsec.clientExportRecoveryDevice(handle, generateRecoveryDeviceLabel());
  }
  return generateNoHandleError<ClientExportRecoveryDeviceError>();
}

export async function importRecoveryDevice(
  deviceLabel: string,
  recoveryData: Uint8Array,
  passphrase: string,
  saveStrategy: DeviceSaveStrategy,
): Promise<Result<AvailableDevice, ImportRecoveryDeviceError>> {
  const result = await libparsec.importRecoveryDevice(getClientConfig(), recoveryData, passphrase, deviceLabel, saveStrategy);
  if (result.ok) {
    result.value.createdOn = DateTime.fromSeconds(result.value.createdOn as any as number);
    result.value.protectedOn = DateTime.fromSeconds(result.value.protectedOn as any as number);
  }
  return result;
}

export async function listOwnDevices(): Promise<Result<Array<OwnDeviceInfo>, ClientListUserDevicesError>> {
  const handle = getConnectionHandle();

  if (handle !== null) {
    const clientResult = await getClientInfo();

    if (clientResult.ok) {
      const result = await listUserDevices(clientResult.value.userId);
      if (result.ok) {
        result.value.map((device) => {
          (device as OwnDeviceInfo).isCurrent = device.id === clientResult.value.deviceId;
          (device as OwnDeviceInfo).isRecovery =
            device.deviceLabel.startsWith(RECOVERY_DEVICE_PREFIX) || device.purpose === DevicePurpose.PassphraseRecovery;
          (device as OwnDeviceInfo).isRegistration = device.purpose === DevicePurpose.Registration;
          (device as OwnDeviceInfo).isShamir = device.purpose === DevicePurpose.ShamirRecovery;
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
  }
  return generateNoHandleError<ClientListUserDevicesError>();
}

export async function listUserDevices(user: UserID): Promise<Result<Array<DeviceInfo>, ClientListUserDevicesError>> {
  const handle = getConnectionHandle();

  if (handle !== null) {
    const result = await libparsec.clientListUserDevices(handle, user);
    if (result.ok) {
      result.value.map((item) => {
        item.createdOn = DateTime.fromSeconds(item.createdOn as any as number);
        return item;
      });
    }
    return result;
  }
  return generateNoHandleError<ClientListUserDevicesError>();
}

export async function archiveDevice(device: AvailableDevice): Promise<Result<null, ArchiveDeviceError>> {
  return await libparsec.archiveDevice(window.getConfigDir(), device.keyFilePath);
}

export async function createDeviceInvitation(sendEmail: boolean): Promise<Result<NewInvitationInfo, ClientNewDeviceInvitationError>> {
  const clientHandle = getConnectionHandle();

  if (clientHandle !== null) {
    const result = await libparsec.clientNewDeviceInvitation(clientHandle, sendEmail);
    return result;
  }
  return generateNoHandleError<ClientNewDeviceInvitationError>();
}
