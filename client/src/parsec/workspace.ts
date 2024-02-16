// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { needsMocks } from '@/parsec/environment';
import { getClientInfo } from '@/parsec/login';
import { getParsecHandle } from '@/parsec/routing';
import {
  BackendOrganizationFileLinkAddr,
  ClientCreateWorkspaceError,
  ClientListWorkspacesError,
  ClientListWorkspaceUsersError,
  ClientShareWorkspaceError,
  ClientStartWorkspaceError,
  GetWorkspaceNameError,
  GetWorkspaceNameErrorTag,
  LinkError,
  Result,
  UserID,
  UserProfile,
  UserTuple,
  WorkspaceHandle,
  WorkspaceID,
  WorkspaceInfo,
  WorkspaceName,
  WorkspaceRole,
} from '@/parsec/types';
import { libparsec, WorkspaceStopError } from '@/plugins/libparsec';
import { DateTime } from 'luxon';

export async function listWorkspaces(): Promise<Result<Array<WorkspaceInfo>, ClientListWorkspacesError>> {
  const handle = getParsecHandle();

  if (handle !== null && !needsMocks()) {
    const result = await libparsec.clientListWorkspaces(handle);

    if (result.ok) {
      const returnValue: Array<WorkspaceInfo> = [];
      for (let i = 0; i < result.value.length; i++) {
        const sharingResult = await getWorkspaceSharing(result.value[i].id, false);
        const info: WorkspaceInfo = {
          id: result.value[i].id,
          name: result.value[i].name,
          selfCurrentRole: result.value[i].selfCurrentRole,
          isStarted: result.value[i].isStarted,
          isBootstrapped: result.value[i].isBootstrapped,
          sharing: [],
          size: 0,
          lastUpdated: DateTime.now(),
          availableOffline: false,
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
        name: 'Trademeet',
        selfCurrentRole: WorkspaceRole.Owner,
        size: 934_583,
        lastUpdated: DateTime.now().minus(2000),
        availableOffline: false,
        isStarted: false,
        isBootstrapped: false,
        sharing: [],
      },
      {
        id: '2',
        name: 'The Copper Coronet',
        selfCurrentRole: WorkspaceRole.Manager,
        size: 3_489_534_274,
        lastUpdated: DateTime.now(),
        availableOffline: false,
        isStarted: false,
        isBootstrapped: true,
        sharing: [],
      },
      {
        id: '3',
        name: "Watcher's Keep",
        selfCurrentRole: WorkspaceRole.Reader,
        size: 56_153_023,
        lastUpdated: DateTime.now(),
        availableOffline: true,
        isStarted: false,
        isBootstrapped: true,
        sharing: [],
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

export async function getWorkspaceRole(workspaceId: WorkspaceID): Promise<WorkspaceRole> {
  const result = await listWorkspaces();

  if (result.ok) {
    const workspaceInfo = result.value.find((wi) => wi.id === workspaceId);
    if (workspaceInfo) {
      return workspaceInfo.selfCurrentRole;
    }
  }

  // Role with lowest permissions by default
  return WorkspaceRole.Reader;
}

export async function createWorkspace(name: WorkspaceName): Promise<Result<WorkspaceID, ClientCreateWorkspaceError>> {
  const handle = getParsecHandle();

  if (handle !== null && !needsMocks()) {
    return await libparsec.clientCreateWorkspace(handle, name);
  } else {
    return { ok: true, value: '1337' };
  }
}

export async function getWorkspaceName(workspaceId: WorkspaceID): Promise<Result<WorkspaceName, GetWorkspaceNameError>> {
  const handle = getParsecHandle();

  if (handle !== null && !needsMocks()) {
    const result = await libparsec.clientListWorkspaces(handle);
    if (result.ok) {
      const workspace = result.value.find((info) => {
        if (info.id === workspaceId) {
          return true;
        }
        return false;
      });
      if (workspace) {
        return { ok: true, value: workspace.name };
      }
    }
    return { ok: false, error: { tag: GetWorkspaceNameErrorTag.NotFound } };
  } else {
    if (workspaceId === '1') {
      return { ok: true, value: 'Trademeet' };
    } else if (workspaceId === '2') {
      return { ok: true, value: 'The Copper Coronet' };
    } else if (workspaceId === '3') {
      return { ok: true, value: "Watcher's Keep" };
    } else {
      return { ok: true, value: 'My Workspace' };
    }
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

export async function startWorkspace(workspaceId: WorkspaceID): Promise<Result<WorkspaceHandle, ClientStartWorkspaceError>> {
  const handle = getParsecHandle();

  if (handle !== null && !needsMocks()) {
    return await libparsec.clientStartWorkspace(handle, workspaceId);
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

export async function getPathLink(
  workspaceId: WorkspaceID,
  path: string,
  timestamp: DateTime | null = null,
): Promise<Result<BackendOrganizationFileLinkAddr, LinkError>> {
  const handle = getParsecHandle();

  const org = 'Org';
  // cspell:disable-next-line
  workspaceId = '94a350f2f629403db2269c44583f7aa1';
  // cspell:disable-next-line
  path = 'MZDXYYNVT5QF27JMZQOOPEPDATV4R4FQHRZ762CTNRNAJHJO3DV3IACWLABY7EA6DC3BNGXTALKSQAQDDDBAssss';
  let link = `parsec://parsec.cloud/${org}?action=file_link&workspace_id=${workspaceId}&path=${path}`;
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
