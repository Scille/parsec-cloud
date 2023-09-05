// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import {
  // Types
  RealmID,
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
  ClientListWorkspacesError,
  ClientWorkspaceCreateError,
} from '@/plugins/libparsec/definitions';
import { libparsec } from '@/plugins/libparsec';
import { getParsecHandle } from '@/router/conditions';
import { DateTime } from 'luxon';

const DEFAULT_HANDLE = 42;

export type WorkspaceID = RealmID;
export type WorkspaceName = EntryName;

export interface UserInvitation extends InviteListItemUser {
  date: DateTime
}

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

export async function listWorkspaces(): Promise<Result<Array<[WorkspaceID, WorkspaceName]>, ClientListWorkspacesError>> {
  const handle = getParsecHandle();

  if (handle !== null && window.isDesktop()) {
    return await libparsec.clientListWorkspaces(handle);
  } else {
    return new Promise<Result<Array<[WorkspaceID, WorkspaceName]>, ClientListWorkspacesError>>((resolve, _reject) => {
      resolve({ok: true, value: [
        ['1', 'Trademeet'],
        ['2', 'The Copper Coronet'],
        ['3', 'The Asylum'],
        ['4', 'Druid Grove'],
        // cspell:disable-next-line
        ['5', 'Menzoberranzan'],
      ]});
    });
  }
}

export async function createWorkspace(name: WorkspaceName): Promise<Result<WorkspaceID, ClientWorkspaceCreateError>> {
  const handle = getParsecHandle();

  if (handle !== null && window.isDesktop()) {
    return await libparsec.clientWorkspaceCreate(handle, name);
  } else {
    return new Promise<Result<WorkspaceID, ClientWorkspaceCreateError>>((resolve, _reject) => {
      resolve({ok: true, value: '1337'});
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

export async function listUserInvitations(): Promise<Result<Array<UserInvitation>, ListInvitationsError>> {
  const handle = getParsecHandle();

  if (handle !== null && window.isDesktop()) {
    const result = await libparsec.clientListInvitations(handle);

    if (!result.ok) {
      return result;
    }
    // No need to add device invitations
    result.value = result.value.filter((item) => item.tag === 'User');
    // Convert InviteListItemUser to UserInvitation
    result.value = result.value.map((item) => {
      (item as UserInvitation).date = DateTime.fromISO(item.createdOn);
      return item;
    });
    return result as any;
  } else {
    return new Promise<Result<Array<UserInvitation>, ListInvitationsError>>((resolve, _reject) => {
      const ret: Array<UserInvitation> = [{
        tag: 'User',
        token: '1234',
        createdOn: DateTime.now().toISO() || '',
        claimerEmail: 'shadowheart@swordcoast.faerun',
        status: {tag: 'Ready'},
        date: DateTime.now(),
      }, {
        tag: 'User',
        token: '5678',
        createdOn: DateTime.now().toISO() || '',
        claimerEmail: 'gale@waterdeep.faerun',
        status: {tag: 'Ready'},
        date: DateTime.now(),
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
