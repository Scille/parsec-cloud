// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import {
  ActiveUsersLimitTag,
  DeviceAccessStrategyKeyring,
  DeviceSaveStrategyKeyring,
  DeviceSaveStrategyPassword,
  libparsec,
} from '@/plugins/libparsec';

import { needsMocks } from '@/parsec/environment';
import { DEFAULT_HANDLE, getClientConfig } from '@/parsec/internals';
import { getParsecHandle } from '@/parsec/routing';
import {
  AvailableDevice,
  ClientChangeAuthenticationError,
  ClientChangeAuthenticationErrorTag,
  ClientEvent,
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
import { EventDistributor, Events } from '@/services/eventDistributor';

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

export async function getOrganizationHandles(orgId: OrganizationID): Promise<Array<ConnectionHandle>> {
  return loggedInDevices.filter((item) => item.device.organizationId === orgId).map((item) => item.handle);
}

export async function listAvailableDevices(): Promise<Array<AvailableDevice>> {
  return await libparsec.listAvailableDevices(window.getConfigDir());
}

export async function login(
  eventDistributor: EventDistributor,
  device: AvailableDevice,
  accessStrategy: DeviceAccessStrategy,
): Promise<Result<ConnectionHandle, ClientStartError>> {
  function parsecEventCallback(event: ClientEvent): void {
    console.log(event);
    switch (event.tag) {
      case ClientEventTag.Online:
        eventDistributor.dispatchEvent(Events.Online, {});
        break;
      case ClientEventTag.Offline:
        eventDistributor.dispatchEvent(Events.Offline, {});
        break;
      case ClientEventTag.InvitationChanged:
        eventDistributor.dispatchEvent(Events.InvitationUpdated, {
          token: (event as ClientEventInvitationChanged).token,
          status: (event as ClientEventInvitationChanged).status,
        });
        break;
      default:
        console.log(`Unhandled event ${event.tag}`);
        break;
    }
  }

  const info = loggedInDevices.find((info) => info.device.slug === device.slug);
  if (info !== undefined) {
    return { ok: true, value: info.handle };
  }

  if (!needsMocks()) {
    const clientConfig = getClientConfig();
    const result = await libparsec.clientStart(clientConfig, parsecEventCallback, accessStrategy);
    if (result.ok) {
      loggedInDevices.push({ handle: result.value, device: device });
    }
    return result;
  } else {
    if (
      accessStrategy.tag === DeviceAccessStrategyTag.Password &&
      ['P@ssw0rd.', 'AVeryL0ngP@ssw0rd'].includes((accessStrategy as DeviceAccessStrategyPassword).password)
    ) {
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

export async function changePassword(
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
