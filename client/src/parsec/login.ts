// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { ClientStartErrorTag, DeviceAccessStrategyTag, libparsec } from '@/plugins/libparsec';

import {
  AvailableDevice,
  DeviceAccessStrategyPassword,
  ClientEvent,
  Handle,
  ClientStartError,
  Result,
  ClientStopError,
  ClientInfo,
  ClientInfoError,
  UserProfile,
} from '@/parsec/types';
import { getParsecHandle } from '@/parsec/routing';
import { DEFAULT_HANDLE, getClientConfig } from '@/parsec/internals';
import { needsMocks } from '@/parsec/environment';

export async function listAvailableDevices(): Promise<Array<AvailableDevice>> {
  return await libparsec.listAvailableDevices(window.getConfigDir());
}

export async function login(device: AvailableDevice, password: string): Promise<Result<Handle, ClientStartError>> {
  function parsecEventCallback(event: ClientEvent): void {
    console.log('Event received', event);
  }

  if (!needsMocks()) {
    const clientConfig = getClientConfig();
    const strategy: DeviceAccessStrategyPassword = {
      tag: DeviceAccessStrategyTag.Password,
      password: password,
      keyFile: device.keyFilePath,
    };
    return await libparsec.clientStart(clientConfig, parsecEventCallback, strategy);
  } else {
    if (password === 'P@ssw0rd.' || password === 'AVeryL0ngP@ssw0rd') {
      return {ok: true, value: DEFAULT_HANDLE };
    }
    return {ok: false, error: {tag: ClientStartErrorTag.LoadDeviceDecryptionFailed, error: 'WrongPassword'}};
  }
}

export async function logout(): Promise<Result<null, ClientStopError>> {
  const handle = getParsecHandle();

  if (handle !== null && !needsMocks()) {
    return await libparsec.clientStop(handle);
  } else {
    return {ok: true, value: null};
  }
}

export async function getClientInfo(): Promise<Result<ClientInfo, ClientInfoError>> {
  const handle = getParsecHandle();

  if (handle !== null && !needsMocks()) {
    return await libparsec.clientInfo(handle);
  } else {
    return {ok: true, value: {
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
    }};
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
  return await getClientProfile() === UserProfile.Admin;
}

export async function isOutsider(): Promise<boolean> {
  return await getClientProfile() === UserProfile.Outsider;
}
