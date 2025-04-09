// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { DeviceAccessStrategyKeyring, DeviceSaveStrategyKeyring, DeviceSaveStrategyPassword, libparsec } from '@/plugins/libparsec';

import { getClientConfig } from '@/parsec/internals';
import { parseParsecAddr } from '@/parsec/organization';
import { getParsecHandle } from '@/parsec/routing';
import {
  AvailableDevice,
  ClientInfo,
  ClientInfoError,
  ClientStartError,
  ClientStopError,
  ConnectionHandle,
  DeviceAccessStrategy,
  DeviceAccessStrategyPassword,
  DeviceAccessStrategyTag,
  DeviceSaveStrategy,
  DeviceSaveStrategyTag,
  OrganizationID,
  Result,
  UpdateDeviceError,
  UserProfile,
} from '@/parsec/types';
import { generateNoHandleError } from '@/parsec/utils';
import { getConnectionHandle } from '@/router';
import { DateTime } from 'luxon';

export interface LoggedInDeviceInfo {
  handle: ConnectionHandle;
  device: AvailableDevice;
  isExpired: boolean;
  isOnline: boolean;
  shouldAcceptTos: boolean;
}

const loggedInDevices: Array<LoggedInDeviceInfo> = [];

export async function getLoggedInDevices(): Promise<Array<LoggedInDeviceInfo>> {
  return loggedInDevices;
}

export function isDeviceLoggedIn(device: AvailableDevice): boolean {
  return loggedInDevices.find((info) => info.device.deviceId === device.deviceId) !== undefined;
}

export function getDeviceHandle(device: AvailableDevice): ConnectionHandle | null {
  const info = loggedInDevices.find((info) => info.device.deviceId === device.deviceId);
  if (info) {
    return info.handle;
  }
  return null;
}

export function getConnectionInfo(handle: ConnectionHandle | null = null): LoggedInDeviceInfo | undefined {
  if (!handle) {
    handle = getParsecHandle();
  }
  return loggedInDevices.find((info) => info.handle === handle);
}

interface OrganizationInfo {
  server?: {
    hostname: string;
    port: number;
  };
  id: OrganizationID;
}

export async function getOrganizationHandle(orgInfo: OrganizationInfo): Promise<ConnectionHandle | null> {
  const matchingDevices = loggedInDevices.filter((item) => item.device.organizationId === orgInfo.id);
  if (!orgInfo.server || matchingDevices.length <= 1) {
    return matchingDevices.length > 0 ? matchingDevices[0].handle : null;
  }
  for (const device of matchingDevices) {
    const clientInfo = await getClientInfo(device.handle);
    if (!clientInfo.ok) {
      continue;
    }
    const parsedAddr = await parseParsecAddr(clientInfo.value.organizationAddr);
    if (!parsedAddr.ok) {
      continue;
    }
    if (parsedAddr.value.hostname === orgInfo.server.hostname && parsedAddr.value.port === orgInfo.server.port) {
      return device.handle;
    }
  }
  return null;
}

export async function getOrganizationHandles(orgId: OrganizationID): Promise<Array<ConnectionHandle>> {
  return loggedInDevices.filter((item) => item.device.organizationId === orgId).map((item) => item.handle);
}

export async function listAvailableDevices(filter = true): Promise<Array<AvailableDevice>> {
  const availableDevices = (await libparsec.listAvailableDevices(window.getConfigDir())).map((d) => {
    d.createdOn = DateTime.fromSeconds(d.createdOn as any as number);
    d.protectedOn = DateTime.fromSeconds(d.protectedOn as any as number);
    return d;
  });

  if (!filter) {
    return availableDevices;
  }
  // Sort them by creation date
  const sortedDevices = availableDevices.sort((d1, d2) => d2.createdOn.toMillis() - d1.createdOn.toMillis());
  const devices: Array<AvailableDevice> = [];

  for (const ad of sortedDevices) {
    // If one is already in, it will have been created more recently since we ordered the devices by creation time
    const found = devices.find((d) => {
      return d.organizationId === ad.organizationId && ad.serverUrl === d.serverUrl && ad.humanHandle.email === d.humanHandle.email;
    });
    if (!found) {
      devices.push(ad);
    }
  }
  return devices;
}

export async function login(
  device: AvailableDevice,
  accessStrategy: DeviceAccessStrategy,
): Promise<Result<ConnectionHandle, ClientStartError>> {
  const info = loggedInDevices.find((info) => info.device.deviceId === device.deviceId);
  if (info !== undefined) {
    return { ok: true, value: info.handle };
  }

  // TODO: event handling has changed !
  const clientConfig = getClientConfig();
  const result = await libparsec.clientStart(clientConfig, accessStrategy);
  if (result.ok) {
    loggedInDevices.push({ handle: result.value, device: device, isExpired: false, isOnline: false, shouldAcceptTos: false });
  }
  return result;
}

export async function logout(handle?: ConnectionHandle | undefined | null): Promise<Result<null, ClientStopError>> {
  if (!handle) {
    handle = getParsecHandle();
  }

  if (handle !== null) {
    const result = await libparsec.clientStop(handle);
    if (result.ok) {
      const index = loggedInDevices.findIndex((info) => info.handle === handle);
      if (index !== -1) {
        loggedInDevices.splice(index, 1);
      }
    }
    return result;
  }
  return generateNoHandleError<ClientStopError>();
}

export async function getClientInfo(handle: ConnectionHandle | null = null): Promise<Result<ClientInfo, ClientInfoError>> {
  if (!handle) {
    handle = getConnectionHandle();
  }

  if (handle !== null) {
    return await libparsec.clientInfo(handle);
  }
  return generateNoHandleError<ClientInfoError>();
}

export async function getClientProfile(): Promise<UserProfile> {
  const result = await getClientInfo();

  if (result.ok) {
    return result.value.currentProfile;
  } else {
    // Outsider is the most restrictive
    return UserProfile.Outsider;
  }
}

export async function isAdmin(): Promise<boolean> {
  return (await getClientProfile()) === UserProfile.Admin;
}

export async function isOutsider(): Promise<boolean> {
  return (await getClientProfile()) === UserProfile.Outsider;
}

export interface CurrentAvailableDeviceError {
  tag: 'NotFound';
}

export async function getCurrentAvailableDevice(): Promise<Result<AvailableDevice, CurrentAvailableDeviceError>> {
  const clientResult = await getClientInfo();

  if (!clientResult.ok) {
    return { ok: false, error: { tag: 'NotFound' } };
  }
  const availableDevices = await listAvailableDevices(false);

  const currentAvailableDevice = availableDevices.find((device) => device.deviceId === clientResult.value.deviceId);
  if (!currentAvailableDevice) {
    return { ok: false, error: { tag: 'NotFound' } };
  }
  return { ok: true, value: currentAvailableDevice };
}

export async function updateDeviceChangeAuthentication(
  accessStrategy: DeviceAccessStrategy,
  saveStrategy: DeviceSaveStrategy,
): Promise<Result<AvailableDevice, UpdateDeviceError>> {
  const clientConfig = getClientConfig();
  return await libparsec.updateDeviceChangeAuthentication(clientConfig.configDir, accessStrategy, saveStrategy);
}

export async function isKeyringAvailable(): Promise<boolean> {
  return await libparsec.isKeyringAvailable();
}

export const AccessStrategy = {
  usePassword(device: AvailableDevice, password: string): DeviceAccessStrategyPassword {
    return {
      tag: DeviceAccessStrategyTag.Password,
      password: password,
      keyFile: device.keyFilePath,
    };
  },
  useKeyring(device: AvailableDevice): DeviceAccessStrategyKeyring {
    return {
      tag: DeviceAccessStrategyTag.Keyring,
      keyFile: device.keyFilePath,
    };
  },
};

export const SaveStrategy = {
  usePassword(password: string): DeviceSaveStrategyPassword {
    return {
      tag: DeviceSaveStrategyTag.Password,
      password: password,
    };
  },
  useKeyring(): DeviceSaveStrategyKeyring {
    return {
      tag: DeviceSaveStrategyTag.Keyring,
    };
  },
};

export async function isAuthenticationValid(device: AvailableDevice, accessStrategy: DeviceAccessStrategy): Promise<boolean> {
  const clientConfig = getClientConfig();
  const result = await libparsec.clientStart(clientConfig, accessStrategy);
  return result.ok;
}
