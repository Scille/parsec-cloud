// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import {
  libparsec,
  // Types
  RealmID,
  AvailableDevice,
  Result,
  Handle,
  ClientEvent,
  ClientEventPing,
  DeviceAccessStrategyPassword,
  ClientConfig,
  EntryName,
  InvitationToken,
  NewUserInvitationError,
  NewDeviceInvitationError,
  InviteListItemUser,
  UserClaimInProgress1Info,
  UserClaimInProgress2Info,
  UserClaimInProgress3Info,
  UserClaimFinalizeInfo,
  SASCode,
  HumanHandle,
  UserOrDeviceClaimInitialInfoUser,
  InvitationEmailSentStatus,
  DeviceFileType,
  ParsedBackendAddr,
  InvitationStatus,
  ClientStartError,
  ListInvitationsError,
  DeleteInvitationError,
  ClientListWorkspacesError,
  ClientWorkspaceCreateError,
  ClientStopError,
  ClaimerRetrieveInfoError,
  ClaimInProgressError,
  BootstrapOrganizationError,
  ParseBackendAddrError,
  OrganizationID,
  BackendAddr,
  InviteListItem,
} from '@/plugins/libparsec';
import { getParsecHandle } from '@/router/conditions';
import { DateTime } from 'luxon';

const DEFAULT_HANDLE = 42;

export type WorkspaceID = RealmID;
export type WorkspaceName = EntryName;

export interface UserInvitation extends InviteListItemUser {
  date: DateTime
}

async function _wait(delay: number): Promise<void> {
  return new Promise((res) => setTimeout(res, delay));
}

export async function listAvailableDevices(): Promise<Array<AvailableDevice>> {
  return await libparsec.listAvailableDevices(window.getConfigDir());
}

function getClientConfig(): ClientConfig {
  return {
    configDir: window.getConfigDir(),
    dataBaseDir: window.getDataBaseDir(),
    mountpointBaseDir: window.getMountpointDir(),
    workspaceStorageCacheSize: {tag: 'Default'},
  };
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
      resolve({ok: true, value: DEFAULT_HANDLE });
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

export async function inviteUser(email: string): Promise<Result<[InvitationToken, InvitationEmailSentStatus], NewUserInvitationError>> {
  const handle = getParsecHandle();

  if (handle !== null && window.isDesktop()) {
    return await libparsec.clientNewUserInvitation(handle, email, true);
  } else {
    return new Promise<Result<[InvitationToken, InvitationEmailSentStatus], NewUserInvitationError>>((resolve, _reject) => {
      resolve({ ok: true, value: ['1234', InvitationEmailSentStatus.Success] });
    });
  }
}

export async function listWorkspaces(): Promise<Result<Array<[WorkspaceID, WorkspaceName]>, ClientListWorkspacesError>> {
  const handle = getParsecHandle();

  if (handle !== null && window.isDesktop()) {
    return await libparsec.clientListWorkspaces(handle);
  } else {
    return new Promise<Result<Array<[WorkspaceID, WorkspaceName]>, ClientListWorkspacesError>>((resolve, _reject) => {
      resolve({
        ok: true, value: [
          ['1', 'Trademeet'],
          ['2', 'The Copper Coronet'],
          ['3', 'The Asylum'],
          ['4', 'Druid Grove'],
          // cspell:disable-next-line
          ['5', 'Menzoberranzan'],
        ],
      });
    });
  }
}

export async function createWorkspace(name: WorkspaceName): Promise<Result<WorkspaceID, ClientWorkspaceCreateError>> {
  const handle = getParsecHandle();

  if (handle !== null && window.isDesktop()) {
    return await libparsec.clientWorkspaceCreate(handle, name);
  } else {
    return new Promise<Result<WorkspaceID, ClientWorkspaceCreateError>>((resolve, _reject) => {
      resolve({ ok: true, value: '1337' });
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
        status: InvitationStatus.Ready,
        date: DateTime.now(),
      }, {
        tag: 'User',
        token: '5678',
        createdOn: DateTime.now().toISO() || '',
        claimerEmail: 'gale@waterdeep.faerun',
        status: InvitationStatus.Ready,
        date: DateTime.now(),
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

export class UserClaim {
  MOCK_HOST_WAITING_TIME = 500;

  handle: Handle | null;
  canceller: Handle | null;
  SASCodeChoices: SASCode[];
  correctSASCode: SASCode;
  guestSASCode: SASCode;
  device: AvailableDevice | null;
  greeter: HumanHandle | null;

  constructor() {
    this.handle = null;
    this.canceller = null;
    this.guestSASCode = '';
    this.correctSASCode = '';
    this.SASCodeChoices = [];
    this.device = null;
    this.greeter = null;
  }

  async abort(): Promise<void> {
    if (this.canceller !== null && window.isDesktop()) {
      libparsec.cancel(this.canceller);
    }
    if (this.handle !== null && window.isDesktop()) {
      libparsec.claimerGreeterAbortOperation(this.handle);
    }
    this.canceller = null;
    this.handle = null;
    this.guestSASCode = '';
    this.correctSASCode = '';
    this.SASCodeChoices = [];
    this.device = null;
    this.greeter = null;
  }

  _assertState(nullCanceller: boolean, nullHandle: boolean): void {
    if (nullCanceller && this.canceller !== null) {
      throw Error('Canceller should be null');
    } else if (!nullCanceller && this.canceller === null) {
      throw Error('Canceller should not be null');
    }
    if (nullHandle && this.handle !== null) {
      throw Error('Handle should be null');
    } else if (!nullHandle && this.handle === null) {
      throw Error('Handle should not be null');
    }
  }

  async retrieveInfo(invitationLink: string):
  Promise<Result<UserOrDeviceClaimInitialInfoUser, ClaimerRetrieveInfoError>> {
    function eventCallback(event: ClientEvent): void {
      console.log('On event', event);
    }

    this._assertState(true, true);

    if (window.isDesktop()) {
      const clientConfig = getClientConfig();
      const result = await libparsec.claimerRetrieveInfo(clientConfig, eventCallback, invitationLink);
      if (result.ok) {
        this.handle = result.value.handle;
        this.greeter = result.value.greeterHumanHandle;
      }
      return result as Result<UserOrDeviceClaimInitialInfoUser, ClaimerRetrieveInfoError>;
    } else {
      await _wait(this.MOCK_HOST_WAITING_TIME);
      this.handle = DEFAULT_HANDLE;
      this.greeter = {
        email: 'gale@waterdeep.faerun',
        // cspell:disable-next-line
        label: 'Gale Dekarios',
      };
      return new Promise<Result<UserOrDeviceClaimInitialInfoUser, ClaimerRetrieveInfoError>>((resolve, _reject) => {
        resolve({ok: true, value: {
          tag: 'User',
          handle: DEFAULT_HANDLE,
          claimerEmail: 'shadowheart@swordcoast.faerun',
          greeterUserId: '1234',
          greeterHumanHandle: {
            email: 'gale@waterdeep.faerun',
            // cspell:disable-next-line
            label: 'Gale Dekarios',
          },
        }});
      });
    }
  }

  async initialWaitHost():
  Promise<Result<UserClaimInProgress1Info, ClaimInProgressError>> {
    this._assertState(true, false);
    if (window.isDesktop()) {
      this.canceller = await libparsec.newCanceller();
      // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
      const result = await libparsec.claimerUserInitialDoWaitPeer(this.canceller, this.handle!);
      if (result.ok) {
        this.SASCodeChoices = result.value.greeterSasChoices;
        this.correctSASCode = result.value.greeterSas;
        this.handle = result.value.handle;
      }
      this.canceller = null;
      return result;
    } else {
      this.SASCodeChoices = ['1ABC', '2DEF', '3GHI', '4JKL'];
      this.correctSASCode = '2DEF';
      return new Promise<Result<UserClaimInProgress1Info, ClaimInProgressError>>((resolve, _reject) => {
        resolve({ok: true, value: {
          handle: DEFAULT_HANDLE,
          greeterSas: '2DEF',
          greeterSasChoices: ['1ABC', '2DEF', '3GHI', '4JKL'],
        }});
      });
    }
  }

  async signifyTrust():
  Promise<Result<UserClaimInProgress2Info, ClaimInProgressError>> {
    this._assertState(true, false);
    if (window.isDesktop()) {
      this.canceller = await libparsec.newCanceller();
      // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
      const result = await libparsec.claimerUserInProgress1DoSignifyTrust(this.canceller, this.handle!);
      if (result.ok) {
        this.guestSASCode = result.value.claimerSas;
        this.handle = result.value.handle;
      }
      this.canceller = null;
      return result;
    } else {
      this.guestSASCode = '1337';
      return new Promise<Result<UserClaimInProgress2Info, ClaimInProgressError>>((resolve, _reject) => {
        resolve({ok: true, value: {
          handle: DEFAULT_HANDLE,
          claimerSas: '1337',
        }});
      });
    }
  }

  async waitHostTrust():
  Promise<Result<UserClaimInProgress3Info, ClaimInProgressError>> {
    this._assertState(true, false);
    if (window.isDesktop()) {
      this.canceller = await libparsec.newCanceller();
      // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
      const result = await libparsec.claimerUserInProgress2DoWaitPeerTrust(this.canceller, this.handle!);
      this.canceller = null;
      if (result.ok) {
        this.handle = result.value.handle;
      }
      return result;
    } else {
      await _wait(this.MOCK_HOST_WAITING_TIME);
      return new Promise<Result<UserClaimInProgress3Info, ClaimInProgressError>>((resolve, _reject) => {
        resolve({ok: true, value: {
          handle: DEFAULT_HANDLE,
        }});
      });
    }
  }

  async doClaim(deviceLabel: string, userName: string, email: string):
  Promise<Result<UserClaimFinalizeInfo, ClaimInProgressError>> {
    this._assertState(true, false);
    if (window.isDesktop()) {
      this.canceller = await libparsec.newCanceller();
      const result = await libparsec.claimerUserInProgress3DoClaim(
        // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
        this.canceller, this.handle!, deviceLabel, {email: email, label: userName},
      );
      if (result.ok) {
        this.handle = result.value.handle;
      }
      this.canceller = null;
      return result;
    } else {
      await _wait(this.MOCK_HOST_WAITING_TIME);
      return new Promise<Result<UserClaimFinalizeInfo, ClaimInProgressError>>((resolve, _reject) => {
        resolve({ok: true, value: {
          handle: DEFAULT_HANDLE,
        }});
      });
    }
  }

  async finalize(password: string):
  Promise<Result<AvailableDevice, ClaimInProgressError>> {
    this._assertState(true, false);
    if (window.isDesktop()) {
      // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
      const result = await libparsec.claimerUserFinalizeSaveLocalDevice(this.handle!, {tag: 'Password', password: password});
      if (result.ok) {
        this.device = result.value;
      }
      this.handle = null;
      return result;
    } else {
      return new Promise<Result<AvailableDevice, ClaimInProgressError>>((resolve, _reject) => {
        this.device = {
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
        };
        resolve({ok: true, value: this.device});
      });
    }
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
    return await libparsec.bootstrapOrganization(
      config,
      parsecEventCallback,
      bootstrapAddr,
      {tag: 'Password', password: password},
      {label: userName, email: email},
      deviceLabel,
      null,
    );
  } else {
    await _wait(500);
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
