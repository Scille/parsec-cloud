// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import {
  libparsec,
  InviteListItem,
} from '@/plugins/libparsec';

import {
  AvailableDevice,
  ClientConfig,
  DeviceAccessStrategyPassword,
  ClientEvent,
  Handle,
  ClientStartError,
  Result,
  ClientStopError,
  UserInvitation,
  InvitationToken,
  InvitationEmailSentStatus,
  NewUserInvitationError,
  InvitationStatus,
  ListInvitationsError,
  NewDeviceInvitationError,
  DeleteInvitationError,
  BootstrapOrganizationError,
  OrganizationID,
  DeviceFileType,
  ParsedBackendAddr,
  BackendAddr,
  ClientEventPing,
  ParseBackendAddrError,
  UserInfo,
  ClientInfo,
  ClientInfoError,
  UserProfile,
  ClientListUsersError,
  DeviceInfo,
  ClientListUserDevicesError,
  UserID,
  CreateOrganizationError,
  NewInvitationInfo,
} from '@/parsec/types';
import { getParsecHandle } from '@/parsec/routing';
import { DateTime } from 'luxon';
import { DEFAULT_HANDLE, MOCK_WAITING_TIME, getClientConfig, wait } from '@/parsec/internals';

export async function listAvailableDevices(): Promise<Array<AvailableDevice>> {
  return await libparsec.listAvailableDevices(window.getConfigDir());
}

export async function login(device: AvailableDevice, password: string): Promise<Result<Handle, ClientStartError>> {
  function parsecEventCallback(event: ClientEvent): void {
    console.log('Event received', event);
  }

  if (window.isDesktop()) {
    const clientConfig = getClientConfig();
    const strategy: DeviceAccessStrategyPassword = {
      tag: 'Password',
      password: password,
      keyFile: device.keyFilePath,
    };
    return await libparsec.clientStart(clientConfig, parsecEventCallback, strategy);
  } else {
    return new Promise<Result<Handle, ClientStartError>>((resolve, _reject) => {
      if (password === 'P@ssw0rd.' || password === 'AVeryL0ngP@ssw0rd') {
        resolve({ok: true, value: DEFAULT_HANDLE });
      }
      resolve({ok: false, error: {tag: 'LoadDeviceDecryptionFailed', error: 'WrongPassword'}});
    });
  }
}

export async function logout(): Promise<Result<null, ClientStopError>> {
  const handle = getParsecHandle();

  if (handle !== null && window.isDesktop()) {
    return await libparsec.clientStop(handle);
  } else {
    return new Promise<Result<null, ClientStopError>>((resolve, _reject) => {
      resolve({ok: true, value: null});
    });
  }
}

export async function inviteUser(email: string): Promise<Result<NewInvitationInfo, NewUserInvitationError>> {
  const handle = getParsecHandle();

  if (handle !== null && window.isDesktop()) {
    return await libparsec.clientNewUserInvitation(handle, email, true);
  } else {
    return { ok: true, value: {
      token: '12346565645645654645645645645645',
      emailSentStatus: InvitationEmailSentStatus.Success,
      addr: 'parsec://parsec.example.com/Org',
    }};
  }
}

export async function inviteDevice(sendEmail: boolean):
  Promise<Result<[InvitationToken, InvitationEmailSentStatus], NewDeviceInvitationError>> {
  const handle = getParsecHandle();

  if (handle !== null && window.isDesktop()) {
    const ret = await libparsec.clientNewDeviceInvitation(handle, sendEmail);
    if (ret.ok) {
      return {ok: true, value: [ret.value.token, ret.value.emailSentStatus] };
    } else {
      return ret;
    }
  }
  return new Promise<Result<[InvitationToken, InvitationEmailSentStatus], NewDeviceInvitationError>>((resolve, _reject) => {
    resolve({ ok: true, value: ['1234', InvitationEmailSentStatus.Success] });
  });
}

export async function listUserInvitations(): Promise<Result<Array<UserInvitation>, ListInvitationsError>> {
  const handle = getParsecHandle();

  if (handle !== null && window.isDesktop()) {
    const result = await libparsec.clientListInvitations(handle);

    if (!result.ok) {
      return result;
    }
    // No need to add device invitations
    result.value = result.value.filter((item: InviteListItem) => item.tag === 'User');
    // Convert InviteListItemUser to UserInvitation
    result.value = result.value.map((item) => {
      item.createdOn = DateTime.fromSeconds(item.createdOn as any as number);
      return item;
    });
    return result as any;
  } else {
    return new Promise<Result<Array<UserInvitation>, ListInvitationsError>>((resolve, _reject) => {
      const ret: Array<UserInvitation> = [{
        tag: 'User',
        addr: 'parsec://parsec.example.com/MyOrg?action=claim_device&token=12346565645645654645645645645645',
        token: '12346565645645654645645645645645',
        createdOn: DateTime.now(),
        claimerEmail: 'shadowheart@swordcoast.faerun',
        status: InvitationStatus.Ready,
      }, {
        tag: 'User',
        addr: 'parsec://parsec.example.com/MyOrg?action=claim_user&token=32346565645645654645645645645645',
        token: '32346565645645654645645645645645',
        createdOn: DateTime.now(),
        claimerEmail: 'gale@waterdeep.faerun',
        status: InvitationStatus.Ready,
      }];
      resolve({ ok: true, value: ret });
    });
  }
}

export async function cancelInvitation(token: InvitationToken): Promise<Result<null, DeleteInvitationError>> {
  const handle = getParsecHandle();

  if (handle !== null && window.isDesktop()) {
    return await libparsec.clientDeleteInvitation(handle, token);
  } else {
    return new Promise<Result<null, DeleteInvitationError>>((resolve, _reject) => {
      resolve({ok: true, value: null});
    });
  }
}

export async function createOrganization(
  backendAddr: BackendAddr, orgName: OrganizationID, userName: string, email: string, password: string, deviceLabel: string,
): Promise<Result<AvailableDevice, BootstrapOrganizationError>> {
  function parsecEventCallback(event: ClientEventPing): void {
    console.log('On event', event);
  }

  const bootstrapAddr = await libparsec.buildBackendOrganizationBootstrapAddr(backendAddr, orgName);

  if (window.isDesktop()) {
    const config: ClientConfig = {
      configDir: window.getConfigDir(),
      dataBaseDir: window.getDataBaseDir(),
      mountpointBaseDir: window.getMountpointDir(),
      workspaceStorageCacheSize: {tag: 'Default'},
    };
    const result = await libparsec.bootstrapOrganization(
      config,
      parsecEventCallback,
      bootstrapAddr,
      {tag: 'Password', password: password},
      {label: userName, email: email},
      deviceLabel,
      null,
    );
    if (!result.ok && result.error.tag === CreateOrganizationError.BadTimestamp) {
      result.error.clientTimestamp = DateTime.fromSeconds(result.error.clientTimestamp as any as number);
      result.error.serverTimestamp = DateTime.fromSeconds(result.error.serverTimestamp as any as number);
    }
    return result;
  } else {
    await wait(MOCK_WAITING_TIME);
    return new Promise<Result<AvailableDevice, BootstrapOrganizationError>>((resolve, _reject) => {
      resolve({ok: true, value: {
        keyFilePath: '/path',
        organizationId: 'MyOrg',
        deviceId: 'deviceid',
        humanHandle: {
          label: 'A',
          email: 'a@b.c',
        },
        deviceLabel: 'a@b',
        slug: 'slug',
        ty: DeviceFileType.Password,
      }});
    });
  }
}

export async function parseBackendAddr(addr: string): Promise<Result<ParsedBackendAddr, ParseBackendAddrError>> {
  return await libparsec.parseBackendAddr(addr);
}

export async function getClientInfo(): Promise<Result<ClientInfo, ClientInfoError>> {
  const handle = getParsecHandle();

  if (handle !== null && window.isDesktop()) {
    return await libparsec.clientInfo(handle);
  } else {
    return new Promise<Result<ClientInfo, ClientInfoError>>((resolve, _reject) => {
      resolve({ok: true, value: {
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
      }});
    });
  }
}

export async function listUsers(skipRevoked = true): Promise<Result<Array<UserInfo>, ClientListUsersError>> {
  const handle = getParsecHandle();

  if (handle !== null && window.isDesktop()) {
    const result = await libparsec.clientListUsers(handle, skipRevoked);
    if (result.ok) {
      result.value.map((item) => {
        item.createdOn = DateTime.fromSeconds(item.createdOn as any as number);
        if (item.revokedOn) {
          item.revokedOn = DateTime.fromSeconds(item.revokedOn as any as number);
        }
        (item as UserInfo).isRevoked = (): boolean => item.revokedOn !== null;
        return item;
      });
    }
    return result as any as Promise<Result<Array<UserInfo>, ClientListUsersError>>;
  } else {
    return new Promise<Result<Array<UserInfo>, ClientListUsersError>>((resolve, _reject) => {
      const value: Array<UserInfo> = [{
        id: 'id1',
        // cspell:disable-next-line
        humanHandle: {label: 'Cernd', email: 'cernd@gmail.com'},
        currentProfile: UserProfile.Standard,
        createdOn: DateTime.now(),
        createdBy: 'device',
        revokedOn: null,
        revokedBy: null,
        isRevoked: (): boolean => false,
      }, {
        id: 'id2',
        // cspell:disable-next-line
        humanHandle: {label: 'Jaheira', email: 'jaheira@gmail.com'},
        currentProfile: UserProfile.Admin,
        createdOn: DateTime.now(),
        createdBy: 'device',
        revokedOn: null,
        revokedBy: null,
        isRevoked: (): boolean => false,
      }, {
        id: 'me',
        humanHandle: {
          email: 'user@host.com',
          label: 'Gordon Freeman',
        },
        currentProfile: UserProfile.Admin,
        createdOn: DateTime.now(),
        createdBy: 'device',
        revokedOn: null,
        revokedBy: null,
        isRevoked: (): boolean => false,
      }];
      if (!skipRevoked) {
        value.push({
          id: 'id3',
          // cspell:disable-next-line
          humanHandle: {label: 'Valygar Corthala', email: 'val@gmail.com'},
          currentProfile: UserProfile.Standard,
          createdOn: DateTime.now(),
          createdBy: 'device',
          revokedOn: DateTime.now(),
          revokedBy: 'device',
          isRevoked: (): boolean => true,
        });
      }
      resolve({ok: true, value: value});
    });
  }
}

export async function listUserDevices(user: UserID): Promise<Result<Array<DeviceInfo>, ClientListUserDevicesError>> {
  const handle = getParsecHandle();

  if (handle !== null && window.isDesktop()) {
    const result = await libparsec.clientListUserDevices(handle, user);
    if (result.ok) {
      result.value.map((item) => {
        item.createdOn = DateTime.fromSeconds(item.createdOn as any as number);
        return item;
      });
    }
    return result as any as Promise<Result<Array<DeviceInfo>, ClientListUserDevicesError>>;
  } else {
    return new Promise<Result<Array<DeviceInfo>, ClientListUserDevicesError>>((resolve, _reject) => {
      resolve({ok: true, value: [{
        id: 'device1',
        deviceLabel: 'My First Device',
        createdOn: DateTime.now(),
        createdBy: 'some_device',
      }, {
        id: 'device2',
        deviceLabel: 'My Second Device',
        createdOn: DateTime.now(),
        createdBy: 'device1',
      }]});
    });
  }
}
