// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { DevicePrimaryProtectionStrategyTag, libparsec, TOTPOpaqueKeyID, X509CertificateReference } from '@/plugins/libparsec';

import { ParsecAccount } from '@/parsec/account';
import { getClientConfig } from '@/parsec/internals';
import { parseParsecAddr } from '@/parsec/organization';
import {
  AccountHandle,
  AvailableDevice,
  AvailableDeviceTypeTag,
  ClientInfo,
  ClientInfoError,
  ClientStartError,
  ClientStopError,
  ConnectionHandle,
  DeviceAccessStrategy,
  DeviceID,
  DevicePrimaryProtectionStrategy,
  DevicePrimaryProtectionStrategyAccountVault,
  DevicePrimaryProtectionStrategyKeyring,
  DevicePrimaryProtectionStrategyOpenBao,
  DevicePrimaryProtectionStrategyPassword,
  DevicePrimaryProtectionStrategyPKI,
  DeviceSaveStrategy,
  ListAvailableDeviceError,
  OrganizationID,
  Result,
  SecretKey,
  UpdateDeviceError,
  UserProfile,
} from '@/parsec/types';
import { generateNoHandleError } from '@/parsec/utils';
import { getConnectionHandle } from '@/router';
import { OpenBaoConnectionInfo } from '@/services/openBao';
import { DateTime } from 'luxon';

export interface LoggedInDeviceInfo {
  handle: ConnectionHandle;
  device: AvailableDevice;
  isOrganizationExpired: boolean;
  isOnline: boolean;
  shouldAcceptTos: boolean;
}

export async function getLoggedInDevices(): Promise<Array<LoggedInDeviceInfo>> {
  const availableDevices = await listAvailableDevices(false);
  const startedDevices = await libparsec.listStartedClients();
  const loggedInDevices: Array<LoggedInDeviceInfo> = [];

  for (const [handle, deviceId] of startedDevices) {
    const device = availableDevices.find((d) => d.deviceId === deviceId);
    if (!device) {
      continue;
    }
    const clientInfoResult = await libparsec.clientInfo(handle);
    if (!clientInfoResult.ok) {
      continue;
    }
    loggedInDevices.push({
      handle: handle,
      device: device,
      isOnline: clientInfoResult.value.isServerOnline,
      shouldAcceptTos: clientInfoResult.value.mustAcceptTos,
      isOrganizationExpired: clientInfoResult.value.isOrganizationExpired,
    });
  }
  return loggedInDevices;
}

export async function isDeviceLoggedIn(device: AvailableDevice): Promise<boolean> {
  const startedDevices = await libparsec.listStartedClients();

  return startedDevices.find(([_handle, deviceId]) => deviceId === device.deviceId) !== undefined;
}

export async function getDeviceHandle(device: AvailableDevice): Promise<ConnectionHandle | undefined> {
  const startedDevices = await libparsec.listStartedClients();
  return startedDevices.find(([_handle, deviceId]) => deviceId === device.deviceId)?.[0];
}

export async function listStartedClients(): Promise<Array<[ConnectionHandle, DeviceID]>> {
  return await libparsec.listStartedClients();
}

interface OrganizationInfo {
  server?: {
    hostname: string;
    port: number;
  };
  id: OrganizationID;
}

export async function getOrganizationHandle(orgInfo: OrganizationInfo): Promise<ConnectionHandle | null> {
  const loggedInDevices = await getLoggedInDevices();
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
  const loggedInDevices = await getLoggedInDevices();
  return loggedInDevices.filter((item) => item.device.organizationId === orgId).map((item) => item.handle);
}

export async function listAvailableDevicesWithError(filter = true): Promise<Result<Array<AvailableDevice>, ListAvailableDeviceError>> {
  const result = await libparsec.listAvailableDevices(window.getConfigDir());

  if (!result.ok) {
    return result;
  }
  if (!ParsecAccount.isLoggedIn()) {
    result.value = result.value.filter((device) => device.ty.tag !== AvailableDeviceTypeTag.AccountVault);
  }
  const availableDevices = result.value.map((d) => {
    d.createdOn = DateTime.fromSeconds(d.createdOn as any as number);
    d.protectedOn = DateTime.fromSeconds(d.protectedOn as any as number);
    return d;
  });

  if (!filter) {
    return { ok: true, value: availableDevices };
  }
  // Sort them by creation date
  const sortedDevices = availableDevices.sort((d1, d2) => d2.createdOn.toMillis() - d1.createdOn.toMillis());
  const devices: Array<AvailableDevice> = [];

  for (const ad of sortedDevices) {
    // If one is already in, it will have been created more recently since we ordered the devices by creation time
    const found = devices.find((d) => {
      return d.organizationId === ad.organizationId && ad.serverAddr === d.serverAddr && ad.humanHandle.email === d.humanHandle.email;
    });
    if (!found) {
      devices.push(ad);
    }
  }
  return { ok: true, value: devices };
}

export async function listAvailableDevices(filter = true): Promise<Array<AvailableDevice>> {
  const result = await listAvailableDevicesWithError(filter);

  return result.ok ? result.value : [];
}

export async function login(
  device: AvailableDevice,
  accessStrategy: DeviceAccessStrategy,
): Promise<Result<ConnectionHandle, ClientStartError>> {
  const startedClients = await libparsec.listStartedClients();
  const foundHandle = startedClients.find(([_handle, deviceId]) => deviceId === device.deviceId)?.[0];
  if (foundHandle !== undefined) {
    return { ok: true, value: foundHandle };
  }

  const clientConfig = getClientConfig();
  return await libparsec.clientStart(clientConfig, accessStrategy);
}

export async function logout(handle?: ConnectionHandle | undefined | null): Promise<Result<null, ClientStopError>> {
  if (!handle) {
    handle = getConnectionHandle();
  }

  if (handle !== null) {
    return await libparsec.clientStop(handle);
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

export const PrimaryProtectionStrategy = {
  usePassword(password: string): DevicePrimaryProtectionStrategyPassword {
    return {
      tag: DevicePrimaryProtectionStrategyTag.Password,
      password: password,
    };
  },
  useKeyring(): DevicePrimaryProtectionStrategyKeyring {
    return {
      tag: DevicePrimaryProtectionStrategyTag.Keyring,
    };
  },
  useSmartcard(certificateRef: X509CertificateReference): DevicePrimaryProtectionStrategyPKI {
    return {
      tag: DevicePrimaryProtectionStrategyTag.PKI,
      certificateRef: certificateRef,
    };
  },
  useOpenBao(connInfo: OpenBaoConnectionInfo): DevicePrimaryProtectionStrategyOpenBao {
    return {
      tag: DevicePrimaryProtectionStrategyTag.OpenBao,
      openbaoServerUrl: connInfo.server,
      openbaoSecretMountPath: connInfo.secretMountpoint,
      openbaoTransitMountPath: connInfo.transitMountpoint,
      openbaoEntityId: connInfo.userId,
      openbaoAuthToken: connInfo.token,
      openbaoPreferredAuthId: connInfo.provider,
    };
  },
  useAccountVault(accountHandle: AccountHandle): DevicePrimaryProtectionStrategyAccountVault {
    return {
      tag: DevicePrimaryProtectionStrategyTag.AccountVault,
      accountHandle: accountHandle,
    };
  },
};

export function constructSaveStrategy(
  primaryProtection: DevicePrimaryProtectionStrategy,
  totp: [TOTPOpaqueKeyID, SecretKey] | null = null,
): DeviceSaveStrategy {
  return {
    totpProtection: totp,
    primaryProtection: primaryProtection,
  };
}

export function constructAccessStrategy(
  device: AvailableDevice,
  primaryProtection: DevicePrimaryProtectionStrategy,
  totp: [TOTPOpaqueKeyID, SecretKey] | null = null,
): DeviceAccessStrategy {
  return {
    keyFile: device.keyFilePath,
    totpProtection: totp,
    primaryProtection: primaryProtection,
  };
}

export async function isAuthenticationValid(_device: AvailableDevice, accessStrategy: DeviceAccessStrategy): Promise<boolean> {
  const clientConfig = getClientConfig();
  const result = await libparsec.clientStart(clientConfig, accessStrategy);
  return result.ok;
}
