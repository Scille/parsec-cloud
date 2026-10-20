// Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS

import {
  getOrganizationInfo,
  getUserInfoFromDeviceID,
  getWorkspaceInfo,
  getWorkspaceSharing,
  listUserDevices,
  listUsers,
  RealmArchivingConfigurationTag,
  WorkspaceHandle,
} from '@/parsec';
import { DateTime } from 'luxon';

export interface DeviceExport {
  id: string;
  label: string;
  purpose: string;
  createdOn: string;
}

export interface UserExport {
  name: string;
  email: string;
  profile: string;
  createdOn: string;
  revokedOn?: string;
  revoked: boolean;
  invitedBy?: {
    name: string;
    email: string;
    id: string;
    deviceId: string;
  };
  revokedBy?: {
    name: string;
    email: string;
    id: string;
    deviceId: string;
  };
  devices: Array<DeviceExport>;
}

export interface WorkspaceUserExport {
  name: string;
  email: string;
  id: string;
  role: string;
  profile: string;
}

export interface WorkspaceExport {
  name: string;
  id: string;
  archived: boolean;
  deletionOn?: string;
  users: Array<WorkspaceUserExport>;
}

export interface ExportHeader {
  organization: string;
  serverAddr: string;
  timestamp: string;
  exportedBy: {
    name: string;
    email: string;
    id: string;
    deviceId: string;
  };
}

interface ISerializer {
  (obj: object): string;
}

const JSONSerializer: ISerializer = function (obj: object) {
  return JSON.stringify(obj, null, 2);
};

async function getExportHeader(): Promise<ExportHeader | undefined> {
  const orgInfoResult = await getOrganizationInfo();

  if (!orgInfoResult.ok) {
    return;
  }
  return {
    organization: orgInfoResult.value.clientInfo.organizationId,
    serverAddr: orgInfoResult.value.clientInfo.organizationAddr,
    timestamp: DateTime.utc().toISO(),
    exportedBy: {
      name: orgInfoResult.value.clientInfo.humanHandle.label,
      email: orgInfoResult.value.clientInfo.humanHandle.email,
      id: orgInfoResult.value.clientInfo.userId,
      deviceId: orgInfoResult.value.clientInfo.deviceId,
    },
  };
}

async function downloadExport(data: string, filename: string): Promise<void> {
  const blob = new Blob([data], { type: 'application/json' });
  const url = URL.createObjectURL(blob);
  const a = document.createElement('a');

  a.href = url;
  a.download = filename;
  document.body.appendChild(a);
  a.click();
  a.remove();
  URL.revokeObjectURL(url);
}

export async function exportAllUsers(serializer: ISerializer = JSONSerializer): Promise<boolean> {
  try {
    const header = await getExportHeader();

    if (!header) {
      return false;
    }
    const usersResult = await listUsers(false);
    if (!usersResult.ok) {
      return false;
    }
    const users: Array<UserExport> = [];
    for (const userInfo of usersResult.value) {
      const user: UserExport = {
        name: userInfo.humanHandle.label,
        email: userInfo.humanHandle.email,
        profile: userInfo.currentProfile,
        createdOn: userInfo.createdOn.toUTC().toISO() || '',
        revokedOn: userInfo.revokedOn ? (userInfo.revokedOn.toUTC().toISO() ?? undefined) : undefined,
        revoked: userInfo.isRevoked(),
        devices: [],
      };
      if (userInfo.createdBy) {
        const inviterResult = await getUserInfoFromDeviceID(userInfo.createdBy);
        if (inviterResult.ok) {
          user.invitedBy = {
            name: inviterResult.value.humanHandle.label,
            email: inviterResult.value.humanHandle.email,
            id: inviterResult.value.id,
            deviceId: userInfo.createdBy,
          };
        }
      }
      if (userInfo.revokedBy) {
        const revokerResult = await getUserInfoFromDeviceID(userInfo.revokedBy);
        if (revokerResult.ok) {
          user.revokedBy = {
            name: revokerResult.value.humanHandle.label,
            email: revokerResult.value.humanHandle.email,
            id: revokerResult.value.id,
            deviceId: userInfo.revokedBy,
          };
        }
      }
      const devicesResult = await listUserDevices(userInfo.id);
      if (devicesResult.ok) {
        user.devices = devicesResult.value.map((device) => {
          return {
            id: device.id,
            label: device.deviceLabel,
            purpose: device.purpose,
            createdOn: device.createdOn.toUTC().toISO() ?? '',
          };
        });
      }
      users.push(user);
    }
    await downloadExport(serializer({ export: header, users: users }), `Export_${header.organization}_Users_${header.timestamp}.json`);
    return true;
  } catch (err: any) {
    window.nativeAPI.log('error', `Failed to export: ${err.toString()}`);
    return false;
  }
}

export async function exportWorkspace(handle: WorkspaceHandle, serializer: ISerializer = JSONSerializer): Promise<boolean> {
  try {
    const header = await getExportHeader();

    if (!header) {
      return false;
    }

    const workspaceResult = await getWorkspaceInfo(handle);
    if (!workspaceResult.ok) {
      return false;
    }

    const workspace: WorkspaceExport = {
      name: workspaceResult.value.name,
      id: workspaceResult.value.id,
      archived: workspaceResult.value.archivingConfiguration.tag === RealmArchivingConfigurationTag.Archived,
      users: [],
    };
    if (workspaceResult.value.archivingConfiguration.tag === RealmArchivingConfigurationTag.DeletionPlanned) {
      const deletionDate = DateTime.fromSeconds(workspaceResult.value.archivingConfiguration.deletionDate as any as number);
      workspace.deletionOn = deletionDate.toUTC().toISO() ?? undefined;
    }
    const sharingResult = await getWorkspaceSharing(workspaceResult.value.id, false, true);
    if (!sharingResult.ok) {
      return false;
    }
    workspace.users = sharingResult.value.map(([user, role]) => {
      return {
        name: user.humanHandle.label,
        email: user.humanHandle.email,
        id: user.id,
        role: role || '',
        profile: user.profile,
      };
    });
    await downloadExport(
      serializer({ export: header, workspace: workspace }),
      `Export_${header.organization}_Workspace_${workspace.name}_${header.timestamp}.json`,
    );
    return true;
  } catch (err: any) {
    window.nativeAPI.log('error', `Failed to export: ${err.toString()}`);
    return false;
  }
}
