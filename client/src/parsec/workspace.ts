// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { DataCache } from '@/common/cache';
import { getClientInfo } from '@/parsec/login';
import {
  ClientCreateWorkspaceError,
  ClientListWorkspaceUsersError,
  ClientListWorkspacesError,
  ClientRenameWorkspaceError,
  ClientShareWorkspaceError,
  ClientStartWorkspaceError,
  ConnectionHandle,
  FsPath,
  MountpointHandle,
  MountpointToOsPathError,
  MountpointToOsPathErrorTag,
  ParsecWorkspacePathAddr,
  Result,
  StartedWorkspaceInfo,
  SystemPath,
  UserID,
  UserTuple,
  WorkspaceDecryptPathAddrError,
  WorkspaceGeneratePathAddrError,
  WorkspaceHandle,
  WorkspaceID,
  WorkspaceInfo,
  WorkspaceInfoError,
  WorkspaceMountError,
  WorkspaceMountErrorTag,
  WorkspaceName,
  WorkspaceRole,
} from '@/parsec/types';
import { generateNoHandleError } from '@/parsec/utils';
import { MountpointUnmountError, WorkspaceStopError, libparsec } from '@/plugins/libparsec';
import { MountpointUnmountError, WorkspaceStopError, libparsec } from '@/plugins/libparsec';
import { getConnectionHandle } from '@/router';
import { DateTime } from 'luxon';

export async function initializeWorkspace(
  workspaceId: WorkspaceID,
  connectionHandle: ConnectionHandle | null = null,
): Promise<Result<StartedWorkspaceInfo, WorkspaceInfoError | ClientStartWorkspaceError>> {
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
  return startedWorkspaceResult;
}

export async function listWorkspaces(
  handle: ConnectionHandle | null = null,
): Promise<Result<Array<WorkspaceInfo>, ClientListWorkspacesError>> {
  if (!handle) {
    handle = getConnectionHandle();
  }

  if (handle !== null) {
    const result = await libparsec.clientListWorkspaces(handle);

    if (result.ok) {
      const returnValue: Array<WorkspaceInfo> = [];
      for (const wkInfo of result.value) {
        const startResult = await startWorkspace(wkInfo.id, handle);
        if (!startResult.ok) {
          console.error(`Failed to start workspace ${wkInfo.currentName}: ${startResult.error}`);
          continue;
        }

        const info: WorkspaceInfo = {
          id: wkInfo.id,
          currentName: wkInfo.currentName,
          currentSelfRole: wkInfo.currentSelfRole,
          isStarted: true,
          isBootstrapped: wkInfo.isBootstrapped,
          sharing: [],
          size: 0,
          lastUpdated: DateTime.now(),
          availableOffline: true,
          mountpoints: [],
          handle: startResult.value,
        };
        returnValue.push(info);
      }
      return { ok: true, value: returnValue };
    } else {
      return result;
    }
  }
  return generateNoHandleError<ClientListWorkspacesError>();
}

export async function getWorkspaceInfo(workspaceHandle: WorkspaceHandle): Promise<Result<StartedWorkspaceInfo, WorkspaceInfoError>> {
  const result = await libparsec.workspaceInfo(workspaceHandle);
  if (result.ok) {
    (result.value as StartedWorkspaceInfo).handle = workspaceHandle;
  }
  return result as Result<StartedWorkspaceInfo, WorkspaceInfoError>;
}

const WORKSPACE_NAMES_CACHE = new DataCache<WorkspaceHandle, string>();

export async function getWorkspaceName(workspaceHandle: WorkspaceHandle, ignoreCache?: boolean): Promise<string> {
  if (!ignoreCache) {
    const name = WORKSPACE_NAMES_CACHE.get(workspaceHandle);

    if (name) {
      return name;
    }
  }

  const result = await getWorkspaceInfo(workspaceHandle);
  if (!result.ok) {
    return '';
  }
  WORKSPACE_NAMES_CACHE.set(workspaceHandle, result.value.currentName);
  return result.value.currentName;
}

export async function createWorkspace(name: WorkspaceName): Promise<Result<WorkspaceID, ClientCreateWorkspaceError>> {
  const handle = getConnectionHandle();

  if (handle !== null) {
    return await libparsec.clientCreateWorkspace(handle, name);
  }
  return generateNoHandleError<ClientCreateWorkspaceError>();
}

export async function renameWorkspace(newName: WorkspaceName, id: WorkspaceID): Promise<Result<null, ClientRenameWorkspaceError>> {
  const handle = getConnectionHandle();

  if (handle !== null) {
    return await libparsec.clientRenameWorkspace(handle, id, newName);
  }
  return generateNoHandleError<ClientRenameWorkspaceError>();
}

export async function getWorkspaceSharing(
  workspaceId: WorkspaceID,
  includeAllUsers = false,
  includeSelf = false,
): Promise<Result<Array<[UserTuple, WorkspaceRole | null]>, ClientListWorkspaceUsersError>> {
  const handle = getConnectionHandle();

  if (handle !== null) {
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
    return result;
  }
  return generateNoHandleError<ClientListWorkspaceUsersError>();
}

export async function shareWorkspace(
  workspaceId: WorkspaceID,
  userId: UserID,
  role: WorkspaceRole | null,
): Promise<Result<null, ClientShareWorkspaceError>> {
  const handle = getConnectionHandle();

  if (handle !== null) {
    return await libparsec.clientShareWorkspace(handle, workspaceId, userId, role);
  }
  return generateNoHandleError<ClientShareWorkspaceError>();
}

export async function startWorkspace(
  workspaceId: WorkspaceID,
  connectionHandle: ConnectionHandle | null = null,
): Promise<Result<WorkspaceHandle, ClientStartWorkspaceError>> {
  if (!connectionHandle) {
    connectionHandle = getConnectionHandle();
  }

  if (connectionHandle !== null) {
    return await libparsec.clientStartWorkspace(connectionHandle, workspaceId);
  }
  return generateNoHandleError<ClientStartWorkspaceError>();
}

export async function stopWorkspace(workspaceHandle: WorkspaceHandle): Promise<Result<null, WorkspaceStopError>> {
  const handle = getConnectionHandle();

  if (handle !== null) {
    return await libparsec.workspaceStop(workspaceHandle);
  }
  return generateNoHandleError<WorkspaceStopError>();
}

export async function mountWorkspace(
  workspaceHandle: WorkspaceHandle,
): Promise<Result<[MountpointHandle, SystemPath], WorkspaceMountError>> {
  const startedWorkspaceResult = await getWorkspaceInfo(workspaceHandle);
  if (!startedWorkspaceResult.ok) {
    console.error(`Failed to get started workspace info: ${startedWorkspaceResult.error}`);
    return { ok: false, error: { tag: WorkspaceMountErrorTag.Internal, error: startedWorkspaceResult.error.error } };
  } else {
    if (startedWorkspaceResult.value.mountpoints.length > 0) {
      return { ok: true, value: startedWorkspaceResult.value.mountpoints[0] };
    }
  }
  return await libparsec.workspaceMount(workspaceHandle);
}

export async function unmountWorkspace(workspace: WorkspaceInfo | StartedWorkspaceInfo): Promise<Result<null, MountpointUnmountError>> {
  let error: MountpointUnmountError | null = null;

  for (let i = workspace.mountpoints.length - 1; i >= 0; i--) {
    const result = await libparsec.mountpointUnmount(workspace.mountpoints[i][0]);

    if (result.ok) {
      workspace.mountpoints.splice(i, 1);
    } else {
      error = result.error;
    }
  }

  if (error) {
    return generateNoHandleError<MountpointUnmountError>();
  }

  return { ok: true, value: null };
}

export async function getPathLink(
  workspaceHandle: WorkspaceHandle,
  path: string,
  timestamp: DateTime | null = null,
): Promise<Result<ParsecWorkspacePathAddr, WorkspaceGeneratePathAddrError>> {
  if (timestamp) {
    window.electronAPI.log('warn', 'Parameter `timestamp` is ignored');
  }
  return await libparsec.workspaceGeneratePathAddr(workspaceHandle, path);
}

export async function decryptFileLink(
  workspaceHandle: WorkspaceHandle,
  link: ParsecWorkspacePathAddr,
): Promise<Result<FsPath, WorkspaceDecryptPathAddrError>> {
  return await libparsec.workspaceDecryptPathAddr(workspaceHandle, link);
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
    const sharingResult = await getWorkspaceSharing(workspace.id, false, false);
    if (!sharingResult.ok) {
      continue;
    }
    const sharing = sharingResult.value.find((item) => item[0].id === user);

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

export async function getSystemPath(
  workspaceHandle: WorkspaceHandle,
  entryPath: FsPath,
): Promise<Result<SystemPath, MountpointToOsPathError>> {
  const infoResult = await getWorkspaceInfo(workspaceHandle);

  if (!infoResult.ok) {
    return { ok: false, error: { tag: MountpointToOsPathErrorTag.Internal, error: 'internal' } };
  }
  if (infoResult.value.mountpoints.length === 0) {
    return { ok: false, error: { tag: MountpointToOsPathErrorTag.Internal, error: 'not mounted' } };
  }
  return await libparsec.mountpointToOsPath(infoResult.value.mountpoints[0][0], entryPath);
}
