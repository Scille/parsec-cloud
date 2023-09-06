// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import {
  // Types
  EntryID,
  AvailableDevice,
  Result,
  Handle,
  ClientEvent,
  DeviceAccessStrategyPassword,
  ClientConfig,
  EntryName,
  InvitationToken,
  InvitationEmailSentStatus,
  NewUserInvitationError,
  NewDeviceInvitationError,
  InviteListItemUser,
  // Errors
  ClientStartError,
  ListInvitationsError,
  DeleteInvitationError,
} from '@/plugins/libparsec/definitions';
import { libparsec } from '@/plugins/libparsec';
import { getParsecHandle } from '@/router/conditions';
import { DateTime } from 'luxon';

const DEFAULT_HANDLE = 42;

export type WorkspaceID = EntryID;
export type WorkspaceName = EntryName;

export async function listAvailableDevices(): Promise<Array<AvailableDevice>> {
  return await libparsec.listAvailableDevices(window.getConfigDir());
}

export async function login(device: AvailableDevice, password: string): Promise<Result<Handle, ClientStartError>> {
  function parsecEventCallback(event: ClientEvent): void {
    console.log('Event received', event);
  }

  if (window.isDesktop()) {
    const clientConfig: ClientConfig = {
      configDir: window.getConfigDir(),
      dataBaseDir: window.getDataBaseDir(),
      mountpointBaseDir: window.getMountpointDir(),
      workspaceStorageCacheSize: {tag: 'Default'},
    };
    const strategy: DeviceAccessStrategyPassword = {
      tag: 'Password',
      password: password,
      keyFile: device.keyFilePath,
    };
    return await libparsec.clientStart(clientConfig, parsecEventCallback, strategy);
  } else {
    return new Promise<Result<Handle, ClientStartError>>((resolve, _reject) => {
      resolve({ok: true, value: DEFAULT_HANDLE });
    });
  }
}

export async function inviteUser(email: string): Promise<Result<[InvitationToken, InvitationEmailSentStatus], NewUserInvitationError>> {
  const handle = getParsecHandle();

  if (handle !== null && window.isDesktop()) {
    return await libparsec.clientNewUserInvitation(handle, email, true);
  } else {
    return new Promise<Result<[InvitationToken, InvitationEmailSentStatus], NewUserInvitationError>>((resolve, _reject) => {
      resolve({ok: true, value: ['1234', {tag: 'Success'}]});
    });
  }
}

export async function inviteDevice(sendEmail: boolean):
  Promise<Result<[InvitationToken, InvitationEmailSentStatus], NewDeviceInvitationError>> {
  const handle = getParsecHandle();

  if (handle !== null && window.isDesktop()) {
    return await libparsec.clientNewDeviceInvitation(handle, sendEmail);
  }
  return new Promise<Result<[InvitationToken, InvitationEmailSentStatus], NewDeviceInvitationError>>((resolve, _reject) => {
    resolve({ok: true, value: ['1234', {tag: 'Success'}]});
  });
}

export async function listUserInvitations(): Promise<Result<Array<InviteListItemUser>, ListInvitationsError>> {
  const handle = getParsecHandle();

  if (handle !== null && window.isDesktop()) {
    const result = await libparsec.clientListInvitations(handle);

    if (!result.ok) {
      return result;
    }
    // No need to add device invitations
    result.value = result.value.filter((item) => item.tag === 'User');
    return result as any;
  } else {
    return new Promise<Result<Array<InviteListItemUser>, ListInvitationsError>>((resolve, _reject) => {
      const ret: Array<InviteListItemUser> = [{
        tag: 'User',
        token: '1234',
        createdOn: DateTime.now().toFormat('yyyy/mm/dd'),
        claimerEmail: 'shadowheart@swordcoast.faerun',
        status: {tag: 'Ready'},
      }, {
        tag: 'User',
        token: '5678',
        createdOn: DateTime.now().toFormat('yyyy/mm/dd'),
        claimerEmail: 'gale@waterdeep.faerun',
        status: {tag: 'Ready'},

      }];
      resolve({ok: true, value: ret});
    });
  }
}

export async function cancelInvitation(token: InvitationToken): Promise<Result<null, DeleteInvitationError>> {
  const handle = getParsecHandle();

  if (handle !== null && window.isDesktop()) {
    return await libparsec.clientDeleteInvitation(handle, token);
  }
  return new Promise<Result<null, DeleteInvitationError>>((resolve, _reject) => {
    resolve({ok: true, value: null});
  });
}
