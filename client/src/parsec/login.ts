// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import {
  ActiveUsersLimitTag,
  DeviceAccessStrategyKeyring,
  DeviceFileType,
  DeviceSaveStrategyKeyring,
  DeviceSaveStrategyPassword,
  libparsec,
} from '@/plugins/libparsec';

import { needsMocks } from '@/parsec/environment';
import { DEFAULT_HANDLE, getClientConfig } from '@/parsec/internals';
import { parseParsecAddr } from '@/parsec/organization';
import { getParsecHandle } from '@/parsec/routing';
import {
  AvailableDevice,
  ClientChangeAuthenticationError,
  ClientChangeAuthenticationErrorTag,
  ClientEvent,
  ClientEventGreetingAttemptCancelled,
  ClientEventGreetingAttemptReady,
  ClientEventInvitationChanged,
  ClientEventTag,
  ClientInfo,
  ClientInfoError,
  ClientStartError,
  ClientStartErrorTag,
  ClientStopError,
  ConnectionHandle,
  DeviceAccessStrategy,
  DeviceAccessStrategyPassword,
  DeviceAccessStrategyTag,
  DeviceSaveStrategy,
  DeviceSaveStrategyTag,
  OrganizationID,
  Result,
  UserProfile,
} from '@/parsec/types';
import { getConnectionHandle } from '@/router';
import { EventDistributor, Events } from '@/services/eventDistributor';
import { DateTime } from 'luxon';

export interface LoggedInDeviceInfo {
  handle: ConnectionHandle;
  device: AvailableDevice;
  isExpired: boolean;
  isOnline: boolean;
  shouldAcceptTos: boolean;
}

const loggedInDevices: Array<LoggedInDeviceInfo> = [];

export function mockLoggedInDevice(): void {
  if (loggedInDevices.length === 0) {
    loggedInDevices.push({
      handle: DEFAULT_HANDLE,
      device: {
        keyFilePath: '/fake',
        createdOn: DateTime.now(),
        protectedOn: DateTime.now(),
        serverUrl: 'parsec3://127.0.0.1:6770?no_ssl=true',
        organizationId: 'MyOrg',
        userId: 'MockUserId',
        deviceId: 'MockDeviceId',
        humanHandle: {
          label: 'Gordon Freeman',
          email: 'gordon.freeman@blackmesa.nm',
        },
        deviceLabel: 'HEV Suit',
        ty: DeviceFileType.Password,
      },
      isExpired: false,
      isOnline: true,
      shouldAcceptTos: false,
    });
  }
}

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
  if (needsMocks()) {
    return {
      handle: DEFAULT_HANDLE,
      device: {} as AvailableDevice,
      isExpired: false,
      isOnline: true,
      shouldAcceptTos: false,
    };
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
  eventDistributor: EventDistributor,
  device: AvailableDevice,
  accessStrategy: DeviceAccessStrategy,
): Promise<Result<ConnectionHandle, ClientStartError>> {
  function parsecEventCallback(distributor: EventDistributor, event: ClientEvent, handle: ConnectionHandle): void {
    const connInfo = getConnectionInfo(handle);
    switch (event.tag) {
      case ClientEventTag.Online:
        if (connInfo) {
          connInfo.isOnline = true;
        }
        distributor.dispatchEvent(Events.Online);
        break;
      case ClientEventTag.Offline:
        if (connInfo) {
          connInfo.isOnline = false;
        }
        distributor.dispatchEvent(Events.Offline);
        break;
      case ClientEventTag.MustAcceptTos:
        if (connInfo) {
          connInfo.shouldAcceptTos = true;
        }
        distributor.dispatchEvent(Events.TOSAcceptRequired, undefined, { delay: 2000 });
        break;
      case ClientEventTag.InvitationChanged:
        distributor.dispatchEvent(Events.InvitationUpdated, {
          token: (event as ClientEventInvitationChanged).token,
          status: (event as ClientEventInvitationChanged).status,
        });
        break;
      case ClientEventTag.GreetingAttemptReady:
        distributor.dispatchEvent(Events.GreetingAttemptReady, {
          token: (event as ClientEventGreetingAttemptReady).token,
          greetingAttempt: (event as ClientEventGreetingAttemptReady).greetingAttempt,
        });
        break;
      case ClientEventTag.GreetingAttemptCancelled:
        distributor.dispatchEvent(Events.GreetingAttemptCancelled, {
          token: (event as ClientEventGreetingAttemptCancelled).token,
          greetingAttempt: (event as ClientEventGreetingAttemptCancelled).greetingAttempt,
        });
        break;
      case ClientEventTag.IncompatibleServer:
        window.electronAPI.log('warn', `IncompatibleServerEvent: ${JSON.stringify(event)}`);
        distributor.dispatchEvent(Events.IncompatibleServer, { reason: event.detail }, { delay: 5000 });
        break;
      case ClientEventTag.RevokedSelfUser:
        eventDistributor.dispatchEvent(Events.ClientRevoked);
        break;
      case ClientEventTag.ExpiredOrganization:
        if (connInfo) {
          connInfo.isExpired = true;
        }
        eventDistributor.dispatchEvent(Events.ExpiredOrganization);
        break;
      case ClientEventTag.WorkspaceLocallyCreated:
        eventDistributor.dispatchEvent(Events.WorkspaceCreated);
        break;
      case ClientEventTag.WorkspacesSelfListChanged:
        eventDistributor.dispatchEvent(Events.WorkspaceUpdated);
        break;
      case ClientEventTag.WorkspaceWatchedEntryChanged:
        eventDistributor.dispatchEvent(Events.EntryUpdated, undefined, { aggregateTime: 1000 });
        break;
      case ClientEventTag.WorkspaceOpsInboundSyncDone:
        eventDistributor.dispatchEvent(Events.EntrySynced, { workspaceId: event.realmId, entryId: event.entryId, way: 'inbound' });
        break;
      case ClientEventTag.WorkspaceOpsOutboundSyncDone:
        eventDistributor.dispatchEvent(Events.EntrySynced, { workspaceId: event.realmId, entryId: event.entryId, way: 'outbound' });
        break;
      case ClientEventTag.WorkspaceOpsOutboundSyncStarted:
        eventDistributor.dispatchEvent(Events.EntrySyncStarted, { workspaceId: event.realmId, entryId: event.entryId, way: 'outbound' });
        break;
      // Ignore those events for now
      case ClientEventTag.WorkspaceOpsOutboundSyncProgress:
      case ClientEventTag.ServerConfigChanged:
        break;
      default:
        window.electronAPI.log('debug', `Unhandled event ${event.tag}`);
        break;
    }
  }

  const info = loggedInDevices.find((info) => info.device.deviceId === device.deviceId);
  if (info !== undefined) {
    return { ok: true, value: info.handle };
  }

  const callback = (handle: ConnectionHandle, event: ClientEvent): void => {
    parsecEventCallback(eventDistributor, event, handle);
  };
  if (!needsMocks()) {
    const clientConfig = getClientConfig();
    const result = await libparsec.clientStart(clientConfig, callback, accessStrategy);
    if (result.ok) {
      loggedInDevices.push({ handle: result.value, device: device, isExpired: false, isOnline: false, shouldAcceptTos: false });
    }
    return result;
  } else {
    if (
      accessStrategy.tag === DeviceAccessStrategyTag.Password &&
      ['P@ssw0rd.', 'AVeryL0ngP@ssw0rd'].includes((accessStrategy as DeviceAccessStrategyPassword).password)
    ) {
      loggedInDevices.push({ handle: DEFAULT_HANDLE, device: device, isExpired: false, isOnline: true, shouldAcceptTos: false });
      callback(DEFAULT_HANDLE, { tag: ClientEventTag.Online });
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

export async function logout(handle?: ConnectionHandle | undefined | null): Promise<Result<null, ClientStopError>> {
  if (!handle) {
    handle = getParsecHandle();
  }

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

export async function getClientInfo(handle: ConnectionHandle | null = null): Promise<Result<ClientInfo, ClientInfoError>> {
  if (!handle) {
    handle = getConnectionHandle();
  }

  if (handle !== null && !needsMocks()) {
    return await libparsec.clientInfo(handle);
  } else {
    return {
      ok: true,
      value: {
        organizationAddr: 'parsec3://example.com/MyOrg',
        organizationId: 'MyOrg',
        deviceId: 'device1',
        deviceLabel: 'My First Device',
        userId: 'me',
        currentProfile: UserProfile.Admin,
        humanHandle: {
          email: 'user@host.com',
          label: 'Gordon Freeman',
        },
        serverConfig: {
          userProfileOutsiderAllowed: true,
          activeUsersLimit: {
            tag: ActiveUsersLimitTag.NoLimit,
          },
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

  if (!needsMocks()) {
    const currentAvailableDevice = availableDevices.find((device) => device.deviceId === clientResult.value.deviceId);
    if (!currentAvailableDevice) {
      return { ok: false, error: { tag: 'NotFound' } };
    }
    return { ok: true, value: currentAvailableDevice };
  } else {
    const device = availableDevices[0];
    // Uncomment this to experience the login as you would with keyring
    // device.ty = DeviceFileType.Keyring;
    return { ok: true, value: device };
  }
}

export async function changeAuthentication(
  accessStrategy: DeviceAccessStrategy,
  saveStrategy: DeviceSaveStrategy,
): Promise<Result<null, ClientChangeAuthenticationError>> {
  if (!needsMocks()) {
    const clientConfig = getClientConfig();
    return await libparsec.clientChangeAuthentication(clientConfig, accessStrategy, saveStrategy);
  } else {
    // Fake an error
    if (
      accessStrategy.tag === DeviceAccessStrategyTag.Password &&
      (accessStrategy as DeviceAccessStrategyPassword).password !== 'P@ssw0rd.'
    ) {
      return { ok: false, error: { tag: ClientChangeAuthenticationErrorTag.DecryptionFailed, error: 'Invalid password' } };
    }
    return { ok: true, value: null };
  }
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
  if (!needsMocks()) {
    const clientConfig = getClientConfig();
    const result = await libparsec.clientStart(clientConfig, (_handle: number, _event: ClientEvent) => {}, accessStrategy);
    return result.ok;
  } else {
    return (
      accessStrategy.tag === DeviceAccessStrategyTag.Password &&
      ['P@ssw0rd.', 'AVeryL0ngP@ssw0rd'].includes((accessStrategy as DeviceAccessStrategyPassword).password)
    );
  }
}
