// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import {
  ClientStartErrorTag,
  DeviceAccessStrategy,
  DeviceAccessStrategyTag,
  DeviceSaveStrategy,
  DeviceSaveStrategyTag,
  libparsec,
} from '@/plugins/libparsec';

import { needsMocks } from '@/parsec/environment';
import { DEFAULT_HANDLE, getClientConfig } from '@/parsec/internals';
import { getParsecHandle } from '@/parsec/routing';
import {
  AvailableDevice,
  ClientEvent,
  ClientInfo,
  ClientInfoError,
  ClientStartError,
  ClientStopError,
  ConnectionHandle,
  DeviceAccessStrategyPassword,
  Result,
  UserProfile,
} from '@/parsec/types';

export interface LoggedInDeviceInfo {
  handle: ConnectionHandle;
  device: AvailableDevice;
}

const loggedInDevices: Array<LoggedInDeviceInfo> = [];

export async function getLoggedInDevices(): Promise<Array<LoggedInDeviceInfo>> {
  return loggedInDevices;
}

export function isDeviceLoggedIn(device: AvailableDevice): boolean {
  return loggedInDevices.find((info) => info.device.slug === device.slug) !== undefined;
}

export function getDeviceHandle(device: AvailableDevice): ConnectionHandle | null {
  const info = loggedInDevices.find((info) => info.device.slug === device.slug);
  if (info) {
    return info.handle;
  }
  return null;
}

export async function listAvailableDevices(): Promise<Array<AvailableDevice>> {
  return await libparsec.listAvailableDevices(window.getConfigDir());
}

export async function login(device: AvailableDevice, password: string): Promise<Result<ConnectionHandle, ClientStartError>> {
  function parsecEventCallback(event: ClientEvent): void {
    console.log('Event received', event);
  }

  const info = loggedInDevices.find((info) => info.device.slug === device.slug);
  if (info !== undefined) {
    return { ok: true, value: info.handle };
  }

  if (!needsMocks()) {
    const clientConfig = getClientConfig();
    const strategy: DeviceAccessStrategyPassword = {
      tag: DeviceAccessStrategyTag.Password,
      password: password,
      keyFile: device.keyFilePath,
    };
    const result = await libparsec.clientStart(clientConfig, parsecEventCallback, strategy);
    if (result.ok) {
      loggedInDevices.push({ handle: result.value, device: device });
    }
    return result;
  } else {
    if (password === 'P@ssw0rd.' || password === 'AVeryL0ngP@ssw0rd') {
      loggedInDevices.push({ handle: DEFAULT_HANDLE, device: device });
      return { ok: true, value: DEFAULT_HANDLE };
    }
    return {
      ok: false,
      error: {
        tag: ClientStartErrorTag.LoadDeviceDecryptionFailed,
        error: 'WrongPassword',
      },
    };
  }
}

export async function logout(): Promise<Result<null, ClientStopError>> {
  const handle = getParsecHandle();

  if (handle !== null && !needsMocks()) {
    const result = await libparsec.clientStop(handle);
    if (result.ok) {
      const index = loggedInDevices.findIndex((info) => info.handle === handle);
      if (index !== -1) {
        loggedInDevices.splice(index, 1);
      }
    }
    return result;
  } else {
    const index = loggedInDevices.findIndex((info) => info.handle === handle);
    if (index !== -1) {
      loggedInDevices.splice(index, 1);
    }
    return { ok: true, value: null };
  }
}

export async function getClientInfo(): Promise<Result<ClientInfo, ClientInfoError>> {
  const handle = getParsecHandle();

  if (handle !== null && !needsMocks()) {
    return await libparsec.clientInfo(handle);
  } else {
    return {
      ok: true,
      value: {
        organizationAddr: 'parsec://example.com/MyOrg',
        organizationId: 'MyOrg',
        deviceId: 'device1',
        deviceLabel: 'My First Device',
        userId: 'me',
        currentProfile: UserProfile.Admin,
        humanHandle: {
          email: 'user@host.com',
          label: 'Gordon Freeman',
        },
      },
    };
  }
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
  const availableDevices = await listAvailableDevices();
  // clientInfo are not real on web right now
  // const currentAvailableDevice = availableDevices.find((device) => device.deviceId === clientResult.value.deviceId);
  const currentAvailableDevice = availableDevices[0];

  if (currentAvailableDevice) {
    return { ok: true, value: currentAvailableDevice };
  }
  return { ok: false, error: { tag: 'NotFound' } };
}

// Temporary, delete this when the bindings are available
export enum ChangeAuthErrorTag {
  DecryptionFailed = 'DecryptionFailed',
  Internal = 'Internal',
}

interface ChangeAuthInternalError {
  tag: ChangeAuthErrorTag.Internal;
}

interface ChangeAuthDecryptionFailedError {
  tag: ChangeAuthErrorTag.DecryptionFailed;
}

export type ChangeAuthError = ChangeAuthInternalError | ChangeAuthDecryptionFailedError;

// Trying to guess what the API will look like
export async function libParsecChangeAuthentication(
  _oldAuth: DeviceAccessStrategy,
  _newAuth: DeviceSaveStrategy,
): Promise<Result<null, ChangeAuthError>> {
  const handle = getParsecHandle();

  if (handle !== null && !needsMocks()) {
    return { ok: true, value: null };
  } else {
    return { ok: true, value: null };
  }
}

export async function changePassword(
  device: AvailableDevice,
  oldPassword: string,
  newPassword: string,
): Promise<Result<null, ChangeAuthError>> {
  // Fake an error
  if (oldPassword !== 'P@ssw0rd.') {
    return { ok: false, error: { tag: ChangeAuthErrorTag.DecryptionFailed } };
  }

  return await libParsecChangeAuthentication(
    {
      tag: DeviceAccessStrategyTag.Password,
      password: oldPassword,
      keyFile: device.keyFilePath,
    },
    {
      tag: DeviceSaveStrategyTag.Password,
      password: newPassword,
    },
  );
}
