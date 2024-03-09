// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { needsMocks } from '@/parsec/environment';
import { getClientInfo } from '@/parsec/login';
import { getParsecHandle } from '@/parsec/routing';
import {
  ClientCreateWorkspaceError,
  ClientListWorkspaceUsersError,
  ClientListWorkspacesError,
  ClientShareWorkspaceError,
  ClientStartWorkspaceError,
  ConnectionHandle,
  LinkError,
  MountpointHandle,
  ParsecOrganizationFileLinkAddr,
  Result,
  StartedWorkspaceInfo,
  SystemPath,
  UserID,
  UserProfile,
  UserTuple,
  WorkspaceHandle,
  WorkspaceID,
  WorkspaceInfo,
  WorkspaceInfoError,
  WorkspaceInfoErrorTag,
  WorkspaceMountError,
  WorkspaceName,
  WorkspaceRole,
} from '@/parsec/types';
import { WorkspaceStopError, libparsec } from '@/plugins/libparsec';
import { DateTime } from 'luxon';

export async function initializeWorkspace(
  workspaceId: WorkspaceID,
  connectionHandle?: ConnectionHandle,
): Promise<Result<StartedWorkspaceInfo, WorkspaceInfoError | ClientStartWorkspaceError | WorkspaceMountError>> {
  const startResult = await startWorkspace(workspaceId, connectionHandle);
  if (!startResult.ok) {
    console.error(`Failed to start workspace ${workspaceId}: ${startResult.error}`);
    return startResult;
  }

  const startedWorkspaceResult = await getWorkspaceInfo(startResult.value);
  if (!startedWorkspaceResult.ok) {
    console.error(`Failed to get started workspace info ${workspaceId}: ${startedWorkspaceResult.error}`);
    return startedWorkspaceResult;
  }
  let mountpoint: [MountpointHandle, SystemPath] | undefined = undefined;
  if (startedWorkspaceResult.value.mountpoints.length === 0) {
    const mountResult = await mountWorkspace(startResult.value);
    if (!mountResult.ok) {
      console.error(`Failed to mount workspace ${workspaceId}: ${mountResult.error}`);
      return mountResult;
    }
    mountpoint = mountResult.value;
  } else {
    mountpoint = startedWorkspaceResult.value.mountpoints[0];
  }
  startedWorkspaceResult.value.mountpoints.push(mountpoint);
  return startedWorkspaceResult;
}

export async function listWorkspaces(): Promise<Result<Array<WorkspaceInfo>, ClientListWorkspacesError>> {
  const handle = getParsecHandle();

  if (handle !== null && !needsMocks()) {
    const result = await libparsec.clientListWorkspaces(handle);

    if (result.ok) {
      const returnValue: Array<WorkspaceInfo> = [];
      for (const wkInfo of result.value) {
        const initResult = await initializeWorkspace(wkInfo.id);
        if (!initResult.ok) {
          continue;
        }

        const sharingResult = await getWorkspaceSharing(wkInfo.id, false);
        const info: WorkspaceInfo = {
          id: wkInfo.id,
          currentName: wkInfo.currentName,
          currentSelfRole: wkInfo.currentSelfRole,
          isStarted: true,
          isBootstrapped: wkInfo.isBootstrapped,
          sharing: [],
          size: 0,
          lastUpdated: DateTime.now(),
          availableOffline: false,
          mountpointHandle: initResult.value.mountpoints[0][0],
          mountpointPath: initResult.value.mountpoints[0][1],
          handle: initResult.value.handle,
        };
        if (sharingResult.ok) {
          info.sharing = sharingResult.value;
        }
        returnValue.push(info);
      }
      return { ok: true, value: returnValue };
    } else {
      return result;
    }
  } else {
    const value: Array<WorkspaceInfo> = [
      {
        id: '1',
        currentName: 'Trademeet',
        currentSelfRole: WorkspaceRole.Owner,
        size: 934_583,
        lastUpdated: DateTime.now().minus(2000),
        availableOffline: false,
        isStarted: false,
        isBootstrapped: false,
        sharing: [],
        mountpointHandle: 1,
        mountpointPath: '/home/a',
        handle: 1,
      },
      {
        id: '2',
        currentName: 'The Copper Coronet',
        currentSelfRole: WorkspaceRole.Manager,
        size: 3_489_534_274,
        lastUpdated: DateTime.now(),
        availableOffline: false,
        isStarted: false,
        isBootstrapped: true,
        sharing: [],
        mountpointHandle: 2,
        mountpointPath: '/home/b',
        handle: 2,
      },
      {
        id: '3',
        currentName: "Watcher's Keep",
        currentSelfRole: WorkspaceRole.Reader,
        size: 56_153_023,
        lastUpdated: DateTime.now(),
        availableOffline: true,
        isStarted: false,
        isBootstrapped: true,
        sharing: [],
        mountpointHandle: 3,
        mountpointPath: '/home/c',
        handle: 3,
      },
    ];

    for (let i = 0; i < value.length; i++) {
      const result = await getWorkspaceSharing(value[i].id, false);
      if (result.ok) {
        value[i].sharing = result.value;
      }
    }

    return { ok: true, value: value };
  }
}

export async function getWorkspaceInfo(workspaceHandle: WorkspaceHandle): Promise<Result<StartedWorkspaceInfo, WorkspaceInfoError>> {
  if (!needsMocks()) {
    const result = await libparsec.workspaceInfo(workspaceHandle);
    if (result.ok) {
      (result.value as StartedWorkspaceInfo).handle = workspaceHandle;
    }
    return result as Result<StartedWorkspaceInfo, WorkspaceInfoError>;
  } else {
    switch (workspaceHandle) {
      case 1:
        return {
          ok: true,
          value: {
            client: 42,
            id: '1',
            currentName: 'Trademeet',
            currentSelfRole: WorkspaceRole.Owner,
            mountpoints: [[1, '/home/a']],
            handle: workspaceHandle,
          },
        };
      case 2:
        return {
          ok: true,
          value: {
            client: 42,
            id: '2',
            currentName: 'The Copper Coronet',
            currentSelfRole: WorkspaceRole.Manager,
            mountpoints: [[1, '/home/b']],
            handle: workspaceHandle,
          },
        };
      case 3:
        return {
          ok: true,
          value: {
            client: 42,
            id: '3',
            currentName: "Watcher's Keep",
            currentSelfRole: WorkspaceRole.Reader,
            mountpoints: [[1, '/home/c']],
            handle: workspaceHandle,
          },
        };
      default:
        return { ok: false, error: { tag: WorkspaceInfoErrorTag.Internal, error: 'internal' } };
    }
  }
}

export async function createWorkspace(name: WorkspaceName): Promise<Result<WorkspaceID, ClientCreateWorkspaceError>> {
  const handle = getParsecHandle();

  if (handle !== null && !needsMocks()) {
    return await libparsec.clientCreateWorkspace(handle, name);
  } else {
    return { ok: true, value: '1337' };
  }
}

export async function getWorkspaceSharing(
  workspaceId: WorkspaceID,
  includeAllUsers = false,
  includeSelf = false,
): Promise<Result<Array<[UserTuple, WorkspaceRole | null]>, ClientListWorkspaceUsersError>> {
  const handle = getParsecHandle();

  if (handle !== null && !needsMocks()) {
    let selfId: UserID | null = null;

    if (!includeSelf) {
      const clientResult = await getClientInfo();
      if (clientResult.ok) {
        selfId = clientResult.value.userId;
      }
    }

    const result = await libparsec.clientListWorkspaceUsers(handle, workspaceId);
    if (result.ok) {
      const value: Array<[UserTuple, WorkspaceRole | null]> = [];

      for (const sharing of result.value) {
        if (includeSelf || (!includeSelf && selfId !== sharing.userId)) {
          value.push([
            {
              id: sharing.userId,
              humanHandle: sharing.humanHandle,
              profile: sharing.currentProfile,
            },
            sharing.currentRole,
          ]);
        }
      }
      if (includeAllUsers) {
        const usersResult = await libparsec.clientListUsers(handle, true);
        if (usersResult.ok) {
          for (const user of usersResult.value) {
            if (!value.find((item) => item[0].id === user.id) && (includeSelf || (!includeSelf && user.id !== selfId))) {
              value.push([
                {
                  id: user.id,
                  humanHandle: user.humanHandle,
                  profile: user.currentProfile,
                },
                null,
              ]);
            }
          }
        }
      }
      return { ok: true, value: value };
    }
    return { ok: false, error: result.error };
  } else {
    const value: Array<[UserTuple, WorkspaceRole | null]> = [
      [
        {
          id: 'id1',
          // cspell:disable-next-line
          humanHandle: { label: 'Korgan Bloodaxe', email: 'korgan@gmail.com' },
          profile: UserProfile.Standard,
        },
        WorkspaceRole.Reader,
      ],
      [
        {
          id: 'id2',
          // cspell:disable-next-line
          humanHandle: { label: 'Cernd', email: 'cernd@gmail.com' },
          profile: UserProfile.Admin,
        },
        WorkspaceRole.Contributor,
      ],
    ];

    if (includeSelf) {
      value.push([
        {
          id: 'me',
          humanHandle: { email: 'user@host.com', label: 'Gordon Freeman' },
          profile: UserProfile.Admin,
        },
        WorkspaceRole.Owner,
      ]);
    }

    if (includeAllUsers) {
      value.push([
        {
          id: 'id3',
          // cspell:disable-next-line
          humanHandle: { label: 'Jaheira', email: 'jaheira@gmail.com' },
          profile: UserProfile.Outsider,
        },
        null,
      ]);
    }

    return { ok: true, value: value };
  }
}

export async function shareWorkspace(
  workspaceId: WorkspaceID,
  userId: UserID,
  role: WorkspaceRole | null,
): Promise<Result<null, ClientShareWorkspaceError>> {
  const handle = getParsecHandle();

  if (handle !== null && !needsMocks()) {
    return await libparsec.clientShareWorkspace(handle, workspaceId, userId, role);
  } else {
    return { ok: true, value: null };
  }
}

export async function startWorkspace(
  workspaceId: WorkspaceID,
  connectionHandle: ConnectionHandle | null = null,
): Promise<Result<WorkspaceHandle, ClientStartWorkspaceError>> {
  if (!connectionHandle) {
    connectionHandle = getParsecHandle();
  }

  if (connectionHandle !== null && !needsMocks()) {
    return await libparsec.clientStartWorkspace(connectionHandle, workspaceId);
  } else {
    return { ok: true, value: 1337 };
  }
}

export async function stopWorkspace(workspaceHandle: WorkspaceHandle): Promise<Result<null, WorkspaceStopError>> {
  const handle = getParsecHandle();

  if (handle !== null && !needsMocks()) {
    return await libparsec.workspaceStop(workspaceHandle);
  } else {
    return { ok: true, value: null };
  }
}

export async function mountWorkspace(
  workspaceHandle: WorkspaceHandle,
): Promise<Result<[MountpointHandle, SystemPath], WorkspaceMountError>> {
  if (!needsMocks) {
    return await libparsec.workspaceMount(workspaceHandle);
  }
  return { ok: true, value: [42, '/home/a'] };
}

export async function getPathLink(
  _workspaceHandle: WorkspaceHandle,
  path: string,
  timestamp: DateTime | null = null,
): Promise<Result<ParsecOrganizationFileLinkAddr, LinkError>> {
  const handle = getParsecHandle();

  const org = 'Org';
  // cspell:disable-next-line
  const workspaceId = '94a350f2f629403db2269c44583f7aa1';
  // cspell:disable-next-line
  path = 'MZDXYYNVT5QF27JMZQOOPEPDATV4R4FQHRZ762CTNRNAJHJO3DV3IACWLABY7EA6DC3BNGXTALKSQAQDDDBAssss';
  let link = `parsec3://parsec.cloud/${org}?action=file_link&workspace_id=${workspaceId}&path=${path}`;
  if (timestamp !== null) {
    // cspell:disable-next-line
    link += '&timestamp=JEFHNUJEF39350JFHNsss';
  }
  // Both are mocked for now
  if (handle !== null && !needsMocks()) {
    return { ok: true, value: link };
  } else {
    return { ok: true, value: link };
  }
}

export interface SharedWithInfo {
  workspace: WorkspaceInfo;
  user: UserID;
  userRole: WorkspaceRole;
}

export async function getWorkspacesSharedWith(user: UserID): Promise<Result<Array<SharedWithInfo>, ClientListWorkspacesError>> {
  const result = await listWorkspaces();

  if (!result.ok) {
    return result;
  }
  const retValue: Array<SharedWithInfo> = [];

  for (const workspace of result.value) {
    const sharing = workspace.sharing.find((item) => item[0].id === user);

    if (sharing && sharing[1]) {
      retValue.push({
        workspace: workspace,
        user: user,
        userRole: sharing[1],
      });
    }
  }

  return {
    ok: true,
    value: retValue,
  };
}
